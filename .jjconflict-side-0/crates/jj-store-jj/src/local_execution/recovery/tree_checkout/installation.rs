//! Recoverable physical file moves; native working-copy metadata stays original.
mod files;
mod progress;
mod storage;
mod validation;
use super::*;
use crate::local_browser::working_tree::current::physical;
use files::{Snapshot, directory_stamp, parent};
use progress::{Intent, Progress};
use rustix::fs::{self, RenameFlags};
use storage::Destination;
/// Only the provider constructs this after exact full namespace validation.
/// It does not prove native checkout completion or command acknowledgement.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct InstalledTreeFiles {
    plan: m::RecordDigest,
    inventory: m::RecordDigest,
    after: m::RecordDigest,
}
impl InstalledTreeFiles {
    pub fn plan(&self) -> m::RecordDigest {
        self.plan
    }
    pub fn inventory(&self) -> m::RecordDigest {
        self.inventory
    }
    pub fn physical(&self) -> m::RecordDigest {
        self.after
    }
}
impl RecoveredHistory {
    /// Trusted original-request composition supplies durably saved input. No
    /// transport can turn client-decoded input into a publication proof.
    pub async fn install_tree_files(
        self,
        published: &PublishedHead,
        plan: TreeCheckoutPlanV1,
        inventory: PhysicalInventoryV1,
        physical: Arc<dyn PhysicalGuard>,
    ) -> Result<(Self, InstalledTreeFiles)> {
        if !self.publication
            || self.checkout.is_some()
            || published.preparation() != &self.key
            || published.witness() != self.witness.id()?
            || published.operation != self.archive.operation
            || published.view != self.retained.view
            || plan.preparation() != &self.key
            || plan.recovery() != self.witness.id()?
            || plan.candidate() != (published.operation, published.view)
            || plan.retained() != (self.retained.commit, self.retained.archive)
        {
            return Err(Error::Invalid);
        }
        plan.encode()?;
        inventory.check_selection(&self.witness.binding(), plan.physical())?;
        let cancellation = tokio_util::sync::CancellationToken::new();
        let cancel = cancellation.clone().drop_guard();
        let runtime = tokio::runtime::Handle::current();
        let result = tokio::task::spawn_blocking(move || {
            runtime.block_on(tokio::task::LocalSet::new().run_until(async {
                let mut budget = recovery_budget();
                budget.cancellation = cancellation;
                let proof = install(&self, &plan, &inventory, &physical, &budget).await?;
                Ok((self, proof))
            }))
        })
        .await
        .map_err(|_| Error::Unavailable)?;
        cancel.disarm();
        result
    }
}
async fn install(
    recovery: &RecoveredHistory,
    plan: &TreeCheckoutPlanV1,
    inventory: &PhysicalInventoryV1,
    guard: &Arc<dyn PhysicalGuard>,
    budget: &Budget,
) -> Result<InstalledTreeFiles> {
    recovery.check(budget).await?;
    validation::effects(recovery, plan, budget)?;
    let head_lock = publication::Destination::open(recovery, budget)?;
    let heads = witness::heads(recovery.access.metadata_path(), budget)?;
    if !publication::ancestry::selected(recovery, &heads, budget).await? {
        return Err(Error::EpochChanged);
    }
    records::validate(recovery, budget).await?;
    let extras = extras::validate(recovery, budget)?;
    let indexes = indexes::validate(recovery, budget).await?;
    let mut destination = Destination::open(recovery, plan, budget)?;
    let mut state = match destination.load(budget)? {
        Some(state) => {
            state.validate(plan, inventory, &destination)?;
            state
        }
        None => {
            check(recovery, guard, budget)
                .await?
                .check_selection(&recovery.witness.binding(), plan.physical())?;
            observation::validate(&recovery.root, plan, budget)?;
            let state = Progress::prepare(recovery, plan, inventory, &destination, budget)?;
            check(recovery, guard, budget)
                .await?
                .check_selection(&recovery.witness.binding(), plan.physical())?;
            destination.save(&state, budget)?;
            state
        }
    };
    loop {
        recovery.check(budget).await?;
        head_lock.check()?;
        destination.check(recovery)?;
        destination.acknowledge(budget)?;
        extras.check(budget)?;
        indexes.check(budget)?;
        state.validate(plan, inventory, &destination)?;
        // A saved intent admits only one exact no-replace transition. Resolve it
        // before scanning, so a crash after rename cannot excuse unrelated edits.
        let moved = validation::pending(recovery, plan, &state, &destination, budget)?;
        let mut observed = state.clone();
        if moved {
            observed.advance(plan)?;
        }
        let next = if moved {
            Some(observed.clone())
        } else if state.intent.is_none() {
            state.next(plan).map(|intent| {
                let mut next = state.clone();
                next.intent = Some(intent);
                next
            })
        } else {
            None
        };
        destination.verify_next(next.as_ref(), budget)?;
        let current = check(recovery, guard, budget).await?;
        validation::namespace(
            recovery,
            plan,
            inventory,
            &observed,
            &destination,
            &current,
            budget,
        )?;
        if moved {
            state = observed;
            destination.save(&state, budget)?;
            continue;
        }
        if state.intent.is_none() {
            let Some(intent) = state.next(plan) else {
                destination.acknowledge(budget)?;
                return Ok(InstalledTreeFiles {
                    plan: plan.id()?,
                    inventory: inventory.id()?,
                    after: current.fingerprint(),
                });
            };
            state.intent = Some(intent);
            destination.save(&state, budget)?;
            continue;
        }
        recovery.check(budget).await?;
        destination.check(recovery)?;
        head_lock.check()?;
        destination.acknowledge(budget)?;
        perform(recovery, plan, &state, &destination, budget)?;
        // Persisted intent remains until full observed validation on the next
        // pass. Failed post-checks never forge a receipt or erase backups.
    }
}
async fn check(
    recovery: &RecoveredHistory,
    guard: &Arc<dyn PhysicalGuard>,
    budget: &Budget,
) -> Result<PhysicalInventoryV1> {
    recovery.check(budget).await?;
    let observed = budget
        .until(guard.observe(&recovery.witness.binding()))
        .await??;
    observed.check_selection(&recovery.witness.binding(), observed.fingerprint())?;
    recovery.check(budget).await?;
    Ok(observed)
}
fn perform(
    recovery: &RecoveredHistory,
    plan: &TreeCheckoutPlanV1,
    state: &Progress,
    destination: &Destination,
    budget: &Budget,
) -> Result<()> {
    let root = physical::directory(&recovery.root)?;
    let (from, source, to, target) = match state.intent.ok_or(Error::Invalid)? {
        Intent::Directory(i) => {
            let path = &plan.directories[i].path;
            let (to, name) =
                files::checked_parent(&root, path, plan, state)?.ok_or(Error::EpochChanged)?;
            (
                destination
                    .directory
                    .try_clone()
                    .map_err(|_| Error::Unavailable)?,
                format!("d-{i}"),
                to,
                name,
            )
        }
        Intent::Backup(i) => {
            let (from, name) = files::checked_parent(&root, &plan.changes[i].path, plan, state)?
                .ok_or(Error::EpochChanged)?;
            (
                from,
                name,
                destination
                    .directory
                    .try_clone()
                    .map_err(|_| Error::Unavailable)?,
                format!("b-{i}"),
            )
        }
        Intent::Install(i) => {
            let (to, name) = files::checked_parent(&root, &plan.changes[i].path, plan, state)?
                .ok_or(Error::EpochChanged)?;
            (
                destination
                    .directory
                    .try_clone()
                    .map_err(|_| Error::Unavailable)?,
                format!("f-{i}"),
                to,
                name,
            )
        }
    };
    budget.check()?;
    // Re-read exact operands immediately before selection. No checked target
    // is replaced; intervening editor creations fail atomically.
    if validation::pending(recovery, plan, state, destination, budget)? {
        return Err(Error::EpochChanged);
    }
    let file = match state.intent.ok_or(Error::Invalid)? {
        Intent::Backup(i) => Some(files::sync_file(
            &from,
            &source,
            state.originals[i].as_ref().ok_or(Error::Corrupt)?,
        )?),
        Intent::Install(i) => Some(files::sync_file(
            &from,
            &source,
            state.candidates[i].as_ref().ok_or(Error::Corrupt)?,
        )?),
        Intent::Directory(_) => None,
    };
    budget.check()?;
    fs::renameat_with(&from, &source, &to, &target, RenameFlags::NOREPLACE)
        .map_err(|_| Error::EpochChanged)?;
    from.sync_all().map_err(|_| Error::Unavailable)?;
    to.sync_all().map_err(|_| Error::Unavailable)?;
    if let Some(file) = file {
        file.sync_all().map_err(|_| Error::Unavailable)?;
    }
    Ok(())
}

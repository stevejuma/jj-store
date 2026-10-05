//! Exact native metadata selection after retained physical installation.
mod native;
mod storage;
use super::*;
use crate::local_browser::working_tree::current::{MAX_CHECKOUT_BYTES, capture, physical};
use installation::{
    files::{self, Snapshot},
    progress::Progress,
};
use storage::Destination;

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
struct State {
    version: u32,
    plan: m::RecordDigest,
    inventory: m::RecordDigest,
    physical: m::RecordDigest,
    private: [u64; 2],
    tree: Snapshot,
    checkout: Snapshot,
}
impl State {
    fn encode(&self) -> Result<Vec<u8>> {
        let bytes = serde_json::to_vec(self).map_err(|_| Error::Corrupt)?;
        if self.version != 1
            || bytes.len() > 65536
            || self.physical == m::RecordDigest::new([0; 32])
        {
            return Err(Error::Corrupt);
        }
        Ok(bytes)
    }
    fn id(&self) -> Result<m::RecordDigest> {
        let mut hash = Sha256::new_with_prefix(b"jj-store/native-tree-completion/v1\0");
        hash.update(self.encode()?);
        Ok(m::RecordDigest::new(hash.finalize().into()))
    }
}
pub(in crate::local_execution::recovery) struct Completion {
    plan: TreeCheckoutPlanV1,
    inventory: PhysicalInventoryV1,
    state: State,
}
/// Native and physical completion, still separate from combination selection
/// and the authenticated original command's durable acknowledgement.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct VerifiedTreeCheckout {
    pub operation: m::OperationId,
    pub view: m::ViewId,
    preparation: StackPreparationKey,
    plan: m::RecordDigest,
    inventory: m::RecordDigest,
    completion: m::RecordDigest,
    physical: m::RecordDigest,
}
impl VerifiedTreeCheckout {
    pub fn preparation(&self) -> &StackPreparationKey {
        &self.preparation
    }
    pub fn plan(&self) -> m::RecordDigest {
        self.plan
    }
    pub fn inventory(&self) -> m::RecordDigest {
        self.inventory
    }
    pub fn completion(&self) -> m::RecordDigest {
        self.completion
    }
    pub fn physical(&self) -> m::RecordDigest {
        self.physical
    }
}
fn inputs(
    recovery: &RecoveredHistory,
    plan: &TreeCheckoutPlanV1,
    inventory: &PhysicalInventoryV1,
) -> Result<()> {
    if !recovery.publication
        || recovery.checkout.is_some()
        || plan.preparation() != &recovery.key
        || plan.recovery() != recovery.witness.id()?
        || plan.retained() != (recovery.retained.commit, recovery.retained.archive)
        || plan.candidate() != (recovery.archive.operation, recovery.retained.view)
    {
        return Err(Error::Invalid);
    }
    plan.encode()?;
    inventory.check_selection(&recovery.witness.binding(), plan.physical())
}
fn installed(
    recovery: &RecoveredHistory,
    plan: &TreeCheckoutPlanV1,
    inventory: &PhysicalInventoryV1,
    budget: &Budget,
) -> Result<(installation::storage::Destination, Progress)> {
    inputs(recovery, plan, inventory)?;
    let destination = installation::storage::Destination::existing(recovery, plan, budget)?;
    let progress = destination.load(budget)?.ok_or(Error::Incomplete)?;
    progress.validate(plan, inventory, &destination)?;
    destination.verify_next(None, budget)?;
    if progress.intent.is_some()
        || progress.next(plan).is_some()
        || progress.phases.iter().any(|phase| *phase != 2)
    {
        return Err(Error::Incomplete);
    }
    destination.acknowledge(budget)?;
    Ok((destination, progress))
}
async fn physical_check(
    recovery: &RecoveredHistory,
    plan: &TreeCheckoutPlanV1,
    inventory: &PhysicalInventoryV1,
    physical: &Arc<dyn PhysicalGuard>,
    expected: m::RecordDigest,
    budget: &Budget,
) -> Result<()> {
    recovery.check(budget).await?;
    let (destination, progress) = installed(recovery, plan, inventory, budget)?;
    let current = budget
        .until(physical.observe(&recovery.witness.binding()))
        .await??;
    current.check_selection(&recovery.witness.binding(), expected)?;
    installation::validation::namespace(
        recovery,
        plan,
        inventory,
        &progress,
        &destination,
        &current,
        budget,
    )?;
    destination.check(recovery)?;
    destination.acknowledge(budget)?;
    recovery.check(budget).await
}
impl Completion {
    pub fn load(
        recovery: &RecoveredHistory,
        plan: TreeCheckoutPlanV1,
        inventory: PhysicalInventoryV1,
        budget: &Budget,
    ) -> Result<Self> {
        inputs(recovery, &plan, &inventory)?;
        installation::validation::effects(recovery, &plan, budget)?;
        let destination = Destination::open(recovery, &plan, false)?;
        let state = destination.load(budget)?.ok_or(Error::NotFound)?;
        let value = Self {
            plan,
            inventory,
            state,
        };
        destination.check(recovery, &value.state, budget)?;
        Ok(value)
    }
    pub async fn check(&self, recovery: &RecoveredHistory, budget: &Budget) -> Result<()> {
        inputs(recovery, &self.plan, &self.inventory)?;
        installed(recovery, &self.plan, &self.inventory, budget)?;
        let heads = witness::heads(recovery.access.metadata_path(), budget)?;
        if !publication::ancestry::selected(recovery, &heads, budget).await? {
            return Err(Error::EpochChanged);
        }
        recovery
            .witness
            .check_normalized(
                &recovery.access,
                &recovery.root,
                &heads,
                Some(witness::Normalization::Tree(self)),
                budget,
            )
            .await
    }
    pub fn normalize(
        &self,
        root: &std::path::Path,
        captured: &mut capture::CheckoutFiles,
        budget: &Budget,
    ) -> Result<()> {
        // The caller already checked retained input linkage and ancestry. Read
        // only exact existing provider progress; never create missing evidence.
        let working = physical::directory(&root.join(".jj/working_copy"))?;
        let native = physical::child(
            &working,
            &format!(".lapis-stack-native-{}", self.plan.id()?),
        )?;
        storage::check_frame(&native, &self.state, budget)?;
        let stage = physical::child(&working, &format!(".lapis-stack-tree-{}", self.plan.id()?))?;
        let data = physical::read(
            &stage,
            "progress",
            installation::progress::MAX_PROGRESS,
            false,
            budget,
        )?
        .ok_or(Error::Incomplete)?;
        let progress: Progress = serde_json::from_slice(&data.bytes).map_err(|_| Error::Corrupt)?;
        if progress.encode()? != data.bytes
            || progress.plan != self.state.plan
            || progress.inventory != self.state.inventory
        {
            return Err(Error::Corrupt);
        }
        let bytes = native::bytes(&self.plan, &progress)?;
        let selected =
            storage::selections(&working, &native, &self.state, &self.plan, &bytes, budget)?;
        if selected == [false, true] {
            return Err(Error::EpochChanged);
        }
        // Both metadata files were just re-read with full inode/content checks.
        // Captured input must be the same observation, not an intervening edit.
        for (name, saved) in [
            ("tree_state", &mut captured.tree),
            ("checkout", &mut captured.checkout),
        ] {
            if physical::read(&working, name, MAX_CHECKOUT_BYTES, false, budget)?.as_ref()
                != Some(saved)
            {
                return Err(Error::EpochChanged);
            }
        }
        captured.tree.fingerprint = self.plan.tree_fingerprint;
        captured.checkout.fingerprint = self.plan.checkout_fingerprint;
        Ok(())
    }
}
impl RecoveredHistory {
    pub async fn complete_tree_checkout(
        mut self,
        installed: InstalledTreeFiles,
        plan: TreeCheckoutPlanV1,
        inventory: PhysicalInventoryV1,
        physical: Arc<dyn PhysicalGuard>,
    ) -> Result<(Self, VerifiedTreeCheckout)> {
        inputs(&self, &plan, &inventory)?;
        if self.tree_checkout.is_some()
            || installed.plan() != plan.id()?
            || installed.inventory() != inventory.id()?
        {
            return Err(Error::Invalid);
        }
        let cancellation = tokio_util::sync::CancellationToken::new();
        let cancel = cancellation.clone().drop_guard();
        let runtime = tokio::runtime::Handle::current();
        let result = tokio::task::spawn_blocking(move || {
            runtime.block_on(tokio::task::LocalSet::new().run_until(async {
                let mut budget = recovery_budget();
                budget.cancellation = cancellation;
                physical_check(
                    &self,
                    &plan,
                    &inventory,
                    &physical,
                    installed.physical(),
                    &budget,
                )
                .await?;
                plan.check_metadata(&self, &budget)?;
                let head_lock = publication::Destination::open(&self, &budget)?;
                let (stage, progress) = installed_progress(&self, &plan, &inventory, &budget)?;
                let bytes = native::bytes(&plan, &progress)?;
                let destination = Destination::open(&self, &plan, true)?;
                let state = destination.prepare(
                    &plan,
                    &inventory,
                    installed.physical(),
                    &bytes,
                    &budget,
                )?;
                physical_check(
                    &self,
                    &plan,
                    &inventory,
                    &physical,
                    installed.physical(),
                    &budget,
                )
                .await?;
                plan.check_metadata(&self, &budget)?;
                head_lock.check()?;
                stage.check(&self)?;
                destination.save(&state, &budget)?;
                destination.check(&self, &state, &budget)?;
                self.tree_checkout = Some(Completion {
                    plan,
                    inventory,
                    state,
                });
                drop(head_lock);
                let proof = finish(&self, &physical, &budget).await?;
                Ok((self, proof))
            }))
        })
        .await
        .map_err(|_| Error::Unavailable)?;
        cancel.disarm();
        result
    }
    /// Resume only an existing provider frame loaded under original admission.
    pub async fn finish_tree_checkout(
        self,
        physical: Arc<dyn PhysicalGuard>,
    ) -> Result<(Self, VerifiedTreeCheckout)> {
        if self.tree_checkout.is_none() {
            return Err(Error::Invalid);
        }
        let cancellation = tokio_util::sync::CancellationToken::new();
        let cancel = cancellation.clone().drop_guard();
        let runtime = tokio::runtime::Handle::current();
        let result = tokio::task::spawn_blocking(move || {
            runtime.block_on(tokio::task::LocalSet::new().run_until(async {
                let mut budget = recovery_budget();
                budget.cancellation = cancellation;
                let proof = finish(&self, &physical, &budget).await?;
                Ok((self, proof))
            }))
        })
        .await
        .map_err(|_| Error::Unavailable)?;
        cancel.disarm();
        result
    }
}
// Distinct name avoids shadowing the sealed public physical proof argument.
use installed as installed_progress;
async fn finish(
    recovery: &RecoveredHistory,
    physical: &Arc<dyn PhysicalGuard>,
    budget: &Budget,
) -> Result<VerifiedTreeCheckout> {
    let completion = recovery.tree_checkout.as_ref().ok_or(Error::Invalid)?;
    let head_lock = publication::Destination::open(recovery, budget)?;
    records::validate(recovery, budget).await?;
    let extras = extras::validate(recovery, budget)?;
    let indexes = indexes::validate(recovery, budget).await?;
    let (stage, progress) = installed(recovery, &completion.plan, &completion.inventory, budget)?;
    let bytes = native::bytes(&completion.plan, &progress)?;
    let destination = Destination::open(recovery, &completion.plan, false)?;
    for index in 0..2 {
        physical_check(
            recovery,
            &completion.plan,
            &completion.inventory,
            physical,
            completion.state.physical,
            budget,
        )
        .await?;
        destination.check(recovery, &completion.state, budget)?;
        head_lock.check()?;
        extras.check(budget)?;
        indexes.check(budget)?;
        stage.check(recovery)?;
        destination.select(index, &completion.state, &completion.plan, &bytes, budget)?;
        physical_check(
            recovery,
            &completion.plan,
            &completion.inventory,
            physical,
            completion.state.physical,
            budget,
        )
        .await?;
    }
    destination.check(recovery, &completion.state, budget)?;
    head_lock.check()?;
    extras.check(budget)?;
    indexes.check(budget)?;
    if destination.selections(&completion.state, &completion.plan, &bytes, budget)? != [true, true]
    {
        return Err(Error::EpochChanged);
    }
    Ok(VerifiedTreeCheckout {
        operation: recovery.archive.operation,
        view: recovery.retained.view,
        preparation: recovery.key.clone(),
        plan: completion.state.plan,
        inventory: completion.state.inventory,
        completion: completion.state.id()?,
        physical: completion.state.physical,
    })
}

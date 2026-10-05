//! Exact head selection under the actual upstream head-store flock.
pub(super) mod ancestry;
use super::*;
use crate::local_browser::working_tree::current::physical;
use records::{destination::same, synced};
use std::fs::File;

/// Code publication only, never a physical checkout or command-success receipt.
/// Only this provider constructs the proof after synced exact head selection.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct PublishedHead {
    pub operation: m::OperationId,
    pub view: m::ViewId,
    preparation: StackPreparationKey,
    witness: m::RecordDigest,
}
impl PublishedHead {
    pub fn preparation(&self) -> &StackPreparationKey {
        &self.preparation
    }
    pub fn witness(&self) -> m::RecordDigest {
        self.witness
    }
}
pub(super) async fn check(recovery: &RecoveredHistory, budget: &Budget) -> Result<()> {
    let heads = witness::heads(recovery.access.metadata_path(), budget)?;
    ancestry::selected(recovery, &heads, budget).await?;
    recovery
        .witness
        .check_heads(&recovery.access, &recovery.root, &heads, budget)
        .await
}
pub(super) struct Destination {
    path: PathBuf,
    heads: File,
    lock: guard::Guard,
}
impl Destination {
    pub(super) fn open(recovery: &RecoveredHistory, budget: &Budget) -> Result<Self> {
        let path = recovery.access.metadata_path().join("op_heads");
        let parent = physical::directory(&path)?;
        if physical::read(&parent, "type", 64, false, budget)?
            .ok_or(Error::Incomplete)?
            .bytes
            != jj_lib::simple_op_heads_store::SimpleOpHeadsStore::name().as_bytes()
        {
            return Err(Error::Invalid);
        }
        let path = path.join("heads");
        let heads = physical::child(&parent, "heads")?;
        let lock =
            guard::Guard::acquire_in(heads.try_clone().map_err(|_| Error::Unavailable)?, "lock")?;
        let value = Self { path, heads, lock };
        value.check()?;
        Ok(value)
    }
    pub(super) fn check(&self) -> Result<()> {
        same(&self.heads, &physical::directory(&self.path)?)?;
        self.lock.check_directory(&self.heads)
    }
}
impl RecoveredHistory {
    /// Informative original-selection status, not permission to publish.
    pub async fn publication_selected(self) -> Result<(Self, bool)> {
        if !self.publication || self.checkout.is_some() {
            return Err(Error::Invalid);
        }
        let cancellation = tokio_util::sync::CancellationToken::new();
        let cancel = cancellation.clone().drop_guard();
        let runtime = tokio::runtime::Handle::current();
        let result = tokio::task::spawn_blocking(move || {
            runtime.block_on(tokio::task::LocalSet::new().run_until(async {
                let mut budget = recovery_budget();
                budget.cancellation = cancellation;
                self.check(&budget).await?;
                let heads = witness::heads(self.access.metadata_path(), &budget)?;
                let selected = ancestry::selected(&self, &heads, &budget).await?;
                self.check(&budget).await?;
                Ok((self, selected))
            }))
        })
        .await
        .map_err(|_| Error::Unavailable)?;
        cancel.disarm();
        result
    }
    /// Original publication acknowledgement only. A vanished candidate may not
    /// be created/reannounced when composition skips current mutation policy.
    pub async fn acknowledge_publication(self) -> Result<(Self, PublishedHead)> {
        self.select_publication(false).await
    }
    /// Publish the original retained candidate only. Interrupted selection is
    /// retryable through resume_publication, without re-preparing or checkout.
    pub async fn publish_head(self) -> Result<(Self, PublishedHead)> {
        self.select_publication(true).await
    }
    async fn select_publication(self, allow_new: bool) -> Result<(Self, PublishedHead)> {
        let cancellation = tokio_util::sync::CancellationToken::new();
        let cancel = cancellation.clone().drop_guard();
        let runtime = tokio::runtime::Handle::current();
        let result = tokio::task::spawn_blocking(move || {
            runtime.block_on(tokio::task::LocalSet::new().run_until(async {
                let mut budget = recovery_budget();
                budget.cancellation = cancellation;
                let proof = publish(&self, allow_new, &budget).await?;
                Ok((self, proof))
            }))
        })
        .await
        .map_err(|_| Error::Unavailable)?;
        cancel.disarm();
        result
    }
}
async fn publish(
    recovery: &RecoveredHistory,
    allow_new: bool,
    budget: &Budget,
) -> Result<PublishedHead> {
    if !recovery.publication || recovery.checkout.is_some() {
        return Err(Error::Invalid);
    }
    recovery.check(budget).await?;
    let destination = Destination::open(recovery, budget)?;
    recovery.check(budget).await?;
    let retained = recovery
        .witness
        .store()?
        .read_stack_prepared(&recovery.key)
        .map_err(evidence_error)?
        .ok_or(Error::NotFound)?;
    if retained.history.as_ref() != Some(&recovery.archive) {
        return Err(Error::EpochChanged);
    }
    // Repairs are explicit preceding operations. Publication never fills a
    // missing dependency or accepts a synthetic commit/index identity.
    records::validate(recovery, budget).await?;
    let extras = extras::validate(recovery, budget)?;
    let indexes = indexes::validate(recovery, budget).await?;
    let candidate = recovery.archive.operation;
    let source = recovery.witness.binding().operation;
    let mut heads = witness::heads(recovery.access.metadata_path(), budget)?;
    let published = ancestry::selected(recovery, &heads, budget).await?;
    if !published && !allow_new {
        return Err(Error::EpochChanged);
    }
    recovery.check(budget).await?;
    destination.check()?;
    extras.check(budget)?;
    indexes.check(budget)?;
    if !published {
        if heads.len() >= 64 {
            return Err(Error::Limit);
        }
        synced::select(&destination.heads, &candidate.to_string(), &[], budget)?;
        heads.push(candidate);
        heads.sort();
    } else if heads.contains(&candidate) {
        synced::acknowledge(&destination.heads, &candidate.to_string(), budget)?;
    }
    // Candidate is synced before old-head removal. Cancellation/revocation here
    // leaves the exact candidate selected alongside the untouched original.
    recovery
        .witness
        .check_heads(&recovery.access, &recovery.root, &heads, budget)
        .await?;
    recovery.guard.check(&recovery.root)?;
    destination.check()?;
    extras.check(budget)?;
    indexes.check(budget)?;
    if heads.contains(&source) {
        physical::read(&destination.heads, &source.to_string(), 0, false, budget)?
            .ok_or(Error::Incomplete)?;
        rustix::fs::unlinkat(
            &destination.heads,
            source.to_string(),
            rustix::fs::AtFlags::empty(),
        )
        .map_err(|_| Error::EpochChanged)?;
        destination
            .heads
            .sync_all()
            .map_err(|_| Error::Unavailable)?;
        heads.retain(|id| *id != source);
    }
    for id in &heads {
        synced::acknowledge(&destination.heads, &id.to_string(), budget)?;
    }
    destination
        .heads
        .sync_all()
        .map_err(|_| Error::Unavailable)?;
    recovery
        .witness
        .check_heads(&recovery.access, &recovery.root, &heads, budget)
        .await?;
    recovery.check(budget).await?;
    destination.check()?;
    extras.check(budget)?;
    indexes.check(budget)?;
    records::validate(recovery, budget).await?;
    if recovery
        .witness
        .store()?
        .read_stack_prepared(&recovery.key)
        .map_err(evidence_error)?
        != Some(retained)
    {
        return Err(Error::EpochChanged);
    }
    recovery.check(budget).await?;
    destination.check()?;
    Ok(PublishedHead {
        operation: candidate,
        view: recovery.archive.operations[&candidate].view_id,
        preparation: recovery.key.clone(),
        witness: recovery.witness.id()?,
    })
}

//! Metadata-only completion for exact unchanged-tree commands.
mod staging;
mod witness;
use super::*;
use crate::local_browser::working_tree::current::{capture, physical};
use records::{destination::same, synced};
pub use witness::CheckoutWitnessV1;

/// Trusted composition must validate the complete registered physical inventory
/// against the original prepared selection, including ignored/untracked entries.
/// This is not a client token or a permission inferred from checkout metadata.
#[async_trait::async_trait]
pub trait PhysicalGuard: Send + Sync {
    async fn validate(&self, binding: &RecoveryBinding, expected: m::RecordDigest) -> Result<()>;
    /// Optional original full inventory capture. Validation-only guards cannot
    /// silently substitute a digest or incomplete rows for recovery input.
    async fn inventory(
        &self,
        _binding: &RecoveryBinding,
        _expected: m::RecordDigest,
    ) -> Result<PhysicalInventoryV1> {
        Err(Error::Invalid)
    }
    /// Complete fresh data for exact provider-owned interrupted transitions.
    /// This does not normalize a changed tree or grant a write capability.
    async fn observe(&self, _binding: &RecoveryBinding) -> Result<PhysicalInventoryV1> {
        Err(Error::Invalid)
    }
}
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct VerifiedUnchangedCheckout {
    pub operation: m::OperationId,
    pub view: m::ViewId,
    preparation: StackPreparationKey,
    witness: m::RecordDigest,
    physical: m::RecordDigest,
}
impl VerifiedUnchangedCheckout {
    pub fn preparation(&self) -> &StackPreparationKey {
        &self.preparation
    }
    pub fn witness(&self) -> m::RecordDigest {
        self.witness
    }
    pub fn physical(&self) -> m::RecordDigest {
        self.physical
    }
}
pub(super) async fn check(
    recovery: &RecoveredHistory,
    checkout: &CheckoutWitnessV1,
    budget: &Budget,
) -> Result<()> {
    checkout.validate(recovery)?;
    let heads = super::witness::heads(recovery.access.metadata_path(), budget)?;
    if !publication::ancestry::selected(recovery, &heads, budget).await? {
        return Err(Error::EpochChanged);
    }
    let files =
        capture::CheckoutFiles::read(&recovery.root, recovery.access.metadata_path(), budget)?;
    if !checkout.is_selected(&files.checkout.bytes) && !heads.contains(&recovery.archive.operation)
    {
        return Err(Error::EpochChanged);
    }
    recovery
        .witness
        .check_normalized(
            &recovery.access,
            &recovery.root,
            &heads,
            Some(super::witness::Normalization::Unchanged(checkout)),
            budget,
        )
        .await
}
async fn guard(
    recovery: &RecoveredHistory,
    physical: &Arc<dyn PhysicalGuard>,
    expected: m::RecordDigest,
    budget: &Budget,
) -> Result<()> {
    recovery.check(budget).await?;
    budget
        .until(physical.validate(&recovery.witness.binding(), expected))
        .await??;
    recovery.check(budget).await
}
impl RecoveredHistory {
    /// Capture without writes. Caller must persist the returned witness with the
    /// original journal before completion; provider recovery never guesses it.
    pub async fn prepare_unchanged_checkout(
        self,
        published: &PublishedHead,
        physical: Arc<dyn PhysicalGuard>,
        expected: m::RecordDigest,
    ) -> Result<(Self, CheckoutWitnessV1)> {
        if self.checkout.is_some()
            || published.preparation() != &self.key
            || published.witness() != self.witness.id()?
            || published.operation != self.archive.operation
            || published.view != self.archive.operations[&self.archive.operation].view_id
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
                guard(&self, &physical, expected, &budget).await?;
                let heads = super::witness::heads(self.access.metadata_path(), &budget)?;
                if !heads.contains(&self.archive.operation) {
                    return Err(Error::EpochChanged);
                }
                let checkout = CheckoutWitnessV1::capture(&self, expected, &budget)?;
                check(&self, &checkout, &budget).await?;
                guard(&self, &physical, expected, &budget).await?;
                Ok((self, checkout))
            }))
        })
        .await
        .map_err(|_| Error::Unavailable)?;
        cancel.disarm();
        result
    }
    pub async fn complete_unchanged_checkout(
        mut self,
        checkout: CheckoutWitnessV1,
        physical: Arc<dyn PhysicalGuard>,
    ) -> Result<(Self, VerifiedUnchangedCheckout)> {
        checkout.validate(&self)?;
        if !self.publication
            || self
                .checkout
                .as_ref()
                .is_some_and(|saved| *saved != checkout)
        {
            return Err(Error::Invalid);
        }
        self.checkout = Some(checkout.clone());
        let cancellation = tokio_util::sync::CancellationToken::new();
        let cancel = cancellation.clone().drop_guard();
        let runtime = tokio::runtime::Handle::current();
        let result = tokio::task::spawn_blocking(move || {
            runtime.block_on(tokio::task::LocalSet::new().run_until(async {
                let mut budget = recovery_budget();
                budget.cancellation = cancellation;
                let proof = complete(&self, &checkout, &physical, &budget).await?;
                Ok((self, proof))
            }))
        })
        .await
        .map_err(|_| Error::Unavailable)?;
        cancel.disarm();
        result
    }
}
async fn complete(
    recovery: &RecoveredHistory,
    checkout: &CheckoutWitnessV1,
    physical_guard: &Arc<dyn PhysicalGuard>,
    budget: &Budget,
) -> Result<VerifiedUnchangedCheckout> {
    guard(recovery, physical_guard, checkout.physical(), budget).await?;
    let head_lock = publication::Destination::open(recovery, budget)?;
    recovery.check(budget).await?;
    records::validate(recovery, budget).await?;
    let extras = extras::validate(recovery, budget)?;
    let indexes = indexes::validate(recovery, budget).await?;
    let path = recovery.root.join(".jj/working_copy");
    let directory = physical::directory(&path)?;
    let before =
        physical::read(&directory, "checkout", 65536, false, budget)?.ok_or(Error::Incomplete)?;
    if !checkout.is_selected(&before.bytes) {
        let (name, staged) = staging::prepare(&directory, checkout, budget)?;
        guard(recovery, physical_guard, checkout.physical(), budget).await?;
        head_lock.check()?;
        extras.check(budget)?;
        indexes.check(budget)?;
        same(&directory, &physical::directory(&path)?)?;
        if physical::read(&directory, "checkout", 65536, false, budget)?.ok_or(Error::Incomplete)?
            != before
        {
            return Err(Error::EpochChanged);
        }
        staging::select(&directory, &name, &staged, checkout.after(), budget)?;
    }
    synced::acknowledge(&directory, "checkout", budget)?;
    directory.sync_all().map_err(|_| Error::Unavailable)?;
    guard(recovery, physical_guard, checkout.physical(), budget).await?;
    head_lock.check()?;
    extras.check(budget)?;
    indexes.check(budget)?;
    same(&directory, &physical::directory(&path)?)?;
    if !checkout.is_selected(
        &physical::read(&directory, "checkout", 65536, false, budget)?
            .ok_or(Error::Incomplete)?
            .bytes,
    ) {
        return Err(Error::EpochChanged);
    }
    Ok(VerifiedUnchangedCheckout {
        operation: recovery.archive.operation,
        view: recovery.archive.operations[&recovery.archive.operation].view_id,
        preparation: recovery.key.clone(),
        witness: checkout.id()?,
        physical: checkout.physical(),
    })
}

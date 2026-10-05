//! Exact source recovery/publication, unchanged completion and changed-tree preflight.
mod checkout;
mod extras;
mod indexes;
mod policy;
mod publication;
mod records;
mod retention;
mod tree_checkout;
mod witness;
use super::*;
use crate::{
    local_browser::budget::Budget,
    local_evidence::OperationArchiveV1,
    local_metadata::{GitOid, MetadataStore, StackPreparationKey},
};
pub use checkout::{CheckoutWitnessV1, PhysicalGuard, VerifiedUnchangedCheckout};
use jj_store_model as m;
pub use publication::PublishedHead;
use std::time::Duration;
pub use tree_checkout::{InstalledTreeFiles, TreeCheckoutPlanV1};
pub use witness::{RecoveryBinding, RecoveryWitnessV1};

pub struct RecoveredHistory {
    guard: guard::Guard,
    access: Arc<dyn Access>,
    root: PathBuf,
    witness: RecoveryWitnessV1,
    key: StackPreparationKey,
    archive: OperationArchiveV1,
    retained: RetainedIdentity,
    publication: bool,
    checkout: Option<CheckoutWitnessV1>,
    _permit: OwnedSemaphorePermit,
}
/// Checked retained selection information, not a publication capability.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct RetainedIdentity {
    pub commit: GitOid,
    pub archive: m::RecordDigest,
    pub operation: m::OperationId,
    pub view: m::ViewId,
}
impl Foreground {
    pub async fn recovery_witness(self) -> Result<(Self, RecoveryWitnessV1)> {
        let cancellation = tokio_util::sync::CancellationToken::new();
        let cancel = cancellation.clone().drop_guard();
        let runtime = tokio::runtime::Handle::current();
        let result = tokio::task::spawn_blocking(move || {
            runtime.block_on(tokio::task::LocalSet::new().run_until(async {
                self.validate_metadata().await?;
                let mut budget = recovery_budget();
                budget.cancellation = cancellation;
                let witness = RecoveryWitnessV1::capture(&self, &budget)?;
                witness.check(&self.access, &self.root, &budget).await?;
                self.guard.check(&self.root)?;
                Ok((self, witness))
            }))
        })
        .await
        .map_err(|_| Error::Unavailable)?;
        cancel.disarm();
        result
    }
}
impl Coordinator {
    /// Trusted composition first matches the original principal/request journal.
    /// This method looks up its exact reserved slot; it never accepts client history.
    /// A failure may leave exact unpublished immutable records for the same retry.
    pub async fn restore_history(
        &self,
        access: Arc<dyn Access>,
        root: PathBuf,
        witness: RecoveryWitnessV1,
        key: StackPreparationKey,
    ) -> Result<RecoveredHistory> {
        self.recover(access, root, witness, key, false, None).await
    }
    /// Trusted original-request composition only, including interrupted head
    /// selection. Proves current ancestry; never prepares a replacement command.
    pub async fn resume_publication(
        &self,
        access: Arc<dyn Access>,
        root: PathBuf,
        witness: RecoveryWitnessV1,
        key: StackPreparationKey,
    ) -> Result<RecoveredHistory> {
        self.recover(access, root, witness, key, true, None).await
    }
    /// Resume only the trusted original request's durably saved checkout
    /// witness, never inbound replacement metadata or an inferred new plan.
    pub async fn resume_unchanged_checkout(
        &self,
        access: Arc<dyn Access>,
        root: PathBuf,
        witness: RecoveryWitnessV1,
        key: StackPreparationKey,
        checkout: CheckoutWitnessV1,
    ) -> Result<RecoveredHistory> {
        self.recover(access, root, witness, key, true, Some(checkout))
            .await
    }
    async fn recover(
        &self,
        access: Arc<dyn Access>,
        root: PathBuf,
        witness: RecoveryWitnessV1,
        key: StackPreparationKey,
        publication: bool,
        checkout: Option<CheckoutWitnessV1>,
    ) -> Result<RecoveredHistory> {
        let permit = self.workers.acquire_preparation()?;
        let cancellation = tokio_util::sync::CancellationToken::new();
        let cancel = cancellation.clone().drop_guard();
        let runtime = tokio::runtime::Handle::current();
        let result = tokio::task::spawn_blocking(move || {
            runtime.block_on(tokio::task::LocalSet::new().run_until(async {
                let _permit = permit;
                let mut budget = recovery_budget();
                budget.cancellation = cancellation;
                witness.check_key(&key)?;
                if !publication {
                    witness.check(&access, &root, &budget).await?;
                } else {
                    budget.until(access.authorize()).await??;
                }
                let guard = guard::Guard::acquire(&root)?;
                guard.check(&root)?;
                let store = witness.store()?;
                let retained = store
                    .read_stack_prepared(&key)
                    .map_err(evidence_error)?
                    .ok_or(Error::NotFound)?;
                let archive = retained.history.as_ref().ok_or(Error::Incomplete)?.clone();
                witness.check_archive(&archive)?;
                let identity = RetainedIdentity {
                    commit: retained.commit,
                    archive: archive.id().map_err(evidence_error)?,
                    operation: archive.operation,
                    view: archive.operations[&archive.operation].view_id,
                };
                budget.check()?;
                let recovery = RecoveredHistory {
                    guard,
                    access,
                    root,
                    witness,
                    key,
                    archive,
                    retained: identity,
                    publication,
                    checkout,
                    _permit,
                };
                recovery.check(&budget).await?;
                records::restore(&recovery, &budget).await?;
                recovery.check(&budget).await?;
                if store
                    .read_stack_prepared(&recovery.key)
                    .map_err(evidence_error)?
                    != Some(retained)
                {
                    return Err(Error::EpochChanged);
                }
                Ok(recovery)
            }))
        })
        .await
        .map_err(|_| Error::Unavailable)?;
        cancel.disarm();
        result
    }
}
impl RecoveredHistory {
    pub fn retained_identity(&self) -> RetainedIdentity {
        self.retained
    }
    /// Explicit archive-only original/candidate index repair; no publication.
    pub async fn restore_indexes(self) -> Result<Self> {
        let cancellation = tokio_util::sync::CancellationToken::new();
        let cancel = cancellation.clone().drop_guard();
        let runtime = tokio::runtime::Handle::current();
        let result = tokio::task::spawn_blocking(move || {
            runtime.block_on(tokio::task::LocalSet::new().run_until(async {
                let mut budget = recovery_budget();
                budget.cancellation = cancellation;
                indexes::restore(&self, &budget).await?;
                self.check(&budget).await?;
                Ok(self)
            }))
        })
        .await
        .map_err(|_| Error::Unavailable)?;
        cancel.disarm();
        result
    }
    /// Trusted original-request recovery only; no publication/index/checkout.
    pub async fn restore_extras(self) -> Result<Self> {
        let cancellation = tokio_util::sync::CancellationToken::new();
        let cancel = cancellation.clone().drop_guard();
        let runtime = tokio::runtime::Handle::current();
        let result = tokio::task::spawn_blocking(move || {
            runtime.block_on(tokio::task::LocalSet::new().run_until(async {
                let mut budget = recovery_budget();
                budget.cancellation = cancellation;
                extras::restore(&self, &budget).await?;
                self.check(&budget).await?;
                Ok(self)
            }))
        })
        .await
        .map_err(|_| Error::Unavailable)?;
        cancel.disarm();
        result
    }
    pub async fn validate_binding(&self) -> Result<()> {
        self.check(&recovery_budget()).await
    }
    async fn check(&self, budget: &Budget) -> Result<()> {
        self.guard.check(&self.root)?;
        if let Some(checkout) = &self.checkout {
            checkout::check(self, checkout, budget).await?;
        } else if self.publication {
            publication::check(self, budget).await?;
        } else {
            self.witness.check(&self.access, &self.root, budget).await?;
        }
        self.guard.check(&self.root)
    }
}
fn recovery_budget() -> Budget {
    Budget::new(Limits {
        timeout: Duration::from_secs(30),
        max_work: 1_000_000,
        ..Default::default()
    })
}
fn evidence_error(error: crate::local_metadata::Error) -> Error {
    match error {
        crate::local_metadata::Error::Limit => Error::Limit,
        _ => Error::Incomplete,
    }
}

//! Explicit exact immutable source recovery; no heads, index or checkout repair.
mod records;
mod retention;
mod witness;
use super::*;
use crate::{
    local_browser::budget::Budget,
    local_evidence::OperationArchiveV1,
    local_metadata::{MetadataStore, StackPreparationKey},
};
use jj_store_model as m;
use std::time::Duration;
pub use witness::{RecoveryBinding, RecoveryWitnessV1};

pub struct RecoveredHistory {
    guard: guard::Guard,
    access: Arc<dyn Access>,
    root: PathBuf,
    witness: RecoveryWitnessV1,
    key: StackPreparationKey,
    archive: OperationArchiveV1,
    _permit: OwnedSemaphorePermit,
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
                witness.check(&access, &root, &budget).await?;
                let guard = guard::Guard::acquire(&root)?;
                guard.check(&root)?;
                witness.check(&access, &root, &budget).await?;
                let store = witness.store()?;
                let retained = store
                    .read_stack_prepared(&key)
                    .map_err(evidence_error)?
                    .ok_or(Error::NotFound)?;
                let archive = retained.history.as_ref().ok_or(Error::Incomplete)?.clone();
                witness.check_archive(&archive)?;
                let recovery = RecoveredHistory {
                    guard,
                    access,
                    root,
                    witness,
                    key,
                    archive,
                    _permit,
                };
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
    pub async fn validate_binding(&self) -> Result<()> {
        self.check(&recovery_budget()).await
    }
    async fn check(&self, budget: &Budget) -> Result<()> {
        self.guard.check(&self.root)?;
        self.witness.check(&self.access, &self.root, budget).await?;
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

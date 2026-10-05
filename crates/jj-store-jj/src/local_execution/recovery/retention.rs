use super::*;
use crate::local_metadata::StackRetentionOutcome;
impl Coordinator {
    /// Resume the saved original unpublished preparation without loading source
    /// operation/index state, preparing again, or restoring source metadata.
    pub async fn retain_history(
        &self,
        access: Arc<dyn Access>,
        root: PathBuf,
        witness: RecoveryWitnessV1,
        key: StackPreparationKey,
        archive: OperationArchiveV1,
    ) -> Result<StackRetentionOutcome> {
        let permit = self.workers.acquire_preparation()?;
        let cancellation = tokio_util::sync::CancellationToken::new();
        let cancel = cancellation.clone().drop_guard();
        let runtime = tokio::runtime::Handle::current();
        let result = tokio::task::spawn_blocking(move || {
            runtime.block_on(tokio::task::LocalSet::new().run_until(async {
                let _permit = permit;
                witness.check_key(&key)?;
                witness.check_archive(&archive)?;
                let mut budget = recovery_budget();
                budget.cancellation = cancellation;
                witness.check(&access, &root, &budget).await?;
                let guard = guard::Guard::acquire(&root)?;
                retain(&guard, &access, &root, &witness, &key, &archive, &budget).await
            }))
        })
        .await
        .map_err(|_| Error::Unavailable)?;
        cancel.disarm();
        result
    }
}
impl Foreground {
    pub async fn retain_history(
        self,
        witness: RecoveryWitnessV1,
        key: StackPreparationKey,
        archive: OperationArchiveV1,
    ) -> Result<(Self, StackRetentionOutcome)> {
        let cancellation = tokio_util::sync::CancellationToken::new();
        let cancel = cancellation.clone().drop_guard();
        let runtime = tokio::runtime::Handle::current();
        let result = tokio::task::spawn_blocking(move || {
            runtime.block_on(tokio::task::LocalSet::new().run_until(async {
                let mut budget = recovery_budget();
                budget.cancellation = cancellation;
                let outcome = retain(
                    &self.guard,
                    &self.access,
                    &self.root,
                    &witness,
                    &key,
                    &archive,
                    &budget,
                )
                .await?;
                Ok((self, outcome))
            }))
        })
        .await
        .map_err(|_| Error::Unavailable)?;
        cancel.disarm();
        result
    }
}
async fn retain(
    guard: &guard::Guard,
    access: &Arc<dyn Access>,
    root: &std::path::Path,
    witness: &RecoveryWitnessV1,
    key: &StackPreparationKey,
    archive: &OperationArchiveV1,
    budget: &Budget,
) -> Result<StackRetentionOutcome> {
    witness.check_key(key)?;
    witness.check_archive(archive)?;
    guard.check(root)?;
    witness.check(access, root, budget).await?;
    let result = witness
        .store()?
        .retain_stack_execution(key, archive)
        .map_err(evidence_error)?;
    witness.check(access, root, budget).await?;
    guard.check(root)?;
    Ok(result)
}

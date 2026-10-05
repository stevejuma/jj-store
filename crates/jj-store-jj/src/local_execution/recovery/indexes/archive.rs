use super::*;
use crate::query::prepared::{CommitFields, Source};
use jj_lib::{
    backend,
    object_id::{HexPrefix, ObjectId as _, PrefixResolution},
    op_store::{self as o, OpStore, OpStoreError, OpStoreResult},
    operation::Operation,
    view::View,
};
use std::time::SystemTime;

pub(super) struct Archive<'a> {
    pub recovery: &'a RecoveredHistory,
    pub budget: &'a Budget,
}
impl Source for Archive<'_> {
    fn check(&self) -> Result<()> {
        self.budget.check()
    }
    fn work(&self, n: usize) -> Result<()> {
        self.budget.work(n)
    }
    fn max_commits(&self) -> usize {
        100_000
    }
    fn max_parents(&self) -> usize {
        256
    }
    fn commit_len(&self) -> usize {
        20
    }
    fn change_len(&self) -> usize {
        16
    }
    async fn operation(&self, id: &o::OperationId) -> Result<Operation> {
        self.budget
            .until(self.recovery.access.authorize())
            .await??;
        let key = m::OperationId::from_slice(id.as_bytes()).map_err(|_| Error::Invalid)?;
        let data = self
            .recovery
            .archive
            .operations
            .get(&key)
            .ok_or(Error::Incomplete)?;
        if data.parents.len() > 256 {
            return Err(Error::Limit);
        }
        let native: o::Operation = data.clone().try_into().map_err(evidence_error)?;
        Ok(Operation::new(
            Arc::new(Denied(o::OperationId::from_bytes(&[0; 64]))),
            id.clone(),
            native,
        ))
    }
    async fn view(&self, operation: &Operation) -> Result<View> {
        self.budget
            .until(self.recovery.access.authorize())
            .await??;
        let key = m::ViewId::from_slice(operation.store_operation().view_id.as_bytes())
            .map_err(|_| Error::Invalid)?;
        let data = self
            .recovery
            .archive
            .views
            .get(&key)
            .ok_or(Error::Incomplete)?;
        Ok(View::new(
            data.clone().try_into().map_err(evidence_error)?,
            false,
        ))
    }
    async fn commit(&self, id: &backend::CommitId) -> Result<CommitFields> {
        self.budget
            .until(self.recovery.access.authorize())
            .await??;
        let key = m::GitSha1Id::from_slice(id.as_bytes()).map_err(|_| Error::Invalid)?;
        let data = self
            .recovery
            .archive
            .commits
            .get(&key)
            .ok_or(Error::Incomplete)?;
        let native: backend::Commit = data.clone().try_into().map_err(evidence_error)?;
        Ok(CommitFields {
            parents: native.parents,
            change: native.change_id,
            timestamp: native.committer.timestamp.timestamp.0,
            // Recovery builds topology; text filters are prepared separately.
            description: None,
        })
    }
}

// Index decoding must never fallback into source commits or mutate a backend.
#[derive(Debug)]
struct Denied(o::OperationId);
fn denied<T>() -> OpStoreResult<T> {
    Err(OpStoreError::Other(Box::new(Error::Denied)))
}
#[async_trait::async_trait]
impl OpStore for Denied {
    fn name(&self) -> &str {
        "archive-index-decode-only"
    }
    fn root_operation_id(&self) -> &o::OperationId {
        &self.0
    }
    async fn read_operation(&self, _: &o::OperationId) -> OpStoreResult<o::Operation> {
        denied()
    }
    async fn read_view(&self, _: &o::ViewId) -> OpStoreResult<o::View> {
        denied()
    }
    async fn write_operation(&self, _: &o::Operation) -> OpStoreResult<o::OperationId> {
        denied()
    }
    async fn write_view(&self, _: &o::View) -> OpStoreResult<o::ViewId> {
        denied()
    }
    async fn resolve_operation_id_prefix(
        &self,
        _: &HexPrefix,
    ) -> OpStoreResult<PrefixResolution<o::OperationId>> {
        denied()
    }
    async fn gc(&self, _: &[o::OperationId], _: SystemTime) -> OpStoreResult<()> {
        denied()
    }
}

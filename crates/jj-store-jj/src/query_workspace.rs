//! Read-only workspace catalog for query loaders that have no checkout.
use std::{path::Path, sync::Arc};

use jj_lib::{
    ref_name::WorkspaceName,
    workspace_store::{WorkspaceStore, WorkspaceStoreError},
};

pub(crate) fn store() -> Arc<dyn WorkspaceStore> {
    #[derive(Debug)]
    struct ReadOnly;
    fn denied() -> Result<(), WorkspaceStoreError> {
        Err(WorkspaceStoreError::Other(
            "query workspace store is read-only".into(),
        ))
    }
    impl WorkspaceStore for ReadOnly {
        fn name(&self) -> &str {
            "query-read-only"
        }
        fn add(&self, _: &WorkspaceName, _: &Path) -> Result<(), WorkspaceStoreError> {
            denied()
        }
        fn forget(&self, _: &[&WorkspaceName]) -> Result<(), WorkspaceStoreError> {
            denied()
        }
        fn rename(&self, _: &WorkspaceName, _: &WorkspaceName) -> Result<(), WorkspaceStoreError> {
            denied()
        }
        fn get_workspace_path(
            &self,
            _: &WorkspaceName,
        ) -> Result<Option<std::path::PathBuf>, WorkspaceStoreError> {
            Ok(None)
        }
    }
    Arc::new(ReadOnly)
}

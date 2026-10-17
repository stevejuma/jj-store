//! Qualified JJ evidence and stack retention over the standalone Git metadata carrier.
mod evidence_files;
#[cfg(feature = "native")]
mod stack_prepared;
pub use jj_store_git::metadata::{objects, *};
#[cfg(feature = "native")]
pub use stack_prepared::{
    STACK_PREPARED_REF_PREFIX, StackPreparationKey, StackRetention, StackRetentionOutcome,
};
use std::path::{Path, PathBuf};
#[derive(Clone, Debug)]
pub struct MetadataStore {
    inner: jj_store_git::metadata::MetadataStore,
    git_dir: PathBuf,
    limits: Limits,
}
impl std::ops::Deref for MetadataStore {
    type Target = jj_store_git::metadata::MetadataStore;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}
impl MetadataStore {
    pub fn open(path: &Path, limits: Limits) -> Result<Self> {
        let inner = jj_store_git::metadata::MetadataStore::open(path, limits)?
            .with_evidence_validator(std::sync::Arc::new(
                crate::local_evidence::JjEvidenceValidator,
            ));
        Ok(Self {
            git_dir: inner.git_dir().to_path_buf(),
            limits,
            inner,
        })
    }
    pub fn carrier(&self) -> &jj_store_git::metadata::MetadataStore {
        &self.inner
    }
}

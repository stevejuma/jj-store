use super::*;

impl MetadataStore {
    pub fn stage_blob(&self, bytes: &[u8]) -> Result<GitOid> {
        if bytes.len() > self.limits.object_bytes {
            return Err(Error::Limit);
        }
        let repo = gix::open(&self.git_dir).map_err(Error::other)?;
        objects::blob(&repo, bytes)
    }
    pub fn read_retained_blob(&self, selected: &Snapshot, blob: GitOid) -> Result<Vec<u8>> {
        if selected.root.retained.get(&blob) != Some(&RetainedKind::Blob) {
            return Err(Error::Stale);
        }
        self.read_staged_blob(blob)
    }
    /// Immutable staging capability for owned reducers. Public read facades
    /// must first prove membership in their selected domain closure.
    pub fn read_staged_blob(&self, blob: GitOid) -> Result<Vec<u8>> {
        let repo = gix::open(&self.git_dir).map_err(Error::other)?;
        objects::object(&repo, blob, gix::objs::Kind::Blob, self.limits.object_bytes)
    }
}

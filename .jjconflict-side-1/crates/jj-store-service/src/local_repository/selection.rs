//! Requested pins are admitted only through the current committed companion history.
use super::{LocalRepository, map_metadata};
use crate::work::{Error, Result};
use jj_store_jj::local_metadata::{GitOid, Snapshot};
impl LocalRepository {
    pub(crate) fn selected_metadata(&self, commit: GitOid) -> Result<Snapshot> {
        let current = self
            .metadata
            .read()
            .map_err(map_metadata)?
            .ok_or(Error::Stale)?;
        self.check_identity(Some(&current))?;
        if current.commit() == commit {
            return Ok(current);
        }
        self.metadata
            .history(&current)
            .map_err(map_metadata)?
            .into_iter()
            .find(|s| s.commit() == commit)
            .ok_or(Error::Stale)
    }
}

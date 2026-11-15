//! Counts are pinned native graph facts, not a network reachability claim.
use super::*;
use crate::vault_workspace::{CommitDivergence, graph_divergence};
use jj_lib::ref_name::{RefName, RemoteName, RemoteRefSymbol};

impl OriginalRepository {
    pub async fn divergence(
        &self,
        remote: &str,
        branch: &str,
        bookmark: &str,
    ) -> Result<CommitDivergence> {
        self.access.authorize().await.map_err(|_| Error::Denied)?;
        let local = self.repo.view().get_local_bookmark(RefName::new(bookmark));
        let local = local.as_normal().ok_or(Error::Binding)?;
        let recorded = self.repo.view().get_remote_bookmark(RemoteRefSymbol {
            name: RefName::new(branch),
            remote: RemoteName::new(remote),
        });
        if !recorded.target.is_resolved() {
            return Err(Error::Unsupported);
        }
        let counts = graph_divergence(self.repo.as_ref(), local, recorded.target.as_normal())
            .await
            .map_err(|_| Error::Native)?;
        self.access.authorize().await.map_err(|_| Error::Denied)?;
        Ok(counts)
    }
}

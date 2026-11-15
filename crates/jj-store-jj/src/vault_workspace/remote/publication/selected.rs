//! Original-writer-only preparation after verified object copying into its private cache.
use super::*;
use jj_lib::{op_store::RefTarget, ref_name::RefName};

impl RemoteTransfer {
    /// Not a public admission API. OriginalRepository proves bookmark and source binding.
    pub(crate) async fn prepare_original_commit(
        &self,
        id: &str,
        after: &str,
        before: Option<String>,
    ) -> Result<Option<(PushIntent, CommitDivergence)>> {
        request_id(id)?;
        if !self.binding.vault_id.starts_with("original-") || self.pending_push()?.is_some() {
            return Err(Error::Busy);
        }
        if !oid(after) || before.as_ref().is_some_and(|value| !oid(value)) {
            return Err(Error::Binding);
        }
        let mut session = Session::open(self.binding.clone()).await?;
        let target_id = CommitId::try_from_hex(after).ok_or(Error::Binding)?;
        let prior = before.as_ref().and_then(CommitId::try_from_hex);
        let backend = git::get_git_backend(session.repo.store()).map_err(jj)?;
        backend
            .import_head_commits(std::iter::once(&target_id).chain(prior.iter()))
            .map_err(jj)?;
        let target = session.repo.store().get_commit(&target_id).map_err(jj)?;
        if target.has_conflict() {
            return Err(Error::Binding);
        }
        let mut transaction = session.repo.start_transaction();
        transaction.repo_mut().add_head(&target).await.map_err(jj)?;
        if let Some(prior) = &prior {
            let prior_commit = session.repo.store().get_commit(prior).map_err(jj)?;
            transaction
                .repo_mut()
                .add_head(&prior_commit)
                .await
                .map_err(jj)?;
            if !transaction
                .repo_mut()
                .index()
                .is_ancestor(prior, &target_id)
                .await
                .map_err(jj)?
            {
                return Ok(None);
            }
        }
        // This is an isolated original transport repository, never the backup binding.
        transaction.repo_mut().set_local_bookmark_target(
            RefName::new("lapis-incoming"),
            RefTarget::normal(target_id),
        );
        session.repo = transaction
            .commit("retain reviewed original commit")
            .await
            .map_err(jj)?;
        let intent = PushIntent {
            binding: self.binding.clone(),
            endpoint: self.endpoint.clone(),
            branch: self.branch.clone(),
            request_id: id.into(),
            before,
            after: after.into(),
        };
        let counts = session.push_divergence(&intent).await?;
        Ok(Some((intent, counts)))
    }
}

impl PushIntent {
    pub fn commit(&self) -> &str {
        &self.after
    }
    pub fn remote_commit(&self) -> Option<&str> {
        self.before.as_deref()
    }
    pub fn request_id(&self) -> &str {
        &self.request_id
    }
    pub(crate) fn matches_original_selection(
        &self,
        transfer: &RemoteTransfer,
        commit: &str,
    ) -> bool {
        self.bound_to(&transfer.binding)
            && self.endpoint == transfer.endpoint
            && self.branch == transfer.branch
            && self.after == commit
    }
}

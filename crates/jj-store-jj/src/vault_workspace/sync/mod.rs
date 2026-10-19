//! Immutable merge proposals; guarded materialization is a separate command.
mod resolution;
use super::remote::publication;
use super::*;
use jj_lib::{backend::CommitId, op_store::RefTarget, ref_name::RefName};
pub use resolution::{SyncConflict, SyncResolution, SyncTerm};
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct SyncProposal {
    pub local: Checkpoint,
    pub incoming: Checkpoint,
    pub candidate: Option<Checkpoint>,
    pub conflicts: Vec<String>,
}
#[derive(Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
struct Saved {
    version: u32,
    local: String,
    incoming: String,
    candidate: String,
}
impl Session {
    pub async fn prepare_sync(
        &mut self,
        incoming: &str,
        exclusions: &[String],
    ) -> Result<SyncProposal> {
        self.capture(exclusions, "Checkpoint before synchronization")
            .await?;
        let local = self.checkpoint()?.ok_or(Error::Binding)?;
        let local_commit = self.retained_commit(&local.commit).await?;
        let incoming_commit = self.retained_commit(incoming).await?;
        if self
            .repo
            .view()
            .get_local_bookmark(RefName::new("lapis-incoming"))
            .as_normal()
            != Some(incoming_commit.id())
        {
            return Err(Error::Binding);
        }
        let mut proposal = SyncProposal {
            local,
            incoming: checkpoint(&incoming_commit, &self.repo),
            candidate: None,
            conflicts: vec![],
        };
        if self
            .repo
            .index()
            .is_ancestor(incoming_commit.id(), local_commit.id())
            .await
            .map_err(jj)?
        {
            return Ok(proposal);
        }
        let path = self.binding.state.join("sync-proposal.json");
        if path.exists() {
            let saved: Saved = publication::read(&path)?;
            if saved.version == 1
                && saved.local == proposal.local.commit
                && saved.incoming == incoming
            {
                self.retain_sync_candidate(&saved).await?;
                let candidate = self.sync_candidate(&saved.candidate).await?;
                proposal.conflicts = conflicts(&candidate, exclusions)?;
                proposal.candidate = Some(checkpoint(&candidate, &self.repo));
                return Ok(proposal);
            }
        }
        let tree = jj_lib::rewrite::merge_commit_trees(
            self.repo.as_ref(),
            &[local_commit.clone(), incoming_commit.clone()],
        )
        .await
        .map_err(jj)?;
        // Exclusions are independent of a remote repository's Git rules too.
        let mut builder = jj_lib::merged_tree_builder::MergedTreeBuilder::new(tree.clone());
        for (path, value) in tree.entries() {
            value.map_err(jj)?;
            if scan::excluded(path.as_internal_file_string(), exclusions) {
                builder.set_or_remove(path, jj_lib::merge::Merge::resolved(None));
            }
        }
        let tree = builder.write_tree().await.map_err(jj)?;
        let fast_forward = self
            .repo
            .index()
            .is_ancestor(local_commit.id(), incoming_commit.id())
            .await
            .map_err(jj)?
            && tree.tree_ids_and_labels() == incoming_commit.tree().tree_ids_and_labels();
        let mut tx = self.repo.start_transaction();
        let candidate = if fast_forward {
            incoming_commit.clone()
        } else {
            tx.repo_mut()
                .new_commit(
                    vec![local_commit.id().clone(), incoming_commit.id().clone()],
                    tree,
                )
                .set_description("Merge remote vault backup")
                .write()
                .await
                .map_err(jj)?
        };
        tx.repo_mut().set_local_bookmark_target(
            RefName::new("lapis-sync"),
            RefTarget::normal(candidate.id().clone()),
        );
        publication::save(
            &path,
            &Saved {
                version: 1,
                local: proposal.local.commit.clone(),
                incoming: incoming.into(),
                candidate: candidate.id().hex(),
            },
        )?;
        self.repo = tx
            .commit("prepare vault synchronization")
            .await
            .map_err(jj)?;
        proposal.conflicts = conflicts(&candidate, exclusions)?;
        proposal.candidate = Some(checkpoint(&candidate, &self.repo));
        Ok(proposal)
    }
    async fn retain_sync_candidate(&mut self, saved: &Saved) -> Result<()> {
        let id = CommitId::try_from_hex(&saved.candidate).ok_or(Error::Binding)?;
        if self
            .repo
            .view()
            .get_local_bookmark(RefName::new("lapis-sync"))
            .as_normal()
            == Some(&id)
        {
            return Ok(());
        }
        let commit = self.repo.store().get_commit(&id).map_err(jj)?;
        let mut tx = self.repo.start_transaction();
        tx.repo_mut().add_head(&commit).await.map_err(jj)?;
        for parent in [&saved.local, &saved.incoming] {
            let parent = CommitId::try_from_hex(parent).ok_or(Error::Binding)?;
            if !tx
                .repo()
                .index()
                .is_ancestor(&parent, &id)
                .await
                .map_err(jj)?
            {
                return Err(Error::Binding);
            }
        }
        tx.repo_mut()
            .set_local_bookmark_target(RefName::new("lapis-sync"), RefTarget::normal(id));
        self.repo = tx
            .commit("recover retained synchronization proposal")
            .await
            .map_err(jj)?;
        Ok(())
    }
    async fn sync_candidate(&self, id: &str) -> Result<jj_lib::commit::Commit> {
        let selected = CommitId::try_from_hex(id).ok_or(Error::Binding)?;
        if self
            .repo
            .view()
            .get_local_bookmark(RefName::new("lapis-sync"))
            .as_normal()
            != Some(&selected)
        {
            return Err(Error::Binding);
        }
        let saved: Saved = publication::read(&self.binding.state.join("sync-proposal.json"))?;
        if saved.version != 1 || saved.candidate != id {
            return Err(Error::Binding);
        }
        self.retained_commit(id).await
    }
    fn sync_base(&self, candidate: &str) -> Result<String> {
        let saved: Saved = publication::read(&self.binding.state.join("sync-proposal.json"))?;
        if saved.version != 1
            || saved.candidate != candidate
            || self.checkpoint()?.map(|c| c.commit) != Some(saved.local.clone())
        {
            return Err(Error::Jj(
                "Local history advanced; prepare synchronization again".into(),
            ));
        }
        Ok(saved.local)
    }
    pub async fn preview_sync(
        &self,
        candidate: &str,
        exclusions: &[String],
    ) -> Result<VaultRestorePreview> {
        let commit = self.sync_candidate(candidate).await?;
        if !conflicts(&commit, exclusions)?.is_empty() {
            return Err(Error::Jj("Resolve synchronization conflicts first".into()));
        }
        let base = self.sync_base(candidate)?;
        let expected = batch_plan::identity(&self.retained_commit(&base).await?.tree())?;
        let preview = self.preview_vault_restore(candidate, exclusions).await?;
        if preview.expected != expected {
            return Err(Error::Jj(
                "Vault files changed; prepare synchronization again".into(),
            ));
        }
        Ok(preview)
    }
    pub async fn apply_sync(
        &mut self,
        request: &VaultRestoreRequest,
        exclusions: &[String],
    ) -> Result<Capture> {
        publication::request_id(&request.request_id)?;
        let receipt = self
            .binding
            .state
            .join("sync-restore-receipts")
            .join(format!("{}.json", request.request_id));
        // A recorded request reconciles through its original durable receipt.
        if !receipt.exists() {
            let saved: Saved = publication::read(&self.binding.state.join("sync-proposal.json"))?;
            if saved.version != 1 || saved.candidate != request.checkpoint {
                return Err(Error::Binding);
            }
            self.retain_sync_candidate(&saved).await?;
            let preview = self.preview_sync(&request.checkpoint, exclusions).await?;
            if preview.expected != request.expected {
                return Err(Error::Binding);
            }
        }
        self.restore_anchored(request, exclusions, "lapis-sync", "sync-restore-receipts")
            .await
    }
}
fn conflicts(commit: &jj_lib::commit::Commit, exclusions: &[String]) -> Result<Vec<String>> {
    let mut paths = Vec::new();
    for (path, value) in commit.tree().conflicts() {
        value.map_err(jj)?;
        let path = path.as_internal_file_string();
        if scan::excluded(path, exclusions) {
            continue;
        }
        if paths.len() >= 10_000 {
            return Err(Error::Binding);
        }
        paths.push(path.to_owned());
    }
    Ok(paths)
}

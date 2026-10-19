use super::*;
use jj_lib::{backend::CommitId, op_store::RefTarget, ref_name::RefName};

impl Session {
    /// Import only the immutable head returned by an admitted transfer. No files
    /// are written; selection/guarded restoration remains a separate command.
    pub async fn import_fetched(&mut self, fetched: &FetchedHead) -> Result<Checkpoint> {
        self.binding.verify_saved()?;
        if self.binding != fetched.binding || !oid(&fetched.commit) {
            return Err(Error::Binding);
        }
        let id = CommitId::try_from_hex(&fetched.commit).ok_or(Error::Binding)?;
        git::get_git_backend(self.repo.store())
            .map_err(jj)?
            .import_head_commits([&id])
            .map_err(jj)?;
        let commit = self.repo.store().get_commit(&id).map_err(jj)?;
        if self
            .repo
            .view()
            .get_local_bookmark(RefName::new("lapis-incoming"))
            .as_normal()
            != Some(&id)
        {
            let mut tx = self.repo.start_transaction();
            tx.repo_mut().add_head(&commit).await.map_err(jj)?;
            tx.repo_mut()
                .set_local_bookmark_target(RefName::new("lapis-incoming"), RefTarget::normal(id));
            self.repo = tx
                .commit(format!("import remote backup branch {}", fetched.branch))
                .await
                .map_err(jj)?;
        }
        Ok(checkpoint(&commit, &self.repo))
    }
    /// Fresh-device restoration retains the imported branch and the original
    /// local safety history. Replays recover a staged native commit identity.
    pub async fn restore_imported(
        &mut self,
        request: &VaultRestoreRequest,
        exclusions: &[String],
    ) -> Result<Capture> {
        self.restore_anchored(
            request,
            exclusions,
            "lapis-incoming",
            "remote-restore-receipts",
        )
        .await
    }
    pub(in crate::vault_workspace) async fn restore_anchored(
        &mut self,
        request: &VaultRestoreRequest,
        exclusions: &[String],
        anchor: &str,
        directory: &str,
    ) -> Result<Capture> {
        self.binding.verify_saved()?;
        publication::request_id(&request.request_id)?;
        let path = self
            .binding
            .state
            .join(directory)
            .join(format!("{}.json", request.request_id));
        let mut receipt: ImportReceipt = if path.exists() {
            let receipt: ImportReceipt = publication::read(&path)?;
            if receipt.version != 1 || receipt.request != *request {
                return Err(Error::Binding);
            }
            receipt
        } else {
            let target = self.repo.view().get_local_bookmark(RefName::new(anchor));
            if target.as_normal().map(|id| id.hex()).as_deref() != Some(&request.checkpoint) {
                return Err(Error::Binding);
            }
            let receipt = ImportReceipt {
                version: 1,
                request: request.clone(),
                candidate: None,
                base: None,
                result: None,
            };
            publication::save(&path, &receipt)?;
            receipt
        };
        if let Some(result) = receipt.result {
            return Ok(result);
        }
        if receipt.candidate.is_none() {
            let applied = self.restore_vault(request, exclusions).await?;
            let wc = CommitId::try_from_hex(&applied.working_copy).ok_or(Error::Binding)?;
            let wc = self.repo.store().get_commit(&wc).map_err(jj)?;
            let [base] = wc.parent_ids() else {
                return Err(Error::Binding);
            };
            if self.checkpoint()?.map(|c| c.commit) != Some(base.hex()) {
                return Err(Error::Jj(
                    "Backup advanced after restoration; review before adopting remote history"
                        .into(),
                ));
            }
            let before = self.repo.store().get_commit(base).map_err(jj)?;
            let incoming = CommitId::try_from_hex(&request.checkpoint).ok_or(Error::Binding)?;
            let mut tx = self.repo.start_transaction();
            let parents = if base == &incoming {
                vec![base.clone()]
            } else {
                vec![base.clone(), incoming]
            };
            let combined = if anchor == "lapis-sync" {
                let target = self.retained_commit(&request.checkpoint).await?;
                if target.tree().tree_ids_and_labels() != before.tree().tree_ids_and_labels() {
                    return Err(Error::Binding);
                }
                target
            } else {
                tx.repo_mut()
                    .new_commit(parents, before.tree())
                    .set_description("Restore remote backup")
                    .write()
                    .await
                    .map_err(jj)?
            };
            receipt.base = Some(base.hex());
            receipt.candidate = Some(combined.id().hex());
            // The immutable candidate is retained before publishing its view.
            publication::save(&path, &receipt)?;
        }
        let candidate = CommitId::try_from_hex(receipt.candidate.as_ref().ok_or(Error::Binding)?)
            .ok_or(Error::Binding)?;
        let combined = self.repo.store().get_commit(&candidate).map_err(jj)?;
        let current = self.checkpoint()?.ok_or(Error::Binding)?;
        let head = CommitId::try_from_hex(&current.commit).ok_or(Error::Binding)?;
        let indexed = self.repo.index().has_id(&candidate).await.map_err(jj)?;
        if !indexed
            || !self
                .repo
                .index()
                .is_ancestor(&candidate, &head)
                .await
                .map_err(jj)?
        {
            if receipt.base.as_deref() != Some(&current.commit) {
                return Err(Error::Jj(
                    "Backup advanced while restoring remote history; later edits were retained"
                        .into(),
                ));
            }
            let workspace_name = self.workspace.workspace_name().to_owned();
            let mut locked = self
                .workspace
                .start_working_copy_mutation()
                .await
                .map_err(jj)?;
            let mut tx = self.repo.start_transaction();
            tx.repo_mut().add_head(&combined).await.map_err(jj)?;
            tx.repo_mut().set_local_bookmark_target(
                RefName::new("lapis-backup"),
                RefTarget::normal(candidate),
            );
            let next = tx
                .repo_mut()
                .check_out(workspace_name, &combined)
                .await
                .map_err(jj)?;
            tx.repo_mut().rebase_descendants().await.map_err(jj)?;
            self.repo = tx
                .commit("retain imported backup ancestry")
                .await
                .map_err(jj)?;
            locked.locked_wc().reset(&next).await.map_err(jj)?;
            locked.finish(self.repo.op_id().clone()).await.map_err(jj)?;
        }
        let wc = self
            .repo
            .view()
            .get_wc_commit_id(self.workspace.workspace_name())
            .ok_or(Error::Binding)?;
        let result = Capture {
            changed: true,
            checkpoint: Some(checkpoint(&combined, &self.repo)),
            operation: self.repo.op_id().hex(),
            working_copy: wc.hex(),
        };
        receipt.result = Some(result.clone());
        publication::save(&path, &receipt)?;
        Ok(result)
    }
}
#[derive(Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
struct ImportReceipt {
    version: u32,
    request: VaultRestoreRequest,
    base: Option<String>,
    candidate: Option<String>,
    result: Option<Capture>,
}

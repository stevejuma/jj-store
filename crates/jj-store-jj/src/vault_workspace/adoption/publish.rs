use super::*;
impl Session {
    pub(super) async fn finish_adoption(
        &mut self,
        path: &Path,
        mut receipt: Receipt,
        progress: &mut impl FnMut(&str) -> Result<()>,
    ) -> Result<Checkpoint> {
        if receipt.version != 1 || receipt.stopped {
            return Err(Error::Binding);
        }
        request_id(&receipt.request.request_id)?;
        let candidate = CommitId::try_from_hex(receipt.candidate.as_deref().ok_or(Error::Binding)?)
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
                return Err(Error::Jj("Backup advanced during history adoption; retain the original request for recovery".into()));
            }
            let name = self.workspace.workspace_name().to_owned();
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
            let next = tx.repo_mut().check_out(name, &combined).await.map_err(jj)?;
            tx.repo_mut().rebase_descendants().await.map_err(jj)?;
            self.repo = tx
                .commit("retain existing repository ancestry")
                .await
                .map_err(jj)?;
            locked.locked_wc().reset(&next).await.map_err(jj)?;
            locked.finish(self.repo.op_id().clone()).await.map_err(jj)?;
        }
        progress("published")?;
        let result = checkpoint(&combined, &self.repo);
        receipt.result = Some(result.clone());
        save(path, &receipt)?;
        clear_active(&self.binding, &receipt.request.request_id)?;
        Ok(result)
    }
    pub(in crate::vault_workspace) async fn recover_adoptions(&mut self) -> Result<()> {
        if let Some(pending) = self.binding.pending_adoption()? {
            let path = self
                .binding
                .state
                .join("adoption-receipts")
                .join(format!("{}.json", pending.request_id));
            let receipt: Receipt = read(&path)?;
            if receipt.result.is_some() {
                clear_active(&self.binding, &receipt.request.request_id)?;
            } else if receipt.candidate.is_some() {
                self.finish_adoption(&path, receipt, &mut |_| Ok(()))
                    .await?;
            }
        }
        Ok(())
    }
    /// Stop tracking an original import without undoing any accepted ancestry.
    pub fn stop_adoption(&self, id: &str) -> Result<()> {
        self.binding.verify_saved()?;
        request_id(id)?;
        let path = self
            .binding
            .state
            .join("adoption-receipts")
            .join(format!("{id}.json"));
        let mut receipt: Receipt = match read(&path) {
            Err(Error::Io(error)) if error.kind() == std::io::ErrorKind::NotFound => {
                // Fence an original request whose UI record was saved before
                // native admission; a delayed delivery cannot revive it.
                save(&path.with_extension("stopped"), &true)?;
                return clear_active(&self.binding, id);
            }
            result => result?,
        };
        if receipt.version != 1 || receipt.request.request_id != id {
            return Err(Error::Binding);
        }
        if receipt.result.is_none() {
            receipt.stopped = true;
            save(&path, &receipt)?;
        }
        clear_active(&self.binding, id)
    }
}
impl Binding {
    pub fn pending_adoption(&self) -> Result<Option<AdoptionRequest>> {
        self.verify_saved()?;
        let active = self.state.join("adoption-receipts/active.json");
        let id: String = match read(&active) {
            Err(Error::Io(error)) if error.kind() == std::io::ErrorKind::NotFound => {
                return Ok(None);
            }
            other => other?,
        };
        request_id(&id)?;
        let receipt: Receipt = read(
            &self
                .state
                .join("adoption-receipts")
                .join(format!("{id}.json")),
        )?;
        if receipt.version != 1 || receipt.request.request_id != id {
            return Err(Error::Binding);
        }
        Ok((!receipt.stopped && receipt.result.is_none()).then_some(receipt.request))
    }
}
pub(super) fn clear_active(binding: &Binding, id: &str) -> Result<()> {
    let path = binding.state.join("adoption-receipts/active.json");
    match read::<String>(&path) {
        Ok(current) if current == id => {
            fs::remove_file(&path)?;
            File::open(path.parent().ok_or(Error::Binding)?)?.sync_all()?;
        }
        Ok(_) => {}
        Err(Error::Io(error)) if error.kind() == std::io::ErrorKind::NotFound => {}
        Err(error) => return Err(error),
    }
    Ok(())
}

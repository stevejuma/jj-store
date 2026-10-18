use super::*;
use jj_lib::{op_store::RefTarget, ref_name::RefName};

impl Session {
    /// Capture only saves native objects and working-copy metadata. It never
    /// checks out files, invokes a shell, or changes an existing user repository.
    pub async fn capture(&mut self, exclusions: &[String], description: &str) -> Result<Capture> {
        self.binding.verify_saved()?;
        if description.len() > 4096 {
            return Err(Error::Binding);
        }
        let name = self.workspace.workspace_name().to_owned();
        let id = self
            .repo
            .view()
            .get_wc_commit_id(&name)
            .ok_or(Error::Binding)?;
        let current = self.repo.store().get_commit(id).map_err(jj)?;
        if let Some(parent) = self
            .repo
            .view()
            .get_local_bookmark(RefName::new("lapis-backup"))
            .as_normal()
        {
            if current.parent_ids() != [parent.clone()] {
                return Err(Error::Binding);
            }
        }
        let mut locked = self
            .workspace
            .start_working_copy_mutation()
            .await
            .map_err(jj)?;
        // Ordinary JJ snapshot scans intentionally omit nested repositories and
        // consult .gitignore. Vault backup has an independent inclusion contract.
        let tree = scan::capture(self.repo.store(), &self.binding.root, exclusions).await?;
        self.binding.verify_saved()?;
        let changed = tree.tree_ids_and_labels() != current.tree().tree_ids_and_labels();
        let mut checkpoint_id = None;
        if changed
            || self
                .repo
                .view()
                .get_local_bookmark(RefName::new("lapis-backup"))
                .is_absent()
        {
            let mut tx = self.repo.start_transaction();
            tx.set_is_snapshot(true);
            let saved = tx
                .repo_mut()
                .rewrite_commit(&current)
                .set_tree(tree)
                .set_description(description)
                .write()
                .await
                .map_err(jj)?;
            tx.repo_mut().rebase_descendants().await.map_err(jj)?;
            tx.repo_mut().set_local_bookmark_target(
                RefName::new("lapis-backup"),
                RefTarget::normal(saved.id().clone()),
            );
            let next = tx.repo_mut().check_out(name, &saved).await.map_err(jj)?;
            self.repo = tx
                .commit("checkpoint saved vault files")
                .await
                .map_err(jj)?;
            checkpoint_id = Some(checkpoint(&saved, &self.repo));
            // Reset only the WC bookkeeping to the same captured tree. No source
            // file writes occur, including while recovering a prior publication.
            locked.locked_wc().reset(&next).await.map_err(jj)?;
        } else {
            locked.locked_wc().reset(&current).await.map_err(jj)?;
        }
        locked.finish(self.repo.op_id().clone()).await.map_err(jj)?;
        let id = self
            .repo
            .view()
            .get_wc_commit_id(self.workspace.workspace_name())
            .ok_or(Error::Binding)?;
        Ok(Capture {
            changed: checkpoint_id.is_some(),
            checkpoint: checkpoint_id,
            operation: self.repo.op_id().hex(),
            working_copy: id.hex(),
        })
    }
}

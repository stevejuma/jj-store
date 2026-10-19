//! Local object import uses the same isolated child policy as remote transfer.
use super::*;
use jj_lib::backend::CommitId;
impl Session {
    pub(in crate::vault_workspace) async fn import_existing_objects(
        &self,
        source: &Path,
        commit: &str,
    ) -> Result<()> {
        if !oid(commit) || !source.is_absolute() || source.canonicalize()? != source {
            return Err(Error::Binding);
        }
        let transfer = RemoteTransfer {
            binding: self.binding.clone(),
            endpoint: source.to_str().ok_or(Error::Binding)?.into(),
            branch: "existing-history".into(),
            git: git::get_git_backend(self.repo.store())
                .map_err(jj)?
                .git_repo_path()
                .to_owned(),
            credential: None,
            local: true,
        };
        let (status, _) = transfer
            .run(&[
                "fetch",
                "--no-tags",
                "--no-recurse-submodules",
                "--no-write-fetch-head",
                "--",
                &transfer.endpoint,
                commit,
            ])
            .await?;
        if status != Some(0) {
            return Err(Error::Jj("Existing repository object import failed".into()));
        }
        let id = CommitId::try_from_hex(commit).ok_or(Error::Binding)?;
        git::get_git_backend(self.repo.store())
            .map_err(jj)?
            .import_head_commits([&id])
            .map_err(jj)?;
        Ok(())
    }
}

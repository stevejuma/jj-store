use super::*;
impl Supervisor {
    pub(super) async fn authorized(
        &self,
        repository: RepositoryId,
        profile: &str,
        write: bool,
    ) -> Result<()> {
        if profile == "local_jj_git" {
            self.0
                .resources
                .folders
                .as_ref()
                .ok_or(Error::Unavailable)?
                .open(repository)
                .map_err(|_| Error::Denied)?;
        } else {
            self.native_request(repository)?
                .authorize(write)
                .await
                .map_err(|_| Error::Denied)?;
        }
        Ok(())
    }
    pub(super) async fn selections(&self, repository: Option<RepositoryId>) -> Result<Vec<Target>> {
        let mut candidates = Vec::new();
        if let Some(folders) = &self.0.resources.folders {
            for binding in folders.list().map_err(|_| Error::Unavailable)?.bindings {
                if repository.is_some_and(|r| r != binding.repository)
                    || binding.availability != jj_store_service::local_browser::Availability::Ready
                {
                    continue;
                }
                let (folder, _) = folders
                    .folder(binding.repository)
                    .map_err(|_| Error::Stale)?;
                candidates.push((
                    format!("folder-{}-{}", binding.repository, binding.generation),
                    binding.repository,
                    binding.name,
                    folder.path,
                    "local_jj_git",
                ));
            }
        }
        if let (Some(links), Some(verifier)) =
            (&self.0.resources.workspaces, &self.0.resources.verifier)
        {
            for entry in self
                .0
                .resources
                .registry
                .snapshot(None)
                .await
                .map_err(|_| Error::Unavailable)?
                .repositories
            {
                if repository.is_some_and(|r| r != entry.id) {
                    continue;
                }
                if self.authorized(entry.id, "native", true).await.is_err() {
                    continue;
                }
                for link in links
                    .list(entry.id)
                    .await
                    .map_err(|_| Error::Unavailable)?
                    .links
                {
                    if link.availability
                        != jj_store_service::local_management::WorkspaceAvailability::Ready
                    {
                        continue;
                    }
                    let verified = verifier.inspect(&link.path).map_err(|_| Error::Stale)?;
                    if verified.repository != entry.id
                        || verified.profile
                            != jj_store_service::local_management::WorkspaceProfile::Native
                    {
                        continue;
                    }
                    candidates.push((link.id, entry.id, entry.name.clone(), link.path, "native"));
                }
            }
        }
        let mut targets = Vec::new();
        for (id, repository, name, folder, profile) in candidates {
            if self.authorized(repository, profile, true).await.is_err() {
                continue;
            }
            let selected = self
                .0
                .resources
                .driver
                .inspect(PathBuf::from(&folder))
                .await?;
            targets.push(Target {
                id,
                repository,
                name,
                folder,
                profile: profile.into(),
                operation: selected.operation,
                commit: selected.commit,
            });
        }
        Ok(targets)
    }
}

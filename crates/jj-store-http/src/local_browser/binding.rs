//! Fresh read boundaries with bounded shared validation inside each traversal.
use super::*;
use local::Access as _;
pub(crate) struct Binding {
    manager: Arc<dyn management::LocalRepositories>,
    pub(super) folder: management::LocalFolder,
    generation: u64,
    admission: Option<Arc<management::admission::FolderAdmission>>,
}
impl Binding {
    async fn verify(&self, force: bool) -> local::Result<()> {
        if !force
            && self
                .admission
                .as_ref()
                .is_some_and(|a| a.cached(&self.folder, self.generation))
        {
            return Ok(());
        }
        let manager = self.manager.clone();
        let repo = self.repository();
        let folder = self.folder.clone();
        let generation = self.generation;
        let admission = self.admission.clone();
        tokio::task::spawn_blocking(move || {
            if let Some(admission) = admission {
                validate_binding(&admission, &folder, generation, force, || {
                    manager.folder(repo)
                })
            } else {
                let (current, current_generation) = manager.folder(repo)?;
                if current != folder || current_generation != generation {
                    return Err(management::Error::IdentityChanged);
                }
                Ok(())
            }
        })
        .await
        .map_err(|_| local::Error::Unavailable)?
        .map_err(|_| local::Error::Denied)
    }
}
/// One physical recheck after a watcher invalidates admission during inspection.
/// No command has been dispatched; identity/generation changes still fail closed.
fn validate_binding(
    admission: &management::admission::FolderAdmission,
    folder: &management::LocalFolder,
    generation: u64,
    force: bool,
    mut load: impl FnMut() -> management::Result<(management::LocalFolder, u64)>,
) -> management::Result<()> {
    let result = admission.validate(folder, generation, force, &mut load);
    if matches!(result, Err(management::Error::Stale)) {
        admission.validate(folder, generation, true, load)
    } else {
        result
    }
}
#[cfg(test)]
mod tests {
    use super::*;
    fn folder() -> management::LocalFolder {
        management::LocalFolder {
            path: "/repo".into(),
            metadata: "/repo/.jj/repo".into(),
            repository: None,
        }
    }
    #[test]
    fn watcher_race_rechecks_before_admitting_the_binding() {
        let admission = management::admission::FolderAdmission::default();
        let folder = folder();
        let mut reads = 0;
        validate_binding(&admission, &folder, 1, true, || {
            reads += 1;
            if reads == 1 {
                admission.invalidate();
            }
            Ok((folder.clone(), 1))
        })
        .unwrap();
        assert_eq!(reads, 2);
        assert!(admission.cached(&folder, 1));
    }
    #[test]
    fn recheck_refuses_changed_identity_and_continuous_invalidation() {
        let admission = management::admission::FolderAdmission::default();
        let folder = folder();
        let mut reads = 0;
        assert_eq!(
            validate_binding(&admission, &folder, 1, true, || {
                reads += 1;
                if reads == 1 {
                    admission.invalidate();
                    Ok((folder.clone(), 1))
                } else {
                    Ok((folder.clone(), 2))
                }
            }),
            Err(management::Error::IdentityChanged)
        );
        assert!(!admission.cached(&folder, 1));
        assert_eq!(
            validate_binding(&admission, &folder, 1, true, || {
                admission.invalidate();
                Ok((folder.clone(), 1))
            }),
            Err(management::Error::Stale)
        );
        assert!(!admission.cached(&folder, 1));
    }
}
#[async_trait::async_trait]
impl local::Access for Binding {
    fn repository(&self) -> m::RepositoryId {
        self.folder.repository.expect("registered identity")
    }
    fn generation(&self) -> u64 {
        self.generation
    }
    fn metadata_path(&self) -> &std::path::Path {
        std::path::Path::new(&self.folder.metadata)
    }
    async fn authorize(&self) -> local::Result<()> {
        self.verify(true).await
    }
    async fn check(&self) -> local::Result<()> {
        self.verify(false).await
    }
}
pub(crate) async fn binding(
    state: &State,
    repo: m::RepositoryId,
) -> Result<Arc<dyn local::Access>> {
    Ok(bound_folder(state, repo).await?)
}
pub(super) async fn bound_folder(state: &State, repo: m::RepositoryId) -> Result<Arc<Binding>> {
    let manager = manager(state)?;
    let check = manager.clone();
    let (folder, generation) = blocking(state, move || check.folder(repo).map_err(map)).await?;
    let admission = manager.read_admission(repo);
    Ok(Arc::new(Binding {
        manager,
        folder,
        generation,
        admission,
    }))
}

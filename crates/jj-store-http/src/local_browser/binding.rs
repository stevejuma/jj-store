//! Fresh read boundaries with bounded shared validation inside each traversal.
use super::*;
use local::Access as _;
pub(crate) struct Binding {
    manager: Arc<dyn management::LocalRepositories>,
    folder: management::LocalFolder,
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
                admission.validate(&folder, generation, force, || manager.folder(repo))
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

//! Concrete JJ provider stays behind the owned workspace interface.
use super::*;
pub struct Driver(pub jj_store_jj::agent_workspaces::Provider);
fn selection(value: &Workspace) -> jj_store_jj::agent_workspaces::Selection {
    jj_store_jj::agent_workspaces::Selection {
        root: value.path.clone(),
        metadata: value.metadata.clone(),
        operation: value.operation,
        commit: value.commit.clone(),
        name: value.name.clone(),
    }
}
fn workspace(value: jj_store_jj::agent_workspaces::Selection, source: PathBuf) -> Workspace {
    Workspace {
        source,
        path: value.root,
        metadata: value.metadata,
        operation: value.operation,
        commit: value.commit,
        name: value.name,
    }
}
#[async_trait::async_trait]
impl WorkspaceDriver for Driver {
    fn ignore_inputs(&self, value: &Workspace) -> Result<Vec<Vec<u8>>> {
        self.0
            .ignore_inputs(&selection(value))
            .map_err(|_| Error::Unavailable)
    }
    async fn initialize_personal(&self, path: PathBuf) -> Result<Workspace> {
        self.0
            .initialize_personal(&path)
            .await
            .map(|w| workspace(w, path))
            .map_err(|_| Error::Unavailable)
    }
    async fn materialize(
        &self,
        repo: RepositoryId,
        path: PathBuf,
        socket: PathBuf,
    ) -> Result<Workspace> {
        self.0
            .materialize(repo, &path, &socket)
            .await
            .map(|w| workspace(w, path))
            .map_err(|_| Error::Unavailable)
    }
    async fn inspect(&self, path: PathBuf) -> Result<Workspace> {
        let selected = self
            .0
            .inspect(&path)
            .await
            .map_err(|_| Error::Unavailable)?;
        Ok(workspace(selected, path))
    }
    async fn verify(&self, value: &Workspace) -> Result<()> {
        let provider = self.0.clone();
        let value = selection(value);
        tokio::task::spawn_blocking(move || provider.verify(&value))
            .await
            .map_err(|_| Error::Unavailable)?
            .map_err(|_| Error::Stale)
    }
    async fn create(
        &self,
        source: Workspace,
        destination: PathBuf,
        operation: OperationId,
        commit: String,
        name: String,
    ) -> Result<Workspace> {
        let result = self
            .0
            .create(&selection(&source), &destination, operation, &commit, &name)
            .await
            .map_err(|_| Error::Unavailable)?;
        Ok(workspace(result, source.source))
    }
    async fn snapshot(&self, value: &Workspace) -> Result<Workspace> {
        let result = self
            .0
            .snapshot(&selection(value))
            .await
            .map_err(|_| Error::Unavailable)?;
        Ok(workspace(result, value.source.clone()))
    }
    async fn cleanup(&self, value: &Workspace) -> Result<()> {
        self.0
            .cleanup(&selection(value))
            .await
            .map_err(|_| Error::Busy)
    }
}

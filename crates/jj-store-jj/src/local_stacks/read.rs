//! Direct local inspection: one existing head, no snapshot or reconciliation.
use super::*;
use futures::{StreamExt as _, TryStreamExt as _};
use jj_lib::{matchers::EverythingMatcher, settings::UserSettings};

pub(super) async fn load(
    access: &Arc<dyn Access>,
    root: &std::path::Path,
) -> Result<(Workspace, Arc<ReadonlyRepo>, Config, UserSettings)> {
    access.authorize().await.map_err(|_| Error::Denied)?;
    let settings = settings::load(root)?;
    let workspace = Workspace::load(
        &settings,
        root,
        &default_backend_factories(),
        &default_working_copy_factories(),
    )
    .map_err(jj)?;
    if workspace.workspace_root() != root
        || workspace.repo_path().canonicalize().map_err(jj)?
            != access.metadata_path().canonicalize().map_err(jj)?
    {
        return Err(Error::Denied);
    }
    let loader = workspace.repo_loader();
    let heads = loader.op_heads_store().get_op_heads().await.map_err(jj)?;
    if heads.len() != 1 {
        return Err(Error::Stale);
    }
    let op = loader.load_operation(&heads[0]).await.map_err(jj)?;
    let repo = loader.load_at(&op).await.map_err(jj)?;
    let config = config::Storage::read_only(root)?;
    access.authorize().await.map_err(|_| Error::Denied)?;
    Ok((workspace, repo, config, settings))
}

/// An exact, read-only local session shared by CLI inspection and native guards.
pub struct ReadSession {
    access: Arc<dyn Access>,
    workspace: Workspace,
    repo: Arc<ReadonlyRepo>,
    config: Config,
    settings: UserSettings,
}
impl ReadSession {
    pub async fn open(access: Arc<dyn Access>, root: PathBuf) -> Result<Self> {
        let (workspace, repo, config, settings) = load(&access, &root).await?;
        Ok(Self {
            access,
            workspace,
            repo,
            config,
            settings,
        })
    }
    pub async fn state(&self) -> Result<State> {
        let result = state(
            &self.repo,
            &self.config,
            self.workspace.workspace_name(),
            None,
        )
        .await?;
        self.access.authorize().await.map_err(|_| Error::Denied)?;
        Ok(result)
    }
    pub fn operation(&self) -> jj_store_model::OperationId {
        self.repo
            .op_id()
            .hex()
            .parse()
            .expect("native operation identity")
    }
    pub fn revision(&self) -> u64 {
        self.config.revision
    }
    pub async fn resolve_revision(&self, text: &str) -> Result<String> {
        let expression = policy::expression(
            self.repo.as_ref(),
            self.workspace.workspace_name(),
            &self.settings,
            text,
        )?;
        let ids = expression
            .evaluate(self.repo.as_ref())
            .map_err(jj)?
            .stream()
            .take(2)
            .try_collect::<Vec<_>>()
            .await
            .map_err(jj)?;
        if ids.len() != 1 {
            return Err(Error::Invalid);
        }
        policy::commit(&self.repo, &ids[0].hex())?;
        self.access.authorize().await.map_err(|_| Error::Denied)?;
        Ok(ids[0].hex())
    }
    pub fn bookmark(&self, name: &str) -> Result<String> {
        Ok(resolve(self.repo.as_ref(), name)?.id().hex())
    }
    pub async fn topology(&self, target: &str) -> Result<Topology> {
        let applied = self
            .config
            .active()
            .map_or(&[][..], |w| w.applied.as_slice());
        let result = topology::discover(&self.repo, target, applied).await?;
        self.access.authorize().await.map_err(|_| Error::Denied)?;
        Ok(result)
    }
    /// Complete bounded bookmark facts for integrated/unavailable recovery choices.
    pub fn bookmarks(&self) -> Result<Vec<(String, Option<String>)>> {
        let result = self
            .repo
            .view()
            .local_bookmarks()
            .filter(|(name, _)| !name.as_str().starts_with(config::INTERNAL_PREFIX))
            .take(4097)
            .map(|(name, value)| {
                (
                    name.as_str().to_owned(),
                    value.as_normal().map(|id| id.hex()),
                )
            })
            .collect::<Vec<_>>();
        if result.len() > 4096 {
            return Err(Error::Invalid);
        }
        Ok(result)
    }
    pub async fn parent(&self, id: &str) -> Result<String> {
        let commit = policy::commit(&self.repo, id)?;
        if commit.parent_ids().len() != 1 {
            return Err(Error::Invalid);
        }
        Ok(commit.parent_ids()[0].hex())
    }
    /// Exact changed paths of a single native commit, never a net branch selection.
    pub async fn changed_paths(&self, id: &str) -> Result<Vec<String>> {
        let commit = policy::commit(&self.repo, id)?;
        let before = commit.parent_tree(self.repo.as_ref()).await.map_err(jj)?;
        let after = commit.tree();
        let mut stream = before.diff_stream(&after, &EverythingMatcher);
        let mut paths = vec![];
        while let Some(entry) = stream.next().await {
            entry.values.map_err(jj)?;
            paths.push(entry.path.as_internal_file_string().to_owned());
            if paths.len() > 256 {
                return Err(Error::Invalid);
            }
            self.access.authorize().await.map_err(|_| Error::Denied)?;
        }
        Ok(paths)
    }
}

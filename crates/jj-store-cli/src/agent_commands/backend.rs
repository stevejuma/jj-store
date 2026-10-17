use super::*;
use jj_store_model::{RepositoryId, RequestId};
use jj_store_service::local_repository::{LocalAgentCommand, LocalAgentRequest, LocalRepository};

pub(super) enum Backend {
    Native(crate::binding::Connection),
    Local(LocalRepository),
}
pub(super) fn workspace_root(cwd: &Path) -> Result<PathBuf, &'static str> {
    let cwd = cwd
        .canonicalize()
        .map_err(|_| "workspace path unavailable")?;
    cwd.ancestors()
        .find(|path| path.join(".jj").exists())
        .map(Path::to_owned)
        .ok_or("agent commands require a JJ workspace")
}
pub(super) fn default_state(workspace: &Path, repository: Option<RepositoryId>) -> PathBuf {
    let digest = Sha256::digest(workspace.as_os_str().as_encoded_bytes());
    dirs::data_local_dir()
        .unwrap_or_else(std::env::temp_dir)
        .join("jj-store/agent")
        .join(repository.map_or_else(|| "uninitialized".into(), |id| id.to_string()))
        .join(hex::encode(&digest[..16]))
}
impl Backend {
    pub(super) fn open(helper: &CommandHelper) -> Result<Self, CommandError> {
        let path = helper.workspace_loader()?.repo_path();
        if std::fs::read_to_string(path.join("store/type"))
            .map_err(error)?
            .trim()
            == "git"
        {
            let store = jj_store_jj::local_repository::metadata_store(helper.settings(), path)
                .map_err(error)?;
            Ok(Self::Local(LocalRepository::new(path, store)))
        } else {
            Ok(Self::Native(
                crate::binding::Connection::read(path).map_err(error)?,
            ))
        }
    }
    pub(super) fn repository(&self) -> Result<Option<RepositoryId>, CommandError> {
        match self {
            Self::Native(connection) => Ok(Some(connection.repository)),
            Self::Local(repo) => repo.repository().map_err(error),
        }
    }
    pub(super) async fn initialize(
        &self,
        request_id: RequestId,
    ) -> Result<serde_json::Value, CommandError> {
        let Self::Local(repo) = self else {
            return Err(error(
                "native AgentStore is initialized by the local service",
            ));
        };
        serde_json::to_value(
            repo.apply_agent_command(&LocalAgentRequest {
                version: 1,
                request_id,
                command: LocalAgentCommand::Initialize,
            })
            .await
            .map_err(error)?,
        )
        .map_err(error)
    }
    pub(super) async fn ensure_repository(&self) -> Result<RepositoryId, CommandError> {
        if let Some(id) = self.repository()? {
            return Ok(id);
        }
        let digest = Sha256::digest(b"jj-store/local-agent-initialization/v1");
        let id = RequestId::new(digest[..16].try_into().expect("digest prefix"));
        let result = self.initialize(id).await?;
        if result["status"] != "committed" {
            return Err(error(format!(
                "local agent initialization outcome unknown; retry with request {id}"
            )));
        }
        self.repository()?
            .ok_or_else(|| error("local metadata identity unavailable"))
    }
    pub(super) async fn publish_pending(&self, state: &Path) {
        match self {
            Self::Native(connection) => {
                crate::agent_hook::publish_pending_best_effort(state, connection).await
            }
            Self::Local(repo) => {
                let publish = async {
                    let path =
                        jj_store_server::policy::outside_checkout(state)?.join("capture/spool");
                    let mut spool =
                        crate::agent_capture::AgentCaptureSpool::open(&path, 64 * 1024 * 1024)?;
                    crate::local_agent_publication::drain_local_capture_spool(&mut spool, repo)
                        .await?;
                    jj_store_jj::operation_stores::AuthorityResult::Ok(())
                };
                let _ = tokio::time::timeout(std::time::Duration::from_secs(3), publish).await;
            }
        }
    }
    pub(super) async fn local_action(
        &self,
        command: &AgentCommand,
        state: &Path,
    ) -> Result<Option<serde_json::Value>, CommandError> {
        match command {
            AgentCommand::Initialize { request_id } => {
                Ok(Some(self.initialize(*request_id).await?))
            }
            AgentCommand::Sessions { limit, cursor } => {
                let Self::Local(repo) = self else {
                    return Err(error(
                        "agent sessions currently lists the local Git profile",
                    ));
                };
                Ok(Some(
                    serde_json::to_value(
                        repo.agent_sessions(*limit, cursor.as_deref())
                            .await
                            .map_err(error)?,
                    )
                    .map_err(error)?,
                ))
            }
            AgentCommand::Receipt { request_id } => {
                let Self::Local(repo) = self else {
                    return Err(error(
                        "agent receipt currently reads local Git capture receipts",
                    ));
                };
                Ok(Some(
                    serde_json::to_value(
                        repo.agent_store_status(*request_id).await.map_err(error)?,
                    )
                    .map_err(error)?,
                ))
            }
            AgentCommand::Drain => {
                let path = jj_store_server::policy::outside_checkout(state)
                    .map_err(error)?
                    .join("capture/spool");
                let mut spool =
                    crate::agent_capture::AgentCaptureSpool::open(&path, 64 * 1024 * 1024)
                        .map_err(error)?;
                let result = match self {
                    Self::Local(repo) => {
                        crate::local_agent_publication::drain_local_capture_spool(&mut spool, repo)
                            .await
                            .map_err(error)?
                    }
                    Self::Native(connection) => crate::agent_publication::drain_capture_spool(
                        &mut spool,
                        connection.repository,
                        &connection.connect().await.map_err(error)?,
                    )
                    .await
                    .map_err(error)?,
                };
                Ok(Some(
                    serde_json::json!({"committed":result.committed,"remaining":result.remaining,"outcome_unknown":result.outcome_unknown}),
                ))
            }
            _ => Ok(None),
        }
    }
    pub(super) fn uninitialized_status(&self) -> Result<serde_json::Value, CommandError> {
        let search = crate::agent_probe::environment_search_paths();
        let statuses = [
            IntegrationId::Codex,
            IntegrationId::Cursor,
            IntegrationId::OpenCode,
        ]
        .into_iter()
        .map(|id| crate::agent_probe::probe(id, &search).map(|value| (id, value)))
        .collect::<Result<std::collections::BTreeMap<_, _>, _>>()
        .map_err(error)?;
        Ok(
            serde_json::json!({"version":2,"profile":"local_jj_git","repository":null,"initialized":false,"integrations":statuses}),
        )
    }
}

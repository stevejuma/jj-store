//! Backend-aware command dispatch; native V1 wire output remains unchanged.
use super::*;
use crate::{binding::Connection, commands::error};
use jj_store_service::local_repository::{LocalRepository, LocalWorkPage};
use jj_store_service::work::WorkPage;
use serde::Serialize;

pub(super) enum Client {
    Native {
        repository: RepositoryId,
        client: Box<jj_store_rpc::client::NativeClient>,
    },
    Local(LocalRepository),
}
#[derive(Serialize)]
#[serde(untagged)]
pub(super) enum Page {
    Native(WorkPage),
    Local(LocalWorkPage),
}
impl Page {
    pub(super) fn rows(
        &self,
    ) -> (
        &[jj_store_service::work::ProjectV1],
        &[jj_store_service::work::TaskV1],
        &[jj_store_service::work::WorkCommentV1],
        Option<&str>,
    ) {
        match self {
            Self::Native(page) => (
                &page.projects,
                &page.tasks,
                &page.comments,
                page.next_cursor.as_deref(),
            ),
            Self::Local(page) => (
                &page.projects,
                &page.tasks,
                &page.comments,
                page.next_cursor.as_deref(),
            ),
        }
    }
    pub(super) fn summary(&self) -> String {
        match self {
            Self::Native(page) => format!(
                "WorkStore {} version {} (publication {})\n",
                page.snapshot.repo_id,
                page.snapshot.work_state_version,
                page.snapshot.published_seq.0
            ),
            Self::Local(page) => format!(
                "Local JJ WorkStore {} version {} (metadata {})\n",
                page.snapshot
                    .repo_id
                    .map_or_else(|| "not initialized".into(), |id| id.to_string()),
                page.snapshot.work_state_version,
                page.snapshot
                    .metadata_commit
                    .map_or_else(|| "absent".into(), |id| id.to_string())
            ),
        }
    }
}
impl Client {
    pub(super) async fn open(helper: &CommandHelper) -> Result<Self, CommandError> {
        let path = helper.workspace_loader()?.repo_path();
        let backend = std::fs::read_to_string(path.join("store/type")).map_err(error)?;
        if backend.trim() == "git" {
            let store = jj_store_jj::local_repository::metadata_store(helper.settings(), path)
                .map_err(error)?;
            return Ok(Self::Local(LocalRepository::new(path, store)));
        }
        let connection = Connection::read(path).map_err(error)?;
        let client = connection.connect().await.map_err(error)?;
        Ok(Self::Native {
            repository: connection.repository,
            client: Box::new(client),
        })
    }
    pub(super) fn entity_id(&self, request: RequestId, kind: &[u8]) -> [u8; 16] {
        match self {
            Self::Native { repository, .. } => super::entity_id(*repository, request, kind),
            Self::Local(_) => {
                let mut hash = Sha256::new_with_prefix(b"jj-store/local-work-cli-entity/v1\0");
                hash.update(request.as_bytes());
                hash.update(kind);
                let mut bytes = [0; 16];
                bytes.copy_from_slice(&hash.finalize()[..16]);
                bytes
            }
        }
    }
    pub(super) async fn work_page(&self, query: &WorkQuery) -> Result<Page, CommandError> {
        match self {
            Self::Native { client, .. } => {
                Ok(Page::Native(client.work_page(query).await.map_err(error)?))
            }
            Self::Local(repository) => Ok(Page::Local(repository.work_page(query).map_err(error)?)),
        }
    }
    pub(super) async fn apply_work_command(
        &self,
        request: &Request,
    ) -> Result<serde_json::Value, CommandError> {
        match self {
            Self::Native { client, .. } => {
                serde_json::to_value(client.apply_work_command(request).await.map_err(error)?)
                    .map_err(error)
            }
            Self::Local(repository) => serde_json::to_value(
                repository
                    .apply_work_command_async(request)
                    .await
                    .map_err(error)?,
            )
            .map_err(error),
        }
    }
    pub(super) async fn work_command_status(
        &self,
        request: RequestId,
    ) -> Result<serde_json::Value, CommandError> {
        match self {
            Self::Native { client, .. } => {
                serde_json::to_value(client.work_command_status(request).await.map_err(error)?)
                    .map_err(error)
            }
            Self::Local(repository) => {
                serde_json::to_value(repository.work_store_status(request).await.map_err(error)?)
                    .map_err(error)
            }
        }
    }
}

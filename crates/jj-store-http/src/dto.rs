//! Transport-owned JSON values. Handler serialization and schemas share these types.
use crate::{Access, BackupStatus, Config, q, reads};
use jj_store_model::RepositoryId;
use jj_store_service::registry::EffectivePermission;
use schemars::JsonSchema;
use serde::Serialize;

#[derive(Serialize, JsonSchema)]
pub(crate) struct Capabilities {
    pub version: u32,
    pub native_profile: &'static str,
    pub authentication: &'static str,
    pub read_api: bool,
    pub browsing_windows: bool,
    pub index_tasks: bool,
    pub events: bool,
    pub search: bool,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub local_workspace_links: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub local_folder_picker: Option<bool>,
    pub typed_mutations: Vec<String>,
    pub mutation_request_bytes: usize,
    pub conflict_upload_bytes: u64,
    pub conflict_resolution_actions: Vec<String>,
    pub event_limits: EventLimits,
    pub limits: ReadLimits,
    pub log_context: LogContext,
}

#[derive(Serialize, JsonSchema)]
#[serde(rename_all = "snake_case")]
pub(crate) enum AccessMode {
    Authenticated,
    LocalTrusted,
}

#[derive(Serialize, JsonSchema)]
pub(crate) struct StorageState {
    pub ready: bool,
}

#[derive(Serialize, JsonSchema)]
pub(crate) struct Bootstrap {
    pub version: u32,
    pub access_mode: AccessMode,
    pub capabilities: Capabilities,
    pub storage: StorageState,
    pub backups: Vec<BackupStatus>,
}

impl From<&Config> for Bootstrap {
    fn from(config: &Config) -> Self {
        let access_mode = match &config.access {
            Access::Authenticated => AccessMode::Authenticated,
            Access::LocalTrusted { .. } => AccessMode::LocalTrusted,
        };
        Self {
            version: 1,
            access_mode,
            capabilities: Capabilities::from(config),
            storage: StorageState {
                ready: config.storage_ready,
            },
            backups: config.backups.clone(),
        }
    }
}

#[derive(Serialize, JsonSchema)]
pub(crate) struct RepositorySummary {
    pub id: RepositoryId,
    pub name: String,
    pub permission: EffectivePermission,
    pub principal: jj_store_model::PrincipalId,
}

#[derive(Serialize, JsonSchema)]
pub(crate) struct RepositoryPage {
    pub version: u32,
    pub generation: u64,
    pub repositories: Vec<RepositorySummary>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_cursor: Option<String>,
}
#[derive(Serialize, JsonSchema)]
pub(crate) struct EventLimits {
    pub streams: usize,
    pub replay_window: u64,
    pub page: usize,
    pub poll_millis: u128,
}
#[derive(Serialize, JsonSchema)]
pub(crate) struct ReadLimits {
    pub requests: usize,
    pub workers: usize,
    pub deadline_millis: u128,
    pub json_bytes: usize,
    pub page: usize,
    pub index_commits: usize,
    pub inline_index_commits: usize,
    pub file_bytes: u64,
}
#[derive(Serialize, JsonSchema)]
pub(crate) struct LogContext {
    pub at_time_millis: &'static str,
    pub workspace: &'static str,
    pub user_email: &'static str,
}
impl From<&Config> for Capabilities {
    fn from(config: &Config) -> Self {
        Self {
            version: 1,
            native_profile: reads::NATIVE_PROFILE,
            authentication: match config.access {
                Access::Authenticated => "repository_bearer",
                Access::LocalTrusted { .. } => "local_trusted",
            },
            local_workspace_links: (config.local_workspaces.is_some()
                && matches!(
                    config.access,
                    Access::LocalTrusted {
                        permission: jj_store_service::auth::Permission::Admin,
                        ..
                    }
                ))
            .then_some(true),
            local_folder_picker: (config.local_workspaces.is_some()
                && config.local_folder_picker.is_some()
                && matches!(
                    config.access,
                    Access::LocalTrusted {
                        permission: jj_store_service::auth::Permission::Admin,
                        ..
                    }
                ))
            .then_some(true),
            read_api: true,
            browsing_windows: true,
            index_tasks: true,
            events: true,
            search: false,
            typed_mutations: [
                "new",
                "describe",
                "bookmark_create",
                "bookmark_move",
                "bookmark_delete",
                "reconcile_operations",
                "abandon",
                "resolve_conflicts",
                "edit_file",
            ]
            .map(str::to_owned)
            .to_vec(),
            mutation_request_bytes: crate::mutations::MAX_REQUEST_BYTES,
            conflict_upload_bytes: config.uploads.max_file_bytes,
            conflict_resolution_actions: ["select_term", "delete", "uploaded_file"]
                .map(str::to_owned)
                .to_vec(),
            event_limits: EventLimits {
                streams: config.event_concurrency,
                replay_window: config.events.replay_window,
                page: config.events.page_size,
                poll_millis: config.events.poll_interval.as_millis(),
            },
            limits: ReadLimits {
                requests: config.concurrency,
                workers: config.workers,
                deadline_millis: config.timeout.as_millis(),
                json_bytes: config.max_json_bytes,
                page: config.graph.max_page,
                index_commits: config.graph.max_index_commits,
                inline_index_commits: config.graph.max_inline_rebuild_commits,
                file_bytes: config.content.max_blob_bytes,
            },
            log_context: LogContext {
                at_time_millis: "required",
                workspace: "explicit when needed",
                user_email: "explicit for identity-relative expressions",
            },
        }
    }
}
#[derive(Serialize, JsonSchema)]
pub(crate) struct OperationDetail {
    pub operation: q::content::Object,
    pub view: q::content::Object,
}
#[derive(Serialize, JsonSchema)]
pub(crate) struct CommitDetail {
    pub object: q::content::Object,
    pub commit: q::Commit,
}
#[derive(Serialize, JsonSchema)]
pub(crate) struct ErrorBody {
    pub version: u32,
    pub code: &'static str,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub snapshot: Option<Box<reads::ReadSnapshot>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub heads: Option<Box<reads::HeadSnapshot>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub search_status: Option<Box<jj_store_tantivy::paging::Status>>,
}

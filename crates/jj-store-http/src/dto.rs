//! Transport-owned JSON values. Handler serialization and schemas share these types.
use crate::{Config, q, reads};
use schemars::JsonSchema;
use serde::Serialize;

#[derive(Serialize, JsonSchema)]
pub(crate) struct Capabilities {
    pub version: u32,
    pub native_profile: &'static str,
    pub authentication: &'static str,
    pub read_api: bool,
    pub index_tasks: bool,
    pub events: bool,
    pub search: bool,
    pub typed_mutations: Vec<String>,
    pub event_limits: EventLimits,
    pub limits: ReadLimits,
    pub log_context: LogContext,
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
            authentication: "repository_bearer",
            read_api: true,
            index_tasks: true,
            events: true,
            search: false,
            typed_mutations: vec![],
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
}

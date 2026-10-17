//! Shared read accounting and cancellation.
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Default, PartialEq, Eq, Serialize, Deserialize, schemars::JsonSchema)]
#[schemars(rename = "GraphStats")]
pub struct Stats {
    pub metadata_nodes: u64,
    pub predicate_file_bytes: u64,
    pub predicate_lines: u64,
    pub object_reads: u64,
    pub object_bytes: u64,
    pub work: u64,
}
/// Cancellation is separate from dropping a request: a task owner can signal it
/// and still await actual worker exit before reporting completion.
#[derive(Clone, Default)]
pub struct Cancellation(tokio_util::sync::CancellationToken);
impl Cancellation {
    pub(crate) fn token(&self) -> tokio_util::sync::CancellationToken {
        self.0.clone()
    }
    pub async fn cancelled(&self) {
        self.0.cancelled().await;
    }
    pub fn is_cancelled(&self) -> bool {
        self.0.is_cancelled()
    }
    pub fn cancel(&self) {
        self.0.cancel();
    }
}

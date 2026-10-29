//! Owned query diagnostics and safe errors; no storage dependency.
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize, JsonSchema)]
pub struct Diagnostic {
    pub kind: String,
    pub message: String,
}
impl std::fmt::Display for Diagnostic {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str(&self.message)
    }
}
#[derive(Clone, Debug, PartialEq, Eq, Serialize, thiserror::Error, schemars::JsonSchema)]
#[schemars(rename = "GraphError")]
#[serde(rename_all = "snake_case")]
pub enum Error {
    #[error("query request was canceled")]
    Canceled,
    #[error("blame progress expired; restart the same selection")]
    BlameReset,
    #[error("query is not authorized")]
    Denied,
    #[error("query object is not committed in this snapshot")]
    NotCommitted,
    #[error("repository epoch changed")]
    EpochChanged,
    #[error("query resource limit exceeded")]
    Limit,
    #[error("query cache or worker is busy")]
    Busy,
    #[error("native index exceeds the configured rebuild budget")]
    IndexBudget,
    #[error("native index requires an explicit preparation task")]
    IndexRebuildRequired,
    #[error("invalid query or missing query context")]
    Invalid,
    #[error("{0}")]
    InvalidQuery(Diagnostic),
    #[error("native query or cache is unavailable")]
    Unavailable,
    #[error("committed native object is corrupt")]
    Corrupt,
    #[error("committed native history is incomplete")]
    Incomplete,
    #[error("repository path is absent")]
    NotFound,
    #[error("repository path is not an unambiguous directory")]
    NotDirectory,
    #[error("file range is invalid or unsatisfiable")]
    Range,
}
pub type Result<T> = std::result::Result<T, Error>;

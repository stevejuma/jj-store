//! Session-bound inspection contracts. Physical observations are not publication proofs.
pub mod files;
use super::*;
pub use jj_store_jj::agent_workspaces::inspection::{WorkspaceReference, WorkspaceRevision};

#[derive(Clone, Copy, Debug, PartialEq, Eq, Serialize, Deserialize, JsonSchema)]
#[serde(rename_all = "snake_case")]
pub enum ComparisonRange {
    SinceLastCommit,
    LastTurn,
    Branch,
}
#[derive(Clone, Debug, Serialize, Deserialize, JsonSchema)]
#[serde(tag = "view", rename_all = "snake_case", deny_unknown_fields)]
pub enum InspectionView {
    Files,
    Changes {
        range: ComparisonRange,
        base: Option<String>,
    },
}
#[derive(Clone, Debug, Serialize, Deserialize, JsonSchema)]
#[serde(deny_unknown_fields)]
#[schemars(rename = "AgentWorkspaceInspectionRequest")]
pub struct Request {
    pub version: u32,
    pub session: String,
    pub repository: RepositoryId,
    pub query: InspectionView,
    pub path: Option<String>,
    pub cursor: Option<String>,
}
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize, JsonSchema)]
#[serde(deny_unknown_fields)]
#[schemars(rename = "AgentWorkspaceFileEntry")]
pub struct FileEntry {
    pub path: String,
    pub kind: String,
    pub bytes: u64,
    pub fingerprint: String,
    pub status: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub additions: Option<u64>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub deletions: Option<u64>,
}
#[derive(Clone, Debug, Serialize, Deserialize, JsonSchema)]
#[serde(deny_unknown_fields)]
#[schemars(rename = "AgentWorkspaceFileContent")]
pub struct FileContent {
    pub path: String,
    pub fingerprint: String,
    pub contents: Option<String>,
    pub before: Option<String>,
    pub unavailable: Option<String>,
}
#[derive(Clone, Debug, Serialize, Deserialize, JsonSchema)]
#[serde(deny_unknown_fields)]
#[schemars(rename = "AgentWorkspaceComparison")]
pub struct Comparison {
    pub range: ComparisonRange,
    pub before_operation: Option<OperationId>,
    pub before_commit: Option<String>,
    pub after_operation: OperationId,
    pub after_commit: String,
    pub current_files: bool,
    pub request_id: Option<String>,
    pub unavailable: Option<String>,
}
#[derive(Clone, Debug, Serialize, Deserialize, JsonSchema)]
#[serde(deny_unknown_fields)]
#[schemars(rename = "AgentWorkspaceInspection")]
pub struct Inspection {
    pub version: u32,
    pub session: String,
    pub repository: RepositoryId,
    pub workspace: String,
    pub location: String,
    pub operation: OperationId,
    pub commit: String,
    pub selection: String,
    pub references: Vec<WorkspaceReference>,
    pub comparison: Option<Comparison>,
    pub incomplete: Option<String>,
    pub files: Vec<FileEntry>,
    pub next: Option<String>,
    pub file: Option<FileContent>,
}
impl Request {
    pub fn validate(&self) -> Result<()> {
        if self.version != 1
            || self.session.parse::<RequestId>().is_err()
            || self
                .path
                .as_deref()
                .is_some_and(|p| !jj_store_jj::agent_workspaces::inspection::safe_path(p))
            || self.cursor.as_ref().is_some_and(|c| c.len() > 128)
        {
            return Err(Error::Invalid);
        }
        if let InspectionView::Changes {
            base: Some(base), ..
        } = &self.query
            && !valid_commit(base)
        {
            return Err(Error::Invalid);
        }
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn legacy_file_entries_keep_unknown_counts_and_new_counts_round_trip() {
        let old = serde_json::json!({"path":"a.txt","kind":"file","bytes":1,"fingerprint":"old","status":"modified"});
        let mut entry: FileEntry = serde_json::from_value(old.clone()).unwrap();
        assert_eq!((entry.additions, entry.deletions), (None, None));
        assert_eq!(serde_json::to_value(&entry).unwrap(), old);
        entry.additions = Some(2);
        entry.deletions = Some(1);
        let value = serde_json::to_value(&entry).unwrap();
        assert_eq!(value["additions"], 2);
        assert_eq!(serde_json::from_value::<FileEntry>(value).unwrap(), entry);
    }
}

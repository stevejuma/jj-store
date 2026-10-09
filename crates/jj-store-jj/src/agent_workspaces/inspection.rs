//! Bounded immutable reads for an already verified agent checkout.
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize, schemars::JsonSchema)]
pub struct WorkspaceReference {
    pub name: String,
    pub commit: String,
}
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize, schemars::JsonSchema)]
pub struct WorkspaceRevision {
    pub commit: String,
    pub parents: Vec<String>,
    pub references: Vec<WorkspaceReference>,
    pub default_base: Option<String>,
}
pub fn safe_path(path: &str) -> bool {
    !path.is_empty()
        && path.len() <= 4096
        && !path.contains('\\')
        && !path.chars().any(char::is_control)
        && path.split('/').all(|p| {
            !p.is_empty()
                && p != "."
                && p != ".."
                && !p.eq_ignore_ascii_case(".jj")
                && !p.eq_ignore_ascii_case(".git")
        })
}

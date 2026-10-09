//! Frozen physical observations awaiting an authorized snapshot and ReviewStore publication.
use crate::*;
use jj_review::{DiffSide, ReviewDigest, TextSelectionV1};
use jj_store_model::{OperationId, RepositoryId, RequestId};
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize, schemars::JsonSchema)]
#[serde(deny_unknown_fields)]
pub struct WorkspaceFileSourceV1 {
    pub session: RequestId,
    pub repository: RepositoryId,
    pub workspace: String,
    pub path: String,
    pub operation: OperationId,
    pub commit: String,
    pub content_sha256: ReviewDigest,
    pub current_fingerprint: Option<String>,
    pub selection: Option<TextSelectionV1>,
    pub diff: Option<WorkspaceFileDiffV1>,
}
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize, schemars::JsonSchema)]
#[serde(deny_unknown_fields)]
pub struct WorkspaceFileDiffV1 {
    pub before_operation: OperationId,
    pub before_commit: String,
    pub after_operation: OperationId,
    pub after_commit: String,
    pub side: DiffSide,
}
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize, schemars::JsonSchema)]
#[serde(deny_unknown_fields)]
pub struct WorkspaceCommentDraftV1 {
    pub id: RequestId,
    pub source: WorkspaceFileSourceV1,
    pub body: String,
    /// Existing exact revision; append to its discussion, or edit it explicitly.
    pub previous: Option<ReactionTargetV1>,
    #[serde(default)]
    pub edit: bool,
}
impl WorkspaceCommentDraftV1 {
    pub fn validate(&self) -> Result<()> {
        let source = &self.source;
        let path = source.path.as_str();
        let valid_id = |s: &str| {
            matches!(s.len(), 40 | 128)
                && s.bytes()
                    .all(|b| b.is_ascii_digit() || (b'a'..=b'f').contains(&b))
        };
        if self.body.trim().is_empty()
            || self.body.len() > 32768
            || self.body.contains('\0')
            || path.is_empty()
            || path.len() > 4096
            || path.contains('\\')
            || path.chars().any(char::is_control)
            || path.split('/').any(|s| {
                s.is_empty()
                    || matches!(s, "." | "..")
                    || s.eq_ignore_ascii_case(".git")
                    || s.eq_ignore_ascii_case(".jj")
            })
            || source.workspace.is_empty()
            || source.workspace.len() > 256
            || source.workspace.chars().any(char::is_control)
            || !valid_id(&source.commit)
            || source
                .current_fingerprint
                .as_ref()
                .is_some_and(|s| s.len() != 64 || !s.bytes().all(|b| b.is_ascii_hexdigit()))
        {
            return Err(Error::Invalid("workspace comment source"));
        }
        if let Some(diff) = &source.diff {
            if !valid_id(&diff.before_commit) || !valid_id(&diff.after_commit) {
                return Err(Error::Invalid("workspace diff source"));
            }
            let (op, commit) = match diff.side {
                DiffSide::Before => (diff.before_operation, &diff.before_commit),
                DiffSide::After => (diff.after_operation, &diff.after_commit),
            };
            if op != source.operation
                || *commit != source.commit
                || (diff.side == DiffSide::Before && source.current_fingerprint.is_some())
            {
                return Err(Error::Invalid("workspace diff side"));
            }
        }
        if let Some(selection) = &source.selection {
            selection
                .validate()
                .map_err(|_| Error::Invalid("workspace text selection"))?;
        }
        if let Some(previous) = &self.previous {
            if !matches!(previous, ReactionTargetV1::RepositoryReviewComment { repository, .. } if *repository == source.repository)
            {
                return Err(Error::Invalid("workspace review repository"));
            }
            previous.validate()?;
        } else if self.edit {
            return Err(Error::Invalid("workspace comment edit"));
        }
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    fn draft() -> WorkspaceCommentDraftV1 {
        WorkspaceCommentDraftV1 {
            id: RequestId::new([1; 16]),
            body: "Keep this".into(),
            previous: None,
            edit: false,
            source: WorkspaceFileSourceV1 {
                session: RequestId::new([2; 16]),
                repository: RepositoryId::new([3; 16]),
                workspace: "agent-fixture".into(),
                path: "src/café.rs".into(),
                operation: OperationId::new([4; 64]),
                commit: "ab".repeat(20),
                content_sha256: ReviewDigest::sha256("🦀".as_bytes()),
                current_fingerprint: Some("cd".repeat(32)),
                selection: None,
                diff: None,
            },
        }
    }
    #[test]
    fn draft_paths_revisions_and_diff_sides_are_explicit() {
        let valid = draft();
        valid.validate().unwrap();
        for path in [
            "/etc/passwd",
            "../source",
            "a/.git/config",
            "a/.JJ/repo",
            "a\\b",
            "a//b",
        ] {
            let mut v = valid.clone();
            v.source.path = path.into();
            assert!(v.validate().is_err(), "{path}");
        }
        let mut v = valid.clone();
        v.edit = true;
        assert!(v.validate().is_err());
        let mut v = valid;
        v.source.diff = Some(WorkspaceFileDiffV1 {
            before_operation: v.source.operation,
            before_commit: v.source.commit.clone(),
            after_operation: v.source.operation,
            after_commit: "ef".repeat(20),
            side: DiffSide::Before,
        });
        assert!(
            v.validate().is_err(),
            "historical before side cannot be current bytes"
        );
        v.source.current_fingerprint = None;
        v.validate().unwrap();
        v.source.commit = "ef".repeat(20);
        assert!(v.validate().is_err());
    }
}

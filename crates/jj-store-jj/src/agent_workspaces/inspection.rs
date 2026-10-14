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

/// Exact counts using the native bounded line diff; refusal is unknown, never zero.
pub fn text_change_counts(before: &str, after: &str) -> Option<(u64, u64)> {
    if before.len() > 256 * 1024
        || after.len() > 256 * 1024
        || before.contains('\0')
        || after.contains('\0')
    {
        return None;
    }
    let spans = crate::line_diff::bounded(before, after, 2_000_000, |_| Ok::<_, ()>(())).ok()??;
    let (mut additions, mut deletions) = (0, 0);
    for span in spans {
        if span.kind == jj_lib::diff::DiffHunkKind::Different {
            additions += after[span.after].lines().count() as u64;
            deletions += before[span.before].lines().count() as u64;
        }
    }
    Some((additions, deletions))
}

/// Capture immutable code evidence from an authorized operation, without snapshotting.
pub async fn evidence(
    metadata: &std::path::Path,
    repository: jj_store_model::RepositoryId,
    operation: jj_store_model::OperationId,
) -> crate::local_metadata::Result<crate::local_evidence::GitEvidenceCapsuleV1> {
    let metadata = metadata.to_owned();
    tokio::task::spawn_blocking(move || {
        let settings = jj_lib::settings::UserSettings::from_config(
            jj_lib::config::StackedConfig::with_defaults(),
        )
        .map_err(|_| crate::local_metadata::Error::Invalid("settings"))?;
        tokio::runtime::Builder::new_current_thread()
            .enable_all()
            .build()
            .map_err(crate::local_metadata::Error::other)?
            .block_on(crate::local_evidence::inspect(
                &settings, &metadata, repository, operation,
            ))
    })
    .await
    .map_err(crate::local_metadata::Error::other)?
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn text_counts_keep_shared_lines_and_refuse_unsupported_content() {
        assert_eq!(
            text_change_counts("same\nold\nend\n", "same\nnew\nextra\nend\n"),
            Some((2, 1))
        );
        assert_eq!(text_change_counts("", "first\nlast"), Some((2, 0)));
        assert_eq!(text_change_counts("removed\r\n", ""), Some((0, 1)));
        assert_eq!(text_change_counts("", ""), Some((0, 0)));
        assert_eq!(text_change_counts("\0", "text"), None);
        assert_eq!(text_change_counts("", &"x".repeat(256 * 1024 + 1)), None);
        assert_eq!(
            text_change_counts(&"old\n".repeat(2000), &"new\n".repeat(2000)),
            None
        );
    }
}

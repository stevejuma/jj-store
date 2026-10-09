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

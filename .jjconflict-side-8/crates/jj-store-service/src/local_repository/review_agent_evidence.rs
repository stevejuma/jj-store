//! Non-publishing validation against exact, committed AgentStore records.
use super::{LocalError, LocalRepository, LocalResult, MetadataCommit};
use jj_review::{ArtifactAnchorV2, ArtifactTargetV2};
use std::collections::BTreeMap;

impl LocalRepository {
    /// Inspect an agent review source at an explicit companion selection.
    /// This grants no write authorization and does not select a ReviewStore root.
    pub async fn validate_portable_agent_review(
        &self,
        commit: MetadataCommit,
        anchor: &ArtifactAnchorV2,
    ) -> LocalResult<()> {
        anchor.validate().map_err(|_| LocalError::Invalid)?;
        if !matches!(
            anchor.target,
            ArtifactTargetV2::AgentSession(_)
                | ArtifactTargetV2::AgentToolArtifact(_)
                | ArtifactTargetV2::AgentToolText(_)
        ) || anchor.created_context.publication_position.is_some()
            || anchor.created_context.code_state_version.is_some()
        {
            return Err(LocalError::Unsupported);
        }
        let selected = self.selected_metadata(commit)?;
        if selected.repository() != anchor.repository_id {
            return Err(LocalError::Invalid);
        }
        let store = self
            .agent_store_at(Some(&selected), selected.repository())
            .await?;
        if anchor
            .created_context
            .agent_root
            .is_some_and(|root| Some(root) != store.snapshot.agent_root)
        {
            return Err(LocalError::Stale);
        }
        let mut objects = BTreeMap::new();
        let mut bytes = 0usize;
        for reference in anchor
            .agent_dependencies()
            .map_err(|_| LocalError::Invalid)?
        {
            let value = store.object(reference).await?;
            bytes = bytes
                .checked_add(
                    serde_json::to_vec(&value)
                        .map_err(|_| LocalError::Corrupt)?
                        .len(),
                )
                .ok_or(LocalError::Limit)?;
            if bytes > 16 * 1024 * 1024 {
                return Err(LocalError::Limit);
            }
            objects.insert(reference, value);
        }
        anchor
            .validate_agent_records(selected.repository(), |reference| {
                objects
                    .get(&reference)
                    .cloned()
                    .ok_or(jj_review::Error::Invalid("unselected agent record"))
            })
            .map_err(|_| LocalError::Invalid)?;
        // A concurrent identity replacement or detached history cannot extend
        // the admitted source. Normal advancement keeps this explicit pin valid.
        self.selected_metadata(commit)?;
        Ok(())
    }
}

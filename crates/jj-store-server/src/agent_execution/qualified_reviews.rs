//! Cross-repository review resolution retains independent repository authority.
use super::*;
use jj_collab::{QualifiedReviewComment, QualifiedReviewRead, ReactionTargetV1};
use jj_store_service::review_model::ArtifactTargetV1;
#[async_trait::async_trait]
impl QualifiedReviewRead for Supervisor {
    async fn comment(
        &self,
        target: &ReactionTargetV1,
    ) -> jj_collab::Result<QualifiedReviewComment> {
        let ReactionTargetV1::RepositoryReviewComment { repository, .. } = target else {
            return Err(jj_collab::Error::Invalid("qualified review target"));
        };
        let local = self
            .0
            .resources
            .folders
            .as_ref()
            .and_then(|folders| folders.open(*repository).ok());
        let profile = if local.is_some() {
            "local_jj_git"
        } else {
            "native"
        };
        self.authorized(*repository, profile, false)
            .await
            .map_err(|_| jj_collab::Error::TargetUnavailable)?;
        let value = if let Some(local) = local {
            local.qualified_file_comment(target).await?
        } else {
            let session = self
                .home_session(*repository)
                .map_err(|_| jj_collab::Error::TargetUnavailable)?;
            let resolved = session.qualified_file_comment_source(target).await?;
            let (operation, commit, path, file_id) = match &resolved.anchor.target {
                ArtifactTargetV1::CodeText(v) => (
                    v.operation_id,
                    v.commit_id.to_string(),
                    v.path.as_str(),
                    v.file_id.to_string(),
                ),
                ArtifactTargetV1::CodeArtifact(v) => {
                    let jj_store_model::ObjectKey::File(file) = v.object else {
                        return Err(jj_collab::Error::TargetUnavailable);
                    };
                    (
                        v.operation_id,
                        v.commit_id.to_string(),
                        v.path.as_str(),
                        file.to_string(),
                    )
                }
                _ => return Err(jj_collab::Error::TargetUnavailable),
            };
            let source = self
                .workspace_source(*repository, profile, operation)
                .await
                .map_err(|_| jj_collab::Error::ReadUnavailable)?;
            if !source
                .files(&commit)
                .await
                .map_err(|_| jj_collab::Error::ReadUnavailable)?
                .iter()
                .any(|e| e.path == path && e.file.as_ref() == Some(&file_id))
            {
                return Err(jj_collab::Error::TargetUnavailable);
            }
            let file = source
                .file(&commit, path)
                .await
                .map_err(|_| jj_collab::Error::ReadUnavailable)?
                .into_bytes();
            resolved
                .anchor
                .validate_text(&file)
                .map_err(|_| jj_collab::Error::TargetUnavailable)?;
            QualifiedReviewComment {
                target: target.clone(),
                revision: resolved.revision,
                body: resolved.body,
                anchor: resolved.anchor,
                file,
            }
        };
        self.authorized(*repository, profile, false)
            .await
            .map_err(|_| jj_collab::Error::TargetUnavailable)?;
        Ok(value)
    }
}

impl Supervisor {
    pub(super) async fn qualified_review_source(
        &self,
        target: &ReactionTargetV1,
    ) -> Result<jj_store_service::qualified_review::ReviewCommentSource> {
        let ReactionTargetV1::RepositoryReviewComment { repository, .. } = target else {
            return Err(Error::Invalid);
        };
        let local = self
            .0
            .resources
            .folders
            .as_ref()
            .and_then(|f| f.open(*repository).ok());
        let profile = if local.is_some() {
            "local_jj_git"
        } else {
            "native"
        };
        self.authorized(*repository, profile, false).await?;
        let result = if let Some(local) = local {
            let store = local.review_store().await.map_err(|_| Error::Unavailable)?;
            jj_store_service::qualified_review::selected_comment(&store, target).await
        } else {
            self.home_session(*repository)?
                .qualified_file_comment_source(target)
                .await
        }
        .map_err(|_| Error::Stale)?;
        self.authorized(*repository, profile, false).await?;
        Ok(result)
    }
}

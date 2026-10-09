//! Independently pinned review references; no foreign objects enter a home graph.
use crate::{auth::Permission, local_repository::LocalRepository, native::NativeSession};
use jj_collab::*;
use jj_review::{ArtifactAnchorV1, CommentRevisionContentV1, ReviewObjectRef, ReviewObjectV1};
use jj_store_wal::publication::{CollabTargetReaderV1, Domain, ReviewSnapshotRead};

pub struct ReviewCommentSource {
    pub revision: TargetRevisionV1,
    pub body: String,
    pub anchor_ref: ReviewObjectRef,
    pub anchor: ArtifactAnchorV1,
}
pub async fn selected_comment(
    store: &dyn ReviewSnapshotRead,
    target: &ReactionTargetV1,
) -> Result<ReviewCommentSource> {
    let ReactionTargetV1::RepositoryReviewComment {
        repository,
        thread_id,
        comment_id,
        revision,
    } = target
    else {
        return Err(Error::Invalid("qualified review kind"));
    };
    if *repository != store.repository() {
        return Err(Error::TargetUnavailable);
    }
    let reader = CollabTargetReaderV1::new(*repository, None, None, Some(store))
        .map_err(|_| Error::ReadUnavailable)?;
    let resolved = resolve_reaction_target(
        &reader,
        &ReactionTargetV1::ReviewComment {
            thread_id: *thread_id,
            comment_id: *comment_id,
            revision: *revision,
        },
    )
    .await?;
    let selected = reader
        .review_selection(*thread_id)
        .await?
        .ok_or(Error::TargetUnavailable)?;
    let anchor_ref = match reader
        .review_object(selected)
        .await?
        .ok_or(Error::TargetUnavailable)?
    {
        ReviewObjectV1::ThreadSnapshot(v) => v.original_anchor,
        ReviewObjectV1::ThreadMerge(v) => v.original_anchor,
        _ => return Err(Error::TargetUnavailable),
    };
    let Some(ReviewObjectV1::Anchor(anchor)) = reader.review_object(anchor_ref).await? else {
        return Err(Error::TargetUnavailable);
    };
    let Some(ReviewObjectV1::CommentRevision(comment)) = reader.review_object(*revision).await?
    else {
        return Err(Error::TargetUnavailable);
    };
    let CommentRevisionContentV1::Visible { body } = comment.content else {
        return Err(Error::TargetUnavailable);
    };
    let Some(ReviewObjectV1::CommentBody(body)) = reader.review_object(body).await? else {
        return Err(Error::TargetUnavailable);
    };
    if body.text.len() > 32768 {
        return Err(Error::Limit);
    }
    Ok(ReviewCommentSource {
        revision: resolved.revision,
        body: body.text,
        anchor_ref,
        anchor: *anchor,
    })
}
impl LocalRepository {
    pub async fn qualified_file_comment(
        &self,
        target: &ReactionTargetV1,
    ) -> Result<QualifiedReviewComment> {
        let actor = self
            .actor()
            .map_err(|_| Error::TargetUnavailable)?
            .ok_or(Error::TargetUnavailable)?;
        let store = self
            .review_store()
            .await
            .map_err(|_| Error::ReadUnavailable)?;
        let resolved = selected_comment(&store, target).await?;
        let file = store
            .file_source(resolved.anchor_ref)
            .await
            .map_err(|_| Error::ReadUnavailable)?;
        if self
            .actor()
            .map_err(|_| Error::TargetUnavailable)?
            .ok_or(Error::TargetUnavailable)?
            != actor
        {
            return Err(Error::TargetUnavailable);
        }
        self.check_identity(
            self.metadata
                .read()
                .map_err(|_| Error::ReadUnavailable)?
                .as_ref(),
        )
        .map_err(|_| Error::TargetUnavailable)?;
        Ok(QualifiedReviewComment {
            target: target.clone(),
            revision: resolved.revision,
            body: resolved.body,
            anchor: resolved.anchor,
            file,
        })
    }
}
impl NativeSession {
    pub async fn qualified_file_comment_source(
        &self,
        target: &ReactionTargetV1,
    ) -> Result<ReviewCommentSource> {
        let principal = self
            .principal(Permission::Read)
            .await
            .map_err(|_| Error::TargetUnavailable)?;
        let pin = self.snapshot().await.map_err(|_| Error::ReadUnavailable)?;
        let root = pin
            .manifest()
            .domain_state(Domain::Review)
            .ok_or(Error::TargetUnavailable)?
            .root;
        let store = self
            .coordinator
            .review_store(root)
            .await
            .map_err(|_| Error::ReadUnavailable)?;
        let result = selected_comment(&store, target).await?;
        if self
            .principal(Permission::Read)
            .await
            .map_err(|_| Error::TargetUnavailable)?
            .id()
            != principal.id()
        {
            return Err(Error::TargetUnavailable);
        }
        Ok(result)
    }
}

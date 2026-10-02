//! Review presentation shares the native reconstruction engine, with its own pin.
use super::{
    LocalError, LocalRepository, LocalResult, LocalReviewSnapshot, LocalReviewStore, MetadataCommit,
};
use crate::collaboration::{
    self as c,
    review_threads::{self, ReviewSelection},
};
use jj_store_wal::publication::{ReviewCatalogKey, ReviewSnapshotRead, ReviewThreadId};
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize, JsonSchema)]
pub struct LocalReviewThreadPage {
    pub version: u32,
    pub snapshot: LocalReviewSnapshot,
    pub threads: Vec<c::ReviewThreadSummary>,
    pub next_after: Option<ReviewCatalogKey>,
}
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize, JsonSchema)]
pub struct LocalReviewThreadDetail {
    pub version: u32,
    pub snapshot: LocalReviewSnapshot,
    pub thread: c::ReviewThreadSummary,
    pub anchor: c::ReviewAnchor,
    pub comments: Vec<c::ReviewComment>,
    pub comment_conflicts: Vec<c::ReviewCommentConflict>,
    pub events: Vec<c::ReviewEvent>,
}
impl LocalRepository {
    async fn selected_review_store(
        &self,
        commit: Option<MetadataCommit>,
    ) -> LocalResult<LocalReviewStore> {
        let selected = match commit {
            Some(commit) => Some(self.selected_metadata(commit)?),
            None => self.metadata.read()?,
        };
        self.check_identity(selected.as_ref())?;
        // The uninitialized store has no records; reads never create an identity.
        self.review_store_at(
            selected.as_ref(),
            selected
                .as_ref()
                .map_or(jj_store_model::RepositoryId::new([0; 16]), |s| {
                    s.repository()
                }),
        )
        .await
    }
    pub async fn review_threads_at(
        &self,
        commit: Option<MetadataCommit>,
        limit: usize,
        after: Option<ReviewCatalogKey>,
    ) -> LocalResult<LocalReviewThreadPage> {
        if !(1..=256).contains(&limit) {
            return Err(LocalError::Limit);
        }
        if after.is_some() && commit.is_none() {
            return Err(LocalError::Invalid);
        }
        let store = self.selected_review_store(commit).await?;
        let page = store.reads().list_threads(after, limit).await?;
        let threads = review_threads::resolve_threads(
            &ReviewSelection {
                repository: store.repository(),
                store: &store,
            },
            page.selections,
        )
        .await
        .map_err(map)?;
        let page = LocalReviewThreadPage {
            version: 1,
            snapshot: store.snapshot,
            threads,
            next_after: page.next_after,
        };
        bounded(&page)?;
        Ok(page)
    }
    pub async fn review_thread_at(
        &self,
        commit: Option<MetadataCommit>,
        id: ReviewThreadId,
    ) -> LocalResult<LocalReviewThreadDetail> {
        let store = self.selected_review_store(commit).await?;
        let content = review_threads::resolve_detail(
            &ReviewSelection {
                repository: store.repository(),
                store: &store,
            },
            id,
        )
        .await
        .map_err(map)?;
        let detail = LocalReviewThreadDetail {
            version: content.version,
            snapshot: store.snapshot,
            thread: content.thread,
            anchor: content.anchor,
            comments: content.comments,
            comment_conflicts: content.comment_conflicts,
            events: content.events,
        };
        bounded(&detail)?;
        Ok(detail)
    }
}
fn bounded(value: &impl Serialize) -> LocalResult<()> {
    if serde_json::to_vec(value)
        .map_err(|_| LocalError::Corrupt)?
        .len()
        > 12 * 1024 * 1024
    {
        return Err(LocalError::Limit);
    }
    Ok(())
}
fn map(error: c::Error) -> LocalError {
    match error {
        c::Error::NotFound => LocalError::NotFound,
        c::Error::Limit => LocalError::Limit,
        c::Error::Busy => LocalError::Busy,
        c::Error::Corrupt => LocalError::Corrupt,
        c::Error::InvalidCursor => LocalError::Invalid,
        c::Error::CursorReset => LocalError::Stale,
        _ => LocalError::Unavailable,
    }
}

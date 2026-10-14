use super::review_cursors::ReviewPin;
use super::review_replication::{count_comment_conflicts, load_graph_events, load_merged_comments};
use super::*;
use jj_store_wal::publication::{
    ArtifactAnchorV1, CommentRevisionContentV1, CommentSelectionV1, Domain, ReviewCatalogKey,
    ReviewObjectRef, ReviewObjectV1, ReviewSnapshotRead, ReviewThreadId,
};
use std::collections::BTreeSet;

pub(crate) struct ReviewSelection<'a> {
    pub repository: m::RepositoryId,
    pub store: &'a dyn ReviewSnapshotRead,
}

pub(crate) struct ReviewDetailContent {
    pub version: u32,
    pub thread: ReviewThreadSummary,
    pub anchor: ReviewAnchor,
    pub comments: Vec<ReviewComment>,
    pub comment_conflicts: Vec<ReviewCommentConflict>,
    pub events: Vec<ReviewEvent>,
}

const MAX_THREAD_EVENTS: usize = 4096;
const MAX_THREAD_BODY_BYTES: usize = 8 * 1024 * 1024;

impl ReviewThreadPager {
    pub async fn begin(
        &self,
        session: Arc<NativeSession>,
        limit: usize,
    ) -> Result<ReviewThreadPage> {
        valid_limit(limit)?;
        let _permit = self.cursors.acquire()?;
        within_timeout(self.cursors.timeout(), self.begin_page(session, limit)).await
    }

    async fn begin_page(
        &self,
        session: Arc<NativeSession>,
        limit: usize,
    ) -> Result<ReviewThreadPage> {
        let pin = self.pin(&session).await?;
        let snapshot = pin.snapshot.clone();
        let page = pin.store.list_threads(None, limit).await.map_err(map_wal)?;
        let threads = resolve_threads(
            &ReviewSelection {
                repository: pin.snapshot.repo_id,
                store: &pin.store,
            },
            page.selections,
        )
        .await?;
        authorize_epoch(&session, snapshot.epoch).await?;
        let next_cursor = page
            .next_after
            .map(|after| self.cursors.retain(pin, limit, after))
            .transpose()?;
        Ok(ReviewThreadPage {
            version: 1,
            snapshot,
            threads,
            next_cursor,
        })
    }

    pub async fn resume(
        &self,
        session: Arc<NativeSession>,
        cursor: &str,
        limit: usize,
    ) -> Result<ReviewThreadPage> {
        valid_limit(limit)?;
        let _permit = self.cursors.acquire()?;
        within_timeout(
            self.cursors.timeout(),
            self.resume_page(session, cursor, limit),
        )
        .await
    }

    async fn resume_page(
        &self,
        session: Arc<NativeSession>,
        cursor: &str,
        limit: usize,
    ) -> Result<ReviewThreadPage> {
        authorize(&session).await?;
        let (claims, pin) = self.cursors.lookup(session.repository, cursor, limit)?;
        authorize_epoch(&session, pin.snapshot.epoch).await?;
        let page = pin
            .store
            .list_threads(Some(claims.after), limit)
            .await
            .map_err(map_wal)?;
        let threads = resolve_threads(
            &ReviewSelection {
                repository: pin.snapshot.repo_id,
                store: &pin.store,
            },
            page.selections,
        )
        .await?;
        authorize_epoch(&session, pin.snapshot.epoch).await?;
        let next_cursor = self.cursors.next(&claims, page.next_after)?;
        Ok(ReviewThreadPage {
            version: 1,
            snapshot: pin.snapshot,
            threads,
            next_cursor,
        })
    }

    pub async fn detail(
        &self,
        session: Arc<NativeSession>,
        thread_id: ReviewThreadId,
    ) -> Result<ReviewThreadDetail> {
        let _permit = self.cursors.acquire()?;
        within_timeout(
            self.cursors.timeout(),
            self.detail_inner(session, thread_id),
        )
        .await
    }

    async fn detail_inner(
        &self,
        session: Arc<NativeSession>,
        thread_id: ReviewThreadId,
    ) -> Result<ReviewThreadDetail> {
        let pin = self.pin(&session).await?;
        let content = resolve_detail(
            &ReviewSelection {
                repository: pin.snapshot.repo_id,
                store: &pin.store,
            },
            thread_id,
        )
        .await?;
        authorize_epoch(&session, pin.snapshot.epoch).await?;
        Ok(ReviewThreadDetail {
            version: content.version,
            snapshot: pin.snapshot,
            thread: content.thread,
            anchor: content.anchor,
            comments: content.comments,
            comment_conflicts: content.comment_conflicts,
            events: content.events,
        })
    }

    pub(super) async fn pin(&self, session: &NativeSession) -> Result<ReviewPin> {
        authorize(session).await?;
        let selected = session.snapshot().await.map_err(map_wal)?;
        let manifest = selected.manifest();
        let domain = manifest
            .domain_state(Domain::Review)
            .ok_or(Error::Corrupt)?;
        let pin = ReviewPin {
            snapshot: ReviewReadSnapshot {
                repo_id: manifest.repository,
                epoch: manifest.epoch,
                published_seq: manifest.published,
                manifest_revision: manifest.revision,
                review_root: domain.root.digest,
                review_state_version: domain.version,
            },
            // Review provider admission must audit the complete closure on
            // every first request, including body-free activity cache hits.
            store: session
                .coordinator
                .review_store(domain.root)
                .await
                .map_err(map_wal)?,
        };
        self.cursors.validate_pin(&pin)?;
        Ok(pin)
    }
}

pub(crate) async fn resolve_detail(
    pin: &ReviewSelection<'_>,
    thread_id: ReviewThreadId,
) -> Result<ReviewDetailContent> {
    let reference = pin
        .store
        .lookup(
            jj_store_wal::publication::ReviewCatalogKind::Threads,
            ReviewCatalogKey::derive("thread", thread_id.as_bytes()).map_err(|_| Error::Corrupt)?,
        )
        .await
        .map_err(map_wal)?
        .ok_or(Error::NotFound)?;
    let (thread, anchor, comments, comment_conflicts, events, version) =
        match pin.store.read_object(reference).await.map_err(map_wal)? {
            ReviewObjectV1::ThreadSnapshot(value) => {
                if value.thread_id != thread_id {
                    return Err(Error::Corrupt);
                }
                let anchor = load_anchor(pin, value.original_anchor).await?;
                let mut thread = review_models::summary(reference, &value, anchor.value.clone());
                let comments = load_comments(pin, thread_id, &value.comments).await?;
                let events = load_events(pin, thread_id, value.event_page_root).await?;
                if events.len() as u64 != value.last_event_sequence {
                    return Err(Error::Corrupt);
                }
                thread.presentation = Some(
                    super::review_presentation::load(pin, &thread, &value.comments, &events)
                        .await?,
                );
                (thread, anchor, comments, Vec::new(), events, 1)
            }
            ReviewObjectV1::ThreadMerge(value) => {
                if value.thread_id != thread_id {
                    return Err(Error::Corrupt);
                }
                let anchor = load_anchor(pin, value.original_anchor).await?;
                let (comments, conflicts) =
                    load_merged_comments(pin, thread_id, &value.comments).await?;
                let mut thread = review_models::merged_summary(
                    reference,
                    &value,
                    anchor.value.clone(),
                    conflicts.len(),
                );
                let events = load_graph_events(pin, thread_id, value.event_graph_root).await?;
                if events.len() as u64 != value.thread_version {
                    return Err(Error::Corrupt);
                }
                thread.presentation = Some(
                    super::review_presentation::load(pin, &thread, &value.comments, &events)
                        .await?,
                );
                (thread, anchor, comments, conflicts, events, 2)
            }
            _ => return Err(Error::Corrupt),
        };
    Ok(ReviewDetailContent {
        version,
        thread,
        anchor,
        comments,
        comment_conflicts,
        events,
    })
}

pub(crate) async fn resolve_threads(
    pin: &ReviewSelection<'_>,
    selections: Vec<(ReviewCatalogKey, ReviewObjectRef)>,
) -> Result<Vec<ReviewThreadSummary>> {
    let mut rows = Vec::with_capacity(selections.len());
    for (key, reference) in selections {
        match pin.store.read_object(reference).await.map_err(map_wal)? {
            ReviewObjectV1::ThreadSnapshot(value) => {
                verify_thread_key(key, value.thread_id)?;
                let anchor =
                    load_store_anchor(pin.store, value.original_anchor, pin.repository).await?;
                let mut thread = review_models::summary(reference, &value, anchor);
                let events = load_events(pin, value.thread_id, value.event_page_root).await?;
                if events.len() as u64 != value.last_event_sequence {
                    return Err(Error::Corrupt);
                }
                thread.presentation = Some(
                    super::review_presentation::load(pin, &thread, &value.comments, &events)
                        .await?,
                );
                rows.push(thread);
            }
            ReviewObjectV1::ThreadMerge(value) => {
                verify_thread_key(key, value.thread_id)?;
                let anchor =
                    load_store_anchor(pin.store, value.original_anchor, pin.repository).await?;
                let conflict_count = count_comment_conflicts(pin.store, &value.comments).await?;
                let mut thread =
                    review_models::merged_summary(reference, &value, anchor, conflict_count);
                let events =
                    load_graph_events(pin, value.thread_id, value.event_graph_root).await?;
                if events.len() as u64 != value.thread_version {
                    return Err(Error::Corrupt);
                }
                thread.presentation = Some(
                    super::review_presentation::load(pin, &thread, &value.comments, &events)
                        .await?,
                );
                rows.push(thread);
            }
            _ => return Err(Error::Corrupt),
        }
    }
    Ok(rows)
}

async fn load_anchor(
    pin: &ReviewSelection<'_>,
    reference: ReviewObjectRef,
) -> Result<ReviewAnchor> {
    let ReviewObjectV1::Anchor(value) = pin.store.read_object(reference).await.map_err(map_wal)?
    else {
        return Err(Error::Corrupt);
    };
    if value.repository_id != pin.repository {
        return Err(Error::Corrupt);
    }
    Ok(ReviewAnchor {
        object_id: reference.id,
        value: *value,
    })
}

async fn load_comments(
    pin: &ReviewSelection<'_>,
    thread_id: ReviewThreadId,
    selections: &[CommentSelectionV1],
) -> Result<Vec<ReviewComment>> {
    let mut comments = Vec::with_capacity(selections.len());
    let mut body_bytes = 0usize;
    for selection in selections {
        let ReviewObjectV1::CommentRevision(value) = pin
            .store
            .read_object(selection.revision)
            .await
            .map_err(map_wal)?
        else {
            return Err(Error::Corrupt);
        };
        if value.thread_id != thread_id || value.comment_id != selection.comment_id {
            return Err(Error::Corrupt);
        }
        let content = match value.content.clone() {
            CommentRevisionContentV1::Visible { body } => {
                let ReviewObjectV1::CommentBody(body_value) =
                    pin.store.read_object(body).await.map_err(map_wal)?
                else {
                    return Err(Error::Corrupt);
                };
                body_bytes = body_bytes
                    .checked_add(body_value.text.len())
                    .filter(|value| *value <= MAX_THREAD_BODY_BYTES)
                    .ok_or(Error::Limit)?;
                ReviewCommentContent::Visible {
                    body_id: body.id,
                    format: body_value.format,
                    text: body_value.text,
                    sanitizer_version: body_value.sanitizer_version,
                }
            }
            CommentRevisionContentV1::Tombstoned { reason } => {
                ReviewCommentContent::Tombstoned { reason }
            }
            CommentRevisionContentV1::Hidden { reason } => ReviewCommentContent::Hidden { reason },
        };
        comments.push(ReviewComment {
            comment_id: value.comment_id,
            revision_id: selection.revision.id,
            parent_comment_id: selection.parent_comment_id,
            author: value.author,
            actor: value.actor,
            created_at_millis: value.created_at_millis,
            previous_revision_id: value.previous_revision.map(|item| item.id),
            superseded_revision_ids: value
                .superseded_revisions
                .into_iter()
                .map(|reference| reference.id)
                .collect(),
            content,
        });
    }
    Ok(comments)
}

async fn load_events(
    pin: &ReviewSelection<'_>,
    thread_id: ReviewThreadId,
    root: ReviewObjectRef,
) -> Result<Vec<ReviewEvent>> {
    let mut events = Vec::new();
    let mut page = Some(root);
    let mut pages = BTreeSet::new();
    let mut prior = None;
    while let Some(reference) = page {
        if !pages.insert(reference) {
            return Err(Error::Corrupt);
        }
        let ReviewObjectV1::EventPage(value) =
            pin.store.read_object(reference).await.map_err(map_wal)?
        else {
            return Err(Error::Corrupt);
        };
        if value.thread_id != thread_id {
            return Err(Error::Corrupt);
        }
        for selection in value.entries {
            let ReviewObjectV1::ThreadEvent(event) = pin
                .store
                .read_object(selection.event)
                .await
                .map_err(map_wal)?
            else {
                return Err(Error::Corrupt);
            };
            if event.thread_id != thread_id || event.sequence != selection.sequence {
                return Err(Error::Corrupt);
            }
            events.push(ReviewEvent {
                event_id: selection.event.id,
                causal_parent_ids: prior
                    .into_iter()
                    .map(|value: ReviewObjectRef| value.id)
                    .collect(),
                sequence: event.sequence,
                actor: event.actor,
                accepted_at_millis: event.accepted_at_millis,
                event: review_models::event_kind(event.event),
            });
            prior = Some(selection.event);
            if events.len() > MAX_THREAD_EVENTS {
                return Err(Error::Limit);
            }
        }
        page = value.next;
    }
    if events
        .iter()
        .enumerate()
        .any(|(index, value)| value.sequence != index as u64 + 1)
    {
        return Err(Error::Corrupt);
    }
    Ok(events)
}

pub(super) fn verify_thread_key(key: ReviewCatalogKey, thread_id: ReviewThreadId) -> Result<()> {
    let expected =
        ReviewCatalogKey::derive("thread", thread_id.as_bytes()).map_err(|_| Error::Corrupt)?;
    if key == expected {
        Ok(())
    } else {
        Err(Error::Corrupt)
    }
}

async fn load_store_anchor(
    store: &dyn ReviewSnapshotRead,
    reference: ReviewObjectRef,
    repository: m::RepositoryId,
) -> Result<ArtifactAnchorV1> {
    let ReviewObjectV1::Anchor(anchor) = store.read_object(reference).await.map_err(map_wal)?
    else {
        return Err(Error::Corrupt);
    };
    if anchor.repository_id != repository {
        return Err(Error::Corrupt);
    }
    Ok(*anchor)
}

use super::Error;
use super::*;
use jj_review::*;
use jj_store_wal::{self as wal, publication::ReviewSnapshotRead};
use std::{collections::BTreeMap, sync::Mutex};

const REPO: m::RepositoryId = m::RepositoryId::new([81; 16]);
const THREAD: ReviewThreadId = ReviewThreadId::new([82; 16]);

struct Store {
    root: ReviewRootV1,
    values: BTreeMap<ReviewObjectRef, ReviewObjectV1>,
    reads: Mutex<Vec<ReviewObjectRef>>,
}
#[async_trait::async_trait]
impl ReviewSnapshotRead for Store {
    fn repository(&self) -> m::RepositoryId {
        REPO
    }
    fn root(&self) -> &ReviewRootV1 {
        &self.root
    }
    async fn lookup(
        &self,
        _: ReviewCatalogKind,
        _: ReviewCatalogKey,
    ) -> wal::Result<Option<ReviewObjectRef>> {
        panic!("activity does not perform independent lookup")
    }
    async fn read_object(&self, reference: ReviewObjectRef) -> wal::Result<ReviewObjectV1> {
        self.reads.lock().unwrap().push(reference);
        // Anchor/comment/body hydration fails the test immediately.
        self.values
            .get(&reference)
            .cloned()
            .ok_or(wal::Error::Corrupt("Unexpected activity object read"))
    }
}
fn reference(kind: ReviewObjectKind, byte: u8) -> ReviewObjectRef {
    ReviewObjectRef::new(kind, ReviewObjectId::new([byte; 32]))
}
fn fixture(merged: bool) -> (Store, ReviewCatalogKey, ReviewObjectRef) {
    let anchor = reference(ReviewObjectKind::Anchor, 1);
    let revision = reference(ReviewObjectKind::CommentRevision, 2);
    let event1 = reference(ReviewObjectKind::ThreadEvent, 3);
    let event2 = reference(ReviewObjectKind::ThreadEvent, 4);
    let page1 = reference(ReviewObjectKind::EventPage, 5);
    let page2 = reference(ReviewObjectKind::EventPage, 6);
    let thread = reference(ReviewObjectKind::ThreadSnapshot, 7);
    let comment = CommentId::new([83; 16]);
    let comments = vec![CommentSelectionV1 {
        comment_id: comment,
        revision,
        parent_comment_id: None,
    }];
    let mut values = BTreeMap::from([
        (
            event1,
            ReviewObjectV1::ThreadEvent(Box::new(ReviewThreadEventV1 {
                schema_version: 1,
                thread_id: THREAD,
                sequence: 1,
                actor: m::PrincipalId::new([84; 16]),
                accepted_at_millis: 900,
                event: ReviewThreadEventKindV1::Created {
                    anchor,
                    initial_comment_id: comment,
                    initial_revision: revision,
                },
            })),
        ),
        (
            event2,
            ReviewObjectV1::ThreadEvent(Box::new(ReviewThreadEventV1 {
                schema_version: 1,
                thread_id: THREAD,
                sequence: 2,
                actor: m::PrincipalId::new([84; 16]),
                accepted_at_millis: 100,
                event: ReviewThreadEventKindV1::Resolved {
                    action_id: ReviewActionId::new([85; 16]),
                    reason: None,
                },
            })),
        ),
    ]);
    if merged {
        values.insert(
            page1,
            ReviewObjectV1::EventGraph(ReviewEventGraphV1 {
                schema_version: 1,
                thread_id: THREAD,
                nodes: vec![
                    ReviewEventNodeV1 {
                        event: event1,
                        parents: vec![],
                    },
                    ReviewEventNodeV1 {
                        event: event2,
                        parents: vec![event1],
                    },
                ],
                heads: vec![event2],
            }),
        );
        values.insert(
            thread,
            ReviewObjectV1::ThreadMerge(Box::new(ReviewThreadMergeV1 {
                schema_version: 1,
                thread_id: THREAD,
                original_anchor: anchor,
                comments,
                event_graph_root: page1,
                status: ThreadStatusV1::Resolved,
                selected_workflow_actions: vec![ReviewActionId::new([85; 16])],
                thread_version: 2,
                content_version: 1,
                workflow_version: 2,
            })),
        );
    } else {
        values.insert(
            page1,
            ReviewObjectV1::EventPage(ReviewEventPageV1 {
                schema_version: 1,
                thread_id: THREAD,
                entries: vec![ReviewEventSelectionV1 {
                    sequence: 1,
                    event: event1,
                }],
                next: Some(page2),
            }),
        );
        values.insert(
            page2,
            ReviewObjectV1::EventPage(ReviewEventPageV1 {
                schema_version: 1,
                thread_id: THREAD,
                entries: vec![ReviewEventSelectionV1 {
                    sequence: 2,
                    event: event2,
                }],
                next: None,
            }),
        );
        values.insert(
            thread,
            ReviewObjectV1::ThreadSnapshot(Box::new(ReviewThreadSnapshotV1 {
                schema_version: 1,
                thread_id: THREAD,
                original_anchor: anchor,
                comments,
                event_page_root: page1,
                last_event_sequence: 2,
                status: ThreadStatusV1::Resolved,
                selected_workflow_action: Some(ReviewActionId::new([85; 16])),
                thread_version: 2,
                content_version: 1,
                workflow_version: 2,
            })),
        );
    }
    (
        Store {
            root: ReviewRootV1::empty(),
            values,
            reads: Mutex::new(vec![]),
        },
        ReviewCatalogKey::derive("thread", THREAD.as_bytes()).unwrap(),
        thread,
    )
}
#[tokio::test]
async fn activity_reads_only_window_events_and_uses_maximum_across_pages() {
    let (store, key, thread) = fixture(false);
    let result = resolve(
        &ReviewSelection {
            repository: REPO,
            store: &store,
        },
        vec![(key, thread)],
        true,
    )
    .await
    .unwrap();
    assert_eq!(
        result,
        ReviewActivity {
            threads: 1,
            threads_partial: true,
            updated_at_millis: Some(900)
        }
    );
    assert_eq!(store.reads.lock().unwrap().len(), 5);
}
#[tokio::test]
async fn merged_activity_reads_no_comment_conflicts_and_uses_every_event_time() {
    let (store, key, thread) = fixture(true);
    let result = resolve(
        &ReviewSelection {
            repository: REPO,
            store: &store,
        },
        vec![(key, thread)],
        false,
    )
    .await
    .unwrap();
    assert_eq!(result.updated_at_millis, Some(900));
    assert!(!result.threads_partial);
    assert_eq!(store.reads.lock().unwrap().len(), 4);
}
#[tokio::test]
async fn activity_empty_selection_reads_nothing_and_keeps_unknown_time() {
    let (store, _, _) = fixture(false);
    let result = resolve(
        &ReviewSelection {
            repository: REPO,
            store: &store,
        },
        vec![],
        false,
    )
    .await
    .unwrap();
    assert_eq!(
        result,
        ReviewActivity {
            threads: 0,
            threads_partial: false,
            updated_at_millis: None
        }
    );
    assert!(store.reads.lock().unwrap().is_empty());
}
#[tokio::test]
async fn activity_rejects_foreign_keys_missing_events_and_inconsistent_versions() {
    for merged in [false, true] {
        let (mut store, key, thread) = fixture(merged);
        let foreign = ReviewCatalogKey::derive("thread", &[99; 16]).unwrap();
        assert_eq!(
            resolve(
                &ReviewSelection {
                    repository: REPO,
                    store: &store
                },
                vec![(foreign, thread)],
                false
            )
            .await
            .unwrap_err(),
            Error::Corrupt
        );
        match store.values.get_mut(&thread).unwrap() {
            ReviewObjectV1::ThreadMerge(value) => value.thread_version = 3,
            ReviewObjectV1::ThreadSnapshot(value) => value.last_event_sequence = 3,
            _ => unreachable!(),
        }
        assert_eq!(
            resolve(
                &ReviewSelection {
                    repository: REPO,
                    store: &store
                },
                vec![(key, thread)],
                false
            )
            .await
            .unwrap_err(),
            Error::Corrupt
        );
        store
            .values
            .remove(&reference(ReviewObjectKind::ThreadEvent, 4));
        assert_eq!(
            resolve(
                &ReviewSelection {
                    repository: REPO,
                    store: &store
                },
                vec![(key, thread)],
                false
            )
            .await
            .unwrap_err(),
            Error::Corrupt
        );
    }
}

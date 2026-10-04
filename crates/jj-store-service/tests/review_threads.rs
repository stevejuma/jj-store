use jj_review::{
    ArtifactAnchorV1, ArtifactTargetV1, CreatedContextV1, ReviewBodyFormatV1,
    ReviewCommandResultV1, ReviewCommandV1, ReviewCommentBodyV1, SensitivityV1,
    decode_command_result,
};
use jj_store_jj::{objects::NativeObjects, policy::Protection};
use jj_store_model as m;
use jj_store_object::memory::MemoryStore;
use jj_store_service::{
    auth::{Permission, SystemClock},
    collaboration::{Error, ReviewCommentContent, ReviewEventKind, ReviewThreadPager},
    native::{NativeSession, SessionLimits, create_repository_local_trusted},
    pagination,
    review_mutations::{Outcome, Request as ReviewRequest, ReviewCommandService},
};
use jj_store_wal::{
    self as wal,
    publication::{
        Coordinator, DomainRequest, Limits, ManifestSnapshot, PublicationGuard, PublishOutcome,
        Request, ReviewCommandRequest, Snapshot,
    },
};
use std::sync::Arc;

const REPOSITORY: m::RepositoryId = m::RepositoryId::new([91; 16]);
const EPOCH: m::EpochId = m::EpochId::new([92; 16]);
const PRINCIPAL: m::PrincipalId = m::PrincipalId::new([93; 16]);

struct Permit;

#[async_trait::async_trait]
impl PublicationGuard for Permit {
    async fn authorize(&self, _: &ManifestSnapshot, _: &Request) -> wal::Result<()> {
        Ok(())
    }

    async fn validate_transition(
        &self,
        _: &ManifestSnapshot,
        _: &Request,
        _: &Snapshot,
    ) -> wal::Result<()> {
        Ok(())
    }

    async fn authorize_domain(&self, _: &ManifestSnapshot, _: &DomainRequest) -> wal::Result<()> {
        Ok(())
    }
}

struct Fixture {
    session: Arc<NativeSession>,
    coordinator: Coordinator,
    request: u8,
}

impl Fixture {
    async fn new() -> Self {
        let store = Arc::new(MemoryStore::default());
        let objects =
            Arc::new(NativeObjects::new(store.clone(), REPOSITORY, 2 * 1024 * 1024).unwrap());
        create_repository_local_trusted(
            store.clone(),
            objects.clone(),
            Permission::Admin,
            EPOCH,
            Limits::default(),
        )
        .await
        .unwrap();
        let coordinator = Coordinator::new(
            store.clone(),
            REPOSITORY,
            objects.clone(),
            Arc::new(Permit),
            Limits::default(),
        )
        .unwrap();
        let session = Arc::new(
            NativeSession::new_local_trusted(
                store,
                objects,
                PRINCIPAL,
                Permission::Admin,
                Arc::new(SystemClock),
                Protection::default(),
                SessionLimits::default(),
            )
            .unwrap(),
        );
        Self {
            session,
            coordinator,
            request: 100,
        }
    }

    async fn publish(&mut self, command: ReviewCommandV1) -> ReviewCommandResultV1 {
        let request = ReviewCommandRequest::new(
            PRINCIPAL,
            m::RequestId::new([self.request; 16]),
            m::PublicationId::new([self.request + 1; 16]),
            command,
        )
        .unwrap();
        self.request += 2;
        let PublishOutcome::Committed(receipt) = self
            .coordinator
            .publish_review_command(&request)
            .await
            .unwrap()
        else {
            panic!("review command did not commit")
        };
        decode_command_result(&receipt.result).unwrap()
    }

    async fn create(&mut self, text: &str) -> ReviewCommandResultV1 {
        self.publish(ReviewCommandV1::CreateThread {
            anchor: Box::new(ArtifactAnchorV1 {
                schema_version: 1,
                repository_id: REPOSITORY,
                target: ArtifactTargetV1::Repository,
                created_context: CreatedContextV1 {
                    publication_position: None,
                    code_state_version: None,
                    agent_root: None,
                },
                sensitivity: SensitivityV1::Repository,
            }),
            body: body(text),
        })
        .await
    }
}

fn body(text: &str) -> ReviewCommentBodyV1 {
    ReviewCommentBodyV1 {
        schema_version: 1,
        format: ReviewBodyFormatV1::SafeMarkdown,
        text: text.into(),
        sanitizer_version: "review-markdown-v1".into(),
    }
}

#[tokio::test]
async fn review_activity_matches_the_pinned_catalog_window_without_publishing() {
    let mut fixture = Fixture::new().await;
    let pager =
        ReviewThreadPager::new(pagination::Limits::default(), Arc::new(SystemClock)).unwrap();
    let empty = pager.activity(fixture.session.clone(), 50).await.unwrap();
    assert_eq!(empty.activity.threads, 0);
    assert_eq!(empty.activity.updated_at_millis, None);
    assert!(!empty.activity.threads_partial);
    fixture.create("First review").await;
    fixture.create("Second review").await;
    let before = fixture.session.snapshot().await.unwrap().manifest().clone();
    for limit in [1, 2, 50, 256] {
        let page = pager.begin(fixture.session.clone(), limit).await.unwrap();
        let summary = pager
            .activity(fixture.session.clone(), limit)
            .await
            .unwrap();
        assert_eq!(summary.snapshot, page.snapshot);
        assert_eq!(summary.activity.threads, page.threads.len());
        assert_eq!(summary.activity.threads_partial, page.next_cursor.is_some());
        assert_eq!(
            summary.activity.updated_at_millis,
            page.threads
                .iter()
                .map(|thread| thread.presentation.as_ref().unwrap().updated_at_millis)
                .max()
        );
        let wire = serde_json::to_string(&summary).unwrap();
        assert!(!wire.contains("First review"));
        assert!(!wire.contains("anchor"));
        assert!(!wire.contains("excerpt"));
    }
    for limit in [0, 257] {
        assert_eq!(
            pager
                .activity(fixture.session.clone(), limit)
                .await
                .unwrap_err(),
            Error::Limit
        );
    }
    assert_eq!(
        fixture.session.snapshot().await.unwrap().manifest(),
        &before
    );
}

#[tokio::test(flavor = "multi_thread", worker_threads = 2)]
async fn review_threads_are_root_pinned_and_details_resolve_selected_objects() {
    let mut fixture = Fixture::new().await;
    let first = fixture.create("Initial review").await;
    let second = fixture.create("Second thread").await;
    let pager =
        ReviewThreadPager::new(pagination::Limits::default(), Arc::new(SystemClock)).unwrap();

    let page = pager.begin(fixture.session.clone(), 1).await.unwrap();
    assert_eq!(page.threads.len(), 1);
    assert_eq!(page.threads[0].anchor.repository_id, REPOSITORY);
    assert_eq!(page.threads[0].anchor.target, ArtifactTargetV1::Repository);
    let wire = serde_json::to_value(&page.threads[0]).unwrap();
    assert_eq!(wire["presentation"]["created_by"], PRINCIPAL.to_string());
    assert!(wire["presentation"]["excerpt"].as_str().is_some());
    let cursor = page.next_cursor.clone().unwrap();
    let initial_snapshot = page.snapshot.clone();

    let reply = fixture
        .publish(ReviewCommandV1::AppendReply {
            thread_id: first.thread_id,
            expected_workflow_version: 1,
            parent_comment_id: first.comment_id,
            body: body("Follow-up"),
        })
        .await;

    let resumed = pager
        .resume(fixture.session.clone(), &cursor, 1)
        .await
        .unwrap();
    assert_eq!(resumed.snapshot, initial_snapshot);
    assert_eq!(resumed.threads.len(), 1);
    assert_ne!(resumed.threads[0].thread_id, page.threads[0].thread_id);
    assert!(resumed.next_cursor.is_none());

    let detail = pager
        .detail(fixture.session.clone(), first.thread_id)
        .await
        .unwrap();
    assert_eq!(detail.thread.snapshot_id, reply.thread_snapshot.id);
    assert_eq!(detail.thread.comment_count, 2);
    assert_eq!(detail.comments.len(), 2);
    assert_eq!(detail.events.len(), 2);
    assert_eq!(detail.anchor.value.target, ArtifactTargetV1::Repository);
    assert!(matches!(
        &detail.comments[0].content,
        ReviewCommentContent::Visible { text, .. } if text == "Initial review"
    ));
    assert!(matches!(
        detail.events[0].event,
        ReviewEventKind::Created { .. }
    ));
    assert!(matches!(
        detail.events[1].event,
        ReviewEventKind::ReplyAdded { .. }
    ));

    assert_eq!(
        pager
            .resume(fixture.session.clone(), &cursor, 2)
            .await
            .unwrap_err(),
        Error::InvalidCursor
    );
    let mut tampered = cursor;
    tampered.push('a');
    assert_eq!(
        pager
            .resume(fixture.session.clone(), &tampered, 1)
            .await
            .unwrap_err(),
        Error::InvalidCursor
    );
    assert_eq!(
        pager
            .detail(fixture.session, jj_review::ReviewThreadId::new([255; 16]),)
            .await
            .unwrap_err(),
        Error::NotFound
    );
    assert_ne!(first.thread_id, second.thread_id);
}

#[tokio::test(flavor = "multi_thread", worker_threads = 2)]
async fn review_thread_detail_limits_combined_visible_body_bytes() {
    let mut fixture = Fixture::new().await;
    let text = "x".repeat(1024 * 1024);
    let created = fixture.create(&text).await;
    for _ in 0..8 {
        fixture
            .publish(ReviewCommandV1::AppendReply {
                thread_id: created.thread_id,
                expected_workflow_version: 1,
                parent_comment_id: created.comment_id,
                body: body(&text),
            })
            .await;
    }
    let pager =
        ReviewThreadPager::new(pagination::Limits::default(), Arc::new(SystemClock)).unwrap();

    assert_eq!(
        pager
            .detail(fixture.session, created.thread_id)
            .await
            .unwrap_err(),
        Error::Limit
    );
}

#[tokio::test(flavor = "multi_thread", worker_threads = 2)]
async fn review_summary_excerpt_tracks_selected_comment_and_keeps_resumed_roots() {
    let mut fixture = Fixture::new().await;
    let first = fixture.create("Original root").await;
    let second = fixture.create("Other root").await;
    let pager =
        ReviewThreadPager::new(pagination::Limits::default(), Arc::new(SystemClock)).unwrap();
    let before = pager.begin(fixture.session.clone(), 1).await.unwrap();
    let target = if before.threads[0].thread_id == first.thread_id {
        &second
    } else {
        &first
    };
    let edited = fixture
        .publish(ReviewCommandV1::EditComment {
            comment_id: target.comment_id.unwrap(),
            expected_revision_id: jj_review::CommentRevisionId::new(
                *target.comment_revision.unwrap().id.as_bytes(),
            ),
            replacement_body: body(&"😀".repeat(300)),
        })
        .await;
    let resumed = pager
        .resume(
            fixture.session.clone(),
            before.next_cursor.as_ref().unwrap(),
            1,
        )
        .await
        .unwrap();
    assert_eq!(resumed.snapshot, before.snapshot);
    assert!(
        !resumed.threads[0]
            .presentation
            .as_ref()
            .unwrap()
            .excerpt
            .as_ref()
            .unwrap()
            .starts_with('😀')
    );
    let fresh = pager.begin(fixture.session.clone(), 256).await.unwrap();
    let selected = fresh
        .threads
        .iter()
        .find(|thread| thread.thread_id == target.thread_id)
        .unwrap()
        .presentation
        .as_ref()
        .unwrap();
    assert_eq!(selected.created_by, PRINCIPAL);
    assert_eq!(selected.excerpt.as_ref().unwrap().chars().count(), 240);
    assert!(selected.updated_at_millis >= selected.created_at_millis);
    fixture
        .publish(ReviewCommandV1::TombstoneComment {
            comment_id: target.comment_id.unwrap(),
            expected_revision_id: jj_review::CommentRevisionId::new(
                *edited.comment_revision.unwrap().id.as_bytes(),
            ),
            reason: None,
        })
        .await;
    let fresh = pager.begin(fixture.session.clone(), 256).await.unwrap();
    assert!(
        fresh
            .threads
            .iter()
            .find(|thread| thread.thread_id == target.thread_id)
            .unwrap()
            .presentation
            .as_ref()
            .unwrap()
            .excerpt
            .is_none()
    );
    let hidden = if target.thread_id == first.thread_id {
        &second
    } else {
        &first
    };
    fixture
        .publish(ReviewCommandV1::HideComment {
            comment_id: hidden.comment_id.unwrap(),
            expected_revision_id: jj_review::CommentRevisionId::new(
                *hidden.comment_revision.unwrap().id.as_bytes(),
            ),
            moderation_reason: "Fixture moderation".into(),
        })
        .await;
    let fresh = pager.begin(fixture.session.clone(), 256).await.unwrap();
    assert!(
        fresh
            .threads
            .iter()
            .all(|thread| thread.presentation.as_ref().unwrap().excerpt.is_none())
    );
}

#[tokio::test(flavor = "multi_thread", worker_threads = 2)]
async fn review_command_wakes_idle_subscription_at_accepted_position() {
    use jj_store_service::events::{EventKind, Limits as EventLimits, Subscription};
    use std::time::Duration;

    let fixture = Fixture::new().await;
    let mut subscription = Subscription::open(
        fixture.session.clone(),
        None,
        EventLimits {
            poll_interval: Duration::from_secs(60),
            ..EventLimits::default()
        },
    )
    .await
    .unwrap();
    assert!(matches!(
        subscription.next().await.unwrap().unwrap().kind,
        EventKind::Ready { .. }
    ));
    let outcome = ReviewCommandService
        .apply(
            fixture.session,
            &ReviewRequest {
                version: 1,
                request_id: m::RequestId::new([77; 16]),
                command: ReviewCommandV1::CreateThread {
                    anchor: Box::new(ArtifactAnchorV1 {
                        schema_version: 1,
                        repository_id: REPOSITORY,
                        target: ArtifactTargetV1::Repository,
                        created_context: CreatedContextV1 {
                            publication_position: None,
                            code_state_version: None,
                            agent_root: None,
                        },
                        sensitivity: SensitivityV1::Repository,
                    }),
                    body: body("Wake the review feed"),
                },
            },
        )
        .await
        .unwrap();
    let Outcome::Committed { receipt } = outcome else {
        panic!("review command outcome was unknown")
    };
    let event = tokio::time::timeout(Duration::from_secs(2), subscription.next())
        .await
        .unwrap()
        .unwrap()
        .unwrap();
    assert!(matches!(event.kind, EventKind::Changed));
    assert_eq!(event.cursor.position, receipt.accepted.position);
}

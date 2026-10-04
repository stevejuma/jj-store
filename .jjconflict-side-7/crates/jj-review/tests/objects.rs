use jj_agent::{AgentObjectId, AgentObjectKind, AgentObjectRef, AgentSessionId, MessageId};
use jj_review::{
    AgentArtifactAnchorV1, AgentContentOwnerV1, ArtifactAnchorV1, ArtifactTargetV1,
    CommentRevisionContentV1, CommentSelectionV1, CreatedContextV1, ReviewBodyFormatV1,
    ReviewCatalogKey, ReviewCommentBodyV1, ReviewCommentRevisionV1, ReviewEventPageV1,
    ReviewEventSelectionV1, ReviewObjectId, ReviewObjectKind, ReviewObjectRef, ReviewObjectV1,
    ReviewRelationPageV1, ReviewRelationSubjectV1, ReviewRootV1, ReviewStatusPageV1,
    ReviewThreadEventKindV1, ReviewThreadEventV1, ReviewThreadId, ReviewThreadRelationV1,
    ReviewThreadSnapshotV1, SensitivityV1, ThreadStatusV1, decode_object, decode_root,
    encode_object, encode_root, review_status_key,
};
use jj_store_model::{CommitId, ObjectKey, OperationId, PrincipalId, RepositoryId};
use std::collections::{BTreeMap, BTreeSet};

fn review_ref(kind: ReviewObjectKind, byte: u8) -> ReviewObjectRef {
    ReviewObjectRef::new(kind, ReviewObjectId::new([byte; 32]))
}

fn anchor() -> ArtifactAnchorV1 {
    ArtifactAnchorV1 {
        schema_version: 1,
        repository_id: RepositoryId::new([1; 16]),
        target: ArtifactTargetV1::Repository,
        created_context: CreatedContextV1 {
            publication_position: Some(3),
            code_state_version: Some(2),
            agent_root: None,
        },
        sensitivity: SensitivityV1::Repository,
    }
}

fn body() -> ReviewCommentBodyV1 {
    ReviewCommentBodyV1 {
        schema_version: 1,
        format: ReviewBodyFormatV1::SafeMarkdown,
        text: "Please keep the original context.".into(),
        sanitizer_version: "review-markdown-v1".into(),
    }
}

fn revision(thread_id: ReviewThreadId, body: ReviewObjectRef) -> ReviewCommentRevisionV1 {
    ReviewCommentRevisionV1 {
        schema_version: 1,
        thread_id,
        comment_id: jj_review::CommentId::new([3; 16]),
        author: PrincipalId::new([4; 16]),
        actor: PrincipalId::new([4; 16]),
        created_at_millis: 10,
        previous_revision: None,
        superseded_revisions: Vec::new(),
        content: CommentRevisionContentV1::Visible { body },
    }
}

#[test]
fn review_root_and_index_pages_are_bounded_typed_and_canonical() {
    let mut root = ReviewRootV1::empty();
    assert_eq!(decode_root(&encode_root(&root).unwrap()).unwrap(), root);
    assert!(root.dependencies().unwrap().is_empty());

    let key = ReviewCatalogKey::derive("thread", b"stable-thread").unwrap();
    let page = ReviewObjectV1::IndexPage(jj_review::ReviewIndexPageV1::Leaf {
        prefix: key.to_string()[..2].into(),
        entries: BTreeMap::from([(key, review_ref(ReviewObjectKind::ThreadSnapshot, 9))]),
    });
    page.validate().unwrap();
    assert_eq!(page.review_dependencies().unwrap().len(), 1);
    assert_eq!(decode_object(&encode_object(&page).unwrap()).unwrap(), page);

    root.threads = Some(review_ref(ReviewObjectKind::IndexPage, 10));
    root.anchor_relations = Some(review_ref(ReviewObjectKind::IndexPage, 11));
    assert_eq!(root.dependencies().unwrap().len(), 2);
    assert_eq!(root.root_id().unwrap(), root.root_id().unwrap());

    root.comments = Some(review_ref(ReviewObjectKind::CommentRevision, 12));
    assert!(root.validate().is_err());
}

#[test]
fn thread_snapshot_closure_preserves_anchor_comment_event_and_versions() {
    let thread_id = ReviewThreadId::new([2; 16]);
    let anchor_ref = review_ref(ReviewObjectKind::Anchor, 1);
    let body_ref = review_ref(ReviewObjectKind::CommentBody, 2);
    let revision_ref = review_ref(ReviewObjectKind::CommentRevision, 3);
    let event_ref = review_ref(ReviewObjectKind::ThreadEvent, 4);
    let page_ref = review_ref(ReviewObjectKind::EventPage, 5);
    let comment = revision(thread_id, body_ref);
    let event = ReviewThreadEventV1 {
        schema_version: 1,
        thread_id,
        sequence: 1,
        actor: PrincipalId::new([4; 16]),
        accepted_at_millis: 10,
        event: ReviewThreadEventKindV1::Created {
            anchor: anchor_ref,
            initial_comment_id: comment.comment_id,
            initial_revision: revision_ref,
        },
    };
    let page = ReviewEventPageV1 {
        schema_version: 1,
        thread_id,
        entries: vec![ReviewEventSelectionV1 {
            sequence: 1,
            event: event_ref,
        }],
        next: None,
    };
    let snapshot = ReviewThreadSnapshotV1 {
        schema_version: 1,
        thread_id,
        original_anchor: anchor_ref,
        comments: vec![CommentSelectionV1 {
            comment_id: comment.comment_id,
            revision: revision_ref,
            parent_comment_id: None,
        }],
        event_page_root: page_ref,
        last_event_sequence: 1,
        status: ThreadStatusV1::Open,
        selected_workflow_action: None,
        thread_version: 1,
        content_version: 1,
        workflow_version: 1,
    };

    let objects = [
        ReviewObjectV1::Anchor(Box::new(anchor())),
        ReviewObjectV1::CommentBody(body()),
        ReviewObjectV1::CommentRevision(Box::new(comment.clone())),
        ReviewObjectV1::ThreadEvent(Box::new(event.clone())),
        ReviewObjectV1::EventPage(page.clone()),
        ReviewObjectV1::ThreadSnapshot(Box::new(snapshot.clone())),
    ];
    for object in objects {
        object.validate().unwrap();
        assert_eq!(
            decode_object(&encode_object(&object).unwrap()).unwrap(),
            object
        );
    }

    assert_eq!(
        comment.review_dependencies().unwrap(),
        BTreeSet::from([body_ref])
    );
    assert_eq!(
        event.review_dependencies().unwrap(),
        BTreeSet::from([anchor_ref, revision_ref])
    );
    assert_eq!(
        page.review_dependencies().unwrap(),
        BTreeSet::from([event_ref])
    );
    assert_eq!(
        snapshot.review_dependencies().unwrap(),
        BTreeSet::from([anchor_ref, revision_ref, page_ref])
    );
    assert_eq!(
        snapshot.snapshot_id().unwrap(),
        snapshot.snapshot_id().unwrap()
    );
    assert_eq!(event.event_id().unwrap(), event.event_id().unwrap());
    assert_eq!(
        comment.revision_id().unwrap(),
        comment.revision_id().unwrap()
    );
}

#[test]
fn review_objects_reject_kind_confusion_unsafe_bodies_and_incoherent_history() {
    let thread_id = ReviewThreadId::new([2; 16]);
    let mut unsafe_body = body();
    unsafe_body.text = "unsafe\0body".into();
    assert!(unsafe_body.validate().is_err());

    let mut tombstone = revision(thread_id, review_ref(ReviewObjectKind::CommentBody, 2));
    tombstone.content = CommentRevisionContentV1::Tombstoned { reason: None };
    assert!(tombstone.validate().is_err());

    let mut page = ReviewEventPageV1 {
        schema_version: 1,
        thread_id,
        entries: vec![
            ReviewEventSelectionV1 {
                sequence: 2,
                event: review_ref(ReviewObjectKind::ThreadEvent, 1),
            },
            ReviewEventSelectionV1 {
                sequence: 2,
                event: review_ref(ReviewObjectKind::ThreadEvent, 2),
            },
        ],
        next: None,
    };
    assert!(page.validate().is_err());
    page.entries[1].sequence = 3;
    page.entries[1].event = review_ref(ReviewObjectKind::CommentRevision, 2);
    assert!(page.validate().is_err());

    let mut snapshot = ReviewThreadSnapshotV1 {
        schema_version: 1,
        thread_id,
        original_anchor: review_ref(ReviewObjectKind::Anchor, 1),
        comments: vec![CommentSelectionV1 {
            comment_id: jj_review::CommentId::new([3; 16]),
            revision: review_ref(ReviewObjectKind::CommentRevision, 3),
            parent_comment_id: Some(jj_review::CommentId::new([9; 16])),
        }],
        event_page_root: review_ref(ReviewObjectKind::EventPage, 5),
        last_event_sequence: 1,
        status: ThreadStatusV1::Resolved,
        selected_workflow_action: None,
        thread_version: 1,
        content_version: 1,
        workflow_version: 1,
    };
    assert!(snapshot.validate().is_err());
    snapshot.status = ThreadStatusV1::Open;
    snapshot.comments[0].parent_comment_id = None;
    snapshot.content_version = 2;
    assert!(snapshot.validate().is_err());
}

#[test]
fn anchor_objects_forward_native_and_agent_dependencies() {
    let mut code = anchor();
    code.target = ArtifactTargetV1::Revision {
        operation_id: OperationId::new([6; 64]),
        commit_id: CommitId::new([7; 64]),
        change_id: None,
    };
    let object = ReviewObjectV1::Anchor(Box::new(code));
    assert_eq!(
        object.native_dependencies().unwrap(),
        BTreeSet::from([
            ObjectKey::Operation(OperationId::new([6; 64])),
            ObjectKey::Commit(CommitId::new([7; 64])),
        ])
    );

    let session = AgentObjectRef::new(
        AgentObjectKind::SessionSnapshot,
        AgentObjectId::new([5; 32]),
    );
    let message = AgentObjectRef::new(
        AgentObjectKind::MessageSnapshot,
        AgentObjectId::new([6; 32]),
    );
    let mut agent = anchor();
    agent.target = ArtifactTargetV1::AgentArtifact(Box::new(AgentArtifactAnchorV1 {
        session_id: AgentSessionId::new([8; 16]),
        session_snapshot: session,
        owner: AgentContentOwnerV1::Message {
            message_id: MessageId::new([9; 16]),
            snapshot: message,
        },
    }));
    let object = ReviewObjectV1::Anchor(Box::new(agent));
    assert_eq!(
        object.agent_dependencies().unwrap(),
        BTreeSet::from([session, message])
    );
}

#[test]
fn relation_and_status_pages_are_typed_bounded_and_canonical() {
    let thread_id = ReviewThreadId::new([2; 16]);
    let snapshot = review_ref(ReviewObjectKind::ThreadSnapshot, 3);
    let next_relation = review_ref(ReviewObjectKind::RelationPage, 4);
    let next_status = review_ref(ReviewObjectKind::StatusPage, 5);
    let subject = ReviewRelationSubjectV1::OriginalAnchor {
        anchor_id: anchor().anchor_id().unwrap(),
    };
    let entry = ReviewThreadRelationV1 {
        thread_id,
        snapshot,
    };
    let relation = ReviewObjectV1::RelationPage(ReviewRelationPageV1 {
        schema_version: 1,
        subject: subject.clone(),
        entries: vec![entry],
        next: Some(next_relation),
    });
    let status = ReviewObjectV1::StatusPage(ReviewStatusPageV1 {
        schema_version: 1,
        status: ThreadStatusV1::Open,
        entries: vec![entry],
        next: Some(next_status),
    });
    for object in [relation, status] {
        object.validate().unwrap();
        assert_eq!(
            decode_object(&encode_object(&object).unwrap()).unwrap(),
            object
        );
        assert_eq!(object.review_dependencies().unwrap().len(), 2);
    }
    assert_eq!(
        subject.catalog_key().unwrap(),
        ReviewCatalogKey::derive("anchor-relation", anchor().anchor_id().unwrap().as_bytes())
            .unwrap()
    );
    assert_eq!(
        review_status_key(ThreadStatusV1::Open).unwrap(),
        ReviewCatalogKey::derive("thread-status", b"open").unwrap()
    );

    let invalid = ReviewRelationPageV1 {
        schema_version: 1,
        subject,
        entries: vec![entry, entry],
        next: None,
    };
    assert!(invalid.validate().is_err());
}

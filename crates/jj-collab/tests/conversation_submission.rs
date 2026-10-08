use jj_collab::*;
use jj_store_model::{PrincipalId, RequestId};

fn actor() -> PrincipalId {
    PrincipalId::new([1; 16])
}
fn fixture() -> (ThreadStateV1, CollabGraphV1, ConversationSubmissionV1) {
    let mut graph = CollabGraphV1::default();
    let (root, parent) = ThreadStateV1::empty()
        .apply(
            &ThreadCommandV1::Create {
                thread_id: ThreadId::new([1; 16]),
                title: "Chat".into(),
                baseline: None,
            },
            actor(),
            RequestId::new([1; 16]),
            1,
            &mut graph,
        )
        .unwrap();
    let (root, message) = ThreadStateV1::read(root, &graph)
        .unwrap()
        .apply(
            &ThreadCommandV1::Message {
                thread_id: parent.thread_id,
                expected_thread: parent.version,
                message_id: HumanMessageId::new([1; 16]),
                expected_revision: None,
                reply_to: None,
                body: Some("Hello world".into()),
            },
            actor(),
            RequestId::new([2; 16]),
            2,
            &mut graph,
        )
        .unwrap();
    let source = ConversationSourceV1 {
        target: ReactionTargetV1::HumanMessage {
            thread_id: parent.thread_id,
            message_id: HumanMessageId::new([1; 16]),
            revision: message.message.unwrap(),
        },
        part: None,
        selection: Some(jj_review::TextSelectionV1 {
            encoding: jj_review::TextEncoding::Utf8,
            start_byte: jj_review::ByteOffset::new(0),
            end_byte: jj_review::ByteOffset::new(5),
            selected_sha256: jj_review::ReviewDigest::sha256(b"Hello"),
        }),
        quote: "Hello".into(),
    };
    let submission = ConversationSubmissionV1 {
        thread_id: parent.thread_id,
        expected_thread: message.version,
        message_id: None,
        text: None,
        comments: vec![ConversationCommentDraftV1 {
            thread_id: ThreadId::new([2; 16]),
            expected_thread: None,
            source,
            message_id: HumanMessageId::new([2; 16]),
            expected_revision: None,
            reply_to: None,
            body: "First question".into(),
        }],
        references: vec![],
    };
    (
        ThreadStateV1::read(root, &graph).unwrap(),
        graph,
        submission,
    )
}
#[test]
fn comments_only_and_mixed_sends_publish_one_batch_with_multiple_same_anchor_comments() {
    for mixed in [false, true] {
        let (state, mut graph, mut submission) = fixture();
        let mut another = submission.comments[0].clone();
        another.message_id = HumanMessageId::new([3; 16]);
        another.body = "Second question".into();
        submission.comments.push(another);
        if mixed {
            submission.text = Some("Please address these".into());
            submission.message_id = Some(HumanMessageId::new([4; 16]));
        }
        let request = RequestId::new([3; 16]);
        let (root, result) = state
            .apply(
                &ThreadCommandV1::Submit {
                    submission: Box::new(submission.clone()),
                },
                actor(),
                request,
                3,
                &mut graph,
            )
            .unwrap();
        let next = ThreadStateV1::read(root, &graph).unwrap();
        assert_eq!(next.threads.len(), 2);
        assert_eq!(next.messages.len(), if mixed { 4 } else { 3 });
        let CollabObjectV1::ThreadLink(link) = graph.get(result.link.unwrap()).unwrap() else {
            panic!()
        };
        let ThreadLinkTargetV1::CommentBatch {
            request_id,
            comments,
            message,
        } = &link.target
        else {
            panic!()
        };
        assert_eq!(*request_id, request);
        assert_eq!(comments.len(), 2);
        assert_eq!(message.is_some(), mixed);
        assert!(
            next.submit_conversation(&submission, actor(), request, 4, &mut graph)
                .is_err()
        );
        // Replays belong to the publication receipt; the projector refuses reapplication.
    }
}
#[test]
fn failed_batch_does_not_partially_stage_or_change_existing_comments() {
    let (state, mut graph, mut submission) = fixture();
    let before = graph.objects.clone();
    let mut second = submission.comments[0].clone();
    second.message_id = HumanMessageId::new([3; 16]);
    second.thread_id = ThreadId::new([3; 16]);
    second.reply_to = Some(CollabObjectRef {
        kind: CollabObjectKind::HumanMessageRevision,
        id: CollabObjectId::new([99; 32]),
    });
    second.expected_thread = Some(submission.expected_thread);
    submission.comments.push(second);
    assert!(
        state
            .submit_conversation(&submission, actor(), RequestId::new([3; 16]), 3, &mut graph)
            .is_err()
    );
    assert_eq!(graph.objects, before);
    submission.comments.pop();
    submission.comments[0].body = "x".repeat(32769);
    assert!(
        state
            .submit_conversation(&submission, actor(), RequestId::new([3; 16]), 3, &mut graph)
            .is_err()
    );
    assert_eq!(graph.objects, before);
}
#[test]
fn edits_require_exact_revisions_and_original_author() {
    let (state, mut graph, submission) = fixture();
    let (root, result) = state
        .submit_conversation(&submission, actor(), RequestId::new([3; 16]), 3, &mut graph)
        .unwrap();
    let next = ThreadStateV1::read(root, &graph).unwrap();
    let mut edit = submission.clone();
    edit.expected_thread = result.version;
    edit.comments[0].expected_thread = Some(next.threads[&edit.comments[0].thread_id]);
    edit.comments[0].expected_revision = Some(next.messages[&edit.comments[0].message_id]);
    edit.comments[0].body = "Edited question".into();
    assert!(
        next.submit_conversation(
            &edit,
            PrincipalId::new([9; 16]),
            RequestId::new([4; 16]),
            4,
            &mut graph
        )
        .is_err()
    );
    let (root, result) = next
        .submit_conversation(&edit, actor(), RequestId::new([4; 16]), 4, &mut graph)
        .unwrap();
    let latest = ThreadStateV1::read(root, &graph).unwrap();
    assert_ne!(
        latest.messages[&edit.comments[0].message_id],
        edit.comments[0].expected_revision.unwrap()
    );
    edit.expected_thread = result.version;
    edit.comments[0].expected_thread = Some(latest.threads[&edit.comments[0].thread_id]);
    assert!(
        latest
            .submit_conversation(&edit, actor(), RequestId::new([5; 16]), 5, &mut graph)
            .is_err()
    );
}

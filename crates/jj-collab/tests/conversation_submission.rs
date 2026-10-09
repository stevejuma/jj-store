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
        workspace_comments: vec![],
        reply_to: None,
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
            ..
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

#[test]
fn ordinary_reply_batch_keeps_its_exact_parent_and_refuses_foreign_parents() {
    let (state, mut graph, mut submission) = fixture();
    let parent = state.messages[&HumanMessageId::new([1; 16])];
    submission.text = Some("A reply with comments".into());
    submission.message_id = Some(HumanMessageId::new([4; 16]));
    submission.reply_to = Some(parent);
    let (root, result) = state
        .submit_conversation(&submission, actor(), RequestId::new([4; 16]), 4, &mut graph)
        .unwrap();
    let next = ThreadStateV1::read(root, &graph).unwrap();
    let CollabObjectV1::HumanMessageRevision(message) = graph
        .get(next.messages[&HumanMessageId::new([4; 16])])
        .unwrap()
    else {
        panic!()
    };
    assert_eq!(message.reply_to, Some(parent));
    submission.expected_thread = result.version;
    submission.message_id = Some(HumanMessageId::new([5; 16]));
    submission.reply_to = Some(next.messages[&HumanMessageId::new([2; 16])]);
    submission.comments.clear();
    let before = graph.objects.clone();
    assert!(
        next.submit_conversation(&submission, actor(), RequestId::new([5; 16]), 5, &mut graph)
            .is_err()
    );
    assert_eq!(graph.objects, before);
}

#[test]
fn comment_only_submission_retains_its_reply_parent_without_a_dummy_message() {
    let (state, mut graph, mut submission) = fixture();
    let parent = *state.messages.values().next().unwrap();
    submission.reply_to = Some(parent);
    let (root, result) = state
        .submit_conversation(&submission, actor(), RequestId::new([8; 16]), 3, &mut graph)
        .unwrap();
    let next = ThreadStateV1::read(root, &graph).unwrap();
    assert_eq!(next.messages.len(), state.messages.len() + 1);
    let CollabObjectV1::ThreadLink(link) = graph.get(result.link.unwrap()).unwrap() else {
        panic!()
    };
    assert!(
        matches!(link.target,ThreadLinkTargetV1::CommentBatch { reply_to:Some(r),message:None,.. } if r==parent)
    );
    assert!(
        CollabObjectV1::ThreadLink(link.clone())
            .dependencies()
            .unwrap()
            .contains(&parent)
    );
}

#[test]
fn legacy_whole_message_discussion_continues_without_fabricating_a_range() {
    let (state, mut graph, mut submission) = fixture();
    let target = ReactionTargetV1::AgentMessage {
        session_id: jj_agent::AgentSessionId::new([61; 16]),
        message_id: jj_agent::MessageId::new([62; 16]),
        event_id: jj_agent::AgentEventId::new([63; 16]),
        batch: jj_agent::AgentObjectRef {
            kind: jj_agent::AgentObjectKind::EventBatch,
            id: jj_agent::AgentObjectId::new([64; 32]),
        },
        content: jj_agent::AgentObjectRef {
            kind: jj_agent::AgentObjectKind::Content,
            id: jj_agent::AgentObjectId::new([65; 32]),
        },
    };
    let (root, discussion) = state
        .apply(
            &ThreadCommandV1::CreateAgentDiscussion {
                thread_id: submission.comments[0].thread_id,
                title: "Legacy discussion".into(),
                target: target.clone(),
            },
            actor(),
            RequestId::new([60; 16]),
            3,
            &mut graph,
        )
        .unwrap();
    submission.comments[0].source = ConversationSourceV1 {
        target,
        part: None,
        selection: None,
        quote: String::new(),
    };
    submission.comments[0].expected_thread = Some(discussion.version);
    let (root, _) = ThreadStateV1::read(root, &graph)
        .unwrap()
        .submit_conversation(
            &submission,
            actor(),
            RequestId::new([61; 16]),
            4,
            &mut graph,
        )
        .unwrap();
    let state = ThreadStateV1::read(root, &graph).unwrap();
    let CollabObjectV1::ThreadVersion(thread) =
        graph.get(state.threads[&discussion.thread_id]).unwrap()
    else {
        panic!()
    };
    assert!(thread.comment_source.is_none());
    assert_eq!(
        thread.agent_message.as_ref(),
        Some(&submission.comments[0].source.target)
    );
}

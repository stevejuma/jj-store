use super::*;
use jj_store_model::PrincipalId;

#[test]
fn prompt_feedback_is_exact_bounded_and_omits_tombstones() {
    let target = ReactionTargetV1::AgentMessage {
        session_id: AgentSessionId::new([1; 16]),
        message_id: MessageId::new([2; 16]),
        event_id: AgentEventId::new([3; 16]),
        batch: AgentObjectRef {
            kind: AgentObjectKind::EventBatch,
            id: AgentObjectId::new([4; 32]),
        },
        content: AgentObjectRef {
            kind: AgentObjectKind::Content,
            id: AgentObjectId::new([5; 32]),
        },
    };
    let revision = CollabObjectRef {
        kind: CollabObjectKind::HumanMessageRevision,
        id: CollabObjectId::new([6; 32]),
    };
    let thread_id = ThreadId::new([7; 16]);
    let message = HumanMessageRevisionV1 {
        emojis: Default::default(),
        schema_version: 1,
        message_id: HumanMessageId::new([8; 16]),
        thread_id,
        actor: PrincipalId::new([9; 16]),
        accepted_at_millis: 10,
        previous: None,
        reply_to: None,
        body: Some("Use the existing repository form component.".into()),
    };
    let mut feedback = Feedback {
        profile: None,
        write_unavailable: None,
        messages: vec![FeedbackMessage {
            row_id: "prompt/assistant".into(),
            target: target.clone(),
            threads: vec![thread_id],
            text_parts: vec![],
            answers: vec![],
            replies: vec![FeedbackReply {
                historical: false,
                batches: vec![],
                thread_id,
                revision,
                message,
                thread_revision: None,
                source: None,
                answers: vec![],
            }],
        }],
    };
    let prepared = context(&feedback).unwrap();
    assert!(prepared.contains("Use the existing repository form component."));
    assert!(prepared.contains(&revision.id.to_string()));
    assert!(prepared.contains("original_message"));
    assert_eq!(context(&feedback).unwrap(), prepared);
    feedback.messages[0].replies[0].historical = true;
    assert_eq!(context(&feedback).unwrap(), "");
    feedback.messages[0].replies[0].historical = false;
    feedback.messages[0].replies[0].message.body = None;
    assert_eq!(context(&feedback).unwrap(), "");
    feedback.messages[0].replies[0].message.body = Some("x".repeat(32769));
    assert!(matches!(context(&feedback), Err(Error::Limit)));
}

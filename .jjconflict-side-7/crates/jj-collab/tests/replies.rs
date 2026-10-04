use jj_collab::*;
use jj_store_model::{PrincipalId, RequestId};

#[test]
fn replies_retain_exact_parent_and_reject_cross_thread_or_self_links() {
    let mut graph = CollabGraphV1::default();
    let actor = PrincipalId::new([1; 16]);
    let thread = ThreadId::new([2; 16]);
    let (root, created) = ThreadStateV1::empty()
        .apply(
            &ThreadCommandV1::Create {
                thread_id: thread,
                title: "Channel".into(),
                baseline: None,
            },
            actor,
            RequestId::new([3; 16]),
            1,
            &mut graph,
        )
        .unwrap();
    let state = ThreadStateV1::read(root, &graph).unwrap();
    let (root, parent) = state
        .apply(
            &ThreadCommandV1::Message {
                thread_id: thread,
                expected_thread: created.version,
                message_id: HumanMessageId::new([4; 16]),
                expected_revision: None,
                reply_to: None,
                body: Some("Original".into()),
            },
            actor,
            RequestId::new([5; 16]),
            2,
            &mut graph,
        )
        .unwrap();
    let state = ThreadStateV1::read(root, &graph).unwrap();
    let reply = ThreadCommandV1::Message {
        thread_id: thread,
        expected_thread: parent.version,
        message_id: HumanMessageId::new([6; 16]),
        expected_revision: None,
        reply_to: parent.message,
        body: Some("Reply".into()),
    };
    let (root, result) = state
        .apply(&reply, actor, RequestId::new([7; 16]), 3, &mut graph)
        .unwrap();
    let CollabObjectV1::HumanMessageRevision(message) = graph.get(result.message.unwrap()).unwrap()
    else {
        panic!()
    };
    assert_eq!(message.reply_to, parent.message);
    assert!(
        CollabObjectV1::HumanMessageRevision(message.clone())
            .dependencies()
            .unwrap()
            .contains(&parent.message.unwrap())
    );
    let reply_state = ThreadStateV1::read(root.clone(), &graph).unwrap();
    let mut edited = reply.clone();
    if let ThreadCommandV1::Message {
        expected_thread,
        expected_revision,
        reply_to,
        body,
        ..
    } = &mut edited
    {
        *expected_thread = result.version;
        *expected_revision = result.message;
        *body = Some("Edited reply".into());
        *reply_to = None;
    }
    assert!(
        reply_state
            .apply(&edited, actor, RequestId::new([13; 16]), 4, &mut graph)
            .is_err()
    );
    if let ThreadCommandV1::Message { reply_to, .. } = &mut edited {
        *reply_to = parent.message;
    }
    let (_, edit) = reply_state
        .apply(&edited, actor, RequestId::new([14; 16]), 4, &mut graph)
        .unwrap();
    let closure = graph.closure([edit.message.unwrap()]).unwrap();
    assert!(closure.contains(&parent.message.unwrap()));
    let cold = CollabGraphV1 {
        objects: closure
            .into_iter()
            .map(|reference| {
                let bytes = encode_object(graph.get(reference).unwrap()).unwrap();
                (reference, decode_object(&bytes).unwrap())
            })
            .collect(),
    };
    assert!(
        cold.closure([edit.message.unwrap()])
            .unwrap()
            .contains(&parent.message.unwrap())
    );
    let mut invalid = reply.clone();
    if let ThreadCommandV1::Message { message_id, .. } = &mut invalid {
        *message_id = HumanMessageId::new([4; 16]);
    }
    assert!(
        state
            .apply(&invalid, actor, RequestId::new([8; 16]), 3, &mut graph)
            .is_err()
    );
    let state = ThreadStateV1::read(root, &graph).unwrap();
    let (root, other) = state
        .apply(
            &ThreadCommandV1::Create {
                thread_id: ThreadId::new([9; 16]),
                title: "Other".into(),
                baseline: None,
            },
            actor,
            RequestId::new([10; 16]),
            4,
            &mut graph,
        )
        .unwrap();
    let state = ThreadStateV1::read(root, &graph).unwrap();
    if let ThreadCommandV1::Message {
        thread_id,
        expected_thread,
        message_id,
        ..
    } = &mut invalid
    {
        *thread_id = ThreadId::new([9; 16]);
        *expected_thread = other.version;
        *message_id = HumanMessageId::new([11; 16]);
    }
    assert!(
        state
            .apply(&invalid, actor, RequestId::new([12; 16]), 5, &mut graph)
            .is_err()
    );
}

#[test]
fn legacy_messages_keep_canonical_bytes_without_reply_field() {
    let legacy = format!(
        "{{\"type\":\"human_message_revision\",\"value\":{{\"schema_version\":1,\"message_id\":\"{}\",\"thread_id\":\"{}\",\"actor\":\"{}\",\"accepted_at_millis\":1,\"previous\":null,\"body\":\"Legacy message\"}}}}",
        "01".repeat(16),
        "02".repeat(16),
        "03".repeat(16)
    );
    let decoded = decode_object(legacy.as_bytes()).unwrap();
    let CollabObjectV1::HumanMessageRevision(ref message) = decoded else {
        panic!()
    };
    assert_eq!(message.reply_to, None);
    assert_eq!(encode_object(&decoded).unwrap(), legacy.as_bytes());
}

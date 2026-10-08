use jj_collab::*;
use jj_store_model::{PrincipalId, RequestId};

#[test]
fn agent_discussion_retains_exact_target_and_legacy_bytes() {
    let target = serde_json::json!({"type":"agent_message","session_id":"11".repeat(16),
        "message_id":"22".repeat(16),"event_id":"33".repeat(16),
        "batch":{"kind":"event_batch","id":"44".repeat(32)},
        "content":{"kind":"content","id":"55".repeat(32)}});
    let command: ThreadCommandV1 = serde_json::from_value(serde_json::json!({
        "type":"create_agent_discussion","thread_id":"66".repeat(16),
        "title":"Reply to agent","target":target
    }))
    .expect("agent messages must support repository discussion");
    let mut graph = CollabGraphV1::default();
    let (root, result) = ThreadStateV1::empty()
        .apply(
            &command,
            PrincipalId::new([1; 16]),
            RequestId::new([2; 16]),
            10,
            &mut graph,
        )
        .unwrap();
    let state = ThreadStateV1::read(root, &graph).unwrap();
    let original = graph.get(result.version).unwrap();
    assert_eq!(
        serde_json::to_value(original).unwrap()["value"]["agent_message"],
        target
    );
    let (_, renamed) = state
        .apply(
            &ThreadCommandV1::Rename {
                thread_id: result.thread_id,
                expected: result.version,
                title: "Follow up".into(),
            },
            PrincipalId::new([1; 16]),
            RequestId::new([3; 16]),
            11,
            &mut graph,
        )
        .unwrap();
    assert_eq!(
        serde_json::to_value(graph.get(renamed.version).unwrap()).unwrap()["value"]["agent_message"],
        target
    );
    let legacy = ThreadCommandV1::Create {
        thread_id: ThreadId::new([9; 16]),
        title: "Legacy".into(),
        baseline: None,
    };
    let (_, legacy) = ThreadStateV1::empty()
        .apply(
            &legacy,
            PrincipalId::new([1; 16]),
            RequestId::new([4; 16]),
            12,
            &mut graph,
        )
        .unwrap();
    let bytes = encode_object(graph.get(legacy.version).unwrap()).unwrap();
    assert!(
        !String::from_utf8(bytes.clone())
            .unwrap()
            .contains("agent_message")
    );
    assert_eq!(
        encode_object(&decode_object(&bytes).unwrap()).unwrap(),
        bytes
    );
}

#[test]
fn range_discussion_retains_source_across_lifecycle_without_changing_old_bytes() {
    let source: ConversationSourceV1 = serde_json::from_value(serde_json::json!({
        "target":{"type":"agent_message","session_id":"11".repeat(16),"message_id":"22".repeat(16),"event_id":"33".repeat(16),"batch":{"kind":"event_batch","id":"44".repeat(32)},"content":{"kind":"content","id":"55".repeat(32)}},
        "part":null,"selection":{"encoding":"utf8","start_byte":"0","end_byte":"5","selected_sha256":jj_review::ReviewDigest::sha256(b"hello")},"quote":"hello"
    })).unwrap();
    let mut graph = CollabGraphV1::default();
    let (root, created) = ThreadStateV1::empty()
        .apply(
            &ThreadCommandV1::CreateCommentDiscussion {
                thread_id: ThreadId::new([7; 16]),
                title: "Selection".into(),
                source: source.clone(),
            },
            PrincipalId::new([1; 16]),
            RequestId::new([1; 16]),
            1,
            &mut graph,
        )
        .unwrap();
    let state = ThreadStateV1::read(root, &graph).unwrap();
    let (_, updated) = state
        .apply(
            &ThreadCommandV1::Rename {
                thread_id: created.thread_id,
                expected: created.version,
                title: "New name".into(),
            },
            PrincipalId::new([1; 16]),
            RequestId::new([2; 16]),
            2,
            &mut graph,
        )
        .unwrap();
    let CollabObjectV1::ThreadVersion(version) = graph.get(updated.version).unwrap() else {
        panic!()
    };
    assert_eq!(version.comment_source.as_ref(), Some(&source));
    let mut forged = version.clone();
    forged.comment_source.as_mut().unwrap().quote = "other".into();
    let forged = graph.stage(CollabObjectV1::ThreadVersion(forged)).unwrap();
    assert!(graph.closure([forged]).is_err());
    let old = ThreadCommandV1::Create {
        thread_id: ThreadId::new([8; 16]),
        title: "Old".into(),
        baseline: None,
    };
    let (_, legacy) = ThreadStateV1::empty()
        .apply(
            &old,
            PrincipalId::new([1; 16]),
            RequestId::new([3; 16]),
            3,
            &mut graph,
        )
        .unwrap();
    let bytes = encode_object(graph.get(legacy.version).unwrap()).unwrap();
    assert!(!String::from_utf8_lossy(&bytes).contains("comment_source"));
    assert_eq!(
        encode_object(&decode_object(&bytes).unwrap()).unwrap(),
        bytes
    );
}

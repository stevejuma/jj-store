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

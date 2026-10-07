use super::*;
use jj_collab::{CollabObjectId, CollabObjectKind, CollabObjectRef, HumanMessageId, ThreadId};
fn fixture(root: &Path) -> (State, String) {
    let id = "44".repeat(16);
    let path = root.join("workspaces").join(format!("agent-{id}"));
    let saved: SavedSession = serde_json::from_value(json!({
        "view":{"id":id,"setup_id":null,"connected":false,"repository":"11".repeat(16),"target_id":"fixture","runner":"codex",
            "title":"Fixture","avatar":"🤖","workspace":path,"base_commit":"22".repeat(20),"review_commit":"22".repeat(20),
            "operation":"33".repeat(64),"state":"idle","adapter_session_id":"adapter","capabilities":{},
            "preferences":{"model":null,"effort":null,"mode":"ask"},"last_sequence":0,"published_sequence":0,"last_error":null},
        "workspace":{"source":root.join("source"),"path":path,"metadata":root.join("source/.jj"),
            "operation":"33".repeat(64),"commit":"22".repeat(20),"name":format!("agent-{id}")},
        "profile":"local_jj_git","instructions":"","context":null,"events":[],"workspace_ready":true
    })).unwrap();
    (
        State {
            sessions: BTreeMap::from([(id.clone(), saved)]),
            ..Default::default()
        },
        id,
    )
}
fn revision(n: u8) -> ReplyRevision {
    ReplyRevision {
        repository: RepositoryId::new([1; 16]),
        thread: ThreadId::new([2; 16]),
        message: HumanMessageId::new([3; 16]),
        revision: CollabObjectRef {
            kind: CollabObjectKind::HumanMessageRevision,
            id: CollabObjectId::new([n; 32]),
        },
    }
}
fn prepare_one(state: &mut State, id: &str, n: u8) -> RequestId {
    let request = RequestId::new([n; 16]);
    let carries = needs_initial_context(state, id).unwrap();
    prepare(state, id, request, format!("digest-{n}"), "Original prompt".into(),
        ContextBatch { revisions:BTreeSet::from([revision(n)]), text: format!("Feedback {n}") },
        json!({"type":"prompt","requestId":request.to_string(),"text":format!("Feedback {n}\nOriginal prompt")}), carries).unwrap();
    request
}
#[test]
fn queued_replies_reserve_release_and_deliver_only_on_exact_started() {
    let root = tempfile::tempdir().unwrap();
    let (mut state, id) = fixture(root.path());
    let a = prepare_one(&mut state, &id, 10);
    assert_eq!(excluded(&state, &id), BTreeSet::from([revision(10)]));
    assert!(state.sessions[&id].delivered_replies.is_empty());
    observe(&mut state, &id, &json!({"type":"queued","requestId":a})).unwrap();
    assert_eq!(excluded(&state, &id).len(), 1);
    observe(
        &mut state,
        &id,
        &json!({"type":"record","requestId":a,"record":{"status":"cancelled"}}),
    )
    .unwrap();
    assert!(excluded(&state, &id).is_empty());
    let b = prepare_one(&mut state, &id, 11);
    observe(&mut state, &id, &json!({"type":"started","requestId":b})).unwrap();
    observe(
        &mut state,
        &id,
        &json!({"type":"record","requestId":b,"record":{"status":"cancelled"}}),
    )
    .unwrap();
    assert_eq!(
        state.sessions[&id].delivered_replies,
        BTreeSet::from([revision(11)])
    );
    let c = prepare_one(&mut state, &id, 12);
    observe(
        &mut state,
        &id,
        &json!({"type":"error","requestId":c,"code":"queue_full"}),
    )
    .unwrap();
    assert!(!excluded(&state, &id).contains(&revision(12)));
    let d = prepare_one(&mut state, &id, 13);
    observe(
        &mut state,
        &id,
        &json!({"type":"error","requestId":d,"code":"duplicate_request"}),
    )
    .unwrap();
    assert!(excluded(&state, &id).contains(&revision(13))); // unknown is not proven rejection
    assert!(
        observe(
            &mut state,
            "foreign",
            &json!({"type":"started","requestId":d})
        )
        .is_err()
    );
    assert!(excluded(&state, "foreign").is_empty());
}
#[test]
fn restart_retains_batch_intent_reservations_and_legacy_defaults() {
    let root = tempfile::tempdir().unwrap();
    let (mut state, id) = fixture(root.path());
    assert!(excluded(&state, &id).is_empty());
    let a = prepare_one(&mut state, &id, 10);
    let b = prepare_one(&mut state, &id, 11);
    observe(&mut state, &id, &json!({"type":"started","requestId":b})).unwrap();
    journal::persist(root.path(), &state).unwrap();
    let restored = journal::read(root.path()).unwrap();
    assert_eq!(restored.intents[&a].outcome.state, "outcome_unknown");
    assert_eq!(
        restored.intents[&a].prompt.as_ref().unwrap().original,
        "Original prompt"
    );
    assert_eq!(
        restored.intents[&a].prompt.as_ref().unwrap().batch.text,
        "Feedback 10"
    );
    assert_eq!(
        excluded(&restored, &id),
        BTreeSet::from([revision(10), revision(11)])
    );
    assert_eq!(
        restored.sessions[&id].delivered_replies,
        BTreeSet::from([revision(11)])
    );
    assert_eq!(restored.sessions[&id].events.len(), 2);
    let mut legacy = serde_json::to_value(state).unwrap();
    legacy["sessions"][&id]
        .as_object_mut()
        .unwrap()
        .remove("delivered_replies");
    for intent in legacy["intents"].as_object_mut().unwrap().values_mut() {
        intent.as_object_mut().unwrap().remove("prompt");
    }
    let legacy: State = serde_json::from_value(legacy).unwrap();
    assert!(excluded(&legacy, &id).is_empty());
}
#[test]
fn continuation_tracks_only_structured_feedback_in_whole_retained_records() {
    let root = tempfile::tempdir().unwrap();
    let (mut state, id) = fixture(root.path());
    let a = prepare_one(&mut state, &id, 10);
    observe(&mut state, &id, &json!({"type":"started","requestId":a})).unwrap();
    append(
        &mut state,
        &id,
        json!({"type":"record","requestId":a,"assistantText":"x".repeat(32768)}),
    )
    .unwrap();
    let b = prepare_one(&mut state, &id, 11);
    observe(&mut state, &id, &json!({"type":"started","requestId":b})).unwrap();
    continuation(&mut state, &id).unwrap();
    assert_eq!(
        state.sessions[&id].context_replies,
        BTreeSet::from([revision(11)])
    );
    assert!(state.sessions[&id].view.adapter_session_id.is_none());
    let context = state.sessions[&id].context.as_ref().unwrap();
    assert!(context.contains("Feedback 11"));
    assert!(!context.contains("Feedback 10"));
    assert!(context.len() <= 32768);
}

#[test]
fn fresh_adapter_keeps_unknown_intent_but_only_reserves_its_own_context_generation() {
    let root = tempfile::tempdir().unwrap();
    let (mut state, id) = fixture(root.path());
    let request = prepare_one(&mut state, &id, 10);
    append(
        &mut state,
        &id,
        json!({"type":"record","assistantText":"x".repeat(32768)}),
    )
    .unwrap();
    assert!(excluded(&state, &id).contains(&revision(10)));
    continuation(&mut state, &id).unwrap();
    assert!(excluded(&state, &id).is_empty());
    assert_eq!(
        state.intents[&request].prompt.as_ref().unwrap().state,
        DeliveryState::Reserved
    );
    assert_eq!(
        state.intents[&request].prompt.as_ref().unwrap().batch.text,
        "Feedback 10"
    );
}

#[test]
fn cancelled_initial_prompt_does_not_lose_continuation_context_to_a_queued_turn() {
    let root = tempfile::tempdir().unwrap();
    let (mut state, id) = fixture(root.path());
    let first = prepare_one(&mut state, &id, 10);
    let queued = prepare_one(&mut state, &id, 11);
    assert!(
        !state.intents[&queued]
            .prompt
            .as_ref()
            .unwrap()
            .carries_initial_context
    );
    observe(
        &mut state,
        &id,
        &json!({"type":"record","requestId":first,"record":{"status":"cancelled"}}),
    )
    .unwrap();
    observe(
        &mut state,
        &id,
        &json!({"type":"started","requestId":queued}),
    )
    .unwrap();
    assert!(needs_initial_context(&state, &id).unwrap());
    let retried = prepare_one(&mut state, &id, 12);
    observe(
        &mut state,
        &id,
        &json!({"type":"started","requestId":retried}),
    )
    .unwrap();
    assert!(!needs_initial_context(&state, &id).unwrap());
}

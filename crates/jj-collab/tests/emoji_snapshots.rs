use jj_collab::*;
use jj_store_model::{PrincipalId, RepositoryId, RequestId};
use std::collections::{BTreeMap, BTreeSet};
fn snapshot() -> EmojiSnapshotV1 {
    let mut bytes = std::io::Cursor::new(Vec::new());
    image::DynamicImage::ImageRgba8(image::RgbaImage::from_pixel(
        1,
        1,
        image::Rgba([0, 255, 0, 255]),
    ))
    .write_to(&mut bytes, image::ImageFormat::Png)
    .unwrap();
    EmojiSnapshotV1 {
        source: format!(
            "installation:{}:{}:{}",
            "ab".repeat(16),
            "cd".repeat(16),
            "ef".repeat(32)
        ),
        shortcode: "party".into(),
        bytes: bytes.into_inner(),
    }
}
#[test]
fn messages_and_reactions_retain_independent_immutable_library_closures() {
    let actor = PrincipalId::new([1; 16]);
    let repo = RepositoryId::new([2; 16]);
    let thread = ThreadId::new([3; 16]);
    let selected = snapshot();
    let mut graph = CollabGraphV1::default();
    let (root, created) = ThreadStateV1::empty()
        .apply(
            &ThreadCommandV1::Create {
                thread_id: thread,
                title: "T".into(),
                baseline: None,
            },
            actor,
            RequestId::new([1; 16]),
            1,
            &mut graph,
        )
        .unwrap();
    let state = ThreadStateV1::read(root, &graph).unwrap();
    let command = ThreadCommandV1::Message {
        emojis: vec![selected.clone()],
        thread_id: thread,
        expected_thread: created.version,
        message_id: HumanMessageId::new([4; 16]),
        expected_revision: None,
        reply_to: None,
        body: Some("Hello :party:".into()),
    };
    let (root, result) = state
        .apply(&command, actor, RequestId::new([2; 16]), 2, &mut graph)
        .unwrap();
    let CollabObjectV1::HumanMessageRevision(message) = graph.get(result.message.unwrap()).unwrap()
    else {
        panic!()
    };
    let definitions = message_emoji_definitions(Some(message), &graph).unwrap();
    assert_eq!(definitions["party"].label, "party");
    let closure = graph.closure(root.dependencies()).unwrap();
    assert!(closure.contains(&definitions["party"].asset));
    assert!(closure.contains(&definitions["party"].preview));
    let mut reactions = CollabGraphV1::default();
    for reference in &closure {
        reactions
            .stage(graph.get(*reference).unwrap().clone())
            .unwrap();
    }
    let value = selected.stage(&mut CollabGraphV1::default()).unwrap();
    let set = ReactionCommandV1::Set {
        emoji: Some(selected.clone()),
        target: ReactionTargetV1::HumanMessage {
            thread_id: thread,
            message_id: message.message_id,
            revision: result.message.unwrap(),
        },
        value: value.clone(),
        observed: BTreeSet::new(),
    };
    let (root, result) = ReactionStateV1::empty()
        .apply(
            &set,
            repo,
            actor,
            RequestId::new([3; 16]),
            3,
            &mut reactions,
        )
        .unwrap();
    assert_eq!(result.value, Some(value));
    let retained = ReactionStateV1::read(repo, root.clone(), &reactions).unwrap();
    let ReactionValueV1::Custom { version, .. } = result.value.unwrap() else {
        panic!()
    };
    assert!(
        reactions
            .closure(root.dependencies())
            .unwrap()
            .contains(&version)
    );
    // A replacement selection gets a different identity while the first bytes remain reachable.
    let mut replacement = selected;
    replacement.source = format!(
        "installation:{}:{}:{}",
        "ab".repeat(16),
        "cd".repeat(16),
        "01".repeat(32)
    );
    assert_ne!(replacement.stage(&mut reactions).unwrap(), set_value(&set));
    assert!(retained.emojis.values().any(|v| *v == version));
}
fn set_value(command: &ReactionCommandV1) -> ReactionValueV1 {
    let ReactionCommandV1::Set { value, .. } = command else {
        panic!()
    };
    value.clone()
}
#[test]
fn legacy_messages_keep_their_canonical_bytes_and_snapshot_claims_do_not_grant_access() {
    let message = HumanMessageRevisionV1 {
        emojis: BTreeMap::new(),
        schema_version: 1,
        message_id: HumanMessageId::new([1; 16]),
        thread_id: ThreadId::new([2; 16]),
        actor: PrincipalId::new([3; 16]),
        accepted_at_millis: 0,
        previous: None,
        reply_to: None,
        body: Some("legacy".into()),
    };
    let json = serde_json::to_string(&message).unwrap();
    assert!(!json.contains("emojis"));
    assert_eq!(
        serde_json::from_str::<HumanMessageRevisionV1>(&json).unwrap(),
        message
    );
    let mut selected = snapshot();
    selected.source = "https://example.com/emoji".into();
    assert!(selected.validate().is_err());
    selected = snapshot();
    selected.bytes = b"<svg onload='alert(1)'/>".to_vec();
    assert!(selected.stage(&mut CollabGraphV1::default()).is_err());
}

use jj_collab::*;
use jj_store_model::{PrincipalId, RequestId};
fn actor() -> PrincipalId {
    PrincipalId::new([1; 16])
}
fn request(n: u8) -> RequestId {
    RequestId::new([n; 16])
}
#[test]
fn legacy_root_bytes_and_metadata_preconditions_are_preserved() {
    let old =
        br#"{"schema_version":1,"threads":null,"messages":null,"links":null,"assignments":null}"#;
    assert_eq!(
        encode_thread_root(&decode_thread_root(old).unwrap()).unwrap(),
        old
    );
    let mut graph = CollabGraphV1::default();
    let id = ThreadId::new([3; 16]);
    let (root, created) = ThreadStateV1::empty()
        .apply(
            &ThreadCommandV1::Create {
                thread_id: id,
                title: "Owner".into(),
                baseline: None,
            },
            actor(),
            request(1),
            1,
            &mut graph,
        )
        .unwrap();
    let target = ConversationMetadataTargetV1::Conversation {
        id: "stable".into(),
    };
    let command = ThreadCommandV1::SetMetadata {
        thread_id: id,
        expected_thread: created.version,
        expected_metadata: None,
        target: target.clone(),
        value: Box::new(ConversationMetadataValueV1 {
            custom_title: Some("My title".into()),
            ..Default::default()
        }),
    };
    let state = ThreadStateV1::read(root, &graph).unwrap();
    let (root, result) = state
        .apply(&command, actor(), request(2), 2, &mut graph)
        .unwrap();
    let state = ThreadStateV1::read(root, &graph).unwrap();
    assert!(matches!(
        state.apply(&command, actor(), request(3), 3, &mut graph),
        Err(Error::Stale)
    ));
    let reference = state.metadata[&target.key().unwrap()];
    let CollabObjectV1::ConversationMetadata(value) = graph.get(reference).unwrap() else {
        panic!()
    };
    assert_eq!(value.value.title(), Some("My title"));
    let CollabObjectV1::ThreadVersion(thread) = graph.get(result.version).unwrap() else {
        panic!()
    };
    assert_eq!(thread.title, "Owner");
    assert!(state.messages.is_empty());
    assert!(state.assignments.is_empty());
    graph.closure(state.root.dependencies()).unwrap();
}
#[test]
fn ambiguous_agent_titles_never_win_and_custom_titles_are_retained() {
    let mut v = ConversationMetadataValueV1::default();
    v.agent_titles.insert("a".into(), "One".into());
    v.agent_titles.insert("b".into(), "Two".into());
    assert_eq!(v.title(), None);
    v.custom_title = Some("Chosen".into());
    assert_eq!(v.title(), Some("Chosen"));
}

#[test]
fn suggestions_require_snapshot_provenance_and_generated_fields_are_bounded() {
    use jj_agent::{AgentObjectId, AgentObjectKind, AgentObjectRef, AgentSessionId};
    let source = ConversationMetadataSourceV1 {
        session: AgentSessionId::new([1; 16]),
        snapshot: AgentObjectRef::new(
            AgentObjectKind::SessionSnapshot,
            AgentObjectId::new([1; 32]),
        ),
    };
    let mut value = ConversationMetadataValueV1::default();
    value.agent_titles.insert("a".into(), "One".into());
    assert!(value.validate().is_err());
    value.agent_title_sources.insert("a".into(), source.clone());
    value.validate().unwrap();
    value.generated = Some(GeneratedConversationMetadataV1 {
        title: "A concise title".into(),
        summary: "Visible conversation summary".into(),
        keywords: vec!["Unicode 🦀".into()],
        goals: vec!["Complete the request".into()],
        sources: vec![source.clone()],
        provider: "fixture".into(),
        model: "test".into(),
        prompt_version: 1,
        generated_at_millis: 1,
        truncated: false,
    });
    value.validate().unwrap();
    assert!(value.search_text().contains("Unicode 🦀"));
    assert_eq!(value.source_refs().len(), 2);
    let original = value.generated.clone().unwrap();
    value
        .generated
        .as_mut()
        .unwrap()
        .keywords
        .push(" unicode 🦀 ".into());
    assert!(value.validate().is_err());
    value.generated = Some(original.clone());
    value.generated.as_mut().unwrap().sources.push(source);
    assert!(value.validate().is_err());
    value.generated = Some(original);
    value.generated.as_mut().unwrap().sources[0].snapshot.kind = AgentObjectKind::Content;
    assert!(value.validate().is_err());
}

#[test]
fn metadata_owner_is_immutable_and_tombstoned_threads_refuse_new_revisions() {
    let mut graph = CollabGraphV1::default();
    let id = ThreadId::new([5; 16]);
    let other = ThreadId::new([6; 16]);
    let (root, first) = ThreadStateV1::empty()
        .apply(
            &ThreadCommandV1::Create {
                thread_id: id,
                title: "Owner".into(),
                baseline: None,
            },
            actor(),
            request(10),
            10,
            &mut graph,
        )
        .unwrap();
    let target = ConversationMetadataTargetV1::Conversation {
        id: "conversation".into(),
    };
    let (root, published) = ThreadStateV1::read(root, &graph)
        .unwrap()
        .apply(
            &ThreadCommandV1::SetMetadata {
                thread_id: id,
                expected_thread: first.version,
                expected_metadata: None,
                target: target.clone(),
                value: Box::default(),
            },
            actor(),
            request(11),
            11,
            &mut graph,
        )
        .unwrap();
    let reference =
        ThreadStateV1::read(root.clone(), &graph).unwrap().metadata[&target.key().unwrap()];
    let (root, created) = ThreadStateV1::read(root, &graph)
        .unwrap()
        .apply(
            &ThreadCommandV1::Create {
                thread_id: other,
                title: "Other owner".into(),
                baseline: None,
            },
            actor(),
            request(12),
            12,
            &mut graph,
        )
        .unwrap();
    let command = ThreadCommandV1::SetMetadata {
        thread_id: other,
        expected_thread: created.version,
        expected_metadata: Some(reference),
        target: target.clone(),
        value: Box::default(),
    };
    assert!(matches!(
        ThreadStateV1::read(root.clone(), &graph).unwrap().apply(
            &command,
            actor(),
            request(13),
            13,
            &mut graph
        ),
        Err(Error::Invalid(_))
    ));
    let (root, activated) = ThreadStateV1::read(root, &graph)
        .unwrap()
        .apply(
            &ThreadCommandV1::Lifecycle {
                thread_id: id,
                expected: published.version,
                action: ThreadActionV1::Activate,
            },
            actor(),
            request(20),
            14,
            &mut graph,
        )
        .unwrap();
    let (root, deleted) = ThreadStateV1::read(root, &graph)
        .unwrap()
        .apply(
            &ThreadCommandV1::Lifecycle {
                thread_id: id,
                expected: activated.version,
                action: ThreadActionV1::Tombstone,
            },
            actor(),
            request(14),
            14,
            &mut graph,
        )
        .unwrap();
    let command = ThreadCommandV1::SetMetadata {
        thread_id: id,
        expected_thread: deleted.version,
        expected_metadata: Some(reference),
        target,
        value: Box::default(),
    };
    assert!(matches!(
        ThreadStateV1::read(root, &graph).unwrap().apply(
            &command,
            actor(),
            request(15),
            15,
            &mut graph
        ),
        Err(Error::TargetUnavailable)
    ));
}

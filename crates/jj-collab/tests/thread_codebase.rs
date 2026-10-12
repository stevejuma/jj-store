use jj_collab::*;
use jj_store_model::{PrincipalId, RepositoryId, RequestId};

fn version(graph: &CollabGraphV1, reference: CollabObjectRef) -> &ThreadVersionV1 {
    let CollabObjectV1::ThreadVersion(value) = graph.get(reference).unwrap() else {
        panic!()
    };
    value
}
#[test]
fn internal_owners_preserve_comments_metadata_and_stay_out_of_user_threads() {
    let mut graph = CollabGraphV1::default();
    let id = ThreadId::new([1; 16]);
    let actor = PrincipalId::new([2; 16]);
    let target = ConversationMetadataTargetV1::Conversation { id: "chat".into() };
    let (root, created) = ThreadStateV1::empty()
        .apply(
            &ThreadCommandV1::CreateConversation {
                thread_id: id,
                title: "Agent conversation".into(),
                target: target.clone(),
            },
            actor,
            RequestId::new([3; 16]),
            1,
            &mut graph,
        )
        .unwrap();
    assert!(!version(&graph, created.version).is_user_thread());
    let (root, metadata) = ThreadStateV1::read(root, &graph)
        .unwrap()
        .apply(
            &ThreadCommandV1::SetMetadata {
                thread_id: id,
                expected_thread: created.version,
                expected_metadata: None,
                target,
                value: Box::new(ConversationMetadataValueV1 {
                    custom_title: Some("Title".into()),
                    ..Default::default()
                }),
            },
            actor,
            RequestId::new([4; 16]),
            2,
            &mut graph,
        )
        .unwrap();
    assert!(!version(&graph, metadata.version).is_user_thread());
    let state = ThreadStateV1::read(root, &graph).unwrap();
    assert!(matches!(
        state.apply(
            &ThreadCommandV1::SetCodebase {
                thread_id: id,
                expected: metadata.version,
                repositories: vec![],
            },
            actor,
            RequestId::new([5; 16]),
            3,
            &mut graph
        ),
        Err(Error::Invalid(_))
    ));
    graph.closure(state.root.dependencies()).unwrap();
}
#[test]
fn intentional_codebases_are_unique_bounded_version_checked_and_retained_by_edits() {
    let mut graph = CollabGraphV1::default();
    let id = ThreadId::new([1; 16]);
    let actor = PrincipalId::new([2; 16]);
    let repos = vec![RepositoryId::new([8; 16]), RepositoryId::new([9; 16])];
    let command = ThreadCommandV1::CreateIntentional {
        thread_id: id,
        title: "Thread".into(),
        repositories: repos.clone(),
    };
    let (root, created) = ThreadStateV1::empty()
        .apply(&command, actor, RequestId::new([3; 16]), 1, &mut graph)
        .unwrap();
    assert!(version(&graph, created.version).is_user_thread());
    assert_eq!(
        version(&graph, created.version).codebase.as_ref(),
        Some(&repos)
    );
    let change = ThreadCommandV1::SetCodebase {
        thread_id: id,
        expected: created.version,
        repositories: vec![],
    };
    let state = ThreadStateV1::read(root, &graph).unwrap();
    let (root, changed) = state
        .apply(&change, actor, RequestId::new([4; 16]), 2, &mut graph)
        .unwrap();
    let state = ThreadStateV1::read(root, &graph).unwrap();
    assert!(matches!(
        state.apply(&change, actor, RequestId::new([5; 16]), 3, &mut graph),
        Err(Error::Stale)
    ));
    let (_, renamed) = state
        .apply(
            &ThreadCommandV1::Rename {
                thread_id: id,
                expected: changed.version,
                title: "Renamed".into(),
            },
            actor,
            RequestId::new([6; 16]),
            4,
            &mut graph,
        )
        .unwrap();
    assert_eq!(version(&graph, renamed.version).codebase, Some(vec![]));
    assert!(validate_codebase(&[repos[0], repos[0]]).is_err());
    assert!(
        validate_codebase(
            &(0..9)
                .map(|i| RepositoryId::new([i; 16]))
                .collect::<Vec<_>>()
        )
        .is_err()
    );
}
#[test]
fn legacy_thread_versions_omit_new_fields_and_remain_user_threads() {
    let mut graph = CollabGraphV1::default();
    let (_, result) = ThreadStateV1::empty()
        .apply(
            &ThreadCommandV1::Create {
                thread_id: ThreadId::new([1; 16]),
                title: "Old".into(),
                baseline: None,
            },
            PrincipalId::new([2; 16]),
            RequestId::new([3; 16]),
            1,
            &mut graph,
        )
        .unwrap();
    let value = version(&graph, result.version);
    assert!(value.is_user_thread());
    let bytes = serde_json::to_vec(value).unwrap();
    let text = std::str::from_utf8(&bytes).unwrap();
    assert!(!text.contains("purpose"));
    assert!(!text.contains("codebase"));
    assert_eq!(
        bytes,
        serde_json::to_vec(&serde_json::from_slice::<ThreadVersionV1>(&bytes).unwrap()).unwrap()
    );
}
#[test]
fn annotation_owner_remains_internal_after_message_and_reload() {
    let mut graph = CollabGraphV1::default();
    let id = ThreadId::new([1; 16]);
    let actor = PrincipalId::new([2; 16]);
    let (root, created) = ThreadStateV1::empty()
        .apply(
            &ThreadCommandV1::CreateAnnotation {
                thread_id: id,
                title: "Recorded conversation comments".into(),
            },
            actor,
            RequestId::new([3; 16]),
            1,
            &mut graph,
        )
        .unwrap();
    let (root, edited) = ThreadStateV1::read(root, &graph)
        .unwrap()
        .apply(
            &ThreadCommandV1::Message {
                emojis: Vec::new(),
                thread_id: id,
                expected_thread: created.version,
                message_id: HumanMessageId::new([4; 16]),
                expected_revision: None,
                reply_to: None,
                body: Some("Retained note".into()),
            },
            actor,
            RequestId::new([5; 16]),
            2,
            &mut graph,
        )
        .unwrap();
    let state = ThreadStateV1::read(root, &graph).unwrap();
    let value = version(&graph, state.threads[&id]);
    assert!(!value.is_user_thread());
    assert_eq!(value.purpose, Some(ThreadPurposeV1::Annotation));
    assert_eq!(edited.version, state.threads[&id]);
    graph.closure(state.root.dependencies()).unwrap();
}

use super::*;
use jj_store_service::agent_execution::conversations as chat;
#[tokio::test]
async fn repository_free_chat_reuses_isolated_bindings_and_never_replays_after_restart() {
    let fixture = Fixture::new().await;
    let actor = fixture.actor;
    let resources = fixture.resources.clone();
    let supervisor = Supervisor::open(fixture.resources).unwrap();
    tokio::time::timeout(Duration::from_secs(240), async {
        while !supervisor.ready() {
            tokio::time::sleep(Duration::from_millis(100)).await;
        }
    })
    .await
    .unwrap();
    for (i, name) in ["helper-one", "helper-two"].into_iter().enumerate() {
        let revision = supervisor.status(actor).await.unwrap().revision;
        supervisor
            .save_setup(
                actor,
                SetupCommand {
                    version: 1,
                    request_id: RequestId::new([130 + i as u8; 16]),
                    expected_revision: revision,
                    setup: Some(Setup {
                        id: name.into(),
                        name: name.into(),
                        avatar: "🤖".into(),
                        description: String::new(),
                        instructions: "Bootstrap instructions".into(),
                        runner: "codex".into(),
                        preferences: Preferences {
                            model: None,
                            effort: None,
                            mode: Some("ask".into()),
                        },
                    }),
                    remove: None,
                },
            )
            .await
            .unwrap();
    }
    let send = chat::Send {
        version: 1,
        request_id: RequestId::new([140; 16]),
        conversation_id: "a1".repeat(16),
        origin: chat::Origin {
            scope: "personal".into(),
            home: None,
            thread: None,
            agent_target: None,
            context: "Initial context".into(),
        },
        agent_ids: vec!["helper-one".into(), "helper-two".into()],
        attachments: vec![],
        text: "First real message".into(),
        max_duration_ms: 5000,
    };
    assert!(!resources.root.join("personal-home").exists());
    let first = supervisor
        .send_conversation(actor, send.clone())
        .await
        .unwrap();
    assert_eq!(first.agents.len(), 2);
    assert!(first.agents.iter().all(|a| a.error.is_none()), "{first:?}");
    let status = wait(&supervisor, actor, |s| {
        s.sessions.len() == 2
            && s.sessions
                .iter()
                .all(|s| s.state == "idle" && s.last_sequence > 3)
    })
    .await;
    assert_eq!(status.conversations.len(), 1);
    let discussion = status.conversations[0]
        .discussion_thread
        .expect("canonical metadata thread");
    let local = resources
        .folders
        .as_ref()
        .unwrap()
        .open(status.conversations[0].home)
        .unwrap();
    let pin = local
        .inbox_selection()
        .await
        .unwrap()
        .snapshot
        .metadata_commit;
    let thread = local.inbox_thread(pin, discussion).await.unwrap();
    assert_eq!(thread.thread.value.title, "Agent conversation");
    for session in &status.sessions {
        let page = supervisor
            .events(actor, session.id.clone(), 0)
            .await
            .unwrap();
        assert!(page.events.iter().any(|e| {
            e.payload["type"] == "prompt"
                && e.payload["text"].as_str().is_some_and(|t| {
                    t.contains("Bootstrap instructions")
                        && t.contains("Initial context")
                        && t.contains("First real message")
                })
        }));
    }
    assert!(resources.root.join("personal-home/.jj").exists());
    assert_ne!(status.sessions[0].workspace, status.sessions[1].workspace);
    assert_eq!(status.sessions[0].repository, status.sessions[1].repository);
    let sequences = status
        .sessions
        .iter()
        .map(|s| s.last_sequence)
        .collect::<Vec<_>>();
    supervisor
        .send_conversation(actor, send.clone())
        .await
        .unwrap();
    assert_eq!(
        supervisor
            .status(actor)
            .await
            .unwrap()
            .sessions
            .iter()
            .map(|s| s.last_sequence)
            .collect::<Vec<_>>(),
        sequences
    );
    let target = supervisor
        .targets(actor, None)
        .await
        .unwrap()
        .into_iter()
        .find(|t| Path::new(&t.folder) == fixture.source.canonicalize().unwrap())
        .unwrap();
    let second_source = fixture._root.path().join("source-two");
    assert!(
        Process::new(env!("CARGO_BIN_EXE_jj"))
            .args(["git", "init"])
            .arg(&second_source)
            .status()
            .unwrap()
            .success()
    );
    fs::write(second_source.join("base.txt"), "second base\n").unwrap();
    jj(&second_source, &["status"]);
    let folders = resources.folders.as_ref().unwrap();
    folders
        .register(RegisterFolder {
            version: 1,
            request_id: RequestId::new([141; 16]),
            expected_revision: folders.list().unwrap().revision,
            path: second_source.to_string_lossy().into_owned(),
            name: "Second".into(),
        })
        .await
        .unwrap();
    let second = supervisor
        .targets(actor, None)
        .await
        .unwrap()
        .into_iter()
        .find(|t| Path::new(&t.folder) == second_source.canonicalize().unwrap())
        .unwrap();
    let mut attached = send.clone();
    attached.request_id = RequestId::new([142; 16]);
    attached.text = "Work in both repositories".into();
    attached.attachments = [target, second]
        .into_iter()
        .map(|t| chat::Attachment {
            repository: t.repository,
            target_id: Some(t.id),
            operation: Some(t.operation),
            commit: Some(t.commit),
        })
        .collect();
    let outcome = supervisor
        .send_conversation(actor, attached.clone())
        .await
        .unwrap();
    assert!(
        outcome.agents.iter().all(|a| a.error.is_none()),
        "{outcome:?}"
    );
    let status = wait(&supervisor, actor, |s| {
        s.sessions.iter().all(|s| {
            s.state == "idle"
                && s.repository_bindings.len() == 2
                && s.last_sequence > sequences[0] + 3
        })
    })
    .await;
    let directories = status
        .sessions
        .iter()
        .map(|s| {
            s.repository_bindings
                .iter()
                .map(|b| b.workspace.clone())
                .collect::<Vec<_>>()
        })
        .collect::<Vec<_>>();
    assert_ne!(directories[0], directories[1]);
    for path in directories.iter().flatten() {
        assert_eq!(
            fs::read_to_string(Path::new(path).join("agent-edit.txt")).unwrap(),
            "review this\n"
        );
    }
    assert_eq!(
        fs::read_to_string(fixture.source.join("source.txt")).unwrap(),
        "dirty\n"
    );
    assert!(!fixture.source.join("agent-edit.txt").exists());
    // A new repository chat may use an immutable code snapshot newer than
    // the source checkout. Its base must be the visible revision, without
    // refreshing or snapshotting the dirty source working files.
    let previous = &status.sessions[0].repository_bindings[0];
    let mut fresh = send.clone();
    fresh.request_id = RequestId::new([143; 16]);
    fresh.conversation_id = "a2".repeat(16);
    fresh.agent_ids = vec!["helper-one".into()];
    fresh.origin.scope = format!("repository:{}", previous.repository);
    fresh.attachments = vec![chat::Attachment {
        repository: previous.repository,
        target_id: None,
        operation: Some(previous.operation),
        commit: Some(previous.commit.clone()),
    }];
    let fresh_outcome = supervisor.send_conversation(actor, fresh).await.unwrap();
    assert!(fresh_outcome.agents[0].error.is_none(), "{fresh_outcome:?}");
    let fresh_id = fresh_outcome.agents[0].session_id.as_ref().unwrap();
    let fresh_status = wait(&supervisor, actor, |s| {
        s.sessions
            .iter()
            .any(|s| s.id == *fresh_id && s.state == "idle" && s.last_sequence > 3)
    })
    .await;
    let binding = &fresh_status
        .sessions
        .iter()
        .find(|s| s.id == *fresh_id)
        .unwrap()
        .repository_bindings[0];
    assert_eq!(binding.base_commit, previous.commit);
    assert_eq!(binding.selected_operation, previous.operation);
    assert_ne!(binding.workspace, previous.workspace);
    assert_eq!(
        fs::read_to_string(fixture.source.join("source.txt")).unwrap(),
        "dirty\n"
    );
    supervisor.shutdown().await.unwrap();
    let reopened = Supervisor::open(resources).unwrap();
    let duplicate = reopened.send_conversation(actor, attached).await.unwrap();
    assert!(duplicate.agents.iter().all(|a| a.state == "completed"));
    let status = reopened.status(actor).await.unwrap();
    assert!(status.sessions.iter().all(|s| !s.connected));
    assert_eq!(
        status
            .sessions
            .iter()
            .filter(|s| s.conversation_id.as_deref() == Some(send.conversation_id.as_str()))
            .map(|s| s
                .repository_bindings
                .iter()
                .map(|b| b.workspace.clone())
                .collect::<Vec<_>>())
            .collect::<Vec<_>>(),
        directories
    );
    reopened.shutdown().await.unwrap();
}

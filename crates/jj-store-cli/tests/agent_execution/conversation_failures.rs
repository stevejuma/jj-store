use super::*;
use jj_store_service::agent_execution::conversations as chat;

async fn agent(fixture: &Fixture) -> Supervisor {
    let supervisor = Supervisor::open(fixture.resources.clone()).unwrap();
    tokio::time::timeout(Duration::from_secs(240), async {
        while !supervisor.ready() {
            tokio::time::sleep(Duration::from_millis(100)).await;
        }
    })
    .await
    .unwrap();
    supervisor
        .save_setup(
            fixture.actor,
            SetupCommand {
                version: 1,
                request_id: RequestId::new([180; 16]),
                expected_revision: supervisor.status(fixture.actor).await.unwrap().revision,
                setup: Some(Setup {
                    options: Default::default(),
                    id: "failure-agent".into(),
                    name: "Failure agent".into(),
                    avatar: "🤖".into(),
                    description: String::new(),
                    instructions: String::new(),
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
    supervisor
}

async fn conversation(supervisor: &Supervisor, actor: PrincipalId, byte: u8) -> String {
    let sent = supervisor
        .send_conversation(
            actor,
            chat::Send {
                version: 1,
                request_id: RequestId::new([byte; 16]),
                conversation_id: format!("{byte:02x}").repeat(16),
                origin: chat::Origin {
                    scope: "personal".into(),
                    home: None,
                    thread: None,
                    agent_target: None,
                    context: String::new(),
                },
                agent_ids: vec!["failure-agent".into()],
                attachments: vec![],
                text: "One explicit prompt".into(),
                max_duration_ms: 5000,
                comments: None,
                thread_codebase: None,
            },
        )
        .await
        .unwrap();
    let id = sent.agents[0].session_id.clone().unwrap();
    wait(supervisor, actor, |s| {
        s.sessions
            .iter()
            .any(|s| s.id == id && s.state == "idle" && s.last_sequence > 3)
    })
    .await;
    id
}

#[tokio::test]
async fn rejected_resume_finishes_receipt_preserves_worker_and_never_replays() {
    rejected_reopen(false).await;
}

#[tokio::test]
async fn rejected_resume_configuration_settles_its_correlated_receipt() {
    rejected_reopen(true).await;
}

async fn rejected_reopen(configuration: bool) {
    let fixture = Fixture::new().await;
    let peer = fs::read_to_string(&fixture.resources.executable).unwrap();
    let peer = if configuration {
        peer.replace("elif kind == 'open':", "elif kind == 'open':\n        loaded_resume = bool(frame.get('loadSessionId'))")
            .replace("elif kind == 'configure':", "elif kind == 'configure':\n        if loaded_resume:\n            emit('error', request, code='agent_rejected')\n            continue")
    } else {
        peer.replace("elif kind == 'open':", "elif kind == 'open':\n        if frame.get('loadSessionId'):\n            emit('error', request, code='agent_rejected')\n            continue")
    };
    fs::write(&fixture.resources.executable, peer).unwrap();
    let supervisor = agent(&fixture).await;
    let id = conversation(&supervisor, fixture.actor, 181).await;
    let other = conversation(&supervisor, fixture.actor, 182).await;
    supervisor
        .command(
            fixture.actor,
            Command {
                version: 1,
                request_id: RequestId::new([183; 16]),
                session_id: id.clone(),
                action: Action::End,
            },
        )
        .await
        .unwrap();
    wait(&supervisor, fixture.actor, |s| {
        s.sessions.iter().any(|s| s.id == id && s.state == "ended")
    })
    .await;
    let request = RequestId::new([184; 16]);
    let outcome = supervisor
        .command(
            fixture.actor,
            Command {
                version: 1,
                request_id: request,
                session_id: id.clone(),
                action: Action::Resume,
            },
        )
        .await
        .expect("Rejected reopen must settle the retained request");
    assert_eq!(outcome.state, "failed");
    assert_eq!(outcome.error.as_deref(), Some("agent_rejected"));
    let status = wait(&supervisor, fixture.actor, |s| {
        s.live_agents[0].state == "idle"
            && s.sessions
                .iter()
                .any(|s| s.id == id && s.state == "interrupted")
            && s.sessions.iter().any(|s| s.id == other && s.connected)
    })
    .await;
    assert!(
        !status
            .sessions
            .iter()
            .find(|s| s.id == id)
            .unwrap()
            .connected
    );
    assert_eq!(
        status
            .sessions
            .iter()
            .find(|s| s.id == other)
            .unwrap()
            .state,
        "idle"
    );
    assert_eq!(
        supervisor
            .outcome(fixture.actor, request)
            .await
            .unwrap()
            .state,
        "failed"
    );
    for session in &status.sessions {
        let events = supervisor
            .events(fixture.actor, session.id.clone(), 0)
            .await
            .unwrap();
        assert_eq!(
            events
                .events
                .iter()
                .filter(|e| e.payload["type"] == "prompt")
                .count(),
            1
        );
    }
    supervisor.shutdown().await.unwrap();
    let reopened = Supervisor::open(fixture.resources.clone()).unwrap();
    assert_eq!(
        reopened
            .outcome(fixture.actor, request)
            .await
            .unwrap()
            .state,
        "failed"
    );
    for session in reopened.status(fixture.actor).await.unwrap().sessions {
        assert_eq!(
            reopened
                .events(fixture.actor, session.id, 0)
                .await
                .unwrap()
                .events
                .iter()
                .filter(|e| e.payload["type"] == "prompt")
                .count(),
            1
        );
    }
    reopened.shutdown().await.unwrap();
}

#[tokio::test]
async fn stop_during_resume_cancels_opening_without_resurrection_or_prompt_replay() {
    let fixture = Fixture::new().await;
    let peer = fs::read_to_string(&fixture.resources.executable).unwrap().replace(
        "elif kind == 'open':",
        "elif kind == 'open':\n        if frame.get('loadSessionId'):\n            import time; time.sleep(2)",
    );
    fs::write(&fixture.resources.executable, peer).unwrap();
    let supervisor = agent(&fixture).await;
    let id = conversation(&supervisor, fixture.actor, 185).await;
    supervisor
        .command(
            fixture.actor,
            Command {
                version: 1,
                request_id: RequestId::new([186; 16]),
                session_id: id.clone(),
                action: Action::End,
            },
        )
        .await
        .unwrap();
    wait(&supervisor, fixture.actor, |s| {
        s.sessions[0].state == "ended"
    })
    .await;
    let resume_request = RequestId::new([187; 16]);
    let opening = supervisor.clone();
    let session_id = id.clone();
    let actor = fixture.actor;
    let resumed = tokio::spawn(async move {
        opening
            .command(
                actor,
                Command {
                    version: 1,
                    request_id: resume_request,
                    session_id,
                    action: Action::Resume,
                },
            )
            .await
    });
    wait(&supervisor, actor, |s| s.sessions[0].state == "opening").await;
    let ended = supervisor
        .command(
            actor,
            Command {
                version: 1,
                request_id: RequestId::new([188; 16]),
                session_id: id.clone(),
                action: Action::End,
            },
        )
        .await
        .unwrap();
    assert!(matches!(ended.state.as_str(), "dispatching" | "ended"));
    let outcome = tokio::time::timeout(Duration::from_secs(10), resumed)
        .await
        .unwrap()
        .unwrap()
        .unwrap();
    assert_eq!(outcome.state, "cancelled");
    wait(&supervisor, actor, |s| {
        s.sessions[0].state == "ended" && s.live_agents[0].state == "idle"
    })
    .await;
    let events = supervisor.events(actor, id.clone(), 0).await.unwrap();
    assert_eq!(
        events
            .events
            .iter()
            .filter(|e| e.payload["type"] == "prompt")
            .count(),
        1
    );
    assert!(!supervisor.status(actor).await.unwrap().sessions[0].connected);
    supervisor.shutdown().await.unwrap();
}

#[tokio::test]
async fn resume_without_an_acknowledged_turn_creates_context_session_without_loading() {
    let fixture = Fixture::new().await;
    let peer = fs::read_to_string(&fixture.resources.executable).unwrap()
        .replace("        emit('started', request)", "        # No prompt-start acknowledgement")
        .replace("elif kind == 'open':", "elif kind == 'open':\n        if frame.get('loadSessionId'):\n            emit('error', request, code='agent_rejected')\n            continue");
    fs::write(&fixture.resources.executable, peer).unwrap();
    let supervisor = agent(&fixture).await;
    let id = conversation(&supervisor, fixture.actor, 189).await;
    supervisor
        .command(
            fixture.actor,
            Command {
                version: 1,
                request_id: RequestId::new([190; 16]),
                session_id: id.clone(),
                action: Action::End,
            },
        )
        .await
        .unwrap();
    wait(&supervisor, fixture.actor, |s| {
        s.sessions[0].state == "ended"
    })
    .await;
    let outcome = supervisor
        .command(
            fixture.actor,
            Command {
                version: 1,
                request_id: RequestId::new([191; 16]),
                session_id: id.clone(),
                action: Action::Resume,
            },
        )
        .await
        .unwrap();
    assert_eq!(outcome.state, "opening");
    let status = supervisor.status(fixture.actor).await.unwrap();
    assert!(status.sessions[0].connected);
    assert_eq!(status.live_agents[0].state, "idle");
    let events = supervisor
        .events(fixture.actor, id, 0)
        .await
        .unwrap()
        .events;
    assert_eq!(
        events
            .iter()
            .filter(|e| e.payload["type"] == "prompt")
            .count(),
        1
    );
    assert!(events.iter().any(|e| e.payload["type"] == "continuation"));
    assert!(!events.iter().any(|e| e.payload["code"] == "agent_rejected"));
    supervisor.shutdown().await.unwrap();
}

//! Explicit installed-runner gate, separate from controlled process fixtures.
use super::*;
async fn observe(
    supervisor: &Supervisor,
    actor: PrincipalId,
    predicate: impl Fn(&Status) -> bool,
) -> Status {
    let mut observed = String::new();
    let result = tokio::time::timeout(Duration::from_secs(180), async {
        loop {
            let status = supervisor.status(actor).await.unwrap();
            observed = status
                .sessions
                .iter()
                .map(|s| format!("{}:{:?}", s.state, s.last_error))
                .collect::<Vec<_>>()
                .join(",");
            if predicate(&status) {
                return status;
            }
            tokio::time::sleep(Duration::from_millis(100)).await;
        }
    })
    .await;
    result.unwrap_or_else(|_| panic!("installed runner target not reached: {observed}"))
}
#[tokio::test]
#[ignore = "requires explicit JJ_STORE_AGENT_TEST_ACP, installed authenticated Codex/Cursor and disposable real edits"]
async fn installed_codex_and_cursor_edit_isolated_workspaces_and_publish_history() {
    for runner in ["codex", "cursor"] {
        if std::env::var("JJ_STORE_AGENT_TEST_RUNNER").is_ok_and(|selected| selected != runner) {
            continue;
        }
        let fixture = Fixture::new().await;
        let actor = fixture.actor;
        let source = fixture.source.clone();
        let mut resources = fixture.resources;
        resources.executable = std::env::var_os("JJ_STORE_AGENT_TEST_ACP")
            .expect("explicit rebuilt ACP executable")
            .into();
        let supervisor = Supervisor::open(resources).unwrap();
        tokio::time::timeout(Duration::from_secs(240), async {
            while !supervisor.ready() {
                tokio::time::sleep(Duration::from_millis(100)).await;
            }
        })
        .await
        .unwrap();
        let publication = supervisor.start_publication();
        let target = supervisor.targets(actor, None).await.unwrap().remove(0);
        let outcome = supervisor
            .launch(
                actor,
                Launch {
                    version: 1,
                    request_id: RequestId::new([2; 16]),
                    target_id: target.id,
                    repository: target.repository,
                    operation: target.operation,
                    commit: target.commit,
                    runner: runner.into(),
                    setup_id: None,
                    context: None,
                },
            )
            .await
            .unwrap();
        let id = outcome.session_id.unwrap();
        let status = observe(&supervisor, actor, |s| {
            s.sessions[0].state == "idle" || s.sessions[0].state == "interrupted"
        })
        .await;
        assert_eq!(
            status.sessions[0].state,
            "idle",
            "{}",
            status.sessions[0].last_error.as_deref().unwrap_or("")
        );
        let modes = status.sessions[0].capabilities["session"]["modes"]["availableModes"]
            .as_array()
            .unwrap();
        let mode = if runner == "codex" {
            "read-only"
        } else {
            "agent"
        };
        assert!(modes.iter().any(|m| m["id"] == mode));
        supervisor
            .command(
                actor,
                Command {
                    version: 1,
                    request_id: RequestId::new([3; 16]),
                    session_id: id.clone(),
                    action: Action::Configure {
                        preferences: Preferences {
                            model: None,
                            effort: None,
                            mode: Some(mode.into()),
                        },
                    },
                },
            )
            .await
            .unwrap();
        observe(&supervisor, actor, |s| {
            s.sessions[0].capabilities["modeReady"] == true
        })
        .await;
        let request = RequestId::new([4; 16]);
        supervisor.command(actor,Command {version:1,request_id:request,session_id:id.clone(),action:Action::Prompt {text:"Create the file agent-acceptance.txt in this workspace with exactly the text ACP integration accepted followed by a newline. Use your file editing tool. Do not change any other file and do not commit. Reply briefly when complete.".into(),max_duration_ms:180000}}).await.unwrap();
        let mut after = 0;
        let mut permission_count = 0;
        let mut decision_count = 0;
        tokio::time::timeout(Duration::from_secs(210), async {
            loop {
                let page = supervisor.events(actor, id.clone(), after).await.unwrap();
                for event in page.events {
                    after = event.sequence;
                    if event.payload["type"] == "permission" {
                        permission_count += 1;
                        let options = event.payload["request"]["options"].as_array().unwrap();
                        let allow = options.iter().find(|o| o["kind"] == "allow_once").unwrap();
                        decision_count += 1;
                        supervisor
                            .command(
                                actor,
                                Command {
                                    version: 1,
                                    request_id: RequestId::new([10 + decision_count; 16]),
                                    session_id: id.clone(),
                                    action: Action::Permission {
                                        permission_id: event.payload["permissionId"]
                                            .as_str()
                                            .unwrap()
                                            .into(),
                                        option_id: Some(allow["optionId"].as_str().unwrap().into()),
                                    },
                                },
                            )
                            .await
                            .unwrap();
                    }
                }
                let outcome = supervisor.outcome(actor, request).await.unwrap();
                if !matches!(outcome.state.as_str(), "dispatching" | "queued" | "started") {
                    assert_eq!(outcome.state, "completed", "{runner}: {:?}", outcome.error);
                    break;
                }
                tokio::time::sleep(Duration::from_millis(100)).await;
            }
        })
        .await
        .unwrap();
        let status = observe(&supervisor, actor, |s| {
            s.sessions[0].state == "idle"
                && s.sessions[0].last_sequence == s.sessions[0].published_sequence
        })
        .await;
        assert_eq!(
            fs::read_to_string(
                Path::new(&status.sessions[0].workspace).join("agent-acceptance.txt")
            )
            .unwrap(),
            "ACP integration accepted\n"
        );
        assert_eq!(
            fs::read_to_string(source.join("source.txt")).unwrap(),
            "dirty\n"
        );
        assert!(!source.join("agent-acceptance.txt").exists());
        println!(
            "{runner}: isolated edit completed; history published; {permission_count} exact permission decisions"
        );
        supervisor.shutdown().await.unwrap();
        publication.abort();
        let _ = publication.await;
    }
}

#[tokio::test]
#[ignore = "requires explicit JJ_STORE_AGENT_TEST_ACP and authenticated installed Codex"]
async fn installed_fleet_warms_general_chat_then_reuses_an_attached_workspace() {
    use jj_store_service::agent_execution::{conversations as chat, fleet::Control};
    let fixture = Fixture::new().await;
    let actor = fixture.actor;
    let source = fixture.source.clone();
    let mut resources = fixture.resources;
    resources.executable = std::env::var_os("JJ_STORE_AGENT_TEST_ACP")
        .expect("explicit rebuilt ACP executable")
        .into();
    let supervisor = Supervisor::open(resources).unwrap();
    tokio::time::timeout(Duration::from_secs(240), async {
        while !supervisor.ready() {
            tokio::time::sleep(Duration::from_millis(100)).await;
        }
    })
    .await
    .unwrap();
    let publication = supervisor.start_publication();
    let revision = supervisor.status(actor).await.unwrap().revision;
    supervisor.save_setup(actor, SetupCommand { version: 1, request_id: RequestId::new([171;16]), expected_revision: revision, setup: Some(Setup { id: "real-fleet".into(), name: "Real fleet".into(), avatar: "🤖".into(), description: String::new(), runner: "codex".into(), instructions: "Follow the explicit user message. Keep all file edits within attached isolated repository directories.".into(), preferences: Preferences { model: None, effort: None, mode: Some("read-only".into()) } }), remove: None }).await.unwrap();
    supervisor
        .control_agent(
            actor,
            Control {
                version: 1,
                request_id: RequestId::new([172; 16]),
                agent_id: "real-fleet".into(),
                start: true,
            },
        )
        .await
        .unwrap();
    let warm = observe(&supervisor, actor, |s| {
        s.live_agents
            .iter()
            .any(|a| a.agent_id == "real-fleet" && a.state == "idle")
    })
    .await;
    assert!(warm.sessions.is_empty());
    let mut send = chat::Send {
        version: 1,
        request_id: RequestId::new([173; 16]),
        conversation_id: "ac".repeat(16),
        origin: chat::Origin {
            scope: "personal".into(),
            home: None,
            thread: None,
            agent_target: None,
            context: "General conversation without an attached repository".into(),
        },
        agent_ids: vec!["real-fleet".into()],
        attachments: vec![],
        text: "Reply with exactly Ready for repositories. Do not access or modify files.".into(),
        max_duration_ms: 180000,
    };
    let first = supervisor
        .send_conversation(actor, send.clone())
        .await
        .unwrap();
    assert!(first.agents.iter().all(|a| a.error.is_none()), "{first:?}");
    let initial = observe(&supervisor, actor, |s| {
        s.sessions.len() == 1 && s.sessions[0].state == "idle" && s.sessions[0].last_sequence > 3
    })
    .await;
    let id = initial.sessions[0].id.clone();
    let target = supervisor
        .targets(actor, None)
        .await
        .unwrap()
        .into_iter()
        .find(|t| Path::new(&t.folder) == source.canonicalize().unwrap())
        .unwrap();
    send.request_id = RequestId::new([174; 16]);
    send.attachments = vec![chat::Attachment {
        repository: target.repository,
        target_id: Some(target.id),
        operation: Some(target.operation),
        commit: Some(target.commit),
    }];
    send.text = format!(
        "Create repositories/{}/fleet-acceptance.txt with exactly Fleet accepted followed by a newline. Use your file editing tool. Do not edit any other file or commit. Reply briefly when complete.",
        target.repository
    );
    let attached = supervisor
        .send_conversation(actor, send.clone())
        .await
        .unwrap();
    assert_eq!(attached.agents[0].session_id.as_deref(), Some(id.as_str()));
    let child = attached.agents[0].request_id;
    let mut after = 0;
    let mut decisions = 0;
    tokio::time::timeout(Duration::from_secs(210), async {
        loop {
            for event in supervisor
                .events(actor, id.clone(), after)
                .await
                .unwrap()
                .events
            {
                after = event.sequence;
                if event.payload["type"] == "permission" {
                    let allow = event.payload["request"]["options"]
                        .as_array()
                        .unwrap()
                        .iter()
                        .find(|o| o["kind"] == "allow_once")
                        .unwrap();
                    decisions += 1;
                    supervisor
                        .command(
                            actor,
                            Command {
                                version: 1,
                                request_id: RequestId::new([180 + decisions; 16]),
                                session_id: id.clone(),
                                action: Action::Permission {
                                    permission_id: event.payload["permissionId"]
                                        .as_str()
                                        .unwrap()
                                        .into(),
                                    option_id: Some(allow["optionId"].as_str().unwrap().into()),
                                },
                            },
                        )
                        .await
                        .unwrap();
                }
            }
            let outcome = supervisor.outcome(actor, child).await.unwrap();
            if !matches!(outcome.state.as_str(), "dispatching" | "queued" | "started") {
                assert_eq!(outcome.state, "completed", "{outcome:?}");
                break;
            }
            tokio::time::sleep(Duration::from_millis(100)).await;
        }
    })
    .await
    .unwrap();
    let finished = observe(&supervisor, actor, |s| {
        s.sessions[0].state == "idle"
            && s.sessions[0].last_sequence == s.sessions[0].published_sequence
    })
    .await;
    let binding = &finished.sessions[0].repository_bindings[0];
    assert_eq!(
        fs::read_to_string(Path::new(&binding.workspace).join("fleet-acceptance.txt")).unwrap(),
        "Fleet accepted\n"
    );
    assert!(!source.join("fleet-acceptance.txt").exists());
    assert_eq!(
        fs::read_to_string(source.join("source.txt")).unwrap(),
        "dirty\n"
    );
    let before = finished.sessions[0].last_sequence;
    supervisor.send_conversation(actor, send).await.unwrap();
    assert_eq!(
        supervisor.status(actor).await.unwrap().sessions[0].last_sequence,
        before
    );
    println!(
        "real Codex warm/general/attached reuse completed; dirty source preserved; {decisions} permission decisions; canonical history published; exact duplicate never replays"
    );
    supervisor.shutdown().await.unwrap();
    publication.abort();
    let _ = publication.await;
}

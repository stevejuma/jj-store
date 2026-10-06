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

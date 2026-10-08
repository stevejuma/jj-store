use super::*;
use jj_store_service::agent_execution::conversations as chat;
use serde_json::{Value, json};

fn cli(binary: &Path, root: &Path, args: &[&str]) -> Value {
    let output = Process::new(binary)
        .arg("--no-pager")
        .arg("-R")
        .arg(root)
        .args(args)
        .output()
        .unwrap();
    assert!(
        output.status.success(),
        "{}",
        String::from_utf8_lossy(&output.stderr)
    );
    serde_json::from_slice(&output.stdout).unwrap()
}
async fn send(supervisor: &Supervisor, actor: PrincipalId, command: &chat::Send) -> Vec<Value> {
    let result = supervisor
        .send_conversation(actor, command.clone())
        .await
        .unwrap();
    for outcome in &result.agents {
        let request = outcome.request_id;
        tokio::time::timeout(Duration::from_secs(30), async {
            while supervisor.outcome(actor, request).await.unwrap().state != "completed" {
                tokio::time::sleep(Duration::from_millis(20)).await;
            }
        })
        .await
        .unwrap();
    }
    let mut frames = Vec::new();
    for outcome in result.agents {
        let events = supervisor
            .events(actor, outcome.session_id.unwrap(), 0)
            .await
            .unwrap();
        frames.push(
            events
                .events
                .into_iter()
                .find(|e| {
                    e.payload["type"] == "prompt"
                        && e.payload["requestId"] == outcome.request_id.to_string()
                })
                .unwrap()
                .payload,
        );
    }
    frames
}
#[tokio::test]
async fn markdown_deltas_and_bound_task_cli_survive_loaded_and_fresh_sessions() {
    let mut fixture = Fixture::new().await;
    let bin_dir = fixture._root.path().join("bundled tools");
    fs::create_dir(&bin_dir).unwrap();
    let binary = bin_dir.join("jj");
    fs::hard_link(env!("CARGO_BIN_EXE_jj"), &binary).unwrap();
    fixture.resources.jj_executable = binary.clone();
    fixture.resources.driver = Arc::new(Driver(jj_store_jj::agent_workspaces::Provider {
        executable: binary.clone(),
    }));
    // Fixture proves the controller exports the path and it is executable
    // through the exact quoted shell form advertised in context.
    let script = fs::read_to_string(&fixture.resources.executable).unwrap().replace("import json", "import json\nimport subprocess").replace("        emit('queued', request)", &format!("        assert os.environ['JJ_STORE_AGENT_JJ_EXECUTABLE'] == {}\n        assert subprocess.run('\"$JJ_STORE_AGENT_JJ_EXECUTABLE\" --help', shell=True, stdout=subprocess.DEVNULL).returncode == 0\n        emit('queued', request)", serde_json::to_string(&binary.canonicalize().unwrap()).unwrap()));
    fs::write(&fixture.resources.executable, script).unwrap();
    let actor = fixture.actor;
    let resources = fixture.resources.clone();
    let supervisor = Supervisor::open(resources.clone()).unwrap();
    tokio::time::timeout(Duration::from_secs(240), async {
        while !supervisor.ready() {
            tokio::time::sleep(Duration::from_millis(100)).await;
        }
    })
    .await
    .unwrap();
    for (i, id) in ["context-one", "context-two"].iter().enumerate() {
        let options = settings::RuntimeOptions::default();
        supervisor
            .save_setup(
                actor,
                SetupCommand {
                    version: 1,
                    request_id: RequestId::new([150 + i as u8; 16]),
                    expected_revision: supervisor.status(actor).await.unwrap().revision,
                    setup: Some(Setup {
                        id: (*id).into(),
                        name: (*id).into(),
                        avatar: "🤖".into(),
                        description: String::new(),
                        instructions: "## Standing instructions\n\nKeep **Markdown**".into(),
                        runner: "codex".into(),
                        preferences: Preferences {
                            model: None,
                            effort: None,
                            mode: Some("ask".into()),
                        },
                        options,
                    }),
                    remove: None,
                },
            )
            .await
            .unwrap();
    }
    let mut command = chat::Send {
        version: 1,
        request_id: RequestId::new([153; 16]),
        conversation_id: "c1".repeat(16),
        origin: chat::Origin {
            scope: "personal".into(),
            home: None,
            thread: None,
            agent_target: None,
            context: json!({"task_content":{"title":"Current task","body":"**Original task**"}})
                .to_string(),
        },
        agent_ids: vec!["context-one".into(), "context-two".into()],
        attachments: vec![],
        text: "First authored turn".into(),
        max_duration_ms: 5000,
    };
    for frame in send(&supervisor, actor, &command).await {
        let wire = frame["context"].as_str().unwrap();
        assert!(wire.contains("## Standing instructions"));
        assert!(wire.contains("**Original task**"));
        assert!(wire.contains("JJ_STORE_AGENT_JJ_EXECUTABLE"));
        assert!(!wire.contains("```json"));
        assert!(!wire.contains(fixture._root.path().to_str().unwrap()));
        assert_eq!(frame["text"], command.text);
        assert_eq!(frame["sentContext"]["wireContext"], frame["context"]);
    }
    command.request_id = RequestId::new([154; 16]);
    command.text = "Second authored turn".into();
    assert!(
        send(&supervisor, actor, &command)
            .await
            .iter()
            .all(|f| f["context"] == "")
    );
    command.request_id = RequestId::new([155; 16]);
    command.origin.context =
        json!({"task_content":{"title":"Current task","body":"**Changed task**"}}).to_string();
    for frame in send(&supervisor, actor, &command).await {
        let wire = frame["context"].as_str().unwrap();
        assert!(wire.contains("**Changed task**"));
        assert!(!wire.contains("Standing instructions"));
        assert!(!wire.contains("JJ_STORE_AGENT_JJ_EXECUTABLE"));
    }
    let status = supervisor.status(actor).await.unwrap();
    let home = Path::new(&status.sessions[0].workspace).join("coordination");
    let projects = cli(&binary, &home, &["project", "list", "--format", "json"]);
    let project = projects["projects"][0]["id"].as_str().unwrap();
    let version = projects["projects"][0]["version"].to_string();
    let request = "ab".repeat(16);
    let task = "cd".repeat(16);
    let args = [
        "task",
        "create",
        "--request-id",
        &request,
        "--id",
        &task,
        "--project",
        project,
        "--expected-project-version",
        &version,
        "--title",
        "Scoped fixture task",
        "--body",
        "**Markdown body**",
        "--format",
        "json",
    ];
    let created = cli(&binary, &home, &args);
    assert_eq!(cli(&binary, &home, &args), created);
    assert_eq!(
        cli(
            &binary,
            &home,
            &["task", "status", &request, "--format", "json"]
        ),
        created
    );
    let shown = cli(&binary, &home, &["task", "show", &task, "--format", "json"]);
    assert_eq!(shown["tasks"][0]["body"], "**Markdown body**");
    let comment = "ef".repeat(16);
    cli(
        &binary,
        &home,
        &[
            "task",
            "comment",
            &task,
            "--request-id",
            &comment,
            "--expected-task-version",
            &shown["tasks"][0]["version"].to_string(),
            "--body",
            "Confirmed from agent CLI",
            "--format",
            "json",
        ],
    );
    assert_eq!(
        cli(&binary, &home, &["task", "show", &task, "--format", "json"])["comments"][0]["body"],
        "Confirmed from agent CLI"
    );
    let target = supervisor
        .targets(actor, None)
        .await
        .unwrap()
        .into_iter()
        .find(|t| Path::new(&t.folder) == fixture.source.canonicalize().unwrap())
        .unwrap();
    command.attachments = vec![chat::Attachment {
        repository: target.repository,
        target_id: Some(target.id),
        operation: Some(target.operation),
        commit: Some(target.commit),
    }];
    command.request_id = RequestId::new([157; 16]);
    for frame in send(&supervisor, actor, &command).await {
        let wire = frame["context"].as_str().unwrap();
        assert!(wire.contains("Code") || wire.contains("Repository:"));
        assert!(!wire.contains("Standing instructions"));
    }
    command.request_id = RequestId::new([158; 16]);
    assert!(
        send(&supervisor, actor, &command)
            .await
            .iter()
            .all(|f| f["context"] == "")
    );
    supervisor.shutdown().await.unwrap();
    // Simulate the prior private journal schema without changing delivery flags.
    let journal_path = resources.root.join("state.json");
    let mut legacy: Value = serde_json::from_slice(&fs::read(&journal_path).unwrap()).unwrap();
    for session in legacy["sessions"].as_object_mut().unwrap().values_mut() {
        session
            .as_object_mut()
            .unwrap()
            .remove("delivered_sections");
        for event in session["events"].as_array_mut().unwrap() {
            if let Some(snapshot) = event["payload"]["sentContext"].as_object_mut() {
                snapshot.remove("sectionRevisions");
            }
        }
    }
    for intent in legacy["intents"].as_object_mut().unwrap().values_mut() {
        if let Some(prompt) = intent["prompt"].as_object_mut() {
            prompt.remove("sections");
        }
    }
    fs::write(&journal_path, serde_json::to_vec(&legacy).unwrap()).unwrap();
    let reopened = Supervisor::open(resources).unwrap();
    tokio::time::timeout(Duration::from_secs(240), async {
        while !reopened.ready() {
            tokio::time::sleep(Duration::from_millis(100)).await;
        }
    })
    .await
    .unwrap();
    command.request_id = RequestId::new([156; 16]);
    assert!(
        send(&reopened, actor, &command)
            .await
            .iter()
            .all(|f| f["context"] == "")
    );
    assert_eq!(
        cli(
            &binary,
            &home,
            &["task", "status", &request, "--format", "json"]
        ),
        created
    );
    for (index, session) in reopened
        .status(actor)
        .await
        .unwrap()
        .sessions
        .iter()
        .enumerate()
    {
        reopened
            .command(
                actor,
                Command {
                    version: 1,
                    request_id: RequestId::new([170 + index as u8; 16]),
                    session_id: session.id.clone(),
                    action: Action::End,
                },
            )
            .await
            .unwrap();
    }
    wait(&reopened, actor, |s| {
        s.sessions.iter().all(|s| s.state == "ended")
    })
    .await;
    for (index, session) in reopened
        .status(actor)
        .await
        .unwrap()
        .sessions
        .iter()
        .enumerate()
    {
        reopened
            .command(
                actor,
                Command {
                    version: 1,
                    request_id: RequestId::new([180 + index as u8; 16]),
                    session_id: session.id.clone(),
                    action: Action::ContinueWithContext,
                },
            )
            .await
            .unwrap();
    }
    command.request_id = RequestId::new([159; 16]);
    for frame in send(&reopened, actor, &command).await {
        let wire = frame["context"].as_str().unwrap();
        assert!(wire.contains("Standing instructions") && wire.contains("First authored turn"));
        assert_eq!(wire.matches("<agent_context>").count(), 1);
        assert!(!wire.contains("sectionRevisions") && !wire.contains("requestId"));
    }
    reopened.shutdown().await.unwrap();
}

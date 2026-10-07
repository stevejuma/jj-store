use super::*;
#[tokio::test]
async fn warming_agent_creates_no_session_or_workspace_and_stop_survives_restart() {
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
    let revision = supervisor.status(actor).await.unwrap().revision;
    supervisor
        .save_setup(
            actor,
            SetupCommand {
                version: 1,
                request_id: RequestId::new([121; 16]),
                expected_revision: revision,
                setup: Some(Setup {
                    id: "warm-agent".into(),
                    name: "Warm agent".into(),
                    avatar: "🤖".into(),
                    description: String::new(),
                    instructions: "Do useful work".into(),
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
    let start = jj_store_service::agent_execution::fleet::Control {
        version: 1,
        request_id: RequestId::new([122; 16]),
        agent_id: "warm-agent".into(),
        start: true,
    };
    let started = supervisor
        .control_agent(actor, start.clone())
        .await
        .unwrap();
    assert_eq!(started.state, "starting");
    let status = wait(&supervisor, actor, |s| {
        s.live_agents.iter().any(|a| a.state == "idle")
    })
    .await;
    assert!(status.sessions.is_empty());
    assert_eq!(
        fs::read_dir(resources.root.join("workspaces"))
            .unwrap()
            .count(),
        0
    );
    assert_eq!(
        fs::read_to_string(fixture.source.join("source.txt")).unwrap(),
        "dirty\n"
    );
    let generation = status.live_agents[0].generation;
    assert_eq!(
        supervisor.control_agent(actor, start).await.unwrap().state,
        "starting"
    );
    assert_eq!(
        supervisor.status(actor).await.unwrap().live_agents[0].generation,
        generation
    );
    supervisor
        .control_agent(
            actor,
            jj_store_service::agent_execution::fleet::Control {
                version: 1,
                request_id: RequestId::new([123; 16]),
                agent_id: "warm-agent".into(),
                start: false,
            },
        )
        .await
        .unwrap();
    // Start immediately after Stop must await the previous process and warm a
    // new generation, rather than retaining a worker that is already closing.
    supervisor
        .control_agent(
            actor,
            jj_store_service::agent_execution::fleet::Control {
                version: 1,
                request_id: RequestId::new([124; 16]),
                agent_id: "warm-agent".into(),
                start: true,
            },
        )
        .await
        .unwrap();
    let rewarmed = wait(&supervisor, actor, |s| {
        s.live_agents
            .iter()
            .any(|a| a.state == "idle" && a.generation > generation)
    })
    .await;
    assert!(rewarmed.sessions.is_empty());
    supervisor
        .control_agent(
            actor,
            jj_store_service::agent_execution::fleet::Control {
                version: 1,
                request_id: RequestId::new([125; 16]),
                agent_id: "warm-agent".into(),
                start: false,
            },
        )
        .await
        .unwrap();
    let status = wait(&supervisor, actor, |s| {
        s.live_agents.iter().all(|a| a.state == "offline")
    })
    .await;
    assert!(!status.live_agents[0].enabled);
    supervisor.shutdown().await.unwrap();
    let reopened = Supervisor::open(resources).unwrap();
    let status = reopened.status(actor).await.unwrap();
    assert!(status.sessions.is_empty());
    assert!(!status.live_agents[0].enabled);
    assert_eq!(status.live_agents[0].state, "offline");
    reopened.shutdown().await.unwrap();
}

#[tokio::test]
async fn model_discovery_before_any_agent_or_chat_is_cached_and_never_prompts() {
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
    let before = supervisor.status(actor).await.unwrap();
    let catalog = supervisor.catalog(actor, "codex".into()).await.unwrap();
    assert_eq!(
        catalog.facts["models"]["availableModels"][0]["modelId"],
        "fixture-model"
    );
    assert!(catalog.facts.get("adapterSessionId").is_none());
    let frames_path = resources.root.join("catalog/frames.jsonl");
    let frames = fs::read_to_string(&frames_path).unwrap();
    let values = frames
        .lines()
        .map(|line| serde_json::from_str::<serde_json::Value>(line).unwrap())
        .collect::<Vec<_>>();
    assert_eq!(
        values
            .iter()
            .map(|v| v["type"].as_str().unwrap())
            .collect::<Vec<_>>(),
        vec!["open", "close"]
    );
    supervisor.catalog(actor, "codex".into()).await.unwrap();
    assert_eq!(fs::read_to_string(&frames_path).unwrap(), frames);
    let after = supervisor.status(actor).await.unwrap();
    assert_eq!(after.revision, before.revision);
    assert!(after.setups.is_empty() && after.sessions.is_empty() && after.conversations.is_empty());
    assert_eq!(
        fs::read_dir(resources.root.join("workspaces"))
            .unwrap()
            .count(),
        0
    );
    assert_eq!(
        fs::read_to_string(fixture.source.join("source.txt")).unwrap(),
        "dirty\n"
    );
    // Unexpected permission traffic must fail and never populate the cache.
    let peer = fs::read_to_string(&resources.executable).unwrap().replace("elif kind == 'open':", "elif kind == 'open':\n        emit('permission', request, permissionId='unexpected')\n        continue");
    fs::write(&resources.executable, peer).unwrap();
    assert!(supervisor.catalog(actor, "cursor".into()).await.is_err());
    assert!(matches!(
        supervisor
            .catalog(PrincipalId::new([0; 16]), "codex".into())
            .await,
        Err(Error::Denied)
    ));
    assert!(matches!(
        supervisor.catalog(actor, "unregistered".into()).await,
        Err(Error::Invalid)
    ));
    assert!(supervisor.status(actor).await.unwrap().sessions.is_empty());
    // A blocked discovery child must be reaped before foreground shutdown completes.
    let hanging = fs::read_to_string(&resources.executable).unwrap().replace(
        "emit('permission', request, permissionId='unexpected')",
        "import time; time.sleep(60)",
    );
    fs::write(&resources.executable, hanging).unwrap();
    let before_frames = fs::read_to_string(&frames_path).unwrap().lines().count();
    let reader = supervisor.clone();
    let probe = tokio::spawn(async move { reader.catalog(actor, "cursor".into()).await });
    tokio::time::timeout(Duration::from_secs(5), async {
        while fs::read_to_string(&frames_path).unwrap().lines().count() <= before_frames {
            tokio::time::sleep(Duration::from_millis(10)).await;
        }
    })
    .await
    .unwrap();
    supervisor.shutdown().await.unwrap();
    assert!(
        tokio::time::timeout(Duration::from_secs(1), probe)
            .await
            .unwrap()
            .unwrap()
            .is_err()
    );
}

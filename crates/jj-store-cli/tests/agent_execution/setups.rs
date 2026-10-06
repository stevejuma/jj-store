use super::*;

#[tokio::test]
async fn saved_setup_revision_is_independent_of_session_activity() {
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
    let observed = supervisor.status(actor).await.unwrap().revision;
    let target = supervisor.targets(actor, None).await.unwrap().remove(0);
    let launch = supervisor
        .launch(
            actor,
            Launch {
                version: 1,
                request_id: RequestId::new([101; 16]),
                target_id: target.id,
                repository: target.repository,
                operation: target.operation,
                commit: target.commit,
                runner: "codex".into(),
                setup_id: None,
                context: None,
            },
        )
        .await
        .unwrap();
    wait(&supervisor, actor, |s| s.sessions[0].state == "idle").await;
    let opened_at = supervisor.status(actor).await.unwrap().sessions[0]
        .last_activity_at_millis
        .expect("retained handshake activity has an observation time");
    assert!(opened_at > 0);
    assert_eq!(supervisor.status(actor).await.unwrap().revision, observed);
    let command = SetupCommand {
        version: 1,
        request_id: RequestId::new([102; 16]),
        expected_revision: observed,
        setup: Some(Setup {
            id: "helper".into(),
            name: "Review helper".into(),
            avatar: "🤖".into(),
            description: String::new(),
            instructions: "Preserve source files.".into(),
            runner: "codex".into(),
            preferences: Preferences {
                model: None,
                effort: None,
                mode: None,
            },
        }),
        remove: None,
    };
    let saved = supervisor.save_setup(actor, command.clone()).await.unwrap();
    assert_eq!(saved.sessions[0].last_activity_at_millis, Some(opened_at));
    assert_eq!(saved.revision, observed + 1);
    assert_eq!(
        supervisor
            .save_setup(actor, command.clone())
            .await
            .unwrap()
            .revision,
        saved.revision
    );
    let stale = SetupCommand {
        request_id: RequestId::new([103; 16]),
        ..command
    };
    assert!(matches!(
        supervisor.save_setup(actor, stale).await,
        Err(Error::Stale)
    ));
    supervisor
        .command(
            actor,
            Command {
                version: 1,
                request_id: RequestId::new([104; 16]),
                session_id: launch.session_id.unwrap(),
                action: Action::End,
            },
        )
        .await
        .unwrap();
    wait(&supervisor, actor, |s| s.sessions[0].state == "ended").await;
    let ended_at = supervisor.status(actor).await.unwrap().sessions[0]
        .last_activity_at_millis
        .unwrap();
    assert!(ended_at >= opened_at);
    supervisor.shutdown().await.unwrap();
    let reopened = Supervisor::open(resources.clone()).unwrap();
    assert_eq!(
        reopened.status(actor).await.unwrap().sessions[0].last_activity_at_millis,
        Some(ended_at)
    );
    assert_eq!(
        reopened.status(actor).await.unwrap().revision,
        saved.revision
    );
    assert_eq!(
        reopened.status(actor).await.unwrap().setups[0].name,
        "Review helper"
    );
    reopened.shutdown().await.unwrap();
    // A pre-split private journal preserves its original conditional revision.
    let journal = resources.root.join("state.json");
    let mut legacy: serde_json::Value =
        serde_json::from_slice(&fs::read(&journal).unwrap()).unwrap();
    legacy.as_object_mut().unwrap().remove("setup_revision");
    let retained_revision = legacy["revision"].as_u64().unwrap();
    fs::write(journal, serde_json::to_vec(&legacy).unwrap()).unwrap();
    let migrated = Supervisor::open(resources).unwrap();
    assert_eq!(
        migrated.status(actor).await.unwrap().revision,
        retained_revision
    );
    migrated
        .save_setup(
            actor,
            SetupCommand {
                version: 1,
                request_id: RequestId::new([105; 16]),
                expected_revision: retained_revision,
                setup: None,
                remove: Some("helper".into()),
            },
        )
        .await
        .unwrap();
    assert_eq!(
        migrated.status(actor).await.unwrap().revision,
        retained_revision + 1
    );
    migrated.shutdown().await.unwrap();
}

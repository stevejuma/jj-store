use super::*;
use jj_store_service::agent_execution::thread_workspace as api;

#[tokio::test]
async fn explicit_threads_publish_without_agents_and_recover_their_codebase_after_restart() {
    let fixture = Fixture::new().await;
    let resources = fixture.resources.clone();
    let supervisor = Supervisor::open(resources.clone()).unwrap();
    let repository = resources.folders.as_ref().unwrap().list().unwrap().bindings[0].repository;
    let request = api::Request {
        version: 1,
        request_id: RequestId::new([210; 16]),
        command: api::Command::Create {
            title: "Intentional task".into(),
            repositories: vec![repository],
        },
    };
    let created = supervisor
        .thread_workspace(fixture.actor, request.clone())
        .await
        .unwrap();
    let repeated = supervisor
        .thread_workspace(fixture.actor, request.clone())
        .await
        .unwrap();
    assert_eq!(created.revision, repeated.revision);
    assert_eq!(created.home, repeated.home);
    let status = supervisor.status(fixture.actor).await.unwrap();
    assert!(status.sessions.is_empty());
    assert!(status.conversations.is_empty());
    let home = resources
        .folders
        .as_ref()
        .unwrap()
        .open(created.home)
        .unwrap();
    let pin = home
        .inbox_selection()
        .await
        .unwrap()
        .snapshot
        .metadata_commit;
    let detail = home.inbox_thread(pin, created.thread).await.unwrap();
    assert!(detail.thread.value.is_user_thread());
    assert_eq!(detail.thread.value.codebase, Some(vec![repository]));
    let mut conflicting = request;
    if let api::Command::Create { title, .. } = &mut conflicting.command {
        *title = "Different intent".into();
    }
    assert!(matches!(
        supervisor
            .thread_workspace(fixture.actor, conflicting)
            .await,
        Err(Error::Conflict)
    ));
    let update = api::Request {
        version: 1,
        request_id: RequestId::new([211; 16]),
        command: api::Command::Codebase {
            home: created.home,
            thread: created.thread,
            expected: created.revision,
            repositories: vec![],
        },
    };
    let changed = supervisor
        .thread_workspace(fixture.actor, update.clone())
        .await
        .unwrap();
    assert_ne!(created.revision, changed.revision);
    drop(supervisor);
    let restarted = Supervisor::open(resources).unwrap();
    let recovered = restarted
        .thread_workspace(fixture.actor, update)
        .await
        .unwrap();
    assert_eq!(recovered.revision, changed.revision);
    assert!(
        restarted
            .status(fixture.actor)
            .await
            .unwrap()
            .sessions
            .is_empty()
    );
    let foreign = api::Request {
        version: 1,
        request_id: RequestId::new([212; 16]),
        command: api::Command::Create {
            title: "Foreign".into(),
            repositories: vec![jj_store_model::RepositoryId::new([250; 16])],
        },
    };
    assert!(
        restarted
            .thread_workspace(fixture.actor, foreign)
            .await
            .is_err()
    );
}

#[tokio::test]
async fn intentional_thread_sends_refuse_foreign_scope_and_continue_in_fresh_workspaces_after_removal()
 {
    use jj_store_service::agent_execution::conversations as chat;
    let fixture = Fixture::new().await;
    let resources = fixture.resources.clone();
    let supervisor = Supervisor::open(resources.clone()).unwrap();
    tokio::time::timeout(Duration::from_secs(240), async {
        while !supervisor.ready() {
            tokio::time::sleep(Duration::from_millis(100)).await;
        }
    })
    .await
    .unwrap();
    let target = supervisor
        .targets(fixture.actor, None)
        .await
        .unwrap()
        .remove(0);
    let revision = supervisor.status(fixture.actor).await.unwrap().revision;
    supervisor
        .save_setup(
            fixture.actor,
            SetupCommand {
                version: 1,
                request_id: RequestId::new([213; 16]),
                expected_revision: revision,
                setup: Some(Setup {
                    id: "thread-helper".into(),
                    name: "Thread helper".into(),
                    avatar: "".into(),
                    description: String::new(),
                    instructions: String::new(),
                    runner: "codex".into(),
                    options: Default::default(),
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
    let created = supervisor
        .thread_workspace(
            fixture.actor,
            api::Request {
                version: 1,
                request_id: RequestId::new([214; 16]),
                command: api::Command::Create {
                    title: "Scoped task".into(),
                    repositories: vec![],
                },
            },
        )
        .await
        .unwrap();
    let mut send = chat::Send {
        version: 1,
        request_id: RequestId::new([215; 16]),
        conversation_id: "d1".repeat(16),
        origin: chat::Origin {
            scope: format!("thread:{}:{}", created.home, created.thread),
            home: Some(created.home),
            thread: Some(created.thread),
            agent_target: None,
            context: "Thread context".into(),
        },
        thread_codebase: Some(created.revision),
        agent_ids: vec!["thread-helper".into()],
        attachments: vec![chat::Attachment {
            repository: target.repository,
            target_id: Some(target.id),
            operation: Some(target.operation),
            commit: Some(target.commit),
        }],
        text: "Read attached code".into(),
        comments: None,
        max_duration_ms: 5000,
    };
    assert!(matches!(
        supervisor
            .send_conversation(fixture.actor, send.clone())
            .await,
        Err(Error::Denied)
    ));
    assert!(
        supervisor
            .status(fixture.actor)
            .await
            .unwrap()
            .sessions
            .is_empty()
    );
    let membership = supervisor
        .thread_workspace(
            fixture.actor,
            api::Request {
                version: 1,
                request_id: RequestId::new([216; 16]),
                command: api::Command::Codebase {
                    home: created.home,
                    thread: created.thread,
                    expected: created.revision,
                    repositories: vec![target.repository],
                },
            },
        )
        .await
        .unwrap();
    assert!(matches!(
        supervisor
            .send_conversation(fixture.actor, send.clone())
            .await,
        Err(Error::Stale)
    ));
    send.thread_codebase = Some(membership.revision);
    let first = supervisor
        .send_conversation(fixture.actor, send.clone())
        .await
        .unwrap();
    assert!(first.agents[0].error.is_none(), "{first:?}");
    let first_id = first.agents[0].session_id.clone().unwrap();
    let mut removal = api::Request {
        version: 1,
        request_id: RequestId::new([217; 16]),
        command: api::Command::Codebase {
            home: created.home,
            thread: created.thread,
            expected: membership.revision,
            repositories: vec![],
        },
    };
    assert!(matches!(
        supervisor
            .thread_workspace(fixture.actor, removal.clone())
            .await,
        Err(Error::Busy)
    ));
    wait(&supervisor, fixture.actor, |s| {
        s.sessions
            .iter()
            .any(|s| s.id == first_id && s.state == "idle" && s.last_sequence > 3)
    })
    .await;
    tokio::time::timeout(Duration::from_secs(30), async {
        loop {
            if supervisor
                .outcome(fixture.actor, first.agents[0].request_id)
                .await
                .unwrap()
                .state
                == "completed"
            {
                break;
            }
            tokio::time::sleep(Duration::from_millis(20)).await;
        }
    })
    .await
    .unwrap();
    let home = resources
        .folders
        .as_ref()
        .unwrap()
        .open(created.home)
        .unwrap();
    let pin = home
        .inbox_selection()
        .await
        .unwrap()
        .snapshot
        .metadata_commit;
    let current = home
        .inbox_thread(pin, created.thread)
        .await
        .unwrap()
        .thread
        .reference;
    if let api::Command::Codebase { expected, .. } = &mut removal.command {
        *expected = current;
    }
    let removed = supervisor
        .thread_workspace(fixture.actor, removal)
        .await
        .unwrap();
    let old = supervisor
        .status(fixture.actor)
        .await
        .unwrap()
        .sessions
        .into_iter()
        .find(|s| s.id == first_id)
        .unwrap();
    send.request_id = RequestId::new([218; 16]);
    send.thread_codebase = Some(removed.revision);
    send.attachments.clear();
    send.text = "Continue without code repositories".into();
    let second = supervisor
        .send_conversation(fixture.actor, send.clone())
        .await
        .unwrap();
    let second_id = second.agents[0].session_id.clone().unwrap();
    assert_ne!(first_id, second_id);
    wait(&supervisor, fixture.actor, |s| {
        s.sessions
            .iter()
            .any(|s| s.id == second_id && s.state == "idle" && s.last_sequence > 3)
    })
    .await;
    let status = supervisor.status(fixture.actor).await.unwrap();
    assert!(
        status
            .sessions
            .iter()
            .find(|s| s.id == second_id)
            .unwrap()
            .repository_bindings
            .is_empty()
    );
    assert_eq!(
        status
            .sessions
            .iter()
            .find(|s| s.id == first_id)
            .unwrap()
            .workspace,
        old.workspace
    );
    assert!(Path::new(&old.repository_bindings[0].workspace).exists());
    let pin = home
        .inbox_selection()
        .await
        .unwrap()
        .snapshot
        .metadata_commit;
    send.thread_codebase = Some(
        home.inbox_thread(pin, created.thread)
            .await
            .unwrap()
            .thread
            .reference,
    );
    send.request_id = RequestId::new([219; 16]);
    let third = supervisor
        .send_conversation(fixture.actor, send)
        .await
        .unwrap();
    assert_eq!(
        third.agents[0].session_id.as_deref(),
        Some(second_id.as_str())
    );
    supervisor.shutdown().await.unwrap();
}

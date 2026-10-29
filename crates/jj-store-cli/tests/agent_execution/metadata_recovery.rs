use super::*;

#[tokio::test]
async fn metadata_stale_generation_and_unknown_publication_recover_without_new_inference() {
    let (f, s) = start().await;
    let id = send(&s, f.actor, 111).await;
    configure(&s, f.actor, 112, Some("METADATA_GATE")).await;
    let refresh = command(&s, f.actor, &id, 113, api::Action::RegenerateTitle).await;
    s.metadata_command(f.actor, refresh.clone()).await.unwrap();
    let runtime = f.resources.root.join("metadata-runtime");
    tokio::time::timeout(Duration::from_secs(15), async {
        while !runtime.join("generation-started").exists() {
            tokio::time::sleep(Duration::from_millis(20)).await;
        }
    })
    .await
    .unwrap();
    let rename = command(
        &s,
        f.actor,
        &id,
        114,
        api::Action::Rename {
            title: "Concurrent title survives".into(),
        },
    )
    .await;
    s.metadata_command(f.actor, rename.clone()).await.unwrap();
    assert_eq!(
        terminal(&s, f.actor, rename.request_id).await.state,
        "completed"
    );
    fs::write(runtime.join("generation-release"), "release").unwrap();
    let failed = terminal(&s, f.actor, refresh.request_id).await;
    assert_eq!(failed.state, "failed");
    assert_eq!(failed.error.as_deref(), Some("metadata_changed"));
    assert!(failed.result.is_some());
    assert_eq!(
        s.metadata(f.actor, query(&id))
            .await
            .unwrap()
            .title
            .as_deref(),
        Some("Concurrent title survives")
    );
    s.shutdown().await.unwrap();

    // Model an interrupted publication journal after its frozen request was saved.
    // Repository selection remains real; recovery must retry exactly that receipt.
    let journal = f.resources.root.join("state.json");
    let mut state: serde_json::Value =
        serde_json::from_slice(&fs::read(&journal).unwrap()).unwrap();
    state["metadata_jobs"][refresh.request_id.to_string()]["view"]["state"] = json!("publishing");
    fs::write(&journal, serde_json::to_vec(&state).unwrap()).unwrap();
    let reopened = Supervisor::open(f.resources.clone()).unwrap();
    assert_eq!(
        reopened
            .metadata_job(f.actor, refresh.request_id)
            .await
            .unwrap()
            .state,
        "outcome_unknown"
    );
    let recovered = reopened.metadata_command(f.actor, refresh).await.unwrap();
    assert_eq!(recovered.state, "failed");
    assert_eq!(
        recovered.error.as_deref(),
        Some("metadata_publication_refused")
    );
    let publish = command(
        &reopened,
        f.actor,
        &id,
        115,
        api::Action::Publish {
            job: recovered.request_id,
        },
    )
    .await;
    reopened
        .metadata_command(f.actor, publish.clone())
        .await
        .unwrap();
    let published = terminal(&reopened, f.actor, publish.request_id).await;
    assert_eq!(published.state, "completed", "{published:?}");
    let value = reopened.metadata(f.actor, query(&id)).await.unwrap();
    assert_eq!(value.title.as_deref(), Some("Concurrent title survives"));
    assert!(value.value.generated.is_some());
    let local = f
        .resources
        .folders
        .as_ref()
        .unwrap()
        .open(value.repository)
        .unwrap();
    // Publication can retain an older committed source without widening the
    // public active-snapshot read capability.
    let source = &value.value.generated.as_ref().unwrap().sources[0];
    assert!(matches!(
        local
            .agent_store()
            .await
            .unwrap()
            .object(source.snapshot)
            .await,
        Err(jj_store_service::local_repository::LocalError::Stale)
    ));
    let selection = local.inbox_selection().await.unwrap();
    let thread = local
        .inbox_thread(selection.snapshot.metadata_commit, value.thread.unwrap())
        .await
        .unwrap();
    let mut foreign = value.value.clone();
    foreign.generated.as_mut().unwrap().sources[0].session =
        jj_agent::AgentSessionId::new([200; 16]);
    let rejected = local
        .apply_collab_command(&jj_store_service::local_repository::LocalCollabRequest {
            version: 1,
            request_id: RequestId::new([117; 16]),
            command: jj_collab::CollabCommandV1::Thread {
                intent: jj_collab::ThreadCommandV1::SetMetadata {
                    thread_id: value.thread.unwrap(),
                    expected_thread: thread.thread.reference,
                    expected_metadata: value.revision,
                    target: value.target.clone(),
                    value: Box::new(foreign),
                },
            },
        })
        .await;
    assert!(matches!(
        rejected,
        Err(jj_store_service::local_repository::LocalError::Corrupt)
    ));
    assert_eq!(
        reopened
            .metadata(f.actor, query(&id))
            .await
            .unwrap()
            .revision,
        value.revision
    );
    assert_eq!(
        fs::read_to_string(runtime.join("inputs.jsonl"))
            .unwrap()
            .lines()
            .count(),
        1
    );
    reopened.shutdown().await.unwrap();
}

#[tokio::test]
async fn metadata_acp_suggestions_are_repository_backed_without_a_generation_provider() {
    let (f, s) = start_with_title(true).await;
    let id = send(&s, f.actor, 116).await;
    let selected = tokio::time::timeout(Duration::from_secs(40), async {
        loop {
            let view = s.metadata(f.actor, query(&id)).await.unwrap();
            if view.value.agent_titles.contains_key(&id) {
                break view;
            }
            tokio::time::sleep(Duration::from_millis(20)).await;
        }
    })
    .await
    .unwrap();
    assert_eq!(selected.title.as_deref(), Some("ACP suggested title"));
    assert!(selected.value.generated.is_none());
    assert!(selected.value.agent_title_sources.contains_key(&id));
    assert!(
        !f.resources
            .root
            .join("metadata-runtime/inputs.jsonl")
            .exists()
    );
    s.shutdown().await.unwrap();
    let reopened = Supervisor::open(f.resources.clone()).unwrap();
    assert_eq!(
        reopened
            .metadata(f.actor, query(&id))
            .await
            .unwrap()
            .revision,
        selected.revision
    );
    assert_eq!(
        reopened.metadata(f.actor, query(&id)).await.unwrap().title,
        selected.title
    );
    reopened.shutdown().await.unwrap();
}

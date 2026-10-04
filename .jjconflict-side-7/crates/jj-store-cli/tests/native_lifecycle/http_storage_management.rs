use super::*;
use jj_store_service::{storage_backend::NativeStorageBackend, storage_management::*};
fn request(suffix: &str, body: Option<Value>) -> Request<Body> {
    let mut r = link_request("unused", 0);
    *r.uri_mut() = format!("/v1/repos/{REPO}/local/storage{suffix}")
        .parse()
        .unwrap();
    if let Some(body) = body {
        *r.body_mut() = Body::from(body.to_string());
    } else {
        *r.method_mut() = axum::http::Method::GET;
        *r.body_mut() = Body::empty();
        r.headers_mut().remove("content-type");
    }
    r
}
pub(super) async fn capture(samples: &mut Vec<Value>) {
    let f = ContentFixture::new().await;
    let cache = tempfile::tempdir().unwrap();
    let state = tempfile::tempdir().unwrap();
    let host = RepositoryHost::new(
        f.f.store.clone(),
        Arc::new(NoOperators),
        Arc::new(SystemClock),
        Protection::default(),
        SessionLimits::default(),
        4 * 1024 * 1024,
    )
    .unwrap();
    let backend = Arc::new(NativeStorageBackend {
        host,
        principal: ACTOR,
    });
    let make = || {
        Arc::new(
            jj_store_server::storage_management::Manager::new(state.path().into(), backend.clone())
                .unwrap(),
        )
    };
    let manager = make();
    let router = |manager: Arc<jj_store_server::storage_management::Manager>, access| {
        api(
            &f,
            cache.path(),
            Config {
                access,
                local_storage: Some(manager),
                ..Default::default()
            },
        )
    };
    let local = router(
        manager.clone(),
        Access::LocalTrusted {
            principal: ACTOR,
            permission: Permission::Admin,
        },
    );
    let id = m::RequestId::new([81; 16]);
    let command = |action| StorageCommand {
        version: 1,
        request_id: id,
        action,
    };
    let overview = manager.overview(REPO).await.unwrap();
    assert!(overview.latest.is_none());
    assert!(
        !state.path().join(format!("storage-{REPO}.json")).exists(),
        "GET creates no journal"
    );
    let before = f.f.session.snapshot().await.unwrap().manifest().clone();
    let prepared = manager
        .command(REPO, command(StorageAction::PrepareCheckpoint))
        .await
        .unwrap();
    assert_eq!(prepared.phase, StoragePhase::Prepared);
    assert_eq!(f.f.session.snapshot().await.unwrap().manifest(), &before);
    assert!(matches!(
        manager.command(REPO, command(StorageAction::Check)).await,
        Err(jj_store_service::storage_management::Error::Conflict)
    ));
    let restarted = make();
    assert_eq!(
        restarted.operation(REPO, id).await.unwrap().target,
        prepared.target
    );
    restarted
        .command(REPO, command(StorageAction::PublishCheckpoint))
        .await
        .unwrap();
    let completed = tokio::time::timeout(std::time::Duration::from_secs(10), async {
        loop {
            let r = restarted.operation(REPO, id).await.unwrap();
            if r.phase != StoragePhase::Running {
                break r;
            }
            tokio::time::sleep(std::time::Duration::from_millis(10)).await;
        }
    })
    .await
    .unwrap();
    assert_eq!(completed.phase, StoragePhase::Complete);
    assert_eq!(completed.selection, Some(CheckpointSelection::Selected));
    assert_eq!(completed.target, prepared.target);
    let after = f.f.session.snapshot().await.unwrap().manifest().clone();
    assert_eq!(after.published, before.published);
    assert_eq!(after.heads, before.heads);
    assert_eq!(after.receipts, before.receipts);
    assert_eq!(
        make().operation(REPO, id).await.unwrap().selection,
        Some(CheckpointSelection::Selected)
    );
    let check_id = m::RequestId::new([82; 16]);
    let run = restarted
        .command(
            REPO,
            StorageCommand {
                version: 1,
                request_id: check_id,
                action: StorageAction::Check,
            },
        )
        .await
        .unwrap();
    assert_eq!(run.phase, StoragePhase::Running);
    let check = tokio::time::timeout(std::time::Duration::from_secs(10), async {
        loop {
            let r = restarted.operation(REPO, check_id).await.unwrap();
            if r.phase != StoragePhase::Running {
                break r;
            }
            tokio::time::sleep(std::time::Duration::from_millis(10)).await;
        }
    })
    .await
    .unwrap();
    assert_eq!(check.phase, StoragePhase::Complete);
    assert_eq!(check.target, run.target);
    // Restart does not resume an interrupted integrity audit or claim success.
    let journal_path = state.path().join(format!("storage-{REPO}.json"));
    let saved = std::fs::read(&journal_path).unwrap();
    let mut journal: Value = serde_json::from_slice(&saved).unwrap();
    journal["entries"][1]["report"]["phase"] = "running".into();
    std::fs::write(&journal_path, serde_json::to_vec(&journal).unwrap()).unwrap();
    assert_eq!(
        make().operation(REPO, check_id).await.unwrap().phase,
        StoragePhase::Interrupted
    );
    std::fs::write(&journal_path, &saved).unwrap();
    let small_host = RepositoryHost::new(
        f.f.store.clone(),
        Arc::new(NoOperators),
        Arc::new(SystemClock),
        Protection::default(),
        SessionLimits {
            inventory_nodes: 1,
            ..Default::default()
        },
        4 * 1024 * 1024,
    )
    .unwrap();
    let small = NativeStorageBackend {
        host: small_host,
        principal: ACTOR,
    };
    let captured = small.capture(REPO).await.unwrap();
    assert!(matches!(
        small.check(REPO, &captured).await,
        Err(jj_store_service::storage_management::Error::Incomplete)
    ));
    let cancel_id = m::RequestId::new([83; 16]);
    let cancel = |action| StorageCommand {
        version: 1,
        request_id: cancel_id,
        action,
    };
    restarted
        .command(REPO, cancel(StorageAction::PrepareCheckpoint))
        .await
        .unwrap();
    assert_eq!(
        restarted
            .command(REPO, cancel(StorageAction::CancelCheckpoint))
            .await
            .unwrap()
            .phase,
        StoragePhase::Canceled
    );
    assert_eq!(
        restarted
            .command(REPO, cancel(StorageAction::PublishCheckpoint))
            .await
            .unwrap()
            .phase,
        StoragePhase::Canceled
    );
    for (suffix, body, schema) in [
        ("".to_owned(), None, ""),
        (
            "/operations".into(),
            Some(serde_json::to_value(command(StorageAction::PrepareCheckpoint)).unwrap()),
            "/operations",
        ),
        (
            format!("/operations/{id}"),
            None,
            "/operations/{request_id}",
        ),
    ] {
        let method = if body.is_some() { "post" } else { "get" };
        let response = local.clone().oneshot(request(&suffix, body)).await.unwrap();
        let status = response.status();
        let data: Value =
            serde_json::from_slice(&to_bytes(response.into_body(), 1024 * 1024).await.unwrap())
                .unwrap();
        assert_eq!(status, StatusCode::OK, "{suffix}: {data}");
        samples.push(serde_json::json!({"path":format!("/v1/repos/{{repo}}/local/storage{schema}"),"method":method,"status":200,"media":"application/json","data":data}));
    }
    for access in [
        Access::Authenticated,
        Access::LocalTrusted {
            principal: ACTOR,
            permission: Permission::Read,
        },
    ] {
        let bearer = matches!(access, Access::Authenticated);
        let denied = router(manager.clone(), access);
        for (suffix, body) in [
            ("".into(), None),
            (format!("/operations/{id}"), None),
            (
                "/operations".into(),
                Some(serde_json::to_value(command(StorageAction::Check)).unwrap()),
            ),
        ] {
            let mut req = request(&suffix, body);
            if bearer {
                req.headers_mut().insert(
                    "authorization",
                    format!("Bearer {}", f.f.token.expose().as_str())
                        .parse()
                        .unwrap(),
                );
            }
            assert_eq!(
                denied.clone().oneshot(req).await.unwrap().status(),
                StatusCode::FORBIDDEN
            );
        }
    }
    let mut invalid = serde_json::to_value(command(StorageAction::Check)).unwrap();
    invalid["path"] = "/tmp/evil".into();
    assert_eq!(
        local
            .clone()
            .oneshot(request("/operations", Some(invalid)))
            .await
            .unwrap()
            .status(),
        StatusCode::BAD_REQUEST
    );
}

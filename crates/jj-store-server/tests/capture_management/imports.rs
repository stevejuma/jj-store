use super::*;
use jj_store_server::{
    capture_import_state::CaptureImportJournal, capture_management::CaptureImportManagement,
};
use reqwest::{Client, Method, StatusCode};
use serde_json::{Value, json};
#[path = "imports/inspection.rs"]
mod inspection;

pub(super) fn attach(f: &mut Fixture) -> CaptureImportJournal {
    let binding = f.binding.as_mut().unwrap();
    let journal = CaptureImportJournal::create(
        &f.config.state_dir.join("agent-capture/imports"),
        [0xab; 16],
        binding.generation,
    )
    .unwrap();
    binding.imports = Some(Arc::new(
        CaptureImportManagement::new(journal.clone(), &f.source.spool).unwrap(),
    ));
    f.manager = Manager::new(
        &f.source.root.path().join("server.toml"),
        &f.config,
        &f.identity,
        Some(binding.clone()),
    )
    .unwrap();
    journal
}
async fn ready(f: &Fixture) -> String {
    tokio::time::timeout(Duration::from_secs(10), async {
        loop {
            match f
                .manager
                .sources(f.actor, CaptureSourceQuery::default())
                .await
            {
                Ok(page) if !page.sources.is_empty() => return page.sources[0].selector.clone(),
                Ok(_) | Err(Error::Busy) => {}
                other => panic!("source observation: {other:?}"),
            }
            tokio::time::sleep(Duration::from_millis(5)).await;
        }
    })
    .await
    .unwrap()
}
async fn settled(f: &Fixture, request_id: RequestId) -> CaptureImportStatus {
    settled_with_timeout(f, request_id, 10).await
}
async fn settled_with_timeout(
    f: &Fixture,
    request_id: RequestId,
    seconds: u64,
) -> CaptureImportStatus {
    let mut last = String::new();
    tokio::time::timeout(Duration::from_secs(seconds), async {
        loop {
            match f
                .manager
                .import_status(f.actor, CaptureImportQuery { request_id })
                .await
            {
                Ok(status) if status.progress.phase == CaptureImportPhase::Completed => {
                    return status;
                }
                Ok(status) => {
                    last = format!("{status:?}");
                }
                Err(Error::Busy | Error::Unavailable) => {}
                other => panic!("import observation: {other:?}"),
            }
            tokio::time::sleep(Duration::from_millis(10)).await;
        }
    })
    .await
    .unwrap_or_else(|e| {
        panic!(
            "import completion {e:?}: {last}; runtime {:?}",
            f.binding
                .as_ref()
                .unwrap()
                .runtime
                .get()
                .unwrap()
                .snapshot()
        )
    })
}
fn sample(samples: &mut Vec<Value>, method: Method, path: &str, status: StatusCode, data: Value) {
    samples.push(
        json!({"path": path.split('?').next().unwrap(), "method": method.as_str().to_lowercase(),
        "status": status.as_u16(), "media":"application/json", "data":data}),
    );
}

#[tokio::test(flavor = "multi_thread", worker_threads = 2)]
async fn import_acceptance_progress_and_replay_are_actual_http_contracts() {
    let mut f = Fixture::new(true).await;
    let journal = attach(&mut f);
    f.source.message("history", "explicit historical import");
    let mut runtime = f.runtime();
    let command = CaptureImportCommand {
        version: 1,
        request_id: RequestId::new([5; 16]),
        generation: hex::encode(f.binding.as_ref().unwrap().generation),
        expected_revision: 0,
        selector: ready(&f).await,
    };
    let listener = tokio::net::TcpListener::bind("127.0.0.1:0").await.unwrap();
    let base = format!("http://{}", listener.local_addr().unwrap());
    let api = jj_store_http::HttpApi::new(
        f.source.host.clone(),
        f.source.root.path().join("import-http"),
        jj_store_http::Config {
            origin: base.clone(),
            access: jj_store_http::Access::LocalTrusted {
                principal: f.actor,
                permission: Permission::Admin,
            },
            local_capture: Some(Arc::new(f.manager.clone())),
            ..Default::default()
        },
    )
    .unwrap();
    let (stop, stopped) = tokio::sync::oneshot::channel();
    let task = tokio::spawn(async move {
        axum::serve(listener, api.router())
            .with_graceful_shutdown(async {
                let _ = stopped.await;
            })
            .await
            .unwrap();
    });
    let client = Client::new();
    let mut samples = vec![];
    let path = format!(
        "/v1/local/capture/imports?request_id={}",
        command.request_id
    );
    let (status, data) = http::request(&client, &base, Method::GET, &path, None, None).await;
    assert_eq!(status, StatusCode::OK);
    assert!(data["receipt"].is_null());
    sample(&mut samples, Method::GET, &path, status, data);
    let body = serde_json::to_value(&command).unwrap();
    let (status, data) = http::request(
        &client,
        &base,
        Method::POST,
        "/v1/local/capture/import",
        Some(body.clone()),
        None,
    )
    .await;
    // Cold sanitizer preparation may outlive the acknowledgement budget.
    // Recover only the original request; a timeout is never proof of rejection.
    let unknown = status == StatusCode::SERVICE_UNAVAILABLE;
    let (status, data) = if unknown {
        assert_eq!(data["code"], "capture_control_outcome_unknown");
        sample(
            &mut samples,
            Method::POST,
            "/v1/local/capture/import",
            status,
            data,
        );
        let completed = settled_with_timeout(&f, command.request_id, 240).await;
        assert_eq!(completed.receipt.as_ref().unwrap().command, command);
        tokio::time::timeout(Duration::from_secs(10), async {
            loop {
                let (status, data) = http::request(
                    &client,
                    &base,
                    Method::POST,
                    "/v1/local/capture/import",
                    Some(body.clone()),
                    None,
                )
                .await;
                if status != StatusCode::CONFLICT {
                    break (status, data);
                }
                assert_eq!(data["code"], "capture_control_busy");
                sample(
                    &mut samples,
                    Method::POST,
                    "/v1/local/capture/import",
                    status,
                    data,
                );
                tokio::time::sleep(Duration::from_millis(20)).await;
            }
        })
        .await
        .unwrap()
    } else {
        (status, data)
    };
    assert_eq!(status, StatusCode::OK, "{data}");
    let accepted: CaptureImportOutcome = serde_json::from_value(data.clone()).unwrap();
    assert_eq!(accepted.receipt.command, command);
    assert_eq!(accepted.replayed, unknown);
    sample(
        &mut samples,
        Method::POST,
        "/v1/local/capture/import",
        status,
        data,
    );
    let completed = settled_with_timeout(&f, command.request_id, 240).await;
    assert!(completed.progress.boundary_available);
    assert_eq!(
        completed.progress.committed_byte_offset,
        accepted.receipt.complete_byte_offset
    );
    runtime.pause().unwrap();
    tokio::time::timeout(Duration::from_secs(10), async {
        while runtime.snapshot().phase
            != jj_store_server::capture_worker::CaptureWorkerPhase::Paused
        {
            tokio::time::sleep(Duration::from_millis(5)).await;
        }
    })
    .await
    .unwrap();
    fs::remove_file(&f.source.transcript).unwrap();
    let original = journal.entries().unwrap();
    for method in [Method::GET, Method::POST] {
        let endpoint = if method == Method::GET {
            path.as_str()
        } else {
            "/v1/local/capture/import"
        };
        let (status, data) = http::request(
            &client,
            &base,
            method.clone(),
            endpoint,
            (method == Method::POST).then(|| body.clone()),
            None,
        )
        .await;
        assert_eq!(status, StatusCode::OK, "{data}");
        assert_eq!(
            data["receipt"],
            serde_json::to_value(&accepted.receipt).unwrap()
        );
        assert_eq!(data["progress"]["phase"], "completed");
        assert_eq!(data["progress"]["tail_phase"], "withheld");
        if method == Method::POST {
            assert_eq!(data["replayed"], true);
        }
        sample(&mut samples, method, endpoint, status, data);
    }
    let mut changed = body.clone();
    changed["expected_revision"] = json!(1);
    let mut forged = body.clone();
    forged["actor"] = json!(f.actor);
    for (method, endpoint, input, expected) in [
        (
            Method::POST,
            "/v1/local/capture/import",
            Some(changed),
            StatusCode::CONFLICT,
        ),
        (
            Method::POST,
            "/v1/local/capture/import",
            Some(forged),
            StatusCode::BAD_REQUEST,
        ),
        (
            Method::GET,
            "/v1/local/capture/imports?request_id=00000000000000000000000000000000",
            None,
            StatusCode::BAD_REQUEST,
        ),
        (
            Method::GET,
            "/v1/local/capture/imports?request_id=05050505050505050505050505050505&actor=spoof",
            None,
            StatusCode::BAD_REQUEST,
        ),
    ] {
        let (status, data) =
            http::request(&client, &base, method.clone(), endpoint, input, None).await;
        assert_eq!(status, expected, "{data}");
        sample(&mut samples, method, endpoint, status, data);
    }
    runtime.shutdown().await.unwrap();
    let (status, data) = http::request(&client, &base, Method::GET, &path, None, None).await;
    assert_eq!(status, StatusCode::OK);
    assert_eq!(data["progress"]["phase"], "completed");
    assert_eq!(data["progress"]["tail_phase"], "unavailable");
    sample(&mut samples, Method::GET, &path, status, data);
    let mut revoked = f.config.clone();
    let Access::LocalTrusted { permission, .. } = &mut revoked.access else {
        unreachable!()
    };
    *permission = Permission::Write;
    f.save(&revoked);
    let (status, data) = http::request(&client, &base, Method::GET, &path, None, None).await;
    assert_eq!(status, StatusCode::FORBIDDEN);
    sample(&mut samples, Method::GET, &path, status, data);
    let mut disabled = f.config.clone();
    disabled.agent_capture.enabled = false;
    f.save(&disabled);
    fs::rename(
        f.config.state_dir.join("agent-capture"),
        f.config.state_dir.join("retained-capture"),
    )
    .unwrap();
    let (status, data) = http::request(&client, &base, Method::GET, &path, None, None).await;
    assert_eq!(status, StatusCode::CONFLICT);
    sample(&mut samples, Method::GET, &path, status, data);
    assert!(!f.config.state_dir.join("agent-capture").exists());
    assert_eq!(original.len(), 1);
    assert!(samples.iter().all(|s| {
        !s.to_string()
            .contains(f.source.root.path().to_str().unwrap())
            && !s.to_string().contains("generation_digest")
            && !s.to_string().contains("physical_digest")
    }));
    if let Some(path) = std::env::var_os("JJ_STORE_CAPTURE_IMPORT_CONTRACT_CAPTURE") {
        fs::write(path, serde_json::to_vec(&samples).unwrap()).unwrap();
    }
    let _ = stop.send(());
    task.await.unwrap();
}

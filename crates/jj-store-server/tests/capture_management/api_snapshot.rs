use super::*;
use reqwest::{Client, Method, StatusCode};
use serde_json::json;
#[path = "../opencode_source/fixture.rs"]
#[allow(dead_code)]
mod native_api;

async fn published(f: &Fixture) -> CaptureSourceCoverage {
    // Cold native sanitizer preparation can outlive the foreground command
    // acknowledgement. Observe the original publication without resending it.
    tokio::time::timeout(Duration::from_secs(120), async {
        loop {
            match f
                .manager
                .sources(f.actor, CaptureSourceQuery::default())
                .await
            {
                Ok(page) => {
                    for row in page.sources {
                        if row.snapshot_publication.as_ref().is_some_and(|p| {
                            p.history_available
                                && p.committed_sequence > 0
                                && p.captured_sequence == p.committed_sequence
                        }) {
                            return row;
                        }
                    }
                }
                Err(Error::Busy) => {}
                other => panic!("source management: {other:?}"),
            }
            tokio::time::sleep(Duration::from_millis(10)).await;
        }
    })
    .await
    .unwrap()
}
#[tokio::test]
async fn api_snapshot_progress_is_native_receipt_backed_and_body_free() {
    let mut f = Fixture::new(true).await;
    let native = native_api::Api::new(
        f.source.workspace(source::A).to_str().unwrap().into(),
        vec![],
        true,
    )
    .await;
    f.config.agent_capture.harnesses = vec![jj_store_server::capture_config::HarnessConfig {
        profile: jj_store_capture::profile::CaptureProfile::OpencodeReadOnlyApiV1,
        product_version: "1.18.30".into(),
        roots: vec![],
        endpoint: Some(native.endpoint.clone()),
    }];
    f.save(&f.config);
    let journal = imports::attach(&mut f);
    let mut runtime = f.runtime();
    let row = published(&f).await;
    let progress = row.snapshot_publication.unwrap();
    assert_eq!(progress.captured_logical_records, 0);
    assert_eq!(progress.committed_logical_records, Some(0));
    assert_eq!(progress.receipts, 1);
    f.manager
        .control(f.actor, f.command(1, 0, CaptureControlAction::Pause))
        .await
        .unwrap();
    tokio::time::timeout(Duration::from_secs(15), async {
        while runtime.snapshot().phase
            != jj_store_server::capture_worker::CaptureWorkerPhase::Paused
        {
            tokio::time::sleep(Duration::from_millis(10)).await;
        }
    })
    .await
    .unwrap();
    let listener = tokio::net::TcpListener::bind("127.0.0.1:0").await.unwrap();
    let base = format!("http://{}", listener.local_addr().unwrap());
    let http = jj_store_http::HttpApi::new(
        f.source.host.clone(),
        f.source.root.path().join("snapshot-http"),
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
        axum::serve(listener, http.router())
            .with_graceful_shutdown(async {
                let _ = stopped.await;
            })
            .await
            .unwrap();
    });
    drop(native); // Management inspection must not consult the harness.
    let policy = f.bytes();
    let pending = f
        .source
        .root
        .path()
        .join("spool/pending/ffffffffffffffffffffffffffffffff.json");
    fs::write(&pending, b"PENDING_BODY_MUST_NOT_BE_READ").unwrap();
    let client = Client::new();
    let mut samples = vec![];
    let (code, data) = http::request(
        &client,
        &base,
        Method::GET,
        "/v1/local/capture/sources",
        None,
        None,
    )
    .await;
    assert_eq!(code, StatusCode::OK, "{data}");
    assert_eq!(
        data["sources"][0]["snapshot_publication"]["committed_logical_records"],
        0
    );
    assert_eq!(data["sources"][0]["snapshot_publication"]["receipts"], 1);
    samples.push(json!({"path":"/v1/local/capture/sources","method":"get","status":200,"media":"application/json","data":data}));
    assert_eq!(
        fs::read(&pending).unwrap(),
        b"PENDING_BODY_MUST_NOT_BE_READ"
    );
    assert_eq!(f.bytes(), policy);
    assert!(journal.entries().unwrap().is_empty());
    let publication = f
        .source
        .root
        .path()
        .join("spool/reader-publication/catalog.json");
    let original = fs::read(&publication).unwrap();
    fs::write(&publication, b"corrupt selected publication").unwrap();
    let (code, data) = http::request(
        &client,
        &base,
        Method::GET,
        "/v1/local/capture/sources",
        None,
        None,
    )
    .await;
    assert_eq!(code, StatusCode::OK);
    assert!(data["sources"][0]["snapshot_publication"].is_null());
    samples.push(json!({"path":"/v1/local/capture/sources","method":"get","status":200,"media":"application/json","data":data}));
    assert_eq!(
        fs::read(&publication).unwrap(),
        b"corrupt selected publication"
    );
    fs::write(&publication, original).unwrap();
    let mut revoked = f.config.clone();
    let Access::LocalTrusted { permission, .. } = &mut revoked.access else {
        unreachable!()
    };
    *permission = Permission::Write;
    f.save(&revoked);
    let (code, data) = http::request(
        &client,
        &base,
        Method::GET,
        "/v1/local/capture/sources",
        None,
        None,
    )
    .await;
    assert_eq!(code, StatusCode::FORBIDDEN);
    assert!(data.get("sources").is_none());
    samples.push(json!({"path":"/v1/local/capture/sources","method":"get","status":code.as_u16(),"media":"application/json","data":data}));
    assert_eq!(f.bytes(), policy);
    fs::remove_file(pending).unwrap();
    f.save(&f.config);
    runtime.shutdown().await.unwrap();
    let (code, data) = http::request(
        &client,
        &base,
        Method::GET,
        "/v1/local/capture/sources",
        None,
        None,
    )
    .await;
    assert_eq!(code, StatusCode::OK);
    assert_eq!(data["phase"], "stopped");
    assert_eq!(data["sources"][0]["snapshot_publication"]["receipts"], 1);
    samples.push(json!({"path":"/v1/local/capture/sources","method":"get","status":200,"media":"application/json","data":data}));
    let mut disabled = f.config.clone();
    disabled.agent_capture.enabled = false;
    f.save(&disabled);
    let (code, data) = http::request(
        &client,
        &base,
        Method::GET,
        "/v1/local/capture/sources",
        None,
        None,
    )
    .await;
    assert_eq!(code, StatusCode::OK);
    assert_eq!(data["enabled"], false);
    assert_eq!(data["sources"], json!([]));
    samples.push(json!({"path":"/v1/local/capture/sources","method":"get","status":200,"media":"application/json","data":data}));
    assert_eq!(f.bytes(), policy);
    assert!(journal.entries().unwrap().is_empty());
    stop.send(()).unwrap();
    task.await.unwrap();
    if let Some(path) = std::env::var_os("JJ_STORE_CAPTURE_SNAPSHOT_CONTRACT_CAPTURE") {
        fs::write(path, serde_json::to_vec_pretty(&samples).unwrap()).unwrap();
    }
}

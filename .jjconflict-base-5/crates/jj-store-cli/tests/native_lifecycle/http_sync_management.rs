use super::*;
use jj_store_rpc::server::{ReplicationControlError as ControlError, ReplicationControls};
use jj_store_service::replication_control as control;
use std::sync::atomic::{AtomicBool, AtomicUsize, Ordering};
#[derive(Default)]
struct Controls {
    configured: AtomicBool,
    paused: AtomicBool,
    calls: AtomicUsize,
    reached: AtomicBool,
}
#[async_trait::async_trait]
impl ReplicationControls for Controls {
    async fn status(
        &self,
        repository: jj_store_model::RepositoryId,
    ) -> std::result::Result<control::StatusV1, ControlError> {
        self.calls.fetch_add(1, Ordering::SeqCst);
        if !self.configured.load(Ordering::SeqCst) {
            return Err(ControlError::NotConfigured);
        }
        Ok(control::StatusV1 {
            version: 1,
            repository,
            local_ready: true,
            offline_ready: true,
            phase: if self.paused.load(Ordering::SeqCst) {
                control::Phase::Paused
            } else {
                control::Phase::Running
            },
            local_replica: None,
            peer_replica: None,
            local_frontier: None,
            peer_frontier: None,
            pushed: None,
            pulled: None,
            local_pending_positions: Some(0),
            peer_pending_positions: None,
            checkpoint_revision: 0,
            operation_heads: 1,
            reconciliation_required: false,
            pull: Default::default(),
            push: Default::default(),
            search: control::SearchStatusV1 {
                configured: false,
                repository_position: None,
                indexed_position: None,
                degraded: false,
            },
        })
    }
    async fn wake(
        &self,
        repository: jj_store_model::RepositoryId,
    ) -> std::result::Result<control::StatusV1, ControlError> {
        self.status(repository).await
    }
    async fn set_paused(
        &self,
        repository: jj_store_model::RepositoryId,
        paused: bool,
    ) -> std::result::Result<control::StatusV1, ControlError> {
        self.paused.store(paused, Ordering::SeqCst);
        self.status(repository).await
    }
    async fn capture_barrier(
        &self,
        repository: jj_store_model::RepositoryId,
    ) -> std::result::Result<control::BarrierV1, ControlError> {
        self.calls.fetch_add(1, Ordering::SeqCst);
        Ok(control::BarrierV1 {
            version: 1,
            repository,
            local_replica: jj_store_model::ReplicaId::new([1; 16]),
            peer_replica: jj_store_model::ReplicaId::new([2; 16]),
            push_epoch: jj_store_model::EpochId::new([3; 16]),
            pull_epoch: jj_store_model::EpochId::new([4; 16]),
            push_through: jj_store_model::WalPosition(9),
            pull_through: jj_store_model::WalPosition(12),
        })
    }
    async fn wait(
        &self,
        repository: jj_store_model::RepositoryId,
        request: &control::WaitRequestV1,
    ) -> std::result::Result<control::WaitOutcomeV1, ControlError> {
        self.calls.fetch_add(1, Ordering::SeqCst);
        if request.barrier.push_epoch != jj_store_model::EpochId::new([3; 16]) {
            return Err(ControlError::Invalid);
        }
        if !self.reached.load(Ordering::SeqCst) {
            return Err(ControlError::Timeout);
        }
        Ok(control::WaitOutcomeV1 {
            version: 1,
            barrier: request.barrier,
            status: self.status(repository).await?,
        })
    }
}
fn command(body: &str) -> Request<Body> {
    Request::builder()
        .method("POST")
        .uri(format!("/v1/repos/{REPO}/local/sync/control"))
        .header("host", "127.0.0.1:7443")
        .header("x-jj-store-protocol", "1")
        .header("content-type", "application/json")
        .body(Body::from(body.to_owned()))
        .unwrap()
}
pub(super) async fn capture(f: &ContentFixture, cache: &std::path::Path, samples: &mut Vec<Value>) {
    let controls = Arc::new(Controls::default());
    let manager = Arc::new(jj_store_server::sync_management::LocalSync(
        controls.clone(),
    ));
    let local = api(
        f,
        cache,
        Config {
            access: Access::LocalTrusted {
                principal: ACTOR,
                permission: Permission::Admin,
            },
            local_sync: Some(manager.clone()),
            ..Config::default()
        },
    );
    let response = local
        .clone()
        .oneshot(protected(&format!("/v1/repos/{REPO}/local/sync")))
        .await
        .unwrap();
    assert_eq!(response.status(), StatusCode::OK);
    let data: Value =
        serde_json::from_slice(&to_bytes(response.into_body(), 1024 * 1024).await.unwrap())
            .unwrap();
    assert_eq!(data["configured"], false);
    samples.push(serde_json::json!({"path":"/v1/repos/{repo}/local/sync","method":"get","status":200,"media":"application/json","data":data}));
    assert_eq!(
        local
            .clone()
            .oneshot(command(r#"{"version":1,"action":"retry"}"#))
            .await
            .unwrap()
            .status(),
        StatusCode::CONFLICT
    );
    controls.configured.store(true, Ordering::SeqCst);
    for (action, phase) in [
        ("pause", "paused"),
        ("pause", "paused"),
        ("resume", "running"),
        ("retry", "running"),
    ] {
        let response = local
            .clone()
            .oneshot(command(&format!(r#"{{"version":1,"action":"{action}"}}"#)))
            .await
            .unwrap();
        assert_eq!(response.status(), StatusCode::OK);
        let data: Value =
            serde_json::from_slice(&to_bytes(response.into_body(), 1024 * 1024).await.unwrap())
                .unwrap();
        assert_eq!(data["worker"]["phase"], phase);
        samples.push(serde_json::json!({"path":"/v1/repos/{repo}/local/sync/control","method":"post","status":200,"media":"application/json","data":data}));
    }
    let mut capture_request = command("{}");
    *capture_request.uri_mut() = format!("/v1/repos/{REPO}/local/sync/barrier")
        .parse()
        .unwrap();
    let response = local.clone().oneshot(capture_request).await.unwrap();
    assert_eq!(response.status(), StatusCode::OK);
    let barrier: Value =
        serde_json::from_slice(&to_bytes(response.into_body(), 1024 * 1024).await.unwrap())
            .unwrap();
    samples.push(serde_json::json!({"path":"/v1/repos/{repo}/local/sync/barrier","method":"post","status":200,"media":"application/json","data":barrier}));
    for reached in [false, true] {
        controls.reached.store(reached, Ordering::SeqCst);
        let mut request = command(
            &serde_json::json!({"version":1,"barrier":barrier,"timeout_millis":1000}).to_string(),
        );
        *request.uri_mut() = format!("/v1/repos/{REPO}/local/sync/wait").parse().unwrap();
        let response = local.clone().oneshot(request).await.unwrap();
        assert_eq!(response.status(), StatusCode::OK);
        let data: Value =
            serde_json::from_slice(&to_bytes(response.into_body(), 1024 * 1024).await.unwrap())
                .unwrap();
        assert_eq!(data["reached"], reached);
        assert_eq!(data["barrier"], barrier);
        samples.push(serde_json::json!({"path":"/v1/repos/{repo}/local/sync/wait","method":"post","status":200,"media":"application/json","data":data}));
    }
    let mut invalid_barrier = barrier.clone();
    invalid_barrier["push_epoch"] = serde_json::json!(jj_store_model::EpochId::new([99; 16]));
    let mut invalid_wait = command(
        &serde_json::json!({"version":1,"barrier":invalid_barrier,"timeout_millis":1000})
            .to_string(),
    );
    *invalid_wait.uri_mut() = format!("/v1/repos/{REPO}/local/sync/wait").parse().unwrap();
    assert_eq!(
        local.clone().oneshot(invalid_wait).await.unwrap().status(),
        StatusCode::BAD_REQUEST
    );
    for timeout in [0, 1001] {
        let mut request = command(
            &serde_json::json!({"version":1,"barrier":barrier,"timeout_millis":timeout})
                .to_string(),
        );
        *request.uri_mut() = format!("/v1/repos/{REPO}/local/sync/wait").parse().unwrap();
        assert_eq!(
            local.clone().oneshot(request).await.unwrap().status(),
            StatusCode::BAD_REQUEST
        );
    }
    let calls = controls.calls.load(Ordering::SeqCst);
    for body in [
        r#"{"version":2,"action":"pause"}"#,
        r#"{"version":1,"action":"configure"}"#,
        r#"{"version":1,"action":"pause","upstream":"evil"}"#,
    ] {
        assert_eq!(
            local.clone().oneshot(command(body)).await.unwrap().status(),
            StatusCode::BAD_REQUEST
        );
    }
    for access in [
        Access::Authenticated,
        Access::LocalTrusted {
            principal: ACTOR,
            permission: Permission::Read,
        },
    ] {
        let authenticated = matches!(access, Access::Authenticated);
        let router = api(
            f,
            cache,
            Config {
                access,
                local_sync: Some(manager.clone()),
                ..Config::default()
            },
        );
        let mut barrier_request = command("{}");
        *barrier_request.uri_mut() = format!("/v1/repos/{REPO}/local/sync/barrier")
            .parse()
            .unwrap();
        let mut wait_request = command(
            &serde_json::json!({"version":1,"barrier":barrier,"timeout_millis":1000}).to_string(),
        );
        *wait_request.uri_mut() = format!("/v1/repos/{REPO}/local/sync/wait").parse().unwrap();
        for mut request in [
            barrier_request,
            wait_request,
            protected(&format!("/v1/repos/{REPO}/local/sync")),
            command(r#"{"version":1,"action":"pause"}"#),
        ] {
            if authenticated {
                request.headers_mut().insert(
                    "authorization",
                    format!("Bearer {}", f.f.token.expose().as_str())
                        .parse()
                        .unwrap(),
                );
            }
            assert_eq!(
                router.clone().oneshot(request).await.unwrap().status(),
                StatusCode::FORBIDDEN
            );
        }
    }
    let mut request = command(r#"{"version":1,"action":"pause"}"#);
    request
        .headers_mut()
        .insert("origin", "http://evil.example".parse().unwrap());
    assert_eq!(
        local.oneshot(request).await.unwrap().status(),
        StatusCode::FORBIDDEN
    );
    assert_eq!(controls.calls.load(Ordering::SeqCst), calls);
}

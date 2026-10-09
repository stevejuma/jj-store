use super::*;
use axum::{
    body::{Body, to_bytes},
    http::Request,
};
use tower::ServiceExt as _;
struct Manager {
    changes: tokio::sync::broadcast::Sender<u64>,
}
#[async_trait::async_trait]
impl agents::Manager for Manager {
    async fn inspect_workspace(
        &self,
        actor: m::PrincipalId,
        query: agents::workspace_inspection::Request,
    ) -> agents::Result<agents::workspace_inspection::Inspection> {
        assert_eq!(actor, m::PrincipalId::new([1; 16]));
        query.validate()?;
        Err(agents::Error::Stale)
    }
    async fn compute_status(&self, _: m::PrincipalId) -> agents::Result<agents::compute::Status> {
        Ok(agents::compute::Status {
            hardware: serde_json::json!({}),
            supported: true,
            model_file_picker: false,
            state: "off".into(),
            settings: Default::default(),
            models: vec![],
            runtime_prepared: false,
            invitation: None,
            identity: None,
            progress: None,
            error: None,
            operation: None,
        })
    }
    async fn catalog_preview(
        &self,
        actor: m::PrincipalId,
        preview: agents::catalog::Preview,
    ) -> agents::Result<agents::catalog::Catalog> {
        assert_eq!(actor, m::PrincipalId::new([1; 16]));
        if preview.version != 1 || preview.runner != "local" {
            return Err(agents::Error::Invalid);
        }
        Ok(agents::catalog::Catalog {
            runner: preview.runner,
            status: Some("ready".into()),
            facts: serde_json::json!({"models":{"availableModels":[{"modelId":"draft-model"}]}}),
        })
    }
    async fn catalog(
        &self,
        actor: m::PrincipalId,
        runner: String,
    ) -> agents::Result<agents::catalog::Catalog> {
        assert_eq!(actor, m::PrincipalId::new([1; 16]));
        if runner != "codex" {
            return Err(agents::Error::Invalid);
        }
        Ok(agents::catalog::Catalog {
            status: None,
            runner,
            facts: serde_json::json!({"models":{"availableModels":[{"modelId":"fixture-model","name":"Fixture model"}]}}),
        })
    }
    async fn runtime_status(
        &self,
        actor: m::PrincipalId,
    ) -> agents::Result<agents::tooling::Status> {
        assert_eq!(actor, m::PrincipalId::new([1; 16]));
        Ok(agents::tooling::Status {
            version: 1,
            runtimes: vec![],
            node_version: None,
            npm_available: false,
            install_supported: false,
            install_unavailable: Some("fixture".into()),
            pending: None,
        })
    }
    async fn install_runtime(
        &self,
        actor: m::PrincipalId,
        command: agents::tooling::Command,
    ) -> agents::Result<agents::tooling::Outcome> {
        assert_eq!(actor, m::PrincipalId::new([1; 16]));
        assert_eq!(command.runtime_id, "codex");
        Ok(agents::tooling::Outcome {
            version: 1,
            request_id: command.request_id,
            runtime_id: command.runtime_id,
            state: "installing".into(),
            error: None,
        })
    }
    async fn runtime_outcome(
        &self,
        actor: m::PrincipalId,
        request: m::RequestId,
    ) -> agents::Result<agents::tooling::Outcome> {
        assert_eq!(actor, m::PrincipalId::new([1; 16]));
        Ok(agents::tooling::Outcome {
            version: 1,
            request_id: request,
            runtime_id: "codex".into(),
            state: "completed".into(),
            error: None,
        })
    }
    async fn status(&self, actor: m::PrincipalId) -> agents::Result<agents::Status> {
        assert_eq!(actor, m::PrincipalId::new([1; 16]));
        Ok(agents::Status {
            version: 1,
            runtime: "ready".into(),
            revision: 0,
            setups: vec![],
            sessions: vec![],
            runners: vec![],
            live_agents: vec![],
            conversations: vec![],
        })
    }
    async fn targets(
        &self,
        _: m::PrincipalId,
        _: Option<m::RepositoryId>,
    ) -> agents::Result<Vec<agents::Target>> {
        Ok(vec![])
    }
    async fn save_setup(
        &self,
        _: m::PrincipalId,
        _: agents::SetupCommand,
    ) -> agents::Result<agents::Status> {
        Err(agents::Error::Invalid)
    }
    async fn launch(
        &self,
        _: m::PrincipalId,
        _: agents::Launch,
    ) -> agents::Result<agents::Outcome> {
        Err(agents::Error::Invalid)
    }
    async fn command(
        &self,
        _: m::PrincipalId,
        _: agents::Command,
    ) -> agents::Result<agents::Outcome> {
        Err(agents::Error::Invalid)
    }
    async fn outcome(&self, _: m::PrincipalId, _: m::RequestId) -> agents::Result<agents::Outcome> {
        Err(agents::Error::Invalid)
    }
    async fn events(
        &self,
        _: m::PrincipalId,
        _: String,
        _: u64,
    ) -> agents::Result<agents::EventPage> {
        Ok(agents::EventPage {
            version: 1,
            events: vec![],
            next: None,
        })
    }
    async fn feedback(
        &self,
        actor: m::PrincipalId,
        session: String,
    ) -> agents::Result<agents::Feedback> {
        assert_eq!(actor, m::PrincipalId::new([1; 16]));
        assert_eq!(session, "fixture");
        Ok(agents::Feedback {
            messages: vec![],
            profile: None,
            write_unavailable: None,
        })
    }
    fn subscribe(&self) -> tokio::sync::broadcast::Receiver<u64> {
        self.changes.subscribe()
    }
    async fn shutdown(&self) -> agents::Result<()> {
        Ok(())
    }
}
#[tokio::test]
async fn runtime_management_is_local_admin_only_with_fixed_typed_inputs_and_original_receipts() {
    let root = tempfile::tempdir().unwrap();
    let id = "88".repeat(16);
    let command =
        serde_json::json!({"version":1,"request_id":id,"runtime_id":"codex","action":"install"});
    for access in [
        Access::Authenticated,
        Access::LocalTrusted {
            principal: m::PrincipalId::new([1; 16]),
            permission: Permission::Read,
        },
    ] {
        let api = api(root.path(), access);
        for (path, body) in [
            ("/v1/local/agents/runtimes", None),
            ("/v1/local/agents/catalog?runner=codex", None),
            ("/v1/local/agents/runtimes/install", Some(command.clone())),
            (
                &format!("/v1/local/agents/runtimes/outcome?request={id}"),
                None,
            ),
        ] {
            assert_eq!(
                api.router()
                    .oneshot(request(path, body))
                    .await
                    .unwrap()
                    .status(),
                StatusCode::FORBIDDEN
            );
        }
    }
    let api = api(
        root.path(),
        Access::LocalTrusted {
            principal: m::PrincipalId::new([1; 16]),
            permission: Permission::Admin,
        },
    );
    assert_eq!(
        api.router()
            .oneshot(request("/v1/local/agents/runtimes", None))
            .await
            .unwrap()
            .status(),
        StatusCode::OK
    );
    let response = api
        .router()
        .oneshot(request(
            "/v1/local/agents/runtimes/install",
            Some(command.clone()),
        ))
        .await
        .unwrap();
    assert_eq!(response.status(), StatusCode::OK);
    let value: serde_json::Value =
        serde_json::from_slice(&to_bytes(response.into_body(), 4096).await.unwrap()).unwrap();
    assert_eq!(value["request_id"], id);
    assert_eq!(value["state"], "installing");
    for key in ["package", "path", "environment", "url", "actor"] {
        let mut injected = command.clone();
        injected[key] = serde_json::json!("untrusted");
        assert_eq!(
            api.router()
                .oneshot(request("/v1/local/agents/runtimes/install", Some(injected)))
                .await
                .unwrap()
                .status(),
            StatusCode::BAD_REQUEST
        );
    }
    assert_eq!(
        api.router()
            .oneshot(request("/v1/local/agents/runtimes?install=true", None))
            .await
            .unwrap()
            .status(),
        StatusCode::BAD_REQUEST
    );
    let response = api
        .router()
        .oneshot(request(
            &format!("/v1/local/agents/runtimes/outcome?request={id}"),
            None,
        ))
        .await
        .unwrap();
    assert_eq!(response.status(), StatusCode::OK);
    let value: serde_json::Value =
        serde_json::from_slice(&to_bytes(response.into_body(), 4096).await.unwrap()).unwrap();
    assert_eq!(value["request_id"], id);
    assert_eq!(value["state"], "completed");
}
#[tokio::test]
async fn feedback_is_read_only_local_admin_and_rejects_unbound_queries() {
    let root = tempfile::tempdir().unwrap();
    for access in [
        Access::Authenticated,
        Access::LocalTrusted {
            principal: m::PrincipalId::new([1; 16]),
            permission: Permission::Read,
        },
    ] {
        let response = api(root.path(), access)
            .router()
            .oneshot(request("/v1/local/agents/feedback?session=fixture", None))
            .await
            .unwrap();
        assert_eq!(response.status(), StatusCode::FORBIDDEN);
    }
    let api = api(
        root.path(),
        Access::LocalTrusted {
            principal: m::PrincipalId::new([1; 16]),
            permission: Permission::Admin,
        },
    );
    let response = api
        .router()
        .oneshot(request("/v1/local/agents/feedback?session=fixture", None))
        .await
        .unwrap();
    assert_eq!(response.status(), StatusCode::OK);
    let feedback: serde_json::Value =
        serde_json::from_slice(&to_bytes(response.into_body(), 4096).await.unwrap()).unwrap();
    assert_eq!(feedback, serde_json::json!({"messages":[]}));
    for path in [
        "/v1/local/agents/feedback?session=fixture&after=1",
        "/v1/local/agents/feedback?session=fixture&actor=caller",
    ] {
        assert_eq!(
            api.router()
                .oneshot(request(path, None))
                .await
                .unwrap()
                .status(),
            StatusCode::BAD_REQUEST
        );
    }
}
fn api(root: &std::path::Path, access: Access) -> HttpApi {
    api_with_picker(root, access, None)
}
#[tokio::test]
async fn workspace_inspection_is_local_admin_and_accepts_no_filesystem_root() {
    let root = tempfile::tempdir().unwrap();
    let payload = serde_json::json!({"version":1,"session":"11".repeat(16),"repository":"22".repeat(16),"query":{"view":"files"},"path":null,"cursor":null});
    for access in [
        Access::Authenticated,
        Access::LocalTrusted {
            principal: m::PrincipalId::new([1; 16]),
            permission: Permission::Read,
        },
    ] {
        assert_eq!(
            api(root.path(), access)
                .router()
                .oneshot(request(
                    "/v1/local/agents/workspace/inspect",
                    Some(payload.clone())
                ))
                .await
                .unwrap()
                .status(),
            StatusCode::FORBIDDEN
        );
    }
    let api = api(
        root.path(),
        Access::LocalTrusted {
            principal: m::PrincipalId::new([1; 16]),
            permission: Permission::Admin,
        },
    );
    assert_eq!(
        api.router()
            .oneshot(request(
                "/v1/local/agents/workspace/inspect",
                Some(payload.clone())
            ))
            .await
            .unwrap()
            .status(),
        StatusCode::CONFLICT
    );
    let mut bad = payload.clone();
    bad["root"] = "/tmp".into();
    assert_eq!(
        api.router()
            .oneshot(request("/v1/local/agents/workspace/inspect", Some(bad)))
            .await
            .unwrap()
            .status(),
        StatusCode::BAD_REQUEST
    );
    let mut bad = payload;
    bad["path"] = "../secret".into();
    assert_eq!(
        api.router()
            .oneshot(request("/v1/local/agents/workspace/inspect", Some(bad)))
            .await
            .unwrap()
            .status(),
        StatusCode::BAD_REQUEST
    );
}
fn api_with_picker(
    root: &std::path::Path,
    access: Access,
    picker: Option<Arc<dyn jj_store_service::local_management::FolderPicker>>,
) -> HttpApi {
    let (changes, _) = tokio::sync::broadcast::channel(32);
    HttpApi::with_search(
        None,
        root.into(),
        Config {
            access,
            local_agents: Some(Arc::new(Manager { changes })),
            local_folder_picker: picker,
            event_concurrency: 1,
            ..Default::default()
        },
        BTreeMap::new(),
    )
    .unwrap()
}
fn request(path: &str, body: Option<serde_json::Value>) -> Request<Body> {
    let mut request = Request::builder()
        .uri(path)
        .header("x-jj-store-protocol", "1")
        .header("host", "127.0.0.1:7443");
    let body = if let Some(body) = body {
        request = request
            .method("POST")
            .header("content-type", "application/json");
        Body::from(body.to_string())
    } else {
        Body::empty()
    };
    request.body(body).unwrap()
}
#[tokio::test]
async fn execution_routes_require_local_admin_and_refuse_browser_launch_configuration() {
    let root = tempfile::tempdir().unwrap();
    for access in [
        Access::Authenticated,
        Access::LocalTrusted {
            principal: m::PrincipalId::new([1; 16]),
            permission: Permission::Read,
        },
    ] {
        let response = api(root.path(), access)
            .router()
            .oneshot(request("/v1/local/agents", None))
            .await
            .unwrap();
        assert_eq!(response.status(), StatusCode::FORBIDDEN);
    }
    let api = api(
        root.path(),
        Access::LocalTrusted {
            principal: m::PrincipalId::new([1; 16]),
            permission: Permission::Admin,
        },
    );
    let response = api
        .router()
        .oneshot(request("/v1/local/agents", None))
        .await
        .unwrap();
    assert_eq!(response.status(), StatusCode::OK);
    let status: serde_json::Value =
        serde_json::from_slice(&to_bytes(response.into_body(), 4096).await.unwrap()).unwrap();
    assert_eq!(status["version"], 1);
    for extra in ["cwd", "executable", "environment", "actor"] {
        let mut body = serde_json::json!({"version":1,"request_id":"11".repeat(16),"target_id":"device-binding","repository":"22".repeat(16),"operation":"33".repeat(64),"commit":"44".repeat(20),"runner":"codex","setup_id":null,"context":null});
        body[extra] = serde_json::json!("untrusted");
        let response = api
            .router()
            .oneshot(request("/v1/local/agents/launch", Some(body)))
            .await
            .unwrap();
        assert_eq!(response.status(), StatusCode::BAD_REQUEST);
    }
    let stream = api
        .router()
        .oneshot(request("/v1/local/agents/stream?session=fixture", None))
        .await
        .unwrap();
    assert_eq!(stream.status(), StatusCode::OK);
    let saturated = api
        .router()
        .oneshot(request("/v1/local/agents/stream?session=fixture", None))
        .await
        .unwrap();
    assert_eq!(saturated.status(), StatusCode::SERVICE_UNAVAILABLE);
    drop(stream);
    let reopened = api
        .router()
        .oneshot(request("/v1/local/agents/stream?session=fixture", None))
        .await
        .unwrap();
    assert_eq!(reopened.status(), StatusCode::OK);
}

#[tokio::test]
async fn local_inbox_collaboration_routes_require_local_admin_without_actor_input() {
    let root = tempfile::tempdir().unwrap();
    let repository = "11".repeat(16);
    let command = serde_json::json!({"version":1,"request_id":"22".repeat(16),"command":{"domain":"thread","intent":{"type":"create","thread_id":"33".repeat(16),"title":"Reply","baseline":null}}});
    let paths = [
        (
            format!("/v1/local/repos/{repository}/collaboration/snapshot"),
            None,
        ),
        (
            format!("/v1/local/repos/{repository}/collaboration/query"),
            Some(
                serde_json::json!({"metadata_commit":null,"query":{"type":"threads","limit":50},"cursor":null}),
            ),
        ),
        (
            format!("/v1/local/repos/{repository}/collaboration/thread"),
            Some(serde_json::json!({"metadata_commit":null,"thread_id":"33".repeat(16)})),
        ),
        (
            format!("/v1/local/repos/{repository}/collaboration/commands"),
            Some(command.clone()),
        ),
        (
            format!(
                "/v1/local/repos/{repository}/collaboration/commands/{}",
                "22".repeat(16)
            ),
            None,
        ),
        (
            format!(
                "/v1/local/repos/{repository}/collaboration/assets/{}",
                "44".repeat(32)
            ),
            None,
        ),
    ];
    for access in [
        Access::Authenticated,
        Access::LocalTrusted {
            principal: m::PrincipalId::new([1; 16]),
            permission: Permission::Read,
        },
    ] {
        let api = api(root.path(), access);
        for (path, body) in &paths {
            assert_eq!(
                api.router()
                    .oneshot(request(path, body.clone()))
                    .await
                    .unwrap()
                    .status(),
                StatusCode::FORBIDDEN,
                "{path}"
            );
        }
    }
    let api = api(
        root.path(),
        Access::LocalTrusted {
            principal: m::PrincipalId::new([1; 16]),
            permission: Permission::Admin,
        },
    );
    let mut injected = command;
    injected["actor"] = serde_json::json!("55".repeat(16));
    assert_eq!(
        api.router()
            .oneshot(request(&paths[3].0, Some(injected)))
            .await
            .unwrap()
            .status(),
        StatusCode::BAD_REQUEST
    );
}

#[tokio::test]
async fn feedback_preparation_refusals_have_specific_transport_codes() {
    use axum::response::IntoResponse;
    for (error, expected_status, code) in [
        (
            agents::Error::FeedbackLimit,
            StatusCode::PAYLOAD_TOO_LARGE,
            "agent_feedback_limit",
        ),
        (
            agents::Error::FeedbackUnavailable,
            StatusCode::SERVICE_UNAVAILABLE,
            "agent_feedback_unavailable",
        ),
    ] {
        let response = failure(error).into_response();
        assert_eq!(response.status(), expected_status);
        let body = to_bytes(response.into_body(), 1024).await.unwrap();
        let value: serde_json::Value = serde_json::from_slice(&body).unwrap();
        assert_eq!(value["code"], code);
    }
}

#[tokio::test]
async fn model_catalog_is_typed_local_admin_discovery() {
    let root = tempfile::tempdir().unwrap();
    let api = api(
        root.path(),
        Access::LocalTrusted {
            principal: m::PrincipalId::new([1; 16]),
            permission: Permission::Admin,
        },
    );
    let response = api
        .router()
        .oneshot(request("/v1/local/agents/catalog?runner=codex", None))
        .await
        .unwrap();
    assert_eq!(response.status(), StatusCode::OK);
    let value: serde_json::Value =
        serde_json::from_slice(&to_bytes(response.into_body(), 4096).await.unwrap()).unwrap();
    assert_eq!(value["runner"], "codex");
    assert_eq!(
        value["facts"]["models"]["availableModels"][0]["modelId"],
        "fixture-model"
    );
    for path in [
        "/v1/local/agents/catalog?runner=other",
        "/v1/local/agents/catalog?runner=codex&cwd=/tmp",
        "/v1/local/agents/catalog?runner=codex&prompt=untrusted",
    ] {
        assert_eq!(
            api.router()
                .oneshot(request(path, None))
                .await
                .unwrap()
                .status(),
            StatusCode::BAD_REQUEST
        );
    }
}

struct ModelPicker(std::sync::atomic::AtomicUsize);
#[async_trait::async_trait]
impl jj_store_service::local_management::FolderPicker for ModelPicker {
    async fn pick(
        &self,
    ) -> std::result::Result<
        jj_store_service::local_management::FolderSelection,
        jj_store_service::local_management::PickerError,
    > {
        panic!("model selection must not open the folder dialog")
    }
    fn model_files_supported(&self) -> bool {
        true
    }
    async fn pick_model_file(
        &self,
    ) -> std::result::Result<
        jj_store_service::local_management::ModelFileSelection,
        jj_store_service::local_management::PickerError,
    > {
        use jj_store_service::local_management::{ModelFileSelection, PickerError};
        match self.0.fetch_add(1, std::sync::atomic::Ordering::SeqCst) {
            0 => Ok(ModelFileSelection {
                version: 1,
                path: Some("/models/My weights.gguf".into()),
            }),
            1 => Ok(ModelFileSelection {
                version: 1,
                path: None,
            }),
            2 => Err(PickerError::Busy),
            3 => Err(PickerError::Timeout),
            4 => Err(PickerError::Invalid),
            _ => Err(PickerError::Unavailable),
        }
    }
}
#[tokio::test]
async fn model_file_picker_requires_local_admin_and_fixed_inputs_without_importing() {
    let root = tempfile::tempdir().unwrap();
    let picker = Arc::new(ModelPicker(std::sync::atomic::AtomicUsize::new(0)));
    let admin = Access::LocalTrusted {
        principal: m::PrincipalId::new([1; 16]),
        permission: Permission::Admin,
    };
    for access in [
        Access::Authenticated,
        Access::LocalTrusted {
            principal: m::PrincipalId::new([1; 16]),
            permission: Permission::Read,
        },
    ] {
        let response = api_with_picker(root.path(), access, Some(picker.clone()))
            .router()
            .oneshot(request(
                "/v1/local/compute/pick",
                Some(serde_json::json!({})),
            ))
            .await
            .unwrap();
        assert_eq!(response.status(), StatusCode::FORBIDDEN);
    }
    let app = api_with_picker(root.path(), admin.clone(), Some(picker.clone()));
    for path in [
        "/v1/local/compute/pick?filter=anything",
        "/v1/local/compute/pick",
    ] {
        let body = if path.contains('?') {
            serde_json::json!({})
        } else {
            serde_json::json!({"script":"untrusted"})
        };
        let response = app
            .router()
            .oneshot(request(path, Some(body)))
            .await
            .unwrap();
        assert_eq!(response.status(), StatusCode::BAD_REQUEST);
    }
    assert_eq!(picker.0.load(std::sync::atomic::Ordering::SeqCst), 0);
    let response = app
        .router()
        .oneshot(request("/v1/local/compute", None))
        .await
        .unwrap();
    let status: serde_json::Value =
        serde_json::from_slice(&to_bytes(response.into_body(), 4096).await.unwrap()).unwrap();
    assert_eq!(status["model_file_picker"], true);
    for (expected, code) in [
        (StatusCode::OK, None),
        (StatusCode::OK, None),
        (StatusCode::CONFLICT, Some("model_file_picker_busy")),
        (
            StatusCode::REQUEST_TIMEOUT,
            Some("model_file_picker_timeout"),
        ),
        (
            StatusCode::BAD_GATEWAY,
            Some("model_file_picker_invalid_output"),
        ),
        (
            StatusCode::SERVICE_UNAVAILABLE,
            Some("model_file_picker_unavailable"),
        ),
    ] {
        let response = app
            .router()
            .oneshot(request(
                "/v1/local/compute/pick",
                Some(serde_json::json!({})),
            ))
            .await
            .unwrap();
        assert_eq!(response.status(), expected);
        let value: serde_json::Value =
            serde_json::from_slice(&to_bytes(response.into_body(), 4096).await.unwrap()).unwrap();
        if let Some(code) = code {
            assert_eq!(value["code"], code);
        } else {
            assert_eq!(value["version"], 1);
            assert!(value["path"].is_null() || value["path"] == "/models/My weights.gguf");
        }
    }
    let unavailable = api(root.path(), admin)
        .router()
        .oneshot(request(
            "/v1/local/compute/pick",
            Some(serde_json::json!({})),
        ))
        .await
        .unwrap();
    assert_eq!(unavailable.status(), StatusCode::SERVICE_UNAVAILABLE);
}

#[tokio::test]
async fn draft_catalog_is_local_admin_only_and_does_not_echo_secrets() {
    let root = tempfile::tempdir().unwrap();
    let body = serde_json::json!({"version":1,"runner":"local","preferences":{},"options":{"provider":"anthropic"},"secrets":{"fixture-reference":"transient-secret"}});
    let admin = api(
        root.path(),
        Access::LocalTrusted {
            principal: m::PrincipalId::new([1; 16]),
            permission: Permission::Admin,
        },
    );
    let response = admin
        .router()
        .oneshot(request("/v1/local/agents/catalog", Some(body.clone())))
        .await
        .unwrap();
    assert_eq!(response.status(), StatusCode::OK);
    let bytes = to_bytes(response.into_body(), 4096).await.unwrap();
    assert!(!String::from_utf8_lossy(&bytes).contains("transient-secret"));
    let mut invalid = body.clone();
    invalid["prompt"] = serde_json::json!("never execute");
    assert_eq!(
        admin
            .router()
            .oneshot(request("/v1/local/agents/catalog", Some(invalid)))
            .await
            .unwrap()
            .status(),
        StatusCode::BAD_REQUEST
    );
    let reader = api(
        root.path(),
        Access::LocalTrusted {
            principal: m::PrincipalId::new([1; 16]),
            permission: Permission::Read,
        },
    );
    assert_eq!(
        reader
            .router()
            .oneshot(request("/v1/local/agents/catalog", Some(body)))
            .await
            .unwrap()
            .status(),
        StatusCode::FORBIDDEN
    );
}

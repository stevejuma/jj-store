use axum::{
    body::{Body, to_bytes},
    http::{Request, StatusCode},
};
use jj_store_http::{Access, Config, HttpApi};
use jj_store_model::{PrincipalId, RepositoryId};
use jj_store_service::auth::Permission;
use tower::ServiceExt as _;
#[tokio::test]
async fn forge_endpoint_rejects_foreign_origin_before_credentials_or_device_state() {
    let root = tempfile::tempdir().unwrap();
    let api = HttpApi::with_search(
        None,
        root.path().join("cache"),
        Config {
            access: Access::LocalTrusted {
                principal: PrincipalId::new([1; 16]),
                permission: Permission::Admin,
            },
            ..Default::default()
        },
        Default::default(),
    )
    .unwrap();
    let token = "fixture-secret-never-echo";
    let request = Request::builder()
        .method("POST")
        .uri(format!(
            "/v1/local/repos/{}/forge",
            RepositoryId::new([2; 16])
        ))
        .header("host", "127.0.0.1:7443")
        .header("origin", "https://foreign.example")
        .header("content-type", "application/json")
        .header("x-jj-store-protocol", "1")
        .body(Body::from(
            serde_json::to_vec(
                &serde_json::json!({"version":1,"token":token,"action":{"kind":"status"}}),
            )
            .unwrap(),
        ))
        .unwrap();
    let response = api.router().oneshot(request).await.unwrap();
    assert_eq!(response.status(), StatusCode::FORBIDDEN);
    let bytes = to_bytes(response.into_body(), 8192).await.unwrap();
    assert!(!String::from_utf8_lossy(&bytes).contains(token));
}

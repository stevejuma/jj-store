use axum::{
    body::Body,
    http::{Request, StatusCode},
};
use jj_store_http::{Access, Config, HttpApi};
use jj_store_model::{PrincipalId, RepositoryId, RequestId};
use jj_store_service::{auth::Permission, repository_deletion::*};
use std::sync::{
    Arc,
    atomic::{AtomicUsize, Ordering},
};
use tower::ServiceExt as _;
struct Deletion(AtomicUsize);
#[async_trait::async_trait]
impl RepositoryDeletion for Deletion {
    async fn delete(
        &self,
        repo: RepositoryId,
        request: DeleteRepository,
    ) -> Result<DeletionOutcome, Error> {
        self.0.fetch_add(1, Ordering::SeqCst);
        Ok(DeletionOutcome {
            version: 1,
            repository: repo,
            request_id: request.request_id,
            state: DeletionState::Pending,
        })
    }
    async fn status(
        &self,
        repo: RepositoryId,
        request: RequestId,
    ) -> Result<DeletionOutcome, Error> {
        Ok(DeletionOutcome {
            version: 1,
            repository: repo,
            request_id: request,
            state: DeletionState::Pending,
        })
    }
}
#[tokio::test]
async fn deletion_requires_local_admin_origin_protocol_and_an_exact_strict_body() {
    for (permission, origin, protocol, extra, expected) in [
        (
            Permission::Read,
            "http://127.0.0.1:7443",
            "1",
            false,
            StatusCode::FORBIDDEN,
        ),
        (
            Permission::Admin,
            "https://foreign.example",
            "1",
            false,
            StatusCode::FORBIDDEN,
        ),
        (
            Permission::Admin,
            "http://127.0.0.1:7443",
            "99",
            false,
            StatusCode::FORBIDDEN,
        ),
        (
            Permission::Admin,
            "http://127.0.0.1:7443",
            "1",
            true,
            StatusCode::BAD_REQUEST,
        ),
        (
            Permission::Admin,
            "http://127.0.0.1:7443",
            "1",
            false,
            StatusCode::OK,
        ),
    ] {
        let root = tempfile::tempdir().unwrap();
        let manager = Arc::new(Deletion(AtomicUsize::new(0)));
        let api = HttpApi::with_search(
            None,
            root.path().join("cache"),
            Config {
                access: Access::LocalTrusted {
                    principal: PrincipalId::new([1; 16]),
                    permission,
                },
                repository_deletion: Some(manager.clone()),
                ..Default::default()
            },
            Default::default(),
        )
        .unwrap();
        let mut body = serde_json::json!({"version":1,"request_id":"dd".repeat(16),"expected_epoch":"cc".repeat(16),"expected_installation":"aa".repeat(16)});
        if extra {
            body["erase_everything"] = serde_json::json!(true);
        }
        let request = Request::builder()
            .method("POST")
            .uri(format!(
                "/v1/local/repositories/{}/delete",
                RepositoryId::new([2; 16])
            ))
            .header("host", "127.0.0.1:7443")
            .header("origin", origin)
            .header("content-type", "application/json")
            .header("x-jj-store-protocol", protocol)
            .body(Body::from(body.to_string()))
            .unwrap();
        let response = api.router().oneshot(request).await.unwrap();
        assert_eq!(response.status(), expected);
        assert_eq!(
            manager.0.load(Ordering::SeqCst),
            usize::from(expected == StatusCode::OK)
        );
    }
}

use super::*;
use jj_store_service::registry::{RegistrySnapshot, RepositoryEntry, RepositoryRegistry};
struct Registry;
struct CheckpointOnly;
#[async_trait::async_trait]
impl PublicationGuard for CheckpointOnly {
    async fn authorize(&self, _: &ManifestSnapshot, _: &Request) -> wal::Result<()> {
        Err(wal::Error::Denied)
    }
    async fn validate_transition(
        &self,
        _: &ManifestSnapshot,
        _: &Request,
        _: &Snapshot,
    ) -> wal::Result<()> {
        Err(wal::Error::Denied)
    }
}
#[async_trait::async_trait]
impl RepositoryRegistry for Registry {
    async fn snapshot(
        &self,
        _: Option<m::TokenId>,
    ) -> std::result::Result<RegistrySnapshot, jj_store_service::registry::Error> {
        Ok(RegistrySnapshot {
            generation: 1,
            repositories: vec![RepositoryEntry {
                id: REPO,
                name: "Events".into(),
            }],
        })
    }
    async fn reserve(
        &self,
        _: m::RepositoryId,
        _: String,
    ) -> std::result::Result<(), jj_store_service::registry::Error> {
        unreachable!()
    }
    async fn register(
        &self,
        _: m::RepositoryId,
        _: String,
        _: Option<m::TokenId>,
    ) -> std::result::Result<(), jj_store_service::registry::Error> {
        unreachable!()
    }
    async fn token_candidate(
        &self,
        _: m::RepositoryId,
        _: m::TokenId,
        _: bool,
    ) -> std::result::Result<(), jj_store_service::registry::Error> {
        unreachable!()
    }
}
fn application(f: &Fixture) -> HttpRequest<Body> {
    let mut request = req(f, "events");
    *request.uri_mut() = "/v1/events".parse().unwrap();
    request
}
#[tokio::test(flavor = "multi_thread", worker_threads = 2)]
async fn application_events_share_committed_observation_keep_checkpoint_narrow_and_reauthorize_delivery()
 {
    let mut f = Fixture::new().await;
    let cache = tempfile::tempdir().unwrap();
    let app = router(
        &f,
        cache.path(),
        Config {
            registry: Some(Arc::new(Registry)),
            ..Default::default()
        },
    );
    let response = app.clone().oneshot(application(&f)).await.unwrap();
    assert_eq!(response.status(), StatusCode::OK);
    let mut stream = response.into_body().into_data_stream();
    let ready = chunk(&mut stream).await;
    assert!(ready.contains("ready"), "{ready}");
    let response = app.clone().oneshot(application(&f)).await.unwrap();
    assert_eq!(response.status(), StatusCode::OK);
    let mut second = response.into_body().into_data_stream();
    chunk(&mut second).await;
    // This session publishes outside this HTTP host; the shared committed metadata probe detects it.
    publish(&f, 120).await;
    let event = tokio::time::timeout(Duration::from_secs(5), stream.next())
        .await
        .unwrap()
        .unwrap()
        .unwrap();
    let event = String::from_utf8(event.to_vec()).unwrap();
    assert!(event.contains("\"domain\":\"storage\""), "{event}");
    assert!(chunk(&mut second).await.contains("\"domain\":\"storage\""));
    // A selected publication also advances the origin ledger. Consume that distinct sync notice.
    assert!(chunk(&mut stream).await.contains("\"domain\":\"sync\""));
    assert!(chunk(&mut second).await.contains("\"domain\":\"sync\""));
    Coordinator::new(
        f.store.clone(),
        REPO,
        f.objects.clone(),
        Arc::new(CheckpointOnly),
        Limits::default(),
    )
    .unwrap()
    .checkpoint()
    .await
    .unwrap();
    let checkpoint = tokio::time::timeout(Duration::from_secs(5), stream.next())
        .await
        .unwrap()
        .unwrap()
        .unwrap();
    let checkpoint = String::from_utf8(checkpoint.to_vec()).unwrap();
    assert!(
        checkpoint.contains("\"domain\":\"storage\""),
        "{checkpoint}"
    );
    assert!(!checkpoint.contains("\"domain\":\"code\""));
    f.policy.revoke(f.reader.id());
    publish(&f, 121).await;
    let denied = tokio::time::timeout(Duration::from_secs(5), stream.next())
        .await
        .unwrap()
        .unwrap();
    assert!(denied.is_err());
    assert_eq!(
        app.oneshot(application(&f)).await.unwrap().status(),
        StatusCode::UNAUTHORIZED
    );
}

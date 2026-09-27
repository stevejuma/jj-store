use super::*;
use axum::{
    Router,
    body::Body,
    http::{Request as HttpRequest, StatusCode},
};
use jj_store_http::{Config, HttpApi};
use tower::ServiceExt as _;
fn router(f: &Fixture, cache: &std::path::Path, config: Config) -> Router {
    HttpApi::new(host(f), cache.to_owned(), config)
        .unwrap()
        .router()
}
fn req(f: &Fixture, path: &str) -> HttpRequest<Body> {
    HttpRequest::builder()
        .uri(format!("/v1/repos/{REPO}/{path}"))
        .header(
            "authorization",
            format!("Bearer {}", f.reader.expose().as_str()),
        )
        .body(Body::empty())
        .unwrap()
}
async fn chunk(stream: &mut axum::body::BodyDataStream) -> String {
    let bytes = tokio::time::timeout(Duration::from_secs(2), stream.next())
        .await
        .unwrap()
        .unwrap()
        .unwrap();
    String::from_utf8(bytes.to_vec()).unwrap()
}
#[tokio::test(flavor = "multi_thread", worker_threads = 2)]
async fn http_events_resume_restart_and_reset_use_selected_publications() {
    let f = Fixture::new().await;
    let cache = tempfile::tempdir().unwrap();
    let app = router(&f, cache.path(), Config::default());
    let response = app.clone().oneshot(req(&f, "events")).await.unwrap();
    assert_eq!(response.status(), StatusCode::OK);
    assert_eq!(response.headers()["content-type"], "text/event-stream");
    let mut stream = response.into_body().into_data_stream();
    let ready = chunk(&mut stream).await;
    assert!(ready.contains("event: ready\n"), "{ready}");
    let id = ready
        .lines()
        .find_map(|line| line.strip_prefix("id: "))
        .unwrap()
        .to_owned();
    publish(&f, 102).await;
    let event = chunk(&mut stream).await;
    assert!(event.contains("event: repository-changed\n"), "{event}");
    assert!(event.contains("\"position\":1"));
    drop(stream);
    drop(app);
    let cold = router(&f, cache.path(), Config::default());
    let mut request = req(&f, "events");
    request
        .headers_mut()
        .insert("last-event-id", id.parse().unwrap());
    let mut resumed = cold
        .oneshot(request)
        .await
        .unwrap()
        .into_body()
        .into_data_stream();
    assert!(chunk(&mut resumed).await.contains("event: ready\n"));
    assert!(
        chunk(&mut resumed)
            .await
            .contains("event: repository-changed\n")
    );
}

#[tokio::test(flavor = "multi_thread", worker_threads = 2)]
async fn http_events_have_separate_admission_and_expire_unpolled_streams() {
    let f = Fixture::new().await;
    let cache = tempfile::tempdir().unwrap();
    let app = router(
        &f,
        cache.path(),
        Config {
            concurrency: 1,
            event_concurrency: 1,
            timeout: Duration::from_millis(120),
            ..Default::default()
        },
    );
    let held = app.clone().oneshot(req(&f, "events")).await.unwrap();
    assert_eq!(held.status(), StatusCode::OK);
    assert_eq!(
        app.clone()
            .oneshot(req(&f, "snapshot"))
            .await
            .unwrap()
            .status(),
        StatusCode::OK
    );
    assert_eq!(
        app.clone()
            .oneshot(req(&f, "events"))
            .await
            .unwrap()
            .status(),
        StatusCode::SERVICE_UNAVAILABLE
    );
    tokio::time::sleep(Duration::from_millis(160)).await;
    assert_eq!(
        app.clone()
            .oneshot(req(&f, "events"))
            .await
            .unwrap()
            .status(),
        StatusCode::OK
    );
    assert!(
        held.into_body()
            .into_data_stream()
            .next()
            .await
            .unwrap()
            .is_err()
    );
}
#[tokio::test(flavor = "multi_thread", worker_threads = 2)]
async fn http_events_validate_resume_headers_and_reset_without_replaying_ahead() {
    let f = Fixture::new().await;
    let cache = tempfile::tempdir().unwrap();
    let app = router(&f, cache.path(), Config::default());
    let zero = Cursor {
        repository: REPO,
        epoch: m::EpochId::new([4; 16]),
        position: m::WalPosition(0),
    };
    let future = Cursor {
        position: m::WalPosition(4),
        ..zero
    };
    let foreign = Cursor {
        repository: m::RepositoryId::new([5; 16]),
        ..zero
    };
    for path in [
        "events?token=secret".to_owned(),
        "events?after=invalid".to_owned(),
        format!("events?after={foreign}"),
    ] {
        assert_eq!(
            app.clone().oneshot(req(&f, &path)).await.unwrap().status(),
            StatusCode::BAD_REQUEST
        );
    }
    let mut mismatch = req(&f, &format!("events?after={zero}"));
    mismatch
        .headers_mut()
        .insert("last-event-id", future.to_string().parse().unwrap());
    assert_eq!(
        app.clone().oneshot(mismatch).await.unwrap().status(),
        StatusCode::BAD_REQUEST
    );
    let mut repeated = req(&f, "events");
    for _ in 0..2 {
        repeated
            .headers_mut()
            .append("last-event-id", zero.to_string().parse().unwrap());
    }
    assert_eq!(
        app.clone().oneshot(repeated).await.unwrap().status(),
        StatusCode::BAD_REQUEST
    );
    let mut no_auth = req(&f, "events");
    no_auth.headers_mut().remove("authorization");
    assert_eq!(
        app.clone().oneshot(no_auth).await.unwrap().status(),
        StatusCode::UNAUTHORIZED
    );
    let mut reset = app
        .oneshot(req(&f, &format!("events?after={future}")))
        .await
        .unwrap()
        .into_body()
        .into_data_stream();
    let event = chunk(&mut reset).await;
    assert!(event.contains("event: reset\n"), "{event}");
    assert!(event.contains("\"reason\":\"future_position\""));
    assert!(event.contains(&format!("id: {zero}\n")));
    assert!(reset.next().await.is_none());
}
#[tokio::test(flavor = "multi_thread", worker_threads = 2)]
async fn http_events_revocation_terminates_queued_delivery() {
    let mut f = Fixture::new().await;
    let cache = tempfile::tempdir().unwrap();
    let app = router(
        &f,
        cache.path(),
        Config {
            events: limits(),
            ..Default::default()
        },
    );
    let mut stream = app
        .clone()
        .oneshot(req(&f, "events"))
        .await
        .unwrap()
        .into_body()
        .into_data_stream();
    chunk(&mut stream).await;
    for number in 110..=112 {
        publish(&f, number).await;
    }
    assert!(chunk(&mut stream).await.contains("\"position\":1"));
    f.policy.revoke(f.reader.id());
    publish(&f, 113).await;
    let error = stream.next().await.unwrap().unwrap_err();
    assert!(!error.to_string().contains(f.reader.expose().as_str()));
    assert_eq!(
        app.oneshot(req(&f, "events")).await.unwrap().status(),
        StatusCode::FORBIDDEN
    );
}

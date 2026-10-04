use jj_store_server::{Error, opencode_read_api::OpenCodeReadApi};
#[path = "opencode_read_api/fixture.rs"]
mod fixture;
use fixture::{Fixture, Mode};

#[tokio::test]
async fn metadata_discovery_selects_only_explicit_busy_or_retry_sessions() {
    let fixture = Fixture::start(Mode::Healthy).await;
    let discovery = fixture.api.discover_active().await.unwrap();
    assert_eq!(discovery.observed, 4);
    assert_eq!(discovery.unsupported, 2);
    assert_eq!(discovery.candidates.len(), 1);
    assert_eq!(discovery.candidates[0].native_session_key(), "ses_one");
    assert_eq!(
        fixture.requests(),
        vec!["/global/health", "/session/status"]
    );
}

#[tokio::test]
async fn acquisition_uses_only_context_bound_gets_and_opaque_message_cursors() {
    let fixture = Fixture::start(Mode::Healthy).await;
    let observed = fixture.api.observe("ses_one").await.unwrap();
    assert!(observed.activity().active());
    let mut pages = fixture.api.messages(observed.session());
    let first = pages.next_page().await.unwrap().unwrap();
    assert_eq!(first.records.len(), 1);
    let second = pages.next_page().await.unwrap().unwrap();
    assert_eq!(
        second.records[0].envelope()["record"]["info"]["id"],
        "msg_old"
    );
    let count = fixture.requests().len();
    assert!(pages.next_page().await.unwrap().is_none());
    assert_eq!(fixture.requests().len(), count);
    assert!(
        fixture
            .requests()
            .iter()
            .any(|r| r.contains("before=opaque%3Anext%2Fwith%3Fquery%3Dvalue"))
    );
    assert!(fixture.requests().iter().all(|r| !r.contains("forbidden")));
}
#[tokio::test]
async fn changed_version_redirect_and_streamed_body_overflow_withhold_acquisition() {
    for mode in [Mode::WrongVersion, Mode::Redirect, Mode::Oversized] {
        let fixture = Fixture::start(mode).await;
        let error = fixture.api.observe("ses_one").await.err().unwrap();
        assert!(matches!(error, Error::Service(_)));
        assert_eq!(fixture.requests(), vec!["/global/health"]);
    }
}
#[tokio::test]
async fn changed_directory_and_repeated_cursor_preserve_the_acquisition_boundary() {
    let rebound = Fixture::start(Mode::Rebound).await;
    assert!(matches!(
        rebound.api.observe("ses_one").await,
        Err(Error::Service("OpenCode session binding changed"))
    ));
    let fixture = Fixture::start(Mode::Cycle).await;
    let observed = fixture.api.observe("ses_one").await.unwrap();
    let mut pages = fixture.api.messages(observed.session());
    assert!(pages.next_page().await.unwrap().is_some());
    assert!(matches!(
        pages.next_page().await,
        Err(Error::Service("OpenCode repeated message cursor"))
    ));
    let first = fixture
        .requests()
        .iter()
        .rfind(|p| p.contains("/message?"))
        .unwrap()
        .clone();
    assert!(pages.next_page().await.is_err());
    let second = fixture
        .requests()
        .iter()
        .rfind(|p| p.contains("/message?"))
        .unwrap()
        .clone();
    assert_eq!(first, second);
}
#[tokio::test]
async fn invalid_endpoint_or_native_session_is_refused_before_native_reads() {
    for url in [
        "http://example.com/",
        "http://localhost:8080/",
        "http://u:p@127.0.0.1/",
        "http://127.0.0.1/?token=test",
        "http://127.0.0.1/subpath",
    ] {
        assert!(OpenCodeReadApi::new(url).is_err());
    }
    let fixture = Fixture::start(Mode::Healthy).await;
    assert!(fixture.api.observe("ses_../../different").await.is_err());
    assert!(fixture.requests().is_empty());
}

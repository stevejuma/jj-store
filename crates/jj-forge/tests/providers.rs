use jj_forge::{client::Transport, *};
use serde_json::{Value, json};
use std::sync::{Arc, Mutex};
struct Fixture {
    calls: Mutex<Vec<(String, String, Option<Value>)>>,
    stack: Mutex<Vec<u64>>,
}
#[async_trait::async_trait(?Send)]
impl Transport for Fixture {
    async fn request(&self, method: &str, path: &str, body: Option<Value>) -> Result<Value> {
        self.calls
            .lock()
            .unwrap()
            .push((method.into(), path.into(), body.clone()));
        if path.contains("/actions/runs?")
            || path.contains("/check-runs?")
            || path.contains("/statuses?")
        {
            return Ok(json!([]));
        }
        if path.ends_with("/merge-async") {
            assert_eq!(method, "PUT");
            let body = body.unwrap();
            assert_eq!(body["sha"], "a".repeat(40));
            assert_eq!(body["bypass_rules"], false);
            assert_eq!(body["merge_action"], "default");
            return Ok(json!({"status": "pending", "details": {"uuid": "queue-1"}}));
        }
        if path.ends_with("/merge-async/queue-1") {
            return Ok(json!({"status": "enqueued", "details": {}}));
        }
        if path == "projects/contributor%2Frepo" {
            return Ok(json!({"id": 11}));
        }
        if path == "projects/team%2Frepo" {
            return Ok(json!({"id": 22}));
        }
        if path.contains("/stacks?") {
            return Ok(
                json!([{"number": 9, "pull_requests": self.stack.lock().unwrap().iter().map(|n| json!({"number": n})).collect::<Vec<_>>() }]),
            );
        }
        if path == "projects/contributor%2Frepo/merge_requests" {
            let b = body.unwrap();
            return Ok(
                json!({"iid": 3, "web_url": "https://gitlab.example/team/repo/-/merge_requests/3", "title": b["title"], "description": b["description"], "state": "opened", "source_branch": b["source_branch"], "target_branch": b["target_branch"], "sha": "a".repeat(40), "draft": true}),
            );
        }
        if path == "repos/team/repo/pulls/3" {
            let title = body
                .as_ref()
                .and_then(|b| b["title"].as_str())
                .unwrap_or("Ready");
            return Ok(
                json!({"number": 3, "html_url": "https://forge.example/team/repo/pulls/3", "title": title, "state": "open", "head": {"ref": "feature", "sha": "a".repeat(40)}, "base": {"ref": "main"}, "draft": title.starts_with("WIP: ")}),
            );
        }
        panic!("Unexpected {method} {path}")
    }
}
fn connection(provider: Provider) -> Connection {
    Connection {
        provider,
        api_url: "https://forge.example/api/v1".into(),
        repository: "team/repo".into(),
        source_repository: "team/repo".into(),
        remote: "origin".into(),
        target_remote: "origin".into(),
        target: "main".into(),
        native_stacks: true,
    }
}
fn fixture() -> Arc<Fixture> {
    Arc::new(Fixture {
        calls: Mutex::new(vec![]),
        stack: Mutex::new(vec![1, 2]),
    })
}
fn pr(number: u64) -> PullRequest {
    PullRequest {
        number,
        url: format!("https://github.com/team/repo/pull/{number}"),
        title: "Feature".into(),
        body: "".into(),
        head: format!("feature-{number}"),
        sha: "a".repeat(40),
        base: "main".into(),
        draft: false,
        state: PrState::Open,
        mergeable: Some(true),
        native_stack: Some(9),
        updated_at: "".into(),
    }
}
#[tokio::test]
async fn native_stack_requires_exact_order_and_no_extra_members() {
    let fixture = fixture();
    let client = Client::with_transport(connection(Provider::Github), fixture.clone()).unwrap();
    assert!(client.native_stack_matches(&[pr(1), pr(2)]).await.unwrap());
    *fixture.stack.lock().unwrap() = vec![2, 1];
    assert!(!client.native_stack_matches(&[pr(1), pr(2)]).await.unwrap());
    *fixture.stack.lock().unwrap() = vec![1, 2, 3];
    assert!(!client.native_stack_matches(&[pr(1), pr(2)]).await.unwrap());
    assert!(matches!(
        client.native_stack(&[pr(1), pr(2)]).await,
        Err(Error::Stale)
    ));
    assert!(
        fixture
            .calls
            .lock()
            .unwrap()
            .iter()
            .all(|(method, _, _)| method == "GET")
    );
}
#[tokio::test]
async fn gitlab_fork_creation_posts_to_source_with_explicit_target_project() {
    let fixture = fixture();
    let mut c = connection(Provider::Gitlab);
    c.source_repository = "contributor/repo".into();
    let client = Client::with_transport(c, fixture.clone()).unwrap();
    let result = client
        .create(
            &Segment {
                branch: "feature".into(),
                head: "a".repeat(40),
                base: "main".into(),
                title: "Feature".into(),
                body: "Description".into(),
                remote_head: None,
                shared: false,
            },
            true,
        )
        .await
        .unwrap();
    assert_eq!(result.number, 3);
    let calls = fixture.calls.lock().unwrap();
    let (_, path, payload) = calls.last().unwrap();
    assert_eq!(path, "projects/contributor%2Frepo/merge_requests");
    assert_eq!(payload.as_ref().unwrap()["target_project_id"], 22);
    assert_eq!(payload.as_ref().unwrap()["title"], "Draft: Feature");
}
#[tokio::test]
async fn forgejo_draft_uses_title_prefix_without_unsupported_draft_field() {
    let fixture = fixture();
    let client = Client::with_transport(connection(Provider::Forgejo), fixture.clone()).unwrap();
    assert!(client.draft(3, true).await.unwrap().draft);
    let calls = fixture.calls.lock().unwrap();
    let (_, _, payload) = calls.last().unwrap();
    assert_eq!(payload.as_ref().unwrap()["title"], "WIP: Ready");
    assert!(payload.as_ref().unwrap().get("draft").is_none());
    assert!(!client.capabilities().rerun);
}

#[tokio::test]
async fn github_async_merge_retains_receipt_and_never_calls_enqueued_merged() {
    let client = Client::with_transport(connection(Provider::Github), fixture()).unwrap();
    let result = client.merge(&pr(1), "squash").await.unwrap();
    assert_eq!(result.state, "pending");
    assert_eq!(result.receipt.as_deref(), Some("queue-1"));
    let result = client.merge_status(1, "queue-1").await.unwrap();
    assert_eq!(result.state, "enqueued");
    assert_eq!(result.receipt.as_deref(), Some("queue-1"));
}

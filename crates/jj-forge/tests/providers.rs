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

struct TimingFixture;
#[async_trait::async_trait(?Send)]
impl Transport for TimingFixture {
    async fn request(&self, method: &str, path: &str, _: Option<Value>) -> Result<Value> {
        assert_eq!(method, "GET");
        let timing = json!({"id": 7, "name":"lint", "head_sha":"a".repeat(40), "status":"completed", "conclusion":"success", "started_at":"2026-10-15T10:00:00Z", "completed_at":"2026-10-15T10:00:04Z", "updated_at":"2026-10-15T11:00:00Z"});
        if path.contains("/check-runs?") {
            return Ok(json!([timing]));
        }
        if path.ends_with("/jobs?") || path.contains("/jobs?") {
            return Ok(json!([timing]));
        }
        if path.contains("/actions/runs?") {
            return Ok(
                json!([{"id":8,"name":"workflow", "head_sha":"a".repeat(40),"run_started_at":"2026-10-15T10:00:00Z","updated_at":"2026-10-15T11:00:00Z"}]),
            );
        }
        if path.contains("/actions/runs/8") {
            return Ok(json!({"head_sha":"a".repeat(40)}));
        }
        if path.contains("/statuses?") {
            return Ok(json!([]));
        }
        panic!("Unexpected {path}");
    }
}
#[tokio::test]
async fn github_check_and_job_timing_preserves_completion_without_inventing_run_duration() {
    let client =
        Client::with_transport(connection(Provider::Github), Arc::new(TimingFixture)).unwrap();
    let checks = client.checks(&pr(3)).await.unwrap();
    let run = checks.checks.iter().find(|c| c.id == "8").unwrap();
    assert_eq!(run.started_at.as_deref(), Some("2026-10-15T10:00:00Z"));
    assert_eq!(run.completed_at, None);
    let check = checks.checks.iter().find(|c| c.id == "check:7").unwrap();
    assert_eq!(check.completed_at.as_deref(), Some("2026-10-15T10:00:04Z"));
    let jobs = client.jobs(&pr(3), "8").await.unwrap();
    assert_eq!(jobs.checks[0].started_at, check.started_at);
    assert_eq!(jobs.checks[0].completed_at, check.completed_at);
    let old: Check = serde_json::from_value(json!({"id":"old","name":"old","state":"passed","sha":"a","url":null,"run_id":null,"log_id":null})).unwrap();
    assert!(old.started_at.is_none() && old.completed_at.is_none());
}

struct LatestChecksFixture;
#[async_trait::async_trait(?Send)]
impl Transport for LatestChecksFixture {
    async fn request(&self, method: &str, path: &str, _: Option<Value>) -> Result<Value> {
        assert_eq!(method, "GET");
        if path.contains("/actions/runs?") {
            return Ok(json!([
                {"id":900,"workflow_id":10,"check_suite_id":200,"run_number":3,"run_attempt":1,"name":"Mock Checks","event":"pull_request","head_sha":"a".repeat(40),"status":"completed","conclusion":"failure"},
                {"id":800,"workflow_id":10,"check_suite_id":100,"run_number":2,"run_attempt":1,"name":"Mock Checks","event":"pull_request","head_sha":"a".repeat(40),"status":"completed","conclusion":"failure"},
                {"id":700,"workflow_id":11,"check_suite_id":300,"run_number":1,"name":"Deploy","event":"push","head_sha":"a".repeat(40),"status":"queued"}
            ]));
        }
        if path.contains("/check-runs?") {
            let leaf = |id: u64,
                        suite: u64,
                        name: &str,
                        conclusion: &str,
                        seconds: u8,
                        app: u64| json!({"id":id,"name":name,"app":{"id":app},"check_suite":{"id":suite},"head_sha":"a".repeat(40),"status":"completed","conclusion":conclusion,"started_at":"2026-10-15T16:59:08Z","completed_at":format!("2026-10-15T16:59:{seconds:02}Z")});
            return Ok(json!([
                leaf(902, 200, "mock-lint", "success", 12, 1),
                leaf(901, 200, "mock-test", "failure", 13, 1),
                leaf(802, 100, "mock-test", "failure", 12, 1),
                leaf(801, 100, "mock-lint", "success", 13, 1),
                leaf(600, 400, "mock-test", "success", 14, 2),
                leaf(500, 401, "mock-test", "failure", 12, 2)
            ]));
        }
        if path.contains("/statuses?") {
            return Ok(json!([]));
        }
        panic!("Unexpected {path}");
    }
}
#[tokio::test]
async fn github_current_checks_deduplicate_executions_keep_apps_and_queued_workflows_distinct() {
    let client =
        Client::with_transport(connection(Provider::Github), Arc::new(LatestChecksFixture))
            .unwrap();
    let result = client.checks(&pr(3)).await.unwrap();
    assert_eq!(result.checks.len(), 4); // two current leaves, another app, one queued workflow
    let current: Vec<_> = result
        .checks
        .iter()
        .filter(|c| c.run_id.as_deref() == Some("900"))
        .collect();
    assert_eq!(current.len(), 2);
    let failing = current
        .iter()
        .find(|c| c.state == CheckState::Failed)
        .unwrap();
    assert_eq!(failing.name, "Mock Checks / mock-test (pull_request)");
    assert_eq!(
        failing.completed_at.as_deref(),
        Some("2026-10-15T16:59:13Z")
    );
    assert!(
        !result
            .checks
            .iter()
            .any(|c| c.run_id.as_deref() == Some("800") || c.id == "900")
    );
    assert!(
        result
            .checks
            .iter()
            .any(|c| c.id == "check:600" && c.run_id.is_none())
    );
    assert!(
        result
            .checks
            .iter()
            .any(|c| c.id == "700" && c.state == CheckState::Queued)
    );
}

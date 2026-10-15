use jj_forge::{client::Transport, *};
use serde_json::{Value, json};
use std::sync::{Arc, Mutex};
#[derive(Default)]
struct Fake {
    prs: Mutex<Vec<Value>>,
    calls: Mutex<Vec<String>>,
    unknown_create: Mutex<bool>,
}
#[async_trait::async_trait(?Send)]
impl Transport for Fake {
    async fn request(&self, method: &str, path: &str, body: Option<Value>) -> Result<Value> {
        self.calls.lock().unwrap().push(format!("{method} {path}"));
        if path.contains("pulls?state=all") || path.contains("pulls?state=open") {
            return Ok(json!(self.prs.lock().unwrap().clone()));
        }
        if method == "POST" && path.ends_with("/pulls") {
            let b = body.unwrap();
            let pr = json!({"number":1,"html_url":"https://github.com/team/repo/pull/1","title":b["title"],"body":b["body"],"head":{"ref":"feature","sha":"aaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaa","repo":{"full_name":"team/repo"}},"base":{"ref":b["base"]},"state":"open","draft":true,"mergeable":true});
            self.prs.lock().unwrap().push(pr.clone());
            if *self.unknown_create.lock().unwrap() {
                return Err(Error::Unknown);
            }
            return Ok(pr);
        }
        if path.ends_with("/pulls/1") {
            let mut prs = self.prs.lock().unwrap();
            let pr = prs.first_mut().unwrap();
            if let Some(b) = body {
                if let Some(body) = b.get("body") {
                    pr["body"] = body.clone();
                }
                if let Some(base) = b.get("base") {
                    pr["base"]["ref"] = base.clone();
                }
            }
            return Ok(pr.clone());
        }
        panic!("Unexpected {method} {path}")
    }
}
#[derive(Default)]
struct Local {
    head: Mutex<Option<String>>,
    pushes: Mutex<usize>,
}
#[async_trait::async_trait(?Send)]
impl engine::Local for Local {
    async fn validate(&self, _: &Plan, _: bool) -> Result<()> {
        Ok(())
    }
    async fn push(&self, s: &Segment) -> Result<()> {
        *self.pushes.lock().unwrap() += 1;
        *self.head.lock().unwrap() = Some(s.head.clone());
        Ok(())
    }
    async fn remote_head(&self, _: &str) -> Result<Option<String>> {
        Ok(self.head.lock().unwrap().clone())
    }
}
fn plan(c: Connection) -> Plan {
    planner::plan(
        "submit-1".into(),
        "a".repeat(128),
        c,
        vec![Segment {
            branch: "feature".into(),
            head: "a".repeat(40),
            base: "main".into(),
            title: "Feature".into(),
            body: "Authored description".into(),
            remote_head: None,
            shared: false,
        }],
        &[],
        true,
    )
    .unwrap()
}
fn connection() -> Connection {
    Connection {
        provider: Provider::Github,
        api_url: "https://api.github.com".into(),
        repository: "team/repo".into(),
        source_repository: "team/repo".into(),
        remote: "origin".into(),
        target_remote: "origin".into(),
        target: "main".into(),
        native_stacks: false,
    }
}
#[tokio::test]
async fn unknown_creation_is_observed_and_never_duplicated() {
    let root = tempfile::tempdir().unwrap();
    let metadata = tempfile::tempdir().unwrap();
    let c = connection();
    let fake = Arc::new(Fake::default());
    *fake.unknown_create.lock().unwrap() = true;
    let client = Client::with_transport(c.clone(), fake.clone()).unwrap();
    let local = Local::default();
    let mut device = journal::Device::at(root.path(), metadata.path()).unwrap();
    device.configure(c.clone()).unwrap();
    let result = engine::submit(&local, &client, &mut device, plan(c.clone()), false)
        .await
        .unwrap();
    assert_eq!(result.outcome, Outcome::Unknown);
    let result = engine::submit(&local, &client, &mut device, plan(c), true)
        .await
        .unwrap();
    assert_eq!(result.outcome, Outcome::Complete);
    assert_eq!(*local.pushes.lock().unwrap(), 1);
    assert_eq!(fake.prs.lock().unwrap().len(), 1);
    assert_eq!(
        fake.calls
            .lock()
            .unwrap()
            .iter()
            .filter(|x| x.as_str() == "POST repos/team/repo/pulls")
            .count(),
        1
    );
    assert!(
        fake.prs.lock().unwrap()[0]["body"]
            .as_str()
            .unwrap()
            .starts_with("Authored description")
    );
}
#[tokio::test]
async fn same_request_returns_receipt_without_repeating_effects() {
    let root = tempfile::tempdir().unwrap();
    let metadata = tempfile::tempdir().unwrap();
    let c = connection();
    let fake = Arc::new(Fake::default());
    let client = Client::with_transport(c.clone(), fake.clone()).unwrap();
    let local = Local::default();
    let mut d = journal::Device::at(root.path(), metadata.path()).unwrap();
    d.configure(c.clone()).unwrap();
    assert_eq!(
        engine::submit(&local, &client, &mut d, plan(c.clone()), false)
            .await
            .unwrap()
            .outcome,
        Outcome::Complete
    );
    let before = fake.calls.lock().unwrap().len();
    engine::submit(&local, &client, &mut d, plan(c), false)
        .await
        .unwrap();
    assert_eq!(fake.calls.lock().unwrap().len(), before);
}

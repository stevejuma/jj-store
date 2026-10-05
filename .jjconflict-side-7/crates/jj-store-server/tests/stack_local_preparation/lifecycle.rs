use super::*;
use jj_store_jj::local_browser::{self as q, Access};
use std::{
    path::Path,
    sync::atomic::{AtomicUsize, Ordering},
    time::Duration,
};
struct Registered(PathBuf);
#[async_trait::async_trait]
impl Access for Registered {
    fn repository(&self) -> m::RepositoryId {
        m::RepositoryId::new([81; 16])
    }
    fn generation(&self) -> u64 {
        1
    }
    fn metadata_path(&self) -> &Path {
        &self.0
    }
    async fn authorize(&self) -> q::Result<()> {
        Ok(())
    }
}
pub(super) struct ResumeOnly {
    pub root: PathBuf,
}
#[async_trait::async_trait]
impl Authority for ResumeOnly {
    async fn authorize(
        &self,
        _actor: m::PrincipalId,
        _request: &s::PrepareRequest,
    ) -> stack_preparation::Result<()> {
        Ok(())
    }
    async fn context(
        &self,
        _actor: m::PrincipalId,
        _request: &s::PrepareRequest,
    ) -> stack_preparation::Result<Context> {
        panic!("saved preparation must never prepare again")
    }
    async fn recovery(
        &self,
        _actor: m::PrincipalId,
        _request: &s::PrepareRequest,
    ) -> stack_preparation::Result<stack_preparation::RecoveryContext> {
        Ok(stack_preparation::RecoveryContext {
            access: Arc::new(Registered(self.root.join(".jj/repo"))),
            root: self.root.clone(),
        })
    }
}
struct Stop {
    root: PathBuf,
    ready: PathBuf,
    at: usize,
    calls: AtomicUsize,
}
#[tokio::test]
async fn cancel_after_pending_selection_releases_lease_and_resumes_exact_archive() {
    let f = Fixture::new().await;
    let state = f.witness().await;
    let req = request(&state);
    let before = f.state();
    let private = tempfile::tempdir().unwrap();
    let path = private.path().join("stacks");
    let ready = private.path().join("ready");
    let journal = Arc::new(StackCommandJournal::create(&path, scope(&state)).unwrap());
    let coordinator = Arc::new(Coordinator::new(1).unwrap());
    let task = tokio::spawn({
        let journal = journal.clone();
        let coordinator = coordinator.clone();
        let req = req.clone();
        let authority = Stop {
            root: f.root(),
            ready: ready.clone(),
            at: 3,
            calls: AtomicUsize::new(0),
        };
        async move {
            stack_preparation::prepare_reword(&journal, &coordinator, &authority, actor(), req)
                .await
        }
    });
    tokio::time::timeout(Duration::from_secs(15), async {
        while !ready.exists() {
            tokio::time::sleep(Duration::from_millis(20)).await;
        }
    })
    .await
    .unwrap();
    task.abort();
    assert!(task.await.unwrap_err().is_cancelled());
    let pending = journal
        .status(actor(), req.request_id, req.intent_digest)
        .unwrap()
        .unwrap();
    assert!(pending.execution.as_ref().unwrap().retention.is_none());
    let lease = coordinator
        .acquire(f.access(), f.root(), state.clone())
        .await
        .unwrap();
    drop(lease);
    let resumed = stack_preparation::prepare_reword(
        &journal,
        &coordinator,
        &ResumeOnly { root: f.root() },
        actor(),
        req.clone(),
    )
    .await
    .unwrap();
    assert_eq!(resumed.preview, pending.preview);
    assert_eq!(resumed.receipt, pending.receipt);
    assert!(
        journal
            .status(actor(), req.request_id, req.intent_digest)
            .unwrap()
            .unwrap()
            .execution
            .unwrap()
            .history
            .is_none()
    );
    assert_eq!(f.state(), before);
}
#[async_trait::async_trait]
impl Authority for Stop {
    async fn authorize(
        &self,
        _actor: m::PrincipalId,
        _request: &s::PrepareRequest,
    ) -> stack_preparation::Result<()> {
        if self.calls.fetch_add(1, Ordering::SeqCst) + 1 == self.at {
            std::fs::write(&self.ready, []).unwrap();
            std::future::pending::<()>().await;
        }
        Ok(())
    }
    async fn context(
        &self,
        _actor: m::PrincipalId,
        request: &s::PrepareRequest,
    ) -> stack_preparation::Result<Context> {
        let s::CodeSelection::GitSha1Jj045 { operation, .. } = request.selection.code else {
            panic!()
        };
        let access: Arc<dyn Access> = Arc::new(Registered(self.root.join(".jj/repo")));
        let current = q::Executor::new(1)
            .unwrap()
            .working_tree_current(access.clone(), self.root.clone(), operation)
            .await?;
        let combination = super::request(&current).selection.combination.unwrap();
        Ok(Context {
            access,
            root: self.root.clone(),
            current,
            working_tree: request.selection.working_tree,
            combination,
        })
    }
}
async fn interrupted(at: usize) {
    let f = Fixture::new().await;
    let before = f.state();
    let state = f.witness().await;
    let req = request(&state);
    let private = tempfile::tempdir().unwrap();
    let path = private.path().join("stacks");
    let ready = private.path().join("ready");
    let payload = private.path().join("payload");
    let journal = StackCommandJournal::create(&path, scope(&state)).unwrap();
    std::fs::write(
        &payload,
        serde_json::to_vec(&(f.root(), path.clone(), ready.clone(), at, req.clone())).unwrap(),
    )
    .unwrap();
    let mut child = std::process::Command::new(std::env::current_exe().unwrap())
        .args([
            "--ignored",
            "--exact",
            "lifecycle::local_preparation_process_helper",
        ])
        .env("JJ_STORE_STACK_LOCAL_PREPARATION", &payload)
        .stdout(std::process::Stdio::null())
        .stderr(std::process::Stdio::null())
        .spawn()
        .unwrap();
    let reached = tokio::time::timeout(Duration::from_secs(15), async {
        while !ready.exists() {
            tokio::time::sleep(Duration::from_millis(20)).await;
        }
    })
    .await;
    if reached.is_err() {
        let _ = child.kill();
        let _ = child.wait();
        panic!("preparation helper did not reach checkpoint {at}");
    }
    child.kill().unwrap();
    assert!(!child.wait().unwrap().success());
    let pending = journal
        .status(actor(), req.request_id, req.intent_digest)
        .unwrap()
        .unwrap();
    assert!(pending.execution.as_ref().unwrap().retention.is_none());
    assert!(pending.execution.as_ref().unwrap().history.is_some());
    let apply = s::ApplyRequest {
        version: 1,
        request_id: req.request_id,
        intent_digest: req.intent_digest,
        prepared: pending.preview.prepared,
        selection: req.selection.clone(),
    };
    assert!(
        journal
            .transition(
                actor(),
                &apply,
                &s::ReceiptState::Prepared,
                &s::ReceiptState::PublicationUnknown
            )
            .is_err()
    );
    assert_eq!(f.state(), before);
    let key = pending.execution.as_ref().unwrap().key.clone();
    let metadata = MetadataStore::open(&f.root().join(".git"), Limits::default()).unwrap();
    assert_eq!(
        metadata.read_stack_prepared(&key).unwrap().is_some(),
        at == 4
    );
    std::fs::remove_dir_all(f.root().join(".jj/repo/op_store")).unwrap();
    std::fs::remove_dir_all(f.root().join(".jj/repo/index")).unwrap();
    let result = std::process::Command::new("git")
        .arg("--git-dir")
        .arg(metadata.git_dir())
        .args(["gc", "--prune=now"])
        .env("GIT_CONFIG_NOSYSTEM", "1")
        .env("GIT_CONFIG_GLOBAL", "/dev/null")
        .output()
        .unwrap();
    assert!(
        result.status.success(),
        "{}",
        String::from_utf8_lossy(&result.stderr)
    );
    let reopened = StackCommandJournal::open(&path, scope(&state)).unwrap();
    let saved = stack_preparation::prepare_reword(
        &reopened,
        &Coordinator::new(1).unwrap(),
        &ResumeOnly { root: f.root() },
        actor(),
        req.clone(),
    )
    .await
    .unwrap();
    assert_eq!(saved.preview, pending.preview);
    assert_eq!(saved.receipt, pending.receipt);
    let record = reopened
        .status(actor(), req.request_id, req.intent_digest)
        .unwrap()
        .unwrap();
    let execution = record.execution.unwrap();
    assert!(execution.history.is_none());
    assert_eq!(
        execution.retention,
        Some(metadata.read_stack_prepared(&key).unwrap().unwrap().commit)
    );
    assert!(!f.root().join(".jj/repo/op_store").exists());
    assert!(!f.root().join(".jj/repo/index").exists());
    assert_eq!(f.state(), before);
}
#[tokio::test]
async fn process_loss_before_retention_replays_saved_candidate_after_source_loss_and_gc() {
    interrupted(3).await;
}
#[tokio::test]
async fn process_loss_after_retention_replays_original_acknowledgement_after_source_loss_and_gc() {
    interrupted(4).await;
}
#[tokio::test]
#[ignore = "invoked by the isolated process-loss parent fixtures"]
async fn local_preparation_process_helper() {
    let payload = std::env::var_os("JJ_STORE_STACK_LOCAL_PREPARATION").unwrap();
    let (root, path, ready, at, request): (PathBuf, PathBuf, PathBuf, usize, s::PrepareRequest) =
        serde_json::from_slice(&std::fs::read(payload).unwrap()).unwrap();
    let selection = &request.selection;
    let sc = Scope {
        installation: [75; 16],
        repository: selection.repository,
        physical_workspace: selection.physical_workspace,
        jj_workspace: selection.jj_workspace.clone(),
        profile: Profile::GitSha1Jj045,
    };
    let journal = StackCommandJournal::open(&path, sc).unwrap();
    let auth = Stop {
        root,
        ready,
        at,
        calls: AtomicUsize::new(0),
    };
    stack_preparation::prepare_reword(
        &journal,
        &Coordinator::new(1).unwrap(),
        &auth,
        actor(),
        request,
    )
    .await
    .unwrap();
    panic!("helper must be killed before acknowledgement");
}

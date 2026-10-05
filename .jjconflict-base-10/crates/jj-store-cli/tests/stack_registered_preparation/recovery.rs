use super::*;
use std::sync::atomic::{AtomicUsize, Ordering};
struct Interrupted {
    registered: RegisteredAuthority,
    entered: Arc<tokio::sync::Notify>,
    calls: AtomicUsize,
}
#[async_trait::async_trait]
impl Authority for Interrupted {
    async fn authorize(&self, actor: m::PrincipalId, request: &s::PrepareRequest) -> p::Result<()> {
        self.registered.authorize(actor, request).await?;
        if self.calls.fetch_add(1, Ordering::SeqCst) == 2 {
            self.entered.notify_one();
            std::future::pending::<()>().await;
        }
        Ok(())
    }
    async fn context(
        &self,
        actor: m::PrincipalId,
        request: &s::PrepareRequest,
    ) -> p::Result<p::Context> {
        self.registered.context(actor, request).await
    }
}
#[tokio::test]
async fn registered_pending_recovery_resumes_saved_archive_after_operation_loss_without_scanning() {
    let f = Installed::new().await;
    let before = f.folder.state();
    let journal = Arc::new(f.journal());
    let coordinator = Arc::new(Coordinator::new(1).unwrap());
    let entered = Arc::new(tokio::sync::Notify::new());
    let task = tokio::spawn({
        let journal = journal.clone();
        let coordinator = coordinator.clone();
        let request = f.request.clone();
        let actor = f.actor;
        let authority = Interrupted {
            registered: f.authority.clone(),
            entered: entered.clone(),
            calls: AtomicUsize::new(0),
        };
        async move { p::prepare_reword(&journal, &coordinator, &authority, actor, request).await }
    });
    tokio::time::timeout(std::time::Duration::from_secs(20), entered.notified())
        .await
        .unwrap();
    task.abort();
    assert!(task.await.unwrap_err().is_cancelled());
    let original = journal
        .status(f.actor, f.request.request_id, f.request.intent_digest)
        .unwrap()
        .unwrap();
    assert!(original.execution.as_ref().unwrap().retention.is_none());
    std::fs::remove_dir_all(f.folder.root().join(".jj/repo/op_store")).unwrap();
    std::fs::remove_dir_all(f.folder.root().join(".jj/repo/index")).unwrap();
    *f.combinations.0.lock().unwrap() = None;
    let gc = std::process::Command::new("git")
        .arg("--git-dir")
        .arg(f.folder.root().join(".git"))
        .args(["gc", "--prune=now"])
        .env("GIT_CONFIG_NOSYSTEM", "1")
        .env("GIT_CONFIG_GLOBAL", "/dev/null")
        .output()
        .unwrap();
    assert!(
        gc.status.success(),
        "{}",
        String::from_utf8_lossy(&gc.stderr)
    );
    let reopened =
        StackCommandJournal::open(&f.state.join("stack-journal"), journal_scope(&f)).unwrap();
    let resumed = p::prepare_reword(
        &reopened,
        &coordinator,
        &f.authority,
        f.actor,
        f.request.clone(),
    )
    .await
    .unwrap();
    assert_eq!(resumed.preview, original.preview);
    assert_eq!(resumed.receipt, original.receipt);
    let execution = reopened
        .status(f.actor, f.request.request_id, f.request.intent_digest)
        .unwrap()
        .unwrap()
        .execution
        .unwrap();
    assert!(execution.retention.is_some());
    assert!(execution.history.is_none());
    assert!(!f.folder.root().join(".jj/repo/op_store").exists());
    assert!(!f.folder.root().join(".jj/repo/index").exists());
    assert_eq!(f.folder.state(), before);
}
pub(super) fn journal_scope(f: &Installed) -> Scope {
    Scope {
        installation: f.installation,
        repository: f.binding.repository,
        physical_workspace: f.request.selection.physical_workspace,
        jj_workspace: f.request.selection.jj_workspace.clone(),
        profile: Profile::GitSha1Jj045,
    }
}

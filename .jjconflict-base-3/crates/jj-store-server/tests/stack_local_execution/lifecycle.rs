use super::*;
use std::{
    fs,
    path::Path,
    process::{Command, Stdio},
    time::Duration,
};
#[tokio::test]
async fn process_loss_at_publication_and_checkout_boundaries_resumes_saved_journal() {
    for boundary in 0..4 {
        let f = Fixture::new().await;
        let (private, journal, authority, apply) = prepared(&f).await;
        let scope = Scope {
            installation: [75; 16],
            repository: authority.current.baseline.repository,
            physical_workspace: s::PhysicalWorkspaceId(authority.current.physical_workspace),
            jj_workspace: authority.current.checkout.workspace.clone(),
            profile: Profile::GitSha1Jj045,
        };
        let payload = private.path().join("payload");
        let ready = private.path().join("ready");
        fs::write(
            &payload,
            serde_json::to_vec(&(
                f.root(),
                private.path().join("stacks"),
                scope.clone(),
                apply.clone(),
                boundary,
                ready.clone(),
            ))
            .unwrap(),
        )
        .unwrap();
        let before = f.state();
        let mut child = Command::new(std::env::current_exe().unwrap())
            .args([
                "--ignored",
                "--exact",
                "lifecycle::execution_process_helper",
            ])
            .env("JJ_STORE_STACK_EXECUTION_PAYLOAD", payload)
            .stdout(Stdio::null())
            .stderr(Stdio::null())
            .spawn()
            .unwrap();
        let reached = tokio::time::timeout(Duration::from_secs(20), async {
            while !ready.exists() {
                tokio::time::sleep(Duration::from_millis(20)).await;
            }
        })
        .await;
        if reached.is_err() {
            let _ = child.kill();
            let _ = child.wait();
            panic!("helper missed boundary {boundary}");
        }
        child.kill().unwrap();
        assert!(!child.wait().unwrap().success());
        let interrupted = journal
            .status(actor(), apply.request_id, apply.intent_digest)
            .unwrap()
            .unwrap();
        assert!(!interrupted.receipt.completed());
        assert_eq!(journal.pending(actor()).unwrap().len(), 1);
        let reopened = StackCommandJournal::open(&private.path().join("stacks"), scope).unwrap();
        let observer = tree::Observer::new(q::Executor::new(1).unwrap(), 1).unwrap();
        let done = p::apply_reword(
            &reopened,
            &Coordinator::new(1).unwrap(),
            &observer,
            &authority,
            actor(),
            apply.clone(),
        )
        .await
        .unwrap();
        assert!(done.receipt.completed());
        assert_eq!(done.preview, interrupted.preview);
        if let Some(saved) = interrupted.execution.unwrap().checkout {
            assert_eq!(
                reopened
                    .status(actor(), apply.request_id, apply.intent_digest)
                    .unwrap()
                    .unwrap()
                    .execution
                    .unwrap()
                    .checkout,
                Some(saved)
            );
        }
        assert_eq!(&f.state()[..2], &before[..2]);
        assert_eq!(f.state()[3], before[3]);
    }
}
struct Reopened(PathBuf);
#[async_trait::async_trait]
impl q::Access for Reopened {
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
struct Crash {
    stop: Stop,
    ready: PathBuf,
}
#[async_trait::async_trait]
impl Authority for Crash {
    async fn authorize(&self, actor: m::PrincipalId, request: &s::PrepareRequest) -> p::Result<()> {
        match self.stop.authorize(actor, request).await {
            Err(p::Error::Forbidden) => {
                fs::write(&self.ready, b"paused").unwrap();
                std::future::pending().await
            }
            value => value,
        }
    }
    async fn context(
        &self,
        actor: m::PrincipalId,
        request: &s::PrepareRequest,
    ) -> p::Result<Context> {
        self.stop.context(actor, request).await
    }
    async fn recovery(
        &self,
        actor: m::PrincipalId,
        request: &s::PrepareRequest,
    ) -> p::Result<RecoveryContext> {
        self.stop.recovery(actor, request).await
    }
}
#[tokio::test]
#[ignore = "invoked by isolated process-loss parent"]
async fn execution_process_helper() {
    let (root, path, scope, apply, boundary, ready): (
        PathBuf,
        PathBuf,
        Scope,
        s::ApplyRequest,
        u8,
        PathBuf,
    ) = serde_json::from_slice(
        &fs::read(std::env::var_os("JJ_STORE_STACK_EXECUTION_PAYLOAD").unwrap()).unwrap(),
    )
    .unwrap();
    let access = Arc::new(Reopened(root.join(".jj/repo")));
    let s::CodeSelection::GitSha1Jj045 { operation, .. } = apply.selection.code else {
        panic!()
    };
    let current = q::Executor::new(1)
        .unwrap()
        .working_tree_current(access.clone(), root.clone(), operation)
        .await
        .unwrap();
    let journal = StackCommandJournal::open(&path, scope).unwrap();
    let crash = Crash {
        ready,
        stop: Stop {
            original: fs::read(root.join(".jj/working_copy/checkout")).unwrap(),
            inner: Allowed {
                access,
                root,
                current,
                physical: apply.selection.working_tree,
            },
            journal: journal.clone(),
            boundary: [
                Boundary::Unknown,
                Boundary::Published,
                Boundary::Saved,
                Boundary::Selected,
            ][usize::from(boundary)],
            stopped: Default::default(),
        },
    };
    p::apply_reword(
        &journal,
        &Coordinator::new(1).unwrap(),
        &tree::Observer::new(q::Executor::new(1).unwrap(), 1).unwrap(),
        &crash,
        actor(),
        apply,
    )
    .await
    .unwrap();
    panic!("helper must be killed before acknowledgement");
}

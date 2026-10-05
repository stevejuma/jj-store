#[path = "../../jj-store-jj/tests/stack_foreground/fixture.rs"]
mod fixture;
use fixture::*;
use jj_store_jj::{
    local_execution::{Coordinator, Reword},
    local_metadata::{Limits, MetadataStore, StackPreparationKey},
};
use jj_store_model as m;
use jj_store_service::working_tree as tree;
use std::fs;

async fn selection(
    f: &Fixture,
) -> (
    jj_store_jj::local_execution::RecoveryBinding,
    m::RecordDigest,
) {
    let state = f.witness().await;
    let physical = tree::scan(
        &f.root(),
        state.baseline.repository,
        state.baseline.generation,
        tree::Limits::default(),
        |_| Ok(()),
    )
    .unwrap();
    (
        jj_store_jj::local_execution::RecoveryBinding {
            repository: state.baseline.repository,
            generation: state.baseline.generation,
            physical_workspace: state.physical_workspace,
            operation: state.baseline.operation,
            view: state.baseline.view,
            commit: state.baseline.commit,
            workspace: state.checkout.workspace,
        },
        physical.fingerprint,
    )
}

#[tokio::test]
async fn full_guard_rejects_binding_revocation_modes_links_ignored_bytes_and_limits() {
    use jj_store_jj::local_browser as code;
    use std::{
        os::unix::fs::{PermissionsExt, symlink},
        sync::Arc,
    };
    struct Revoked(Arc<dyn code::Access>);
    #[async_trait::async_trait]
    impl code::Access for Revoked {
        fn repository(&self) -> m::RepositoryId {
            self.0.repository()
        }
        fn generation(&self) -> u64 {
            self.0.generation()
        }
        fn metadata_path(&self) -> &std::path::Path {
            self.0.metadata_path()
        }
        async fn authorize(&self) -> code::Result<()> {
            Err(code::Error::Denied)
        }
    }
    for mode in [
        "repository",
        "generation",
        "workspace",
        "revoked",
        "executable",
        "symlink",
        "ignored",
        "oversized",
    ] {
        let f = Fixture::new().await;
        let (mut binding, expected) = selection(&f).await;
        let observer = tree::Observer::new(code::Executor::new(1).unwrap(), 1).unwrap();
        let access = if mode == "revoked" {
            Arc::new(Revoked(f.access())) as Arc<dyn code::Access>
        } else {
            f.access()
        };
        let guard = observer.physical_guard(access, f.root());
        match mode {
            "repository" => binding.repository = m::RepositoryId::new([0; 16]),
            "generation" => binding.generation += 1,
            "workspace" => binding.physical_workspace = m::RecordDigest::new([0; 32]),
            "executable" => {
                fs::set_permissions(f.root().join("file.txt"), fs::Permissions::from_mode(0o755))
                    .unwrap()
            }
            "symlink" => symlink("file.txt", f.root().join("new-link")).unwrap(),
            "ignored" => fs::write(f.root().join("ignored.txt"), "new ignored bytes\n").unwrap(),
            "oversized" => fs::File::create(f.root().join("large-obstacle"))
                .unwrap()
                .set_len(2 * 1024 * 1024 + 1)
                .unwrap(),
            _ => {}
        }
        let before = f.state();
        let error = guard.validate(&binding, expected).await.unwrap_err();
        let expected_error = match mode {
            "revoked" => code::Error::Denied,
            "oversized" => code::Error::Limit,
            _ => code::Error::EpochChanged,
        };
        assert_eq!(error, expected_error, "{mode}");
        assert_eq!(f.state(), before, "{mode}");
    }
}

#[tokio::test(flavor = "multi_thread", worker_threads = 2)]
async fn full_guard_cancellation_retains_shared_admission_until_worker_exits() {
    use jj_store_jj::local_browser as code;
    use std::sync::{
        Arc,
        atomic::{AtomicUsize, Ordering},
    };
    struct Paused {
        inner: Arc<dyn code::Access>,
        calls: AtomicUsize,
        entered: tokio::sync::Notify,
    }
    #[async_trait::async_trait]
    impl code::Access for Paused {
        fn repository(&self) -> m::RepositoryId {
            self.inner.repository()
        }
        fn generation(&self) -> u64 {
            self.inner.generation()
        }
        fn metadata_path(&self) -> &std::path::Path {
            self.inner.metadata_path()
        }
        async fn authorize(&self) -> code::Result<()> {
            if self.calls.fetch_add(1, Ordering::SeqCst) >= 2 {
                self.entered.notify_one();
                std::future::pending::<()>().await;
            }
            self.inner.authorize().await
        }
    }
    let f = Fixture::new().await;
    let (binding, expected) = selection(&f).await;
    let before = f.state();
    let observer = tree::Observer::new(code::Executor::new(1).unwrap(), 1).unwrap();
    let access = Arc::new(Paused {
        inner: f.access(),
        calls: AtomicUsize::new(0),
        entered: tokio::sync::Notify::new(),
    });
    let first = observer.physical_guard(access.clone(), f.root());
    let copy = binding.clone();
    let task = tokio::spawn(async move { first.validate(&copy, expected).await });
    tokio::time::timeout(std::time::Duration::from_secs(3), access.entered.notified())
        .await
        .unwrap();
    let second = observer.physical_guard(f.access(), f.root());
    assert_eq!(
        second.validate(&binding, expected).await.unwrap_err(),
        code::Error::Busy
    );
    task.abort();
    assert!(task.await.unwrap_err().is_cancelled());
    tokio::time::timeout(std::time::Duration::from_secs(3), async {
        loop {
            match second.validate(&binding, expected).await {
                Ok(()) => break,
                Err(code::Error::Busy) => tokio::task::yield_now().await,
                Err(error) => panic!("{error}"),
            }
        }
    })
    .await
    .unwrap();
    assert_eq!(f.state(), before);
}
#[tokio::test]
async fn complete_registered_inventory_guards_real_unchanged_checkout_and_new_untracked_files() {
    for changed in [false, true] {
        let f = Fixture::new().await;
        let state = f.witness().await;
        let physical = tree::scan(
            &f.root(),
            state.baseline.repository,
            state.baseline.generation,
            tree::Limits::default(),
            |_| Ok(()),
        )
        .unwrap();
        let observer =
            tree::Observer::new(jj_store_jj::local_browser::Executor::new(1).unwrap(), 1).unwrap();
        let guard = observer.physical_guard(f.access(), f.root());
        let lease = Coordinator::new(1)
            .unwrap()
            .acquire(f.access(), f.root(), state.clone())
            .await
            .unwrap();
        let (lease, witness) = lease.recovery_witness().await.unwrap();
        let (lease, candidate) = lease
            .prepare_reword(Reword {
                principal: m::PrincipalId::new([85; 16]),
                immutable_heads: vec![],
                commit: state.baseline.commit,
                message: "Real complete physical guard".into(),
            })
            .await
            .unwrap();
        let key = StackPreparationKey {
            repository: state.baseline.repository,
            physical_workspace: state.physical_workspace,
            principal: m::PrincipalId::new([85; 16]),
            request: m::RequestId::new([94; 16]),
            intent: m::IntentDigest::new([95; 32]),
            prepared: m::RecordDigest::new([96; 32]),
        };
        MetadataStore::open(&f.root().join(".git"), Limits::default())
            .unwrap()
            .retain_stack_execution(&key, &candidate.history)
            .unwrap();
        drop(lease);
        let recovery = Coordinator::new(1)
            .unwrap()
            .resume_publication(f.access(), f.root(), witness, key)
            .await
            .unwrap()
            .restore_extras()
            .await
            .unwrap()
            .restore_indexes()
            .await
            .unwrap();
        let (recovery, published) = recovery.publish_head().await.unwrap();
        let (recovery, checkout) = recovery
            .prepare_unchanged_checkout(&published, guard.clone(), physical.fingerprint)
            .await
            .unwrap();
        if changed {
            fs::write(
                f.root().join("new-untracked.txt"),
                "keep this new obstacle\n",
            )
            .unwrap();
        }
        let before = f.state();
        let result = recovery.complete_unchanged_checkout(checkout, guard).await;
        if changed {
            assert!(matches!(
                result,
                Err(jj_store_jj::local_browser::Error::EpochChanged)
            ));
            assert_eq!(f.state(), before);
            assert_eq!(
                fs::read(f.root().join("new-untracked.txt")).unwrap(),
                b"keep this new obstacle\n"
            );
        } else {
            let (_recovery, proof) = result.unwrap();
            assert_eq!(proof.operation, published.operation);
            assert_eq!(proof.physical(), physical.fingerprint);
            let after = f.state();
            assert_eq!(after[0], before[0]);
            assert_eq!(after[1], before[1]);
            assert_eq!(after[3], before[3]);
            assert_ne!(after[2], before[2]);
        }
    }
}

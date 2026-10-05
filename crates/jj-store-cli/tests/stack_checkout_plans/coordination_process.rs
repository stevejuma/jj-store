use super::*;
use coordination::Joined;
#[tokio::test]
async fn process_loss_after_native_evidence_and_final_ack_resumes_original_join() {
    for phase in ["native", "evidence", "ack"] {
        let f = Joined::new().await;
        let (recovery, proof) = f.complete().await;
        drop(recovery);
        let root = f.input.folder.root();
        let input = f.private.path().join("process-input");
        private_write(
            &input,
            &serde_json::to_vec(&(
                f.input.scope.clone(),
                f.input.preview.clone(),
                f.input.execution.clone(),
            ))
            .unwrap(),
        );
        let output = std::process::Command::new(std::env::current_exe().unwrap())
            .args([
                "--ignored",
                "--exact",
                "coordination_process::exit_after_join_boundary",
                "--nocapture",
            ])
            .env("JJ_STORE_JOIN_PRIVATE", f.private.path())
            .env("JJ_STORE_JOIN_ROOT", &root)
            .env("JJ_STORE_JOIN_PHASE", phase)
            .output()
            .unwrap();
        assert_eq!(
            output.status.code(),
            Some(98),
            "{phase}: {}",
            String::from_utf8_lossy(&output.stderr)
        );
        let before = f.input.folder.state();
        let (recovery, repeated) = f.resume().await.unwrap();
        assert_eq!(proof, repeated);
        let was_complete = f.record().receipt.completed();
        assert_eq!(was_complete, phase == "ack");
        if phase != "ack" {
            assert!(f.store.snapshot().unwrap().active.is_none());
            assert_eq!(f.journal.pending(actor()).unwrap().len(), 1);
        }
        f.journal
            .save_local_tree_completion(actor(), &f.input.apply(), &repeated)
            .unwrap();
        let selected = f.finish(&repeated).unwrap();
        assert_eq!(selected.replayed, phase == "ack");
        assert!(selected.receipt.completed());
        assert_eq!(f.store.snapshot().unwrap(), f.after);
        assert_eq!(f.input.folder.state(), before);
        drop(recovery);
    }
}
struct Access {
    path: PathBuf,
    repository: m::RepositoryId,
}
#[async_trait::async_trait]
impl local_browser::Access for Access {
    fn repository(&self) -> m::RepositoryId {
        self.repository
    }
    fn generation(&self) -> u64 {
        1
    }
    fn metadata_path(&self) -> &std::path::Path {
        &self.path
    }
    async fn authorize(&self) -> local_browser::Result<()> {
        Ok(())
    }
}
#[tokio::test]
#[ignore = "isolated worker exits after original native join boundary"]
async fn exit_after_join_boundary() {
    let private = PathBuf::from(std::env::var_os("JJ_STORE_JOIN_PRIVATE").unwrap());
    let root = PathBuf::from(std::env::var_os("JJ_STORE_JOIN_ROOT").unwrap());
    let (scope, preview, execution): (Scope, s::Preview, LocalExecutionV1) =
        serde_json::from_slice(&fs::read(private.join("process-input")).unwrap()).unwrap();
    let apply = s::ApplyRequest {
        version: 1,
        request_id: preview.plan.request.request_id,
        intent_digest: preview.plan.request.intent_digest,
        prepared: preview.prepared,
        selection: preview.plan.request.selection.clone(),
    };
    let journal = StackCommandJournal::open(&private.join("journal"), scope.clone()).unwrap();
    let plans = StackCheckoutPlans::open(&private.join("plans"), scope.clone()).unwrap();
    let store = jj_store_server::stack_combinations::CombinationStore::open(
        &private.join("combinations"),
        scope.clone(),
        actor(),
    )
    .unwrap();
    let plan = journal
        .load_local_tree_checkout(&plans, actor(), &apply)
        .unwrap();
    let inventory = journal
        .load_local_physical_inventory(&plans, actor(), &apply)
        .unwrap();
    let access: Arc<dyn local_browser::Access> = Arc::new(Access {
        path: root.join(".jj/repo"),
        repository: scope.repository,
    });
    let observer =
        jj_store_service::working_tree::Observer::new(local_browser::Executor::new(1).unwrap(), 1)
            .unwrap();
    let guard = observer.physical_guard(access.clone(), root.clone());
    let (recovery, proof) = Coordinator::new(1)
        .unwrap()
        .resume_tree_checkout(
            access,
            root,
            execution.witness,
            execution.key,
            plan,
            inventory,
        )
        .await
        .unwrap()
        .finish_tree_checkout(guard)
        .await
        .unwrap();
    let phase = std::env::var("JJ_STORE_JOIN_PHASE").unwrap();
    if phase == "native" {
        std::process::exit(98);
    }
    journal
        .save_local_tree_completion(actor(), &apply, &proof)
        .unwrap();
    if phase == "evidence" {
        std::process::exit(98);
    }
    journal
        .finish_local_tree_combination(&store, actor(), &apply, &proof)
        .unwrap();
    assert_eq!(phase, "ack");
    drop(recovery);
    std::process::exit(98);
}

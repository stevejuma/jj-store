use super::*;
#[path = "setup/recovery.rs"]
mod recovery_tests;
use jj_store_server::stack_combinations::CombinationStore;

struct SetupInstalled {
    f: Installed,
    store: CombinationStore,
    authority: RegisteredAuthority,
    request: s::PrepareRequest,
    observer: Observer,
}
impl SetupInstalled {
    async fn new() -> Self {
        let f = Installed::new().await;
        std::fs::write(f.folder.root().join("file.txt"), "original\n").unwrap();
        stock(&f.folder.root(), &["bookmark", "create", "main", "-r", "@"]);
        std::fs::write(f.folder.root().join("file.txt"), "dirty external edit\n").unwrap();
        let operation = stock(
            &f.folder.root(),
            &[
                "--ignore-working-copy",
                "op",
                "log",
                "--no-graph",
                "-n",
                "1",
                "-T",
                "id",
            ],
        )
        .parse()
        .unwrap();
        let current = q::Executor::new(1)
            .unwrap()
            .working_tree_current(f.folder.access(), f.folder.root(), operation)
            .await
            .unwrap();
        f.journal();
        let store = CombinationStore::create(
            &f.state.join("setup-combinations"),
            recovery::journal_scope(&f),
            f.actor,
        )
        .unwrap();
        let observer = Observer::new(q::Executor::new(2).unwrap(), 1).unwrap();
        // The initial combination authority is the actual empty private store.
        // Installed's reword-only fixture selection is never consulted by setup.
        let authority = RegisteredAuthority::new(
            &Access::LocalTrusted {
                principal: f.actor,
                permission: Permission::Admin,
            },
            f.manager.clone(),
            Arc::new(store.clone()),
            observer.clone(),
        )
        .unwrap();
        let mut selection = f.request.selection.clone();
        selection.combination = None;
        selection.code = s::CodeSelection::GitSha1Jj045 {
            operation,
            view: current.baseline.view,
            working_copy: current.baseline.commit,
        };
        let mut request = s::PrepareRequest::new(
            m::RequestId::new([95; 16]),
            selection,
            s::Command::Setup {
                name: "Local workspace".into(),
                target: s::BookmarkTarget {
                    name: "main".into(),
                    remote: None,
                    head: s::CommitIdentity::GitSha1(current.baseline.commit),
                },
            },
        )
        .unwrap();
        request.selection.working_tree = authority
            .setup_context(f.actor, &request)
            .await
            .unwrap()
            .working_tree;
        request.intent_digest = request.intent().unwrap();
        Self {
            f,
            store,
            authority,
            request,
            observer,
        }
    }
    fn journal(&self) -> StackCommandJournal {
        StackCommandJournal::open(&self.f.state.join("stack-journal"), self.store_scope()).unwrap()
    }
    fn store_scope(&self) -> Scope {
        Scope {
            installation: self.f.installation,
            repository: self.f.binding.repository,
            physical_workspace: self.request.selection.physical_workspace,
            jj_workspace: self.request.selection.jj_workspace.clone(),
            profile: Profile::GitSha1Jj045,
        }
    }
    async fn prepare(
        &self,
        journal: &StackCommandJournal,
    ) -> p::Result<jj_store_server::stack_command_journal::Selection> {
        p::prepare_setup(
            journal,
            &self.store,
            &Coordinator::new(1).unwrap(),
            &self.observer,
            &self.authority,
            self.f.actor,
            self.request.clone(),
        )
        .await
    }
    async fn apply(
        &self,
        journal: &StackCommandJournal,
        apply: s::ApplyRequest,
    ) -> p::Result<jj_store_server::stack_command_journal::Selection> {
        p::apply_setup(
            &self.store,
            journal,
            &Coordinator::new(1).unwrap(),
            &self.observer,
            &self.authority,
            self.f.actor,
            apply,
        )
        .await
    }
    fn apply_request(&self, preview: &s::Preview) -> s::ApplyRequest {
        s::ApplyRequest {
            version: 1,
            request_id: self.request.request_id,
            intent_digest: self.request.intent_digest,
            prepared: preview.prepared,
            selection: self.request.selection.clone(),
        }
    }
}
#[tokio::test]
async fn registered_setup_creates_real_merge_wip_and_selects_combination_only_after_native_checkout()
 {
    let f = SetupInstalled::new().await;
    let journal = f.journal();
    let before = f.f.folder.state();
    let prepared = f.prepare(&journal).await.unwrap();
    assert_eq!(f.f.folder.state(), before);
    assert_eq!(f.store.snapshot().unwrap().revision, 0);
    assert!(
        f.store
            .current(
                f.f.binding.repository,
                f.request.selection.physical_workspace,
                &f.request.selection.jj_workspace
            )
            .await
            .is_err()
    );
    let record = journal
        .status(f.f.actor, f.request.request_id, f.request.intent_digest)
        .unwrap()
        .unwrap();
    assert!(record.execution.unwrap().combination.is_some());
    let status_request = s::StatusRequest {
        version: 1,
        scope: s::RecoveryScope::from(&f.request.selection),
        request_id: f.request.request_id,
        intent_digest: f.request.intent_digest,
    };
    let public = journal.public_status(f.f.actor, &status_request).unwrap();
    public.validate().unwrap();
    assert_eq!(public.result.as_ref().unwrap().preview, prepared.preview);
    let public_json = serde_json::to_value(&public).unwrap();
    let result_fields = public_json["result"].as_object().unwrap();
    assert_eq!(result_fields.len(), 4);
    assert!(!result_fields.contains_key("execution"));
    assert!(!result_fields.contains_key("principal"));
    let pending_request = s::PendingRequest {
        version: 1,
        scope: status_request.scope.clone(),
    };
    assert_eq!(
        journal
            .public_pending(f.f.actor, &pending_request)
            .unwrap()
            .commands[0]
            .preview,
        prepared.preview
    );
    assert_eq!(f.f.folder.state(), before);
    let replay = f.prepare(&journal).await.unwrap();
    assert!(replay.replayed);
    assert_eq!(replay.preview, prepared.preview);
    let done = f
        .apply(&journal, f.apply_request(&prepared.preview))
        .await
        .unwrap();
    assert!(done.receipt.completed());
    let selected = f.store.snapshot().unwrap();
    assert_eq!(selected.revision, 1);
    assert_eq!(selected.combinations.len(), 1);
    let combination = &selected.combinations[0];
    assert_eq!(combination.name, "Local workspace");
    assert_eq!(combination.selection, prepared.preview.plan.combination);
    assert!(combination.stacks.is_empty());
    let s::CodeSelection::GitSha1Jj045 { working_copy, .. } = combination.code else {
        panic!("Git profile");
    };
    assert_eq!(
        stock(
            &f.f.folder.root(),
            &[
                "--ignore-working-copy",
                "log",
                "--no-graph",
                "-r",
                "@",
                "-T",
                "commit_id"
            ]
        )
        .trim(),
        working_copy.to_string()
    );
    assert_eq!(
        stock(
            &f.f.folder.root(),
            &[
                "--ignore-working-copy",
                "log",
                "--no-graph",
                "-r",
                "@-",
                "-T",
                "commit_id"
            ]
        )
        .trim(),
        combination.merge.to_string()
    );
    assert_eq!(&f.f.folder.state()[..2], &before[..2]);
    assert_eq!(f.f.folder.state()[3], before[3]);
    assert_ne!(f.f.folder.state()[2], before[2]);
    let actual = f
        .authority
        .context(
            f.f.actor,
            &s::PrepareRequest::new(
                m::RequestId::new([96; 16]),
                s::Selection {
                    code: combination.code.clone(),
                    combination: Some(combination.selection.clone()),
                    ..f.request.selection.clone()
                },
                s::Command::Reword {
                    commit: s::CommitIdentity::GitSha1(working_copy),
                    message: "future context only".into(),
                },
            )
            .unwrap(),
        )
        .await
        .unwrap();
    assert_eq!(actual.combination, combination.selection);
    let before_public_read = f.f.folder.state();
    let public_catalog = f
        .store
        .public_catalog(f.f.actor, &status_request.scope)
        .unwrap();
    let public_state = s::WorkspaceState {
        version: 1,
        state: s::WorkspaceReadiness::Ready,
        selection: s::Selection {
            code: s::CodeSelection::GitSha1Jj045 {
                operation: actual.current.baseline.operation,
                view: actual.current.baseline.view,
                working_copy: actual.current.baseline.commit,
            },
            working_tree: actual.working_tree,
            combination: Some(actual.combination.clone()),
            ..f.request.selection.clone()
        },
        catalog: Some(public_catalog),
    };
    public_state
        .validate_request(&s::WorkspaceRequest {
            version: 1,
            repository: f.request.selection.repository,
            binding_generation: f.request.selection.binding_generation,
            operation: actual.current.baseline.operation,
        })
        .unwrap();
    assert_eq!(
        public_state
            .catalog
            .as_ref()
            .unwrap()
            .active()
            .unwrap()
            .target,
        combination.target
    );
    assert_eq!(f.f.folder.state(), before_public_read);
    std::fs::write(
        f.f.folder.root().join("file.txt"),
        "later unsnapshotted bytes\n",
    )
    .unwrap();
    let edited = f.f.folder.state();
    let replay = f
        .apply(&f.journal(), f.apply_request(&prepared.preview))
        .await
        .unwrap();
    assert!(replay.replayed);
    assert_eq!(replay.receipt, done.receipt);
    assert_eq!(f.f.folder.state(), edited);
    assert_eq!(f.store.snapshot().unwrap(), selected);
}
#[tokio::test]
async fn registered_setup_rejects_stale_physical_selection_without_publication() {
    let f = SetupInstalled::new().await;
    let journal = f.journal();
    let prepared = f.prepare(&journal).await.unwrap();
    std::fs::write(f.f.folder.root().join("file.txt"), "racing editor\n").unwrap();
    let before = f.f.folder.state();
    assert!(
        f.apply(&journal, f.apply_request(&prepared.preview))
            .await
            .is_err()
    );
    assert_eq!(f.f.folder.state(), before);
    assert_eq!(f.store.snapshot().unwrap().revision, 0);
    assert_eq!(
        journal
            .status(f.f.actor, f.request.request_id, f.request.intent_digest)
            .unwrap()
            .unwrap()
            .receipt
            .state,
        s::ReceiptState::Prepared
    );
}
#[tokio::test]
async fn registered_setup_does_not_recreate_missing_staged_state() {
    let f = SetupInstalled::new().await;
    let journal = f.journal();
    let prepared = f.prepare(&journal).await.unwrap();
    // Remove the actual state directory, not a caller-supplied snapshot.
    std::fs::remove_dir_all(f.f.state.join("setup-combinations")).unwrap();
    let before = f.f.folder.state();
    assert!(
        f.apply(&journal, f.apply_request(&prepared.preview))
            .await
            .is_err()
    );
    assert_eq!(f.f.folder.state(), before);
    assert!(!f.f.state.join("setup-combinations").exists());
}

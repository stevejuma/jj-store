#![cfg(unix)]
#[path = "stack_registered_preparation/apply.rs"]
mod apply;
#[path = "../../jj-store-jj/tests/stack_foreground/fixture.rs"]
mod fixture;
#[path = "stack_registered_preparation/policy.rs"]
mod policy;
#[path = "stack_registered_preparation/recovery.rs"]
mod recovery;
#[path = "stack_registered_preparation/setup.rs"]
mod setup;
use fixture::*;
use jj_store_cli::local_browser::LocalFolders;
use jj_store_http::Access;
use jj_store_jj::{local_browser as q, local_execution::Coordinator};
use jj_store_model as m;
use jj_store_server::{
    installation::{Identity, initialize_registry},
    local_repositories::DeviceRepositories,
    registry::Registry,
    stack_command_journal::{Profile, Scope, StackCommandJournal},
    stack_preparation::{
        self as p, ActiveCombination, Authority, CombinationAuthority, RegisteredAuthority,
    },
};
use jj_store_service::{
    auth::Permission, local_browser::*, stack_workspace as s, working_tree::Observer,
};
use std::sync::{Arc, Mutex};
struct Combinations(Mutex<Option<ActiveCombination>>);
#[async_trait::async_trait]
impl CombinationAuthority for Combinations {
    async fn current(
        &self,
        _repository: m::RepositoryId,
        _physical: s::PhysicalWorkspaceId,
        _workspace: &str,
    ) -> p::Result<Option<ActiveCombination>> {
        Ok(self.0.lock().unwrap().clone())
    }
}
struct Installed {
    folder: Fixture,
    _private: tempfile::TempDir,
    state: std::path::PathBuf,
    manager: Arc<DeviceRepositories>,
    binding: LocalBinding,
    actor: m::PrincipalId,
    installation: [u8; 16],
    combinations: Arc<Combinations>,
    authority: RegisteredAuthority,
    request: s::PrepareRequest,
}
impl Installed {
    async fn new() -> Self {
        let folder = Fixture::new().await;
        let current = folder.witness().await;
        let private = tempfile::tempdir().unwrap();
        let installed = initialize_registry(&private.path().join("state")).unwrap();
        let identity = Identity::load(&installed.installation_file).unwrap();
        let actor = identity.principal(b"browser");
        let installation = hex::decode(&identity.installation)
            .unwrap()
            .try_into()
            .unwrap();
        let manager = Arc::new(
            DeviceRepositories::new(
                installed.state_dir.clone(),
                identity.clone(),
                Arc::new(LocalFolders),
                Arc::new(Registry::new(installed.catalog, identity).unwrap()),
            )
            .unwrap(),
        );
        let RegistrationOutcome::Committed { binding, .. } = manager
            .register(RegisterFolder {
                version: 1,
                request_id: m::RequestId::new([81; 16]),
                expected_revision: 0,
                path: folder.root().to_str().unwrap().into(),
                name: "Stacks fixture".into(),
            })
            .await
            .unwrap()
        else {
            panic!("registration must commit")
        };
        let combination = s::CombinationSelection {
            id: s::CombinationId(m::RequestId::new([82; 16])),
            generation: 1,
        };
        let combinations = Arc::new(Combinations(Mutex::new(Some(ActiveCombination {
            selection: combination.clone(),
            code: s::CodeSelection::GitSha1Jj045 {
                operation: current.baseline.operation,
                view: current.baseline.view,
                working_copy: current.baseline.commit,
            },
            immutable_heads: vec![m::GitSha1Id::new([0; 20])],
        }))));
        let observer = Observer::new(q::Executor::new(2).unwrap(), 1).unwrap();
        let authority = RegisteredAuthority::new(
            &Access::LocalTrusted {
                principal: actor,
                permission: Permission::Admin,
            },
            manager.clone(),
            combinations.clone(),
            observer,
        )
        .unwrap();
        let mut request = s::PrepareRequest::new(
            m::RequestId::new([83; 16]),
            s::Selection {
                version: 1,
                repository: binding.repository,
                binding_generation: binding.generation,
                physical_workspace: s::PhysicalWorkspaceId(current.physical_workspace),
                jj_workspace: current.checkout.workspace.clone(),
                code: s::CodeSelection::GitSha1Jj045 {
                    operation: current.baseline.operation,
                    view: current.baseline.view,
                    working_copy: current.baseline.commit,
                },
                combination: Some(combination),
                working_tree: m::RecordDigest::new([1; 32]),
            },
            s::Command::Reword {
                commit: s::CommitIdentity::GitSha1(current.baseline.commit),
                message: "Registered exact preparation".into(),
            },
        )
        .unwrap();
        let context = authority.context(actor, &request).await.unwrap();
        request.selection.working_tree = context.working_tree;
        request.intent_digest = request.intent().unwrap();
        Self {
            folder,
            _private: private,
            state: installed.state_dir,
            manager,
            binding,
            actor,
            installation,
            combinations,
            authority,
            request,
        }
    }
    fn journal(&self) -> StackCommandJournal {
        StackCommandJournal::create(
            &self.state.join("stack-journal"),
            Scope {
                installation: self.installation,
                repository: self.binding.repository,
                physical_workspace: self.request.selection.physical_workspace,
                jj_workspace: self.request.selection.jj_workspace.clone(),
                profile: Profile::GitSha1Jj045,
            },
        )
        .unwrap()
    }
}

#[tokio::test]
async fn registered_authority_executes_retained_reword_with_real_physical_guard_and_historical_retry()
 {
    let f = Installed::new().await;
    let journal = f.journal();
    let coordinator = Coordinator::new(1).unwrap();
    let saved = p::prepare_reword(
        &journal,
        &coordinator,
        &f.authority,
        f.actor,
        f.request.clone(),
    )
    .await
    .unwrap();
    let apply = s::ApplyRequest {
        version: 1,
        request_id: f.request.request_id,
        intent_digest: f.request.intent_digest,
        prepared: saved.preview.prepared,
        selection: f.request.selection.clone(),
    };
    let before = f.folder.state();
    let observer = Observer::new(q::Executor::new(1).unwrap(), 1).unwrap();
    let done = p::apply_reword(
        &journal,
        &coordinator,
        &observer,
        &f.authority,
        f.actor,
        apply.clone(),
    )
    .await
    .unwrap();
    assert!(done.receipt.completed());
    assert_eq!(&f.folder.state()[..2], &before[..2]);
    assert_eq!(f.folder.state()[3], before[3]);
    assert_ne!(f.folder.state()[2], before[2]);
    // The active combination remains an explicit isolated fixture. This proves
    // installed registration/physical policy and execution, not its selection.
    std::fs::write(
        f.folder.root().join("file.txt"),
        "new editor bytes after receipt\n",
    )
    .unwrap();
    let edited = f.folder.state();
    let replay = p::apply_reword(
        &journal,
        &coordinator,
        &observer,
        &f.authority,
        f.actor,
        apply,
    )
    .await
    .unwrap();
    assert!(replay.replayed);
    assert_eq!(replay.receipt, done.receipt);
    assert_eq!(f.folder.state(), edited);
}
#[tokio::test]
async fn registered_folder_and_real_observation_gate_preparation_and_original_replay() {
    let f = Installed::new().await;
    let before = f.folder.state();
    let journal = f.journal();
    let coordinator = Coordinator::new(1).unwrap();
    let saved = p::prepare_reword(
        &journal,
        &coordinator,
        &f.authority,
        f.actor,
        f.request.clone(),
    )
    .await
    .unwrap();
    assert_eq!(f.folder.state(), before);
    std::fs::write(
        f.folder.root().join("file.txt"),
        "Changed by external editor\n",
    )
    .unwrap();
    let edited = f.folder.state();
    let original = p::prepare_reword(
        &journal,
        &coordinator,
        &f.authority,
        f.actor,
        f.request.clone(),
    )
    .await
    .unwrap();
    assert_eq!(original.preview, saved.preview);
    assert_eq!(original.receipt, saved.receipt);
    let mut stale = f.request.clone();
    stale.request_id = m::RequestId::new([85; 16]);
    assert!(matches!(
        p::prepare_reword(&journal, &coordinator, &f.authority, f.actor, stale).await,
        Err(p::Error::Stale)
    ));
    assert_eq!(f.folder.state(), edited);
    f.manager
        .remove(
            f.binding.repository,
            RemoveBinding {
                version: 1,
                expected_revision: f.manager.list().unwrap().revision,
            },
        )
        .unwrap();
    assert!(matches!(
        p::prepare_reword(
            &journal,
            &coordinator,
            &f.authority,
            f.actor,
            f.request.clone()
        )
        .await,
        Err(p::Error::Forbidden)
    ));
    assert_eq!(f.folder.state(), edited);
}
#[tokio::test]
async fn caller_mode_missing_combination_and_immutable_ancestry_reject_before_selection() {
    let f = Installed::new().await;
    let observer = || Observer::new(q::Executor::new(1).unwrap(), 1).unwrap();
    for access in [
        Access::Authenticated,
        Access::LocalTrusted {
            principal: f.actor,
            permission: Permission::Read,
        },
    ] {
        assert!(
            RegisteredAuthority::new(
                &access,
                f.manager.clone(),
                f.combinations.clone(),
                observer()
            )
            .is_err()
        );
    }
    let before = f.folder.state();
    let journal = f.journal();
    let coordinator = Coordinator::new(1).unwrap();
    assert!(matches!(
        p::prepare_reword(
            &journal,
            &coordinator,
            &f.authority,
            m::PrincipalId::new([86; 16]),
            f.request.clone()
        )
        .await,
        Err(p::Error::Forbidden)
    ));
    let mut stale_binding = f.request.clone();
    stale_binding.selection.binding_generation += 1;
    stale_binding.intent_digest = stale_binding.intent().unwrap();
    assert!(matches!(
        p::prepare_reword(&journal, &coordinator, &f.authority, f.actor, stale_binding).await,
        Err(p::Error::Stale)
    ));
    let mut stale_combination = f.combinations.0.lock().unwrap().clone().unwrap();
    stale_combination.code = s::CodeSelection::GitSha1Jj045 {
        operation: m::OperationId::new([99; 64]),
        view: m::ViewId::new([98; 64]),
        working_copy: m::GitSha1Id::new([97; 20]),
    };
    *f.combinations.0.lock().unwrap() = Some(stale_combination);
    assert!(matches!(
        p::prepare_reword(
            &journal,
            &coordinator,
            &f.authority,
            f.actor,
            f.request.clone()
        )
        .await,
        Err(p::Error::Stale)
    ));
    *f.combinations.0.lock().unwrap() = None;
    assert!(matches!(
        p::prepare_reword(
            &journal,
            &coordinator,
            &f.authority,
            f.actor,
            f.request.clone()
        )
        .await,
        Err(p::Error::Unsupported)
    ));
    let s::Command::Reword {
        commit: s::CommitIdentity::GitSha1(commit),
        ..
    } = f.request.command
    else {
        panic!()
    };
    *f.combinations.0.lock().unwrap() = Some(ActiveCombination {
        selection: f.request.selection.combination.clone().unwrap(),
        code: f.request.selection.code.clone(),
        immutable_heads: vec![commit],
    });
    assert!(matches!(
        p::prepare_reword(
            &journal,
            &coordinator,
            &f.authority,
            f.actor,
            f.request.clone()
        )
        .await,
        Err(p::Error::Provider(q::Error::Denied))
    ));
    assert!(journal.pending(f.actor).unwrap().is_empty());
    assert_eq!(f.folder.state(), before);
}

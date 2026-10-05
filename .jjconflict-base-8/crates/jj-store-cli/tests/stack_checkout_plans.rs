#![cfg(unix)]
#[path = "stack_checkout_plans/candidate.rs"]
mod candidate;
#[path = "../../jj-store-jj/tests/stack_foreground/fixture.rs"]
mod fixture;
#[path = "stack_checkout_plans/installation.rs"]
mod installation;
#[path = "stack_checkout_plans/installation_faults.rs"]
mod installation_faults;
#[path = "stack_checkout_plans/installation_process.rs"]
mod installation_process;
#[path = "stack_checkout_plans/inventory.rs"]
mod inventory;
#[path = "stack_checkout_plans/inventory_process.rs"]
mod inventory_process;
#[path = "stack_checkout_plans/linkage.rs"]
mod linkage;
#[path = "stack_checkout_plans/process.rs"]
mod process;
#[path = "stack_checkout_plans/storage.rs"]
mod storage;
use fixture::Fixture;
use jj_store_jj::{
    local_browser::{self, working_tree::content_digest},
    local_execution::{Coordinator, PhysicalGuard, RecoveryBinding, TreeCheckoutPlanV1},
    local_metadata::{Limits, MetadataStore},
};
use jj_store_model as m;
use jj_store_server::{
    stack_checkout_plans::StackCheckoutPlans,
    stack_command_journal::{
        Error, LocalExecutionV1, Profile, Scope, StackCommandJournal, preparation_key,
    },
};
use jj_store_service::stack_workspace as s;
use std::{fs, path::PathBuf, sync::Arc};
struct Files {
    root: PathBuf,
}
#[async_trait::async_trait]
impl PhysicalGuard for Files {
    async fn validate(
        &self,
        _: &RecoveryBinding,
        expected: m::RecordDigest,
    ) -> local_browser::Result<()> {
        if expected != content_digest(b"original\n")
            || fs::read(self.root.join("file.txt")).unwrap() != b"original\n"
        {
            return Err(local_browser::Error::EpochChanged);
        }
        Ok(())
    }
}
struct Input {
    folder: Fixture,
    scope: Scope,
    preview: s::Preview,
    execution: LocalExecutionV1,
    plan: TreeCheckoutPlanV1,
}
impl Input {
    async fn new() -> Self {
        Self::build(false).await
    }
    async fn build(real_physical: bool) -> Self {
        Self::build_effects(
            real_physical,
            vec![
                ("file.txt", Some((b"candidate\n", true))),
                ("src/new", Some((b"new\n", false))),
            ],
            |_| {},
        )
        .await
    }
    async fn build_effects(
        real_physical: bool,
        changes: candidate::FileChanges<'_>,
        setup: impl FnOnce(&std::path::Path),
    ) -> Self {
        let folder = Fixture::new().await;
        fs::write(folder.root().join("file.txt"), b"original\n").unwrap();
        setup(&folder.root());
        let paths = changes.iter().map(|(path, _)| (*path).into()).collect();
        let state = folder.witness().await;
        let physical = if real_physical {
            jj_store_service::working_tree::scan(
                &folder.root(),
                state.baseline.repository,
                state.baseline.generation,
                Default::default(),
                |_| Ok(()),
            )
            .unwrap()
            .fingerprint
        } else {
            content_digest(b"original\n")
        };
        let archive = candidate::candidate(&folder, changes).await;
        let capsule = archive.capsule().unwrap();
        let after = archive.views[&capsule.view_id].wc_commit_ids["default"];
        let combination = s::CombinationSelection {
            id: s::CombinationId(m::RequestId::new([2; 16])),
            generation: 1,
        };
        let request = s::PrepareRequest::new(
            m::RequestId::new([3; 16]),
            s::Selection {
                version: 1,
                repository: state.baseline.repository,
                binding_generation: 1,
                physical_workspace: s::PhysicalWorkspaceId(state.physical_workspace),
                jj_workspace: "default".into(),
                code: s::CodeSelection::GitSha1Jj045 {
                    operation: state.baseline.operation,
                    view: state.baseline.view,
                    working_copy: state.baseline.commit,
                },
                combination: Some(combination.clone()),
                working_tree: physical,
            },
            s::Command::ApplyStack {
                stack: s::StackId(m::RequestId::new([4; 16])),
                head: s::CommitIdentity::GitSha1(after),
            },
        )
        .unwrap();
        let plan = s::PreparedPlan {
            version: 1,
            request,
            operation: capsule.operation_id,
            view: capsule.view_id,
            checkout: s::CheckoutEffect {
                before: s::TreeIdentity::GitSha1(state.baseline.trees[0]).into(),
                after: s::TreeIdentity::GitSha1(archive.commits[&after].root_tree[0]).into(),
                paths,
            },
            combination,
            rewrites: vec![],
            bookmarks: vec![],
            conflicts: vec![],
        };
        let preview = s::Preview {
            prepared: plan.identity().unwrap(),
            plan,
        };
        let coordinator = Coordinator::new(1).unwrap();
        let lease = coordinator
            .acquire(folder.access(), folder.root(), state.clone())
            .await
            .unwrap();
        let (lease, witness) = lease.recovery_witness().await.unwrap();
        let key = preparation_key(actor(), &preview);
        let metadata = MetadataStore::open(&folder.root().join(".git"), Limits::default()).unwrap();
        metadata.retain_stack_execution(&key, &archive).unwrap();
        let retained = metadata.read_stack_prepared(&key).unwrap().unwrap();
        let execution =
            LocalExecutionV1::from_retention(actor(), &preview, witness.clone(), &retained)
                .unwrap();
        drop(lease);
        let recovered = coordinator
            .restore_history(folder.access(), folder.root(), witness, key)
            .await
            .unwrap();
        let before = folder.state();
        let guard: Arc<dyn PhysicalGuard> = if real_physical {
            jj_store_service::working_tree::Observer::new(
                local_browser::Executor::new(1).unwrap(),
                1,
            )
            .unwrap()
            .physical_guard(folder.access(), folder.root())
        } else {
            Arc::new(Files {
                root: folder.root(),
            })
        };
        let (recovered, plan) = recovered
            .prepare_tree_checkout(guard, preview.plan.request.selection.working_tree)
            .await
            .unwrap();
        drop(recovered);
        assert_eq!(before, folder.state());
        let scope = Scope {
            installation: [1; 16],
            repository: state.baseline.repository,
            physical_workspace: s::PhysicalWorkspaceId(state.physical_workspace),
            jj_workspace: "default".into(),
            profile: Profile::GitSha1Jj045,
        };
        Self {
            folder,
            scope,
            preview,
            execution,
            plan,
        }
    }
    async fn inventory(&self) -> jj_store_jj::local_execution::PhysicalInventoryV1 {
        jj_store_service::working_tree::Observer::new(local_browser::Executor::new(1).unwrap(), 1)
            .unwrap()
            .physical_guard(self.folder.access(), self.folder.root())
            .inventory(&self.execution.witness.binding(), self.plan.physical())
            .await
            .unwrap()
    }
    fn journal(&self, path: &std::path::Path) -> StackCommandJournal {
        let journal = StackCommandJournal::create(path, self.scope.clone()).unwrap();
        journal
            .record_local_prepared(actor(), &self.preview, &self.execution)
            .unwrap();
        journal
    }
    fn apply(&self) -> s::ApplyRequest {
        let request = &self.preview.plan.request;
        s::ApplyRequest {
            version: 1,
            request_id: request.request_id,
            intent_digest: request.intent_digest,
            prepared: self.preview.prepared,
            selection: request.selection.clone(),
        }
    }
}
fn actor() -> m::PrincipalId {
    m::PrincipalId::new([5; 16])
}
fn json(path: &std::path::Path) -> serde_json::Value {
    serde_json::from_slice(&fs::read(path.join("state.json")).unwrap()).unwrap()
}
fn private_write(path: &std::path::Path, bytes: &[u8]) {
    use std::os::unix::fs::PermissionsExt as _;
    fs::write(path, bytes).unwrap();
    fs::set_permissions(path, fs::Permissions::from_mode(0o600)).unwrap();
}

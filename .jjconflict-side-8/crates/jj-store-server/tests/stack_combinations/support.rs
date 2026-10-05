use super::*;
use jj_store_jj::{
    local_execution::{Coordinator, Reword},
    local_metadata::{StackRetention, StackRetentionOutcome},
};
use jj_store_model as m;
use jj_store_server::stack_command_journal::{
    LocalExecutionV1, Profile, Scope, StackCommandJournal, preparation_key,
};
use jj_store_service::stack_workspace as s;
pub struct Prepared {
    pub folder: Fixture,
    pub private: tempfile::TempDir,
    pub scope: Scope,
    pub actor: m::PrincipalId,
    pub preview: s::Preview,
    pub after: c::Snapshot,
    pub retained: StackRetention,
    pub journal: StackCommandJournal,
    pub target: m::GitSha1Id,
}
impl Prepared {
    pub async fn new() -> Self {
        let folder = Fixture::new().await;
        let current = folder.witness().await;
        let actor = m::PrincipalId::new([72; 16]);
        let scope = Scope {
            installation: [75; 16],
            repository: current.baseline.repository,
            physical_workspace: s::PhysicalWorkspaceId(current.physical_workspace),
            jj_workspace: current.checkout.workspace.clone(),
            profile: Profile::GitSha1Jj045,
        };
        let coordinator = Coordinator::new(1).unwrap();
        let lease = coordinator
            .acquire(folder.access(), folder.root(), current.clone())
            .await
            .unwrap();
        let (lease, witness) = lease.recovery_witness().await.unwrap();
        let (lease, candidate) = lease
            .prepare_reword(Reword {
                principal: actor,
                commit: current.baseline.commit,
                message: "Unpublished candidate for device-state fixture".into(),
                immutable_heads: vec![],
            })
            .await
            .unwrap();
        let capsule = candidate.history.capsule().unwrap();
        let target = m::GitSha1Id::new([0; 20]);
        let bookmark = s::BookmarkTarget {
            name: "main".into(),
            remote: None,
            head: s::CommitIdentity::GitSha1(target),
        };
        let selection = s::Selection {
            version: 1,
            repository: scope.repository,
            binding_generation: current.baseline.generation,
            physical_workspace: scope.physical_workspace,
            jj_workspace: scope.jj_workspace.clone(),
            code: s::CodeSelection::GitSha1Jj045 {
                operation: current.baseline.operation,
                view: current.baseline.view,
                working_copy: current.baseline.commit,
            },
            combination: None,
            working_tree: m::RecordDigest::new([71; 32]),
        };
        // This fixture qualifies private metadata coordination only. Setup/merge and
        // publication/checkout remain separate provider/runtime acceptance gates.
        let request = s::PrepareRequest::new(
            m::RequestId::new([73; 16]),
            selection,
            s::Command::Setup {
                target: bookmark.clone(),
                name: "Workspace".into(),
            },
        )
        .unwrap();
        let combination = s::CombinationSelection {
            id: s::CombinationId(m::RequestId::new([74; 16])),
            generation: 1,
        };
        let plan = s::PreparedPlan {
            version: 1,
            request,
            operation: capsule.operation_id,
            view: capsule.view_id,
            checkout: s::CheckoutEffect {
                before: s::TreeSelection {
                    terms: vec![s::TreeIdentity::GitSha1(current.baseline.trees[0])],
                },
                after: s::TreeSelection {
                    terms: vec![s::TreeIdentity::GitSha1(current.baseline.trees[0])],
                },
                paths: vec![],
            },
            combination: combination.clone(),
            rewrites: vec![],
            bookmarks: vec![],
            conflicts: vec![],
        };
        let preview = s::Preview {
            prepared: plan.identity().unwrap(),
            plan,
        };
        let key = preparation_key(actor, &preview);
        let (lease, retained) = lease
            .retain_history(witness.clone(), key, candidate.history)
            .await
            .unwrap();
        drop(lease);
        let StackRetentionOutcome::Retained(retained) = retained else {
            panic!()
        };
        let private = tempfile::tempdir().unwrap();
        let journal =
            StackCommandJournal::create(&private.path().join("journal"), scope.clone()).unwrap();
        let execution =
            LocalExecutionV1::from_retention(actor, &preview, witness, &retained).unwrap();
        journal
            .record_local_prepared(actor, &preview, &execution)
            .unwrap();
        let wip = capsule.view.wc_commit_ids[&scope.jj_workspace];
        let after = c::Snapshot {
            version: 1,
            revision: 1,
            active: Some(combination.id),
            combinations: vec![c::Combination {
                selection: combination,
                name: "Workspace".into(),
                target: bookmark,
                immutable_heads: vec![target],
                merge: target,
                code: s::CodeSelection::GitSha1Jj045 {
                    operation: capsule.operation_id,
                    view: capsule.view_id,
                    working_copy: wip,
                },
                stacks: vec![],
                retention: c::RetainedCode {
                    key: retained.key.clone(),
                    commit: retained.commit,
                    archive: retained.history.as_ref().unwrap().id().unwrap(),
                },
            }],
        };
        Self {
            folder,
            private,
            scope,
            actor,
            preview,
            after,
            retained: *retained,
            journal,
            target,
        }
    }
    pub fn apply(&self) -> s::ApplyRequest {
        s::ApplyRequest {
            version: 1,
            request_id: self.preview.plan.request.request_id,
            intent_digest: self.preview.plan.request.intent_digest,
            prepared: self.preview.prepared,
            selection: self.preview.plan.request.selection.clone(),
        }
    }
    pub fn mark_completed(&self) {
        // This metadata-owner fixture loads an explicit historical version-2
        // outcome. It has never qualified Setup execution or checkout. Current
        // local execution receipts require actual provider proofs, so they must
        // not be fabricated through the runtime transition API.
        let done = s::ReceiptState::Published {
            operation: self.preview.plan.operation,
            view: self.preview.plan.view,
            checkout: s::CheckoutState::Verified {
                fingerprint: m::RecordDigest::new([90; 32]),
            },
        };
        let path = self.private.path().join("journal/state.json");
        let mut saved: serde_json::Value =
            serde_json::from_slice(&std::fs::read(&path).unwrap()).unwrap();
        assert_eq!(saved["version"], 2);
        saved["records"][0]["receipt"]["state"] = serde_json::to_value(done).unwrap();
        std::fs::write(path, serde_json::to_vec(&saved).unwrap()).unwrap();
    }
}

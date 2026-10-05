#![cfg(unix)]
#[path = "stack_local_preparation/admission.rs"]
mod admission;
#[path = "../../jj-store-jj/tests/stack_foreground/fixture.rs"]
mod fixture;
#[path = "stack_local_preparation/lifecycle.rs"]
mod lifecycle;
#[path = "stack_local_preparation/persisted.rs"]
mod persisted;
use fixture::*;
use jj_store_jj::{
    local_execution::Coordinator,
    local_metadata::{Limits, MetadataStore},
};
use jj_store_model as m;
use jj_store_server::{
    stack_command_journal::{Profile, Scope, StackCommandJournal},
    stack_preparation::{self, Authority, Context},
};
use jj_store_service::stack_workspace as s;
use std::{path::PathBuf, sync::Arc};

struct Allowed {
    access: Arc<dyn jj_store_jj::local_browser::Access>,
    root: PathBuf,
    state: jj_store_jj::local_browser::working_tree::CurrentState,
}
#[async_trait::async_trait]
impl Authority for Allowed {
    async fn authorize(
        &self,
        _principal: m::PrincipalId,
        _request: &s::PrepareRequest,
    ) -> stack_preparation::Result<()> {
        Ok(())
    }
    async fn context(
        &self,
        _principal: m::PrincipalId,
        _request: &s::PrepareRequest,
    ) -> stack_preparation::Result<Context> {
        Ok(Context {
            access: self.access.clone(),
            root: self.root.clone(),
            current: self.state.clone(),
            working_tree: m::RecordDigest::new([71; 32]),
            combination: request(&self.state).selection.combination.unwrap(),
        })
    }
}
fn actor() -> m::PrincipalId {
    m::PrincipalId::new([72; 16])
}
fn request(state: &jj_store_jj::local_browser::working_tree::CurrentState) -> s::PrepareRequest {
    s::PrepareRequest::new(
        m::RequestId::new([73; 16]),
        s::Selection {
            version: 1,
            repository: state.baseline.repository,
            binding_generation: state.baseline.generation,
            physical_workspace: s::PhysicalWorkspaceId(state.physical_workspace),
            jj_workspace: state.checkout.workspace.clone(),
            code: s::CodeSelection::GitSha1Jj045 {
                operation: state.baseline.operation,
                view: state.baseline.view,
                working_copy: state.baseline.commit,
            },
            combination: Some(s::CombinationSelection {
                id: s::CombinationId(m::RequestId::new([74; 16])),
                generation: 1,
            }),
            working_tree: m::RecordDigest::new([71; 32]),
        },
        s::Command::Reword {
            commit: s::CommitIdentity::GitSha1(state.baseline.commit),
            message: "Real server prepared reword".into(),
        },
    )
    .unwrap()
}
fn scope(state: &jj_store_jj::local_browser::working_tree::CurrentState) -> Scope {
    Scope {
        installation: [75; 16],
        repository: state.baseline.repository,
        physical_workspace: s::PhysicalWorkspaceId(state.physical_workspace),
        jj_workspace: state.checkout.workspace.clone(),
        profile: Profile::GitSha1Jj045,
    }
}

#[tokio::test]
async fn real_preparation_joins_exact_preview_retention_and_private_recovery_witness() {
    let f = Fixture::new().await;
    let before = f.state();
    let state = f.witness().await;
    let req = request(&state);
    let private = tempfile::tempdir().unwrap();
    let path = private.path().join("stacks");
    let journal = StackCommandJournal::create(&path, scope(&state)).unwrap();
    let auth = Allowed {
        access: f.access(),
        root: f.root(),
        state,
    };
    let saved = stack_preparation::prepare_reword(
        &journal,
        &Coordinator::new(1).unwrap(),
        &auth,
        actor(),
        req.clone(),
    )
    .await
    .unwrap();
    saved.preview.validate().unwrap();
    let record = journal
        .status(actor(), req.request_id, req.intent_digest)
        .unwrap()
        .unwrap();
    let execution = record.execution.as_ref().unwrap();
    let retained = MetadataStore::open(&f.root().join(".git"), Limits::default())
        .unwrap()
        .read_stack_prepared(&execution.key)
        .unwrap()
        .unwrap();
    assert_eq!(execution.retention, Some(retained.commit));
    assert_eq!(execution.archive, retained.history.unwrap().id().unwrap());
    assert_eq!(execution.key.prepared, saved.preview.prepared.0);
    assert_eq!(execution.key.intent, req.intent_digest);
    assert_eq!(execution.key.principal, actor());
    assert_eq!(execution.witness.id().unwrap(), execution.witness_id);
    assert_eq!(f.state(), before);
    drop(journal);
    let reopened = StackCommandJournal::open(&path, scope(&auth.state)).unwrap();
    assert_eq!(
        reopened
            .status(actor(), req.request_id, req.intent_digest)
            .unwrap()
            .unwrap(),
        record
    );
}

struct NoLiveContext;
#[async_trait::async_trait]
impl Authority for NoLiveContext {
    async fn authorize(
        &self,
        _principal: m::PrincipalId,
        _request: &s::PrepareRequest,
    ) -> stack_preparation::Result<()> {
        Ok(())
    }
    async fn context(
        &self,
        _principal: m::PrincipalId,
        _request: &s::PrepareRequest,
    ) -> stack_preparation::Result<Context> {
        panic!("historical exact replay must precede any live context")
    }
}
#[tokio::test]
async fn exact_preparation_replay_survives_source_loss_without_live_validation_or_reprepare() {
    let f = Fixture::new().await;
    let state = f.witness().await;
    let req = request(&state);
    let private = tempfile::tempdir().unwrap();
    let path = private.path().join("stacks");
    let journal = StackCommandJournal::create(&path, scope(&state)).unwrap();
    let auth = Allowed {
        access: f.access(),
        root: f.root(),
        state,
    };
    let saved = stack_preparation::prepare_reword(
        &journal,
        &Coordinator::new(1).unwrap(),
        &auth,
        actor(),
        req.clone(),
    )
    .await
    .unwrap();
    std::fs::remove_dir_all(f.root().join(".jj/repo/op_store")).unwrap();
    let replay = stack_preparation::prepare_reword(
        &journal,
        &Coordinator::new(1).unwrap(),
        &NoLiveContext,
        actor(),
        req,
    )
    .await
    .unwrap();
    assert!(replay.replayed);
    assert_eq!(replay.preview, saved.preview);
    assert_eq!(replay.receipt, saved.receipt);
    assert!(!f.root().join(".jj/repo/op_store").exists());
}

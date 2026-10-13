use jj_store_model::{EpochId, ObjectKey, PrincipalId, PublicationId, RepositoryId, RequestId};
use jj_store_object::memory::MemoryStore;
use jj_store_wal::{
    self as wal,
    inventory::{DependencySource, Representation, SourceScope, VerifiedNode},
    publication::{
        Coordinator, DomainRequest, Limits, ManifestSnapshot, ProjectId, PublicationGuard,
        PublishOutcome, Request, Snapshot, WorkCommandRequest, WorkCommandResultV1, WorkCommandV1,
    },
};
use std::{collections::BTreeSet, sync::Arc};
#[path = "workflow_store/fault.rs"]
mod fault;

const REPOSITORY: RepositoryId = RepositoryId::new([1; 16]);
const ACTOR: PrincipalId = PrincipalId::new([3; 16]);

struct Source(SourceScope);
#[async_trait::async_trait]
impl DependencySource for Source {
    fn scope(&self) -> SourceScope {
        self.0
    }
    async fn load_verified(&self, key: ObjectKey, _: u64) -> wal::inventory::Result<VerifiedNode> {
        Ok(VerifiedNode {
            key,
            representation: Representation::Virtual,
            dependencies: BTreeSet::new(),
        })
    }
}

struct Permit;
#[async_trait::async_trait]
impl PublicationGuard for Permit {
    async fn authorize_replication_seed(
        &self,
        _: &ManifestSnapshot,
        _: &wal::publication::SeedReplicationRequestV1,
    ) -> wal::Result<()> {
        Ok(())
    }
    async fn authorize(&self, _: &ManifestSnapshot, _: &Request) -> wal::Result<()> {
        Ok(())
    }
    async fn validate_transition(
        &self,
        _: &ManifestSnapshot,
        _: &Request,
        _: &Snapshot,
    ) -> wal::Result<()> {
        Ok(())
    }
    async fn authorize_domain(&self, _: &ManifestSnapshot, _: &DomainRequest) -> wal::Result<()> {
        Ok(())
    }
}

fn coordinator(store: Arc<dyn jj_store_object::ObjectStore>) -> Coordinator {
    Coordinator::new(
        store,
        REPOSITORY,
        Arc::new(Source(SourceScope::new(REPOSITORY))),
        Arc::new(Permit),
        Limits::default(),
    )
    .unwrap()
}
#[tokio::test]
async fn workflow_manifest_timeouts_keep_atomic_visibility_and_original_receipts() {
    for after in [false, true] {
        let store = Arc::new(fault::FaultStore::default());
        let c = coordinator(store.clone());
        c.initialize(EpochId::new([2; 16]), Vec::new())
            .await
            .unwrap();
        apply(
            &c,
            9,
            WorkCommandV1::CreateProject {
                id: ProjectId::new([9; 16]),
                title: "Automation".into(),
                body: String::new(),
            },
        )
        .await;
        let before = c.read().await.unwrap();
        let request = workflow_request(
            ACTOR,
            10,
            w::Command::Put {
                workflow: w::WorkflowId::new([10; 16]),
                expected: None,
                definition: definition(),
            },
        );
        store.arm(after);
        let outcome = tokio::time::timeout(
            std::time::Duration::from_millis(50),
            c.publish_workflow_command(&request),
        )
        .await;
        if !after {
            assert!(matches!(
                outcome.unwrap().unwrap(),
                PublishOutcome::OutcomeUnknown { .. }
            ));
        } else {
            assert!(outcome.is_err());
        }
        let cold = coordinator(store);
        let snapshot = cold.read().await.unwrap();
        assert_eq!(snapshot.manifest().workflow_root.is_some(), after);
        if !after {
            assert_eq!(snapshot.digest(), before.digest());
        }
        let selected = cold
            .receipt_at(&snapshot, ACTOR, request.request_id)
            .await
            .unwrap();
        assert_eq!(selected.is_some(), after);
        let PublishOutcome::Committed(receipt) =
            cold.publish_workflow_command(&request).await.unwrap()
        else {
            panic!("retry committed")
        };
        if after {
            assert_eq!(receipt, selected.unwrap());
        }
        cold.checkpoint().await.unwrap();
        cold.recover(10000).await.unwrap();
        let PublishOutcome::Committed(replay) =
            cold.publish_workflow_command(&request).await.unwrap()
        else {
            panic!("replay committed")
        };
        assert_eq!(replay, receipt);
        let state = cold
            .workflow_store(cold.read().await.unwrap().manifest().workflow_root.unwrap())
            .await
            .unwrap();
        assert_eq!(state.state().definitions.len(), 1);
    }
}
async fn fixture() -> (Arc<MemoryStore>, Coordinator) {
    let store = Arc::new(MemoryStore::default());
    let coordinator = coordinator(store.clone());
    coordinator
        .initialize(EpochId::new([2; 16]), Vec::new())
        .await
        .unwrap();
    (store, coordinator)
}
fn request(byte: u8, command: WorkCommandV1) -> WorkCommandRequest {
    WorkCommandRequest::new(
        ACTOR,
        RequestId::new([byte; 16]),
        PublicationId::new([byte; 16]),
        command,
    )
    .unwrap()
}
async fn apply(c: &Coordinator, byte: u8, command: WorkCommandV1) -> WorkCommandResultV1 {
    let PublishOutcome::Committed(receipt) = c
        .publish_work_command(&request(byte, command))
        .await
        .unwrap()
    else {
        panic!("commit required")
    };
    serde_json::from_slice(&receipt.result).unwrap()
}

use jj_workflow as w;
fn definition() -> w::Definition {
    w::Definition {
        name: "Build".into(),
        description: String::new(),
        enabled: true,
        repository: REPOSITORY,
        trigger: w::Trigger::CodePublished,
        labels: BTreeSet::new(),
        actions: vec![w::Action {
            id: "build".into(),
            executor: w::Executor::Command {
                name: "build".into(),
            },
            inputs: Default::default(),
            timeout_seconds: 300,
        }],
    }
}
fn workflow_request(
    actor: PrincipalId,
    byte: u8,
    command: w::Command,
) -> wal::publication::WorkflowCommandRequest {
    wal::publication::WorkflowCommandRequest::new(
        actor,
        RequestId::new([byte; 16]),
        PublicationId::new([byte; 16]),
        command,
    )
    .unwrap()
}
async fn workflow_apply(c: &Coordinator, byte: u8, command: w::Command) -> w::CommandResult {
    let PublishOutcome::Committed(receipt) = c
        .publish_workflow_command(&workflow_request(ACTOR, byte, command))
        .await
        .unwrap()
    else {
        panic!("expected committed")
    };
    serde_json::from_slice(&receipt.result).unwrap()
}
#[tokio::test]
async fn one_event_one_run_one_owner_with_recovery() {
    let (store, c) = fixture().await;
    apply(
        &c,
        9,
        WorkCommandV1::CreateProject {
            id: ProjectId::new([9; 16]),
            title: "Automation".into(),
            body: String::new(),
        },
    )
    .await;
    let workflow = w::WorkflowId::new([10; 16]);
    workflow_apply(
        &c,
        10,
        w::Command::Put {
            workflow,
            expected: None,
            definition: definition(),
        },
    )
    .await;
    let event = w::SourceEvent::new(
        "origin/replica/epoch/1".into(),
        w::EventKind::CodePublished,
        w::Source {
            repository: REPOSITORY,
            operation: Some(jj_store_model::OperationId::new([2; 64])),
            commit: Some(jj_store_model::CommitId::new([4; 64])),
            bookmark: None,
            conversation: None,
        },
    )
    .unwrap();
    let a = workflow_request(
        ACTOR,
        11,
        w::Command::Evaluate {
            definitions: None,
            event: event.clone(),
        },
    );
    let b = workflow_request(
        PrincipalId::new([5; 16]),
        12,
        w::Command::Evaluate {
            definitions: None,
            event,
        },
    );
    let (left, right) = tokio::join!(
        c.publish_workflow_command(&a),
        c.publish_workflow_command(&b)
    );
    assert!(matches!(left.unwrap(), PublishOutcome::Committed(_)));
    assert!(matches!(right.unwrap(), PublishOutcome::Committed(_)));
    let snap = c.read().await.unwrap();
    assert_eq!(snap.manifest().format, 6);
    let state = c
        .workflow_store(snap.manifest().workflow_root.unwrap())
        .await
        .unwrap();
    assert_eq!(state.state().runs.len(), 1);
    let run = state.state().runs.values().next().unwrap();
    let claim_a = workflow_request(
        ACTOR,
        13,
        w::Command::Claim {
            run: run.id,
            attempt: run.current().id,
            host: "host-a".into(),
        },
    );
    let claim_b = workflow_request(
        PrincipalId::new([5; 16]),
        14,
        w::Command::Claim {
            run: run.id,
            attempt: run.current().id,
            host: "host-b".into(),
        },
    );
    let (left, right) = tokio::join!(
        c.publish_workflow_command(&claim_a),
        c.publish_workflow_command(&claim_b)
    );
    assert_eq!(
        [left.is_ok(), right.is_ok()]
            .into_iter()
            .filter(|v| *v)
            .count(),
        1
    );
    let selected = c
        .workflow_store(c.read().await.unwrap().manifest().workflow_root.unwrap())
        .await
        .unwrap();
    let selected = selected.state().runs.values().next().unwrap();
    let owner = selected.current().owner.unwrap();
    let report = workflow_request(
        owner,
        20,
        w::Command::Report {
            run: selected.id,
            attempt: selected.current().id,
            sequence: 0,
            data: w::EvidenceData::Artifact {
                name: "result.txt".into(),
                media_type: "text/plain".into(),
                bytes: b"durable artifact".to_vec(),
            },
        },
    );
    assert!(matches!(
        c.publish_workflow_command(&report).await.unwrap(),
        PublishOutcome::Committed(_)
    ));
    // A later WorkStore publication must preserve the format-6 workflow root.
    apply(
        &c,
        21,
        WorkCommandV1::CreateProject {
            id: ProjectId::new([21; 16]),
            title: "Second project".into(),
            body: String::new(),
        },
    )
    .await;
    c.checkpoint().await.unwrap();
    c.recover(10000).await.unwrap();
    let fresh = coordinator(store);
    let after = fresh.read().await.unwrap();
    assert!(after.manifest().workflow_root.is_some());
    let state = fresh
        .workflow_store(after.manifest().workflow_root.unwrap())
        .await
        .unwrap();
    assert_eq!(state.state().runs.len(), 1);
    assert!(
        state
            .state()
            .runs
            .values()
            .next()
            .unwrap()
            .current()
            .owner
            .is_some()
    );
    assert_eq!(state.state().evidence.len(), 1);
    let dest = Arc::new(MemoryStore::default());
    fresh
        .copy_native_to(
            dest.clone(),
            Arc::new(Source(SourceScope::new(REPOSITORY))),
            10_000,
        )
        .await
        .unwrap();
    let copied = coordinator(dest);
    let snapshot = copied.recover(10_000).await.unwrap();
    let copy = copied
        .workflow_store(snapshot.manifest().workflow_root.unwrap())
        .await
        .unwrap();
    assert_eq!(copy.state(), state.state());
}

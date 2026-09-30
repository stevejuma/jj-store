use jj_store_model::*;
use jj_store_object::{ObjectStore, memory::MemoryStore};
use jj_store_replication::{TransferLimits, transfer_closures};
use jj_store_wal::{
    self as wal,
    inventory::{DependencySource, InventoryError, SourceScope, VerifiedNode},
    publication::*,
};
use std::{collections::BTreeMap, sync::Arc};

const REPOSITORY: RepositoryId = RepositoryId::new([151; 16]);
const SESSION: AgentSessionId = AgentSessionId::new([152; 16]);

struct Permit;

#[async_trait::async_trait]
impl PublicationGuard for Permit {
    async fn authorize(&self, _: &ManifestSnapshot, _: &Request) -> wal::Result<()> {
        Ok(())
    }
    async fn authorize_domain(&self, _: &ManifestSnapshot, _: &DomainRequest) -> wal::Result<()> {
        Ok(())
    }
    async fn authorize_replication(
        &self,
        _: &ManifestSnapshot,
        _: &ApplyOriginsRequestV1,
    ) -> wal::Result<()> {
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
}

#[derive(Clone)]
struct VirtualSource;

#[async_trait::async_trait]
impl DependencySource for VirtualSource {
    fn scope(&self) -> SourceScope {
        SourceScope::new(REPOSITORY)
    }
    async fn load_verified(&self, key: ObjectKey, _: u64) -> wal::inventory::Result<VerifiedNode> {
        if key == ObjectKey::Operation(OperationId::new([0; 64])) {
            Ok(VerifiedNode {
                key,
                representation: wal::inventory::Representation::Virtual,
                dependencies: Default::default(),
            })
        } else {
            Err(InventoryError::Source("unexpected native object".into()))
        }
    }
}

struct Replica {
    coordinator: Coordinator,
    store: Arc<dyn ObjectStore>,
}

impl Replica {
    async fn new(seed: u8) -> Self {
        let store: Arc<dyn ObjectStore> = Arc::new(MemoryStore::default());
        let coordinator = Coordinator::new(
            store.clone(),
            REPOSITORY,
            Arc::new(VirtualSource),
            Arc::new(Permit),
            Limits::default(),
        )
        .unwrap();
        coordinator
            .initialize_replica(
                EpochId::new([seed; 16]),
                vec![seed],
                ReplicaId::new([seed.wrapping_add(1); 16]),
                WriterEpoch::new([seed.wrapping_add(2); 16]),
            )
            .await
            .unwrap();
        Self { coordinator, store }
    }

    async fn feed(&self) -> Vec<OriginEnvelopeV1> {
        let snapshot = self.coordinator.read().await.unwrap();
        self.coordinator
            .committed_origin_feed(&snapshot, WalPosition(0), 32)
            .await
            .unwrap()
            .origins
            .into_iter()
            .map(|item| item.envelope)
            .collect()
    }

    async fn transfer_to(&self, destination: &Replica) {
        let snapshot = self.coordinator.read().await.unwrap();
        let feed = self
            .coordinator
            .committed_origin_feed(&snapshot, WalPosition(0), 32)
            .await
            .unwrap();
        let mut closures = Vec::new();
        for item in feed.origins {
            closures.push(
                self.coordinator
                    .origin_closure(&snapshot, item.envelope.publication.origin_key, 20_000)
                    .await
                    .unwrap(),
            );
        }
        transfer_closures(
            self.store.clone(),
            destination.store.clone(),
            REPOSITORY,
            &closures,
            TransferLimits::default(),
        )
        .await
        .unwrap();
    }

    async fn cold_copy(&self) -> Self {
        self.coordinator.checkpoint().await.unwrap();
        let store: Arc<dyn ObjectStore> = Arc::new(MemoryStore::default());
        let source: Arc<dyn DependencySource> = Arc::new(VirtualSource);
        self.coordinator
            .copy_native_to(store.clone(), source.clone(), 512)
            .await
            .unwrap();
        let coordinator = Coordinator::new(
            store.clone(),
            REPOSITORY,
            source,
            Arc::new(Permit),
            Limits::default(),
        )
        .unwrap();
        coordinator.recover(512).await.unwrap();
        Self { coordinator, store }
    }
}

async fn publish_session(replica: &Replica, seed: u8, tools: &[(u8, ToolState)]) {
    let mut selected = BTreeMap::new();
    for (tool_seed, state) in tools {
        let tool_id = ToolCallId::new([*tool_seed; 16]);
        let reference = replica
            .coordinator
            .stage_agent_object(&AgentObjectV1::ToolSnapshot(ToolInvocationSnapshotV1 {
                schema_version: 1,
                tool_call_id: tool_id,
                session_id: SESSION,
                turn_id: None,
                native_name: format!("tool-{tool_seed}"),
                kind: ToolKind::Execute,
                state: *state,
                attempts: Default::default(),
                input: None,
                output: None,
                evidence: Vec::new(),
            }))
            .await
            .unwrap();
        selected.insert(tool_id, reference);
    }
    let session = AgentSessionSnapshotV1 {
        schema_version: 1,
        session_id: SESSION,
        integration: IntegrationId::Codex,
        state: SessionState::Active,
        parent_session_id: None,
        evidence: Vec::new(),
        event_batches: Vec::new(),
        transcripts: Vec::new(),
        tools: selected,
        coverage: CoverageV1 {
            completeness: CompletenessV1::Partial,
            gaps: 0,
            redacted_fields: 0,
            truncated: false,
        },
    };
    let key = session.catalog_key().unwrap();
    let replacement = replica
        .coordinator
        .stage_agent_object(&AgentObjectV1::SessionSnapshot(session))
        .await
        .unwrap();
    let request = AgentDeltaRequest::new(
        PrincipalId::new([160; 16]),
        RequestId::new([seed; 16]),
        PublicationId::new([seed; 16]),
        AgentDeltaV1 {
            schema_version: 1,
            edits: vec![AgentCatalogEditV1 {
                catalog: AgentCatalogKind::Sessions,
                key,
                expected: None,
                replacement: Some(replacement),
            }],
        },
        vec![seed],
    )
    .unwrap();
    assert!(matches!(
        replica
            .coordinator
            .publish_agent_delta(&request)
            .await
            .unwrap(),
        PublishOutcome::Committed(_)
    ));
}

async fn merge(first: &Replica, second: &Replica, seed: u8, reverse: bool) -> Replica {
    let destination = Replica::new(seed).await;
    first.transfer_to(&destination).await;
    second.transfer_to(&destination).await;
    let mut origins = first
        .feed()
        .await
        .into_iter()
        .chain(second.feed().await)
        .map(|origin| (origin.publication.origin_key, origin))
        .collect::<BTreeMap<_, _>>()
        .into_values()
        .collect::<Vec<_>>();
    if reverse {
        origins.reverse();
    }
    destination
        .coordinator
        .apply_origins(&ApplyOriginsRequestV1 {
            schema_version: 1,
            principal: PrincipalId::new([seed; 16]),
            request_id: RequestId::new([seed.wrapping_add(1); 16]),
            publication_id: PublicationId::new([seed.wrapping_add(2); 16]),
            origins,
        })
        .await
        .unwrap();
    destination
}

async fn selected_session(replica: &Replica) -> AgentObjectV1 {
    let snapshot = replica.coordinator.read().await.unwrap();
    let store = replica
        .coordinator
        .agent_store(snapshot.manifest().domains().unwrap().agent.root)
        .await
        .unwrap();
    let key = AgentCatalogKey::derive("session", SESSION.as_bytes()).unwrap();
    store
        .read_object(
            store
                .lookup(AgentCatalogKind::Sessions, key)
                .await
                .unwrap()
                .unwrap(),
        )
        .await
        .unwrap()
}

#[tokio::test]
async fn session_facts_union_and_terminal_conflicts_remain_explicit() {
    let first = Replica::new(1).await;
    let second = Replica::new(11).await;
    publish_session(&first, 21, &[(31, ToolState::Running)]).await;
    publish_session(&second, 22, &[(32, ToolState::Succeeded)]).await;
    let forward = merge(&first, &second, 41, false).await;
    let reverse = merge(&first, &second, 51, true).await;
    let AgentObjectV1::SessionSnapshot(session) = selected_session(&forward).await else {
        panic!("disjoint session facts should reduce to one session")
    };
    assert_eq!(session.tools.len(), 2);
    let forward_root = forward
        .coordinator
        .read()
        .await
        .unwrap()
        .manifest()
        .domains()
        .unwrap()
        .agent
        .root;
    let reverse_root = reverse
        .coordinator
        .read()
        .await
        .unwrap()
        .manifest()
        .domains()
        .unwrap()
        .agent
        .root;
    assert_eq!(forward_root, reverse_root);

    let failed = Replica::new(61).await;
    let succeeded = Replica::new(71).await;
    publish_session(&failed, 81, &[(91, ToolState::Failed)]).await;
    publish_session(&succeeded, 82, &[(91, ToolState::Succeeded)]).await;
    let conflicted = merge(&failed, &succeeded, 101, true).await;
    let AgentObjectV1::CatalogConflict(conflict) = selected_session(&conflicted).await else {
        panic!("incomparable terminal outcomes should stay explicit")
    };
    assert_eq!(conflict.catalog, AgentCatalogKind::Sessions);
    assert_eq!(conflict.alternatives.len(), 2);

    let cold = conflicted.cold_copy().await;
    let AgentObjectV1::CatalogConflict(cold_conflict) = selected_session(&cold).await else {
        panic!("cold copy should retain the explicit terminal conflict")
    };
    assert_eq!(cold_conflict, conflict);
}

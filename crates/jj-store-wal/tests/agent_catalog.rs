use jj_agent::{
    AgentCatalogEditV1, AgentCatalogKey, AgentCatalogKind, AgentDeltaV1, AgentObjectRef,
    AgentObjectV1, AgentSessionId, AgentSessionSnapshotV1, CompletenessV1, CoverageV1,
    IntegrationId, SessionState,
};
use jj_store_model::{EpochId, ObjectKey, OperationId, RepositoryId};
use jj_store_object::memory::MemoryStore;
use jj_store_wal::{
    self as wal,
    inventory::{DependencySource, Representation, SourceScope, VerifiedNode},
    publication::{Coordinator, Limits, PublicationGuard, Request, Snapshot},
};
use std::{collections::BTreeMap, collections::BTreeSet, sync::Arc};

const REPOSITORY: RepositoryId = RepositoryId::new([1; 16]);

struct Source(SourceScope);
#[async_trait::async_trait]
impl DependencySource for Source {
    fn scope(&self) -> SourceScope {
        self.0
    }
    async fn load_verified(&self, key: ObjectKey, _: u64) -> wal::inventory::Result<VerifiedNode> {
        let expected = ObjectKey::Operation(OperationId::new([0; 64]));
        if key != expected {
            return Err(wal::inventory::InventoryError::Source(
                "unexpected fixture object".into(),
            ));
        }
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
    async fn authorize(
        &self,
        _: &wal::publication::ManifestSnapshot,
        _: &Request,
    ) -> wal::Result<()> {
        Ok(())
    }
    async fn validate_transition(
        &self,
        _: &wal::publication::ManifestSnapshot,
        _: &Request,
        _: &Snapshot,
    ) -> wal::Result<()> {
        Ok(())
    }
}

async fn fixture() -> Coordinator {
    let coordinator = Coordinator::new(
        Arc::new(MemoryStore::default()),
        REPOSITORY,
        Arc::new(Source(SourceScope::new(REPOSITORY))),
        Arc::new(Permit),
        Limits::default(),
    )
    .unwrap();
    coordinator
        .initialize(EpochId::new([2; 16]), Vec::new())
        .await
        .unwrap();
    coordinator
}

fn session(byte: u8) -> AgentObjectV1 {
    AgentObjectV1::SessionSnapshot(AgentSessionSnapshotV1 {
        schema_version: 1,
        session_id: AgentSessionId::new([byte; 16]),
        integration: IntegrationId::Codex,
        state: SessionState::Active,
        parent_session_id: None,
        evidence: Vec::new(),
        event_batches: Vec::new(),
        transcripts: Vec::new(),
        tools: BTreeMap::new(),
        coverage: CoverageV1 {
            completeness: CompletenessV1::Complete,
            gaps: 0,
            redacted_fields: 0,
            truncated: false,
        },
    })
}

fn edit(
    key: AgentCatalogKey,
    expected: Option<AgentObjectRef>,
    replacement: AgentObjectRef,
) -> AgentCatalogEditV1 {
    AgentCatalogEditV1 {
        catalog: AgentCatalogKind::Sessions,
        key,
        expected,
        replacement: Some(replacement),
    }
}

#[tokio::test]
async fn persistent_catalog_splits_and_keeps_old_root_pinned() {
    let coordinator = fixture().await;
    let initial = coordinator.read().await.unwrap();
    let initial_root = initial.manifest().agent_root.unwrap();
    let pinned = coordinator.agent_store(initial_root).await.unwrap();
    let mut selections = Vec::new();
    let mut edits = Vec::new();
    for byte in 100..140 {
        let value = coordinator
            .stage_agent_object(&session(byte))
            .await
            .unwrap();
        let key = AgentCatalogKey::derive("session", &[byte; 16]).unwrap();
        selections.push((key, value));
        edits.push(edit(key, None, value));
    }
    let first_root = coordinator
        .stage_agent_delta(
            &pinned,
            &AgentDeltaV1 {
                schema_version: 1,
                edits,
            },
        )
        .await
        .unwrap();
    let first = coordinator.agent_store(first_root).await.unwrap();
    for (key, value) in &selections {
        assert_eq!(
            first
                .lookup(AgentCatalogKind::Sessions, *key)
                .await
                .unwrap(),
            Some(*value)
        );
    }

    let replacement = coordinator.stage_agent_object(&session(200)).await.unwrap();
    let (key, previous) = selections[17];
    let second_root = coordinator
        .stage_agent_delta(
            &first,
            &AgentDeltaV1 {
                schema_version: 1,
                edits: vec![edit(key, Some(previous), replacement)],
            },
        )
        .await
        .unwrap();
    let second = coordinator.agent_store(second_root).await.unwrap();
    assert_eq!(
        first.lookup(AgentCatalogKind::Sessions, key).await.unwrap(),
        Some(previous)
    );
    assert_eq!(
        second
            .lookup(AgentCatalogKind::Sessions, key)
            .await
            .unwrap(),
        Some(replacement)
    );
    let (neighbor_key, neighbor) = selections[18];
    assert_eq!(
        second
            .lookup(AgentCatalogKind::Sessions, neighbor_key)
            .await
            .unwrap(),
        Some(neighbor)
    );
}

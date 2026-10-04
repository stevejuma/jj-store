use jj_agent::*;
use jj_store_model::{EpochId, ObjectKey, OperationId, RepositoryId, portable::*};
use jj_store_object::memory::MemoryStore;
use jj_store_wal::{
    self as wal,
    inventory::{DependencySource, Representation, SourceScope, VerifiedNode},
    publication::*,
};
use std::{
    collections::{BTreeMap, BTreeSet},
    sync::Arc,
};
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
    async fn authorize_domain(
        &self,
        _: &wal::publication::ManifestSnapshot,
        _: &wal::publication::DomainRequest,
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

#[tokio::test]
async fn shared_native_provenance_projects_stale_safe_catalog_and_relations() {
    let coordinator = fixture().await;
    let root = coordinator
        .read()
        .await
        .unwrap()
        .manifest()
        .agent_root
        .unwrap();
    let empty = coordinator.agent_store(root).await.unwrap();
    let session = AgentSessionSnapshotV1 {
        schema_version: 1,
        session_id: AgentSessionId::new([2; 16]),
        integration: IntegrationId::Codex,
        state: SessionState::Active,
        parent_session_id: None,
        evidence: vec![],
        event_batches: vec![],
        transcripts: vec![],
        tools: BTreeMap::new(),
        coverage: CoverageV1 {
            completeness: CompletenessV1::Complete,
            gaps: 0,
            redacted_fields: 0,
            truncated: false,
        },
    };
    let reference = coordinator
        .stage_agent_object(&AgentObjectV1::SessionSnapshot(session.clone()))
        .await
        .unwrap();
    let root = coordinator
        .stage_agent_delta(
            &empty,
            &AgentDeltaV1 {
                schema_version: 1,
                edits: vec![AgentCatalogEditV1 {
                    catalog: AgentCatalogKind::Sessions,
                    key: session.catalog_key().unwrap(),
                    expected: None,
                    replacement: Some(reference),
                }],
            },
        )
        .await
        .unwrap();
    let base = coordinator.agent_store(root).await.unwrap();
    let value = CodeAssociationV1 {
        schema_version: 1,
        association_id: AssociationId::new([3; 16]),
        checkpoint_id: None,
        session_id: session.session_id,
        turn_id: None,
        tool_call_ids: BTreeSet::new(),
        scope_id: None,
        relation: AssociationRelation::ManualAssertion,
        before: CodeAnchorV1::default(),
        after: CodeAnchorV1 {
            operations: BTreeSet::from([OperationId::new([0; 64])]),
            commits: BTreeSet::new(),
        },
        changes: BTreeSet::new(),
        paths: vec![],
        granularity: AssociationGranularity::Operation,
        evidence_method: EvidenceMethod::ManualAssertion,
        source_trust: SourceTrust::LocalUserHook,
        exclusive_write_scope: false,
        evidence: vec![],
        coverage: session.coverage,
        created_at_millis: 1,
        supersedes: None,
        correction_reason: None,
    };
    let object = AgentObjectV1::Association(value.clone());
    let delta = AgentProjector::new(&coordinator)
        .stage_agent_provenance(&base, None, &object)
        .await
        .unwrap();
    let root = coordinator.stage_agent_delta(&base, &delta).await.unwrap();
    let current = coordinator.agent_store(root).await.unwrap();
    assert_eq!(
        current
            .relations(
                &AgentRelationSubjectV1::Session {
                    session_id: session.session_id
                },
                None,
                20
            )
            .await
            .unwrap()
            .selections
            .len(),
        1
    );
    assert!(matches!(
        AgentProjector::new(&coordinator)
            .stage_agent_provenance(&current, None, &object)
            .await,
        Err(wal::Error::Stale)
    ));
    let portable = AgentObjectV1::PortableAssociation(CodeAssociationV2 {
        schema_version: 2,
        before: CodeAnchorV2::default(),
        after: CodeAnchorV2 {
            operations: BTreeSet::from([CodeEvidenceRefV1 {
                repository: REPOSITORY,
                profile: CodeProfileV1::GitSha1Jj045,
                operation: OperationId::new([0; 64]),
                capsule: jj_store_model::CodeCapsuleId::new([4; 32]),
            }]),
            commits: BTreeSet::new(),
        },
        facts: value.facts(),
    });
    assert!(matches!(
        coordinator.stage_agent_object(&portable).await,
        Err(wal::Error::Unsupported(_))
    ));
}

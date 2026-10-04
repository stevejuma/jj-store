use jj_agent::{
    AgentCheckpointId, AgentCheckpointVersionV1, AgentObjectId, AgentObjectKind, AgentObjectRef,
    AgentObjectV1, AgentSessionId, AssociationGranularity, AssociationId, AssociationRelation,
    CaptureStatusV1, CodeAnchorV1, CodeAssociationV1, CodeRelation, CompletenessV1, CoverageV1,
    EvidenceMethod, ExecutionOutcomeV1, ProvenanceScopeId, SourceTrust, ToolCallId, TurnId,
    decode_object, encode_object,
};
use jj_store_model::{ChangeId, CommitId, ObjectKey, OperationId};
use std::collections::{BTreeMap, BTreeSet};

fn reference(kind: AgentObjectKind, byte: u8) -> AgentObjectRef {
    AgentObjectRef::new(kind, AgentObjectId::new([byte; 32]))
}

fn anchor(operation: u8, commit: u8) -> CodeAnchorV1 {
    CodeAnchorV1 {
        operations: BTreeSet::from([OperationId::new([operation; 64])]),
        commits: BTreeSet::from([CommitId::new([commit; 64])]),
    }
}

fn coverage() -> CoverageV1 {
    CoverageV1 {
        completeness: CompletenessV1::Complete,
        gaps: 0,
        redacted_fields: 0,
        truncated: false,
    }
}

fn association() -> CodeAssociationV1 {
    CodeAssociationV1 {
        schema_version: 1,
        association_id: AssociationId::new([1; 16]),
        checkpoint_id: Some(AgentCheckpointId::new([2; 16])),
        session_id: AgentSessionId::new([3; 16]),
        turn_id: Some(TurnId::new([4; 16])),
        tool_call_ids: BTreeSet::from([ToolCallId::new([5; 16])]),
        scope_id: Some(ProvenanceScopeId::new([6; 16])),
        relation: AssociationRelation::ContributedToSnapshot,
        before: anchor(7, 8),
        after: anchor(9, 10),
        changes: BTreeSet::from([ChangeId::new([11; 16])]),
        paths: vec!["src/lib.rs".into()],
        granularity: AssociationGranularity::File,
        evidence_method: EvidenceMethod::HookBoundary,
        source_trust: SourceTrust::LocalUserHook,
        exclusive_write_scope: false,
        evidence: vec![reference(AgentObjectKind::ObservationBatch, 12)],
        coverage: coverage(),
        created_at_millis: 1_790_000_000_000,
        supersedes: None,
        correction_reason: None,
    }
}

#[test]
fn associations_separate_agent_edges_from_native_retention_roots() {
    let value = association();
    assert_eq!(
        value.dependencies().unwrap(),
        BTreeSet::from([reference(AgentObjectKind::ObservationBatch, 12)])
    );
    assert_eq!(
        value.native_dependencies().unwrap(),
        BTreeSet::from([
            ObjectKey::Operation(OperationId::new([7; 64])),
            ObjectKey::Commit(CommitId::new([8; 64])),
            ObjectKey::Operation(OperationId::new([9; 64])),
            ObjectKey::Commit(CommitId::new([10; 64])),
        ])
    );
    let object = AgentObjectV1::Association(value);
    assert_eq!(
        decode_object(&encode_object(&object).unwrap()).unwrap(),
        object
    );
}

#[test]
fn checkpoints_pin_exact_agent_versions_and_native_anchors() {
    let checkpoint = AgentCheckpointVersionV1 {
        schema_version: 1,
        checkpoint_id: AgentCheckpointId::new([20; 16]),
        session_id: AgentSessionId::new([21; 16]),
        turn_id: Some(TurnId::new([22; 16])),
        scope_id: Some(ProvenanceScopeId::new([23; 16])),
        session_snapshot: reference(AgentObjectKind::SessionSnapshot, 24),
        transcript_manifest: reference(AgentObjectKind::TranscriptManifest, 25),
        event_batches: vec![reference(AgentObjectKind::EventBatch, 26)],
        tools: BTreeMap::from([(
            ToolCallId::new([27; 16]),
            reference(AgentObjectKind::ToolSnapshot, 28),
        )]),
        before: anchor(29, 30),
        after: anchor(31, 32),
        code_relation: CodeRelation::Changed,
        associations: vec![reference(AgentObjectKind::Association, 33)],
        evidence_method: EvidenceMethod::ControlledScope,
        source_trust: SourceTrust::AuthenticatedController,
        exclusive_write_scope: true,
        coverage: coverage(),
        capture_status: CaptureStatusV1::Complete,
        execution_outcome: ExecutionOutcomeV1::Succeeded,
        supersedes: None,
    };
    assert_eq!(checkpoint.dependencies().unwrap().len(), 5);
    assert_eq!(checkpoint.native_dependencies().unwrap().len(), 4);
    let object = AgentObjectV1::CheckpointVersion(checkpoint);
    assert_eq!(
        decode_object(&encode_object(&object).unwrap()).unwrap(),
        object
    );
}

#[test]
fn invalid_causality_and_paths_fail_closed() {
    let mut invalid = association();
    invalid.paths = vec!["/private/source.rs".into()];
    assert!(invalid.validate().is_err());

    let mut no_change = association();
    no_change.before = anchor(1, 2);
    no_change.after = anchor(3, 4);
    no_change.relation = AssociationRelation::ObservedDuringInterval;
    assert!(
        AgentCheckpointVersionV1::validate_code_relation(
            &no_change.before,
            &no_change.after,
            CodeRelation::NoChange,
        )
        .is_err()
    );

    let mut corrected = association();
    corrected.supersedes = Some(reference(AgentObjectKind::Association, 40));
    assert!(corrected.validate().is_err());
    corrected.correction_reason = Some("native source corrected the range".into());
    corrected.validate().unwrap();
}

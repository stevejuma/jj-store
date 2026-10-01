use jj_agent::*;
use jj_store_model::{CodeCapsuleId, GitSha1Id, OperationId, RepositoryId, portable::*};
use std::collections::{BTreeMap, BTreeSet};

fn native() -> CodeAssociationV1 {
    let AgentObjectV1::Association(value) =
        decode_object(include_bytes!("fixtures/native-association-v1.json")).unwrap()
    else {
        panic!("association fixture")
    };
    value
}
fn evidence(id: u8) -> CodeEvidenceRefV1 {
    CodeEvidenceRefV1 {
        repository: RepositoryId::new([1; 16]),
        profile: CodeProfileV1::GitSha1Jj045,
        operation: OperationId::new([id; 64]),
        capsule: CodeCapsuleId::new([id; 32]),
    }
}
fn anchor(id: u8) -> CodeAnchorV2 {
    CodeAnchorV2 {
        operations: BTreeSet::from([evidence(id)]),
        commits: BTreeSet::from([CodeCommitEvidenceV2 {
            evidence: evidence(id),
            commit: CodeCommitIdV1::GitSha1Jj045(GitSha1Id::new([id; 20])),
        }]),
    }
}
fn association() -> CodeAssociationV2 {
    CodeAssociationV2 {
        schema_version: 2,
        before: anchor(1),
        after: anchor(2),
        facts: native().facts(),
    }
}
fn checkpoint() -> AgentCheckpointVersionV2 {
    let reference = |kind, id| AgentObjectRef::new(kind, AgentObjectId::new([id; 32]));
    AgentCheckpointVersionV2 {
        schema_version: 2,
        before: anchor(1),
        after: anchor(2),
        code_relation: CodeRelation::Changed,
        facts: AgentCheckpointFactsV1 {
            checkpoint_id: AgentCheckpointId::new([1; 16]),
            session_id: AgentSessionId::new([2; 16]),
            turn_id: None,
            scope_id: None,
            session_snapshot: reference(AgentObjectKind::SessionSnapshot, 3),
            transcript_manifest: reference(AgentObjectKind::TranscriptManifest, 4),
            event_batches: vec![reference(AgentObjectKind::EventBatch, 5)],
            tools: BTreeMap::new(),
            associations: vec![reference(AgentObjectKind::Association, 6)],
            evidence_method: EvidenceMethod::HookBoundary,
            source_trust: SourceTrust::LocalUserHook,
            exclusive_write_scope: false,
            coverage: CoverageV1 {
                completeness: CompletenessV1::Partial,
                gaps: 1,
                redacted_fields: 0,
                truncated: false,
            },
            capture_status: CaptureStatusV1::Partial,
            execution_outcome: ExecutionOutcomeV1::Failed,
            supersedes: None,
        },
    }
}
#[test]
fn portable_provenance_retains_typed_source_evidence_and_native_v1_bytes() {
    let legacy = native();
    assert_eq!(
        encode_object(&AgentObjectV1::Association(legacy.clone())).unwrap(),
        include_bytes!("fixtures/native-association-v1.json")
    );
    let value = association();
    let object = AgentObjectV1::PortableAssociation(value.clone());
    assert_eq!(
        decode_object(&encode_object(&object).unwrap()).unwrap(),
        object
    );
    assert_eq!(object.kind(), AgentObjectKind::Association);
    assert!(object.native_dependencies().unwrap().is_empty());
    assert_eq!(
        object.code_evidence().unwrap(),
        value.code_evidence().unwrap()
    );
    assert_eq!(value.facts, legacy.facts());
    assert_eq!(
        CodeAssociationV2::decode(&value.encode().unwrap()).unwrap(),
        value
    );
    assert_eq!(
        value.dependencies().unwrap(),
        legacy.dependencies().unwrap()
    );
    assert_eq!(
        value.code_evidence().unwrap(),
        BTreeSet::from([evidence(1), evidence(2)])
    );
    assert_eq!(
        value.before.commits.first().unwrap().commit,
        CodeCommitIdV1::GitSha1Jj045(GitSha1Id::new([1; 20]))
    );
    assert_eq!(legacy.native_dependencies().unwrap().len(), 4);
}
#[test]
fn portable_associations_share_paths_trust_correction_and_evidence_policy() {
    let invalid = [
        {
            let mut f = native().facts();
            f.paths = vec!["../secret".into()];
            f
        },
        {
            let mut f = native().facts();
            f.evidence[0].kind = AgentObjectKind::SessionSnapshot;
            f
        },
        {
            let mut f = native().facts();
            f.relation = AssociationRelation::ManualAssertion;
            f
        },
        {
            let mut f = native().facts();
            f.supersedes = Some(AgentObjectRef::new(
                AgentObjectKind::Association,
                AgentObjectId::new([9; 32]),
            ));
            f
        },
    ];
    for facts in invalid {
        assert!(facts.validate().is_err());
        let mut portable = association();
        portable.facts = facts.clone();
        assert!(portable.validate().is_err());
        let mut legacy = serde_json::to_value(native()).unwrap();
        for (key, value) in serde_json::to_value(facts).unwrap().as_object().unwrap() {
            legacy[key] = value.clone();
        }
        assert!(
            serde_json::from_value::<CodeAssociationV1>(legacy)
                .unwrap()
                .validate()
                .is_err()
        );
    }
    let mut corrected = association();
    corrected.facts.supersedes = Some(AgentObjectRef::new(
        AgentObjectKind::Association,
        AgentObjectId::new([9; 32]),
    ));
    corrected.facts.correction_reason = Some("Exact source evidence was corrected".into());
    corrected.validate().unwrap();
    assert_eq!(corrected.dependencies().unwrap().len(), 2);
}
#[test]
fn portable_checkpoint_keeps_code_capture_and_execution_outcomes_independent() {
    let mut value = checkpoint();
    let object = AgentObjectV1::PortableCheckpointVersion(value.clone());
    assert_eq!(
        decode_object(&encode_object(&object).unwrap()).unwrap(),
        object
    );
    assert_eq!(object.kind(), AgentObjectKind::CheckpointVersion);
    value.validate().unwrap();
    assert_eq!(
        AgentCheckpointVersionV2::decode(&value.encode().unwrap()).unwrap(),
        value
    );
    assert_eq!(value.dependencies().unwrap().len(), 4);
    value.code_relation = CodeRelation::NoChange;
    assert!(value.validate().is_err());
    value.after = value.before.clone();
    value.validate().unwrap();
    value.code_relation = CodeRelation::Changed;
    assert!(value.validate().is_err());
    value.code_relation = CodeRelation::NotObserved;
    assert!(value.validate().is_err());
    value.after = CodeAnchorV2::default();
    value.validate().unwrap();
    value.facts.capture_status = CaptureStatusV1::Complete;
    assert!(value.validate().is_err());
    value.facts.coverage.completeness = CompletenessV1::Complete;
    value.facts.coverage.gaps = 0;
    value.validate().unwrap();
    assert_eq!(value.facts.execution_outcome, ExecutionOutcomeV1::Failed);
}
#[test]
fn mixed_profiles_unknown_versions_and_noncanonical_portable_bytes_fail_closed() {
    let value = association();
    let mut mixed = value.clone();
    let mut commit = mixed.before.commits.pop_first().unwrap();
    commit.evidence.profile = CodeProfileV1::NativeV1;
    mixed.before.commits.insert(commit);
    assert!(mixed.validate().is_err());
    let mut wrong = value.clone();
    wrong.schema_version = 1;
    assert!(wrong.validate().is_err());
    let mut bytes = value.encode().unwrap();
    bytes.push(b'\n');
    assert!(CodeAssociationV2::decode(&bytes).is_err());
    let mut unknown = serde_json::to_value(&value).unwrap();
    unknown["extra"] = serde_json::json!(true);
    assert!(CodeAssociationV2::decode(&serde_json::to_vec(&unknown).unwrap()).is_err());
    let mut over = value;
    over.before.operations = (0..257)
        .map(|n| {
            let mut e = evidence(1);
            let mut id = [0; 64];
            id[..2].copy_from_slice(&(n as u16).to_be_bytes());
            e.operation = OperationId::new(id);
            e
        })
        .collect();
    assert!(over.validate().is_err());
}

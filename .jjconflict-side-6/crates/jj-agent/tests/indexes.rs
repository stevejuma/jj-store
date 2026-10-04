use jj_agent::{
    AgentAliasV1, AgentCatalogEditV1, AgentCatalogKey, AgentCatalogKind, AgentCheckpointId,
    AgentDeltaV1, AgentEntityRefV1, AgentObjectId, AgentObjectKind, AgentObjectRef, AgentObjectV1,
    AgentProjectionCatalogV1, AgentRelationPageV1, AgentRelationSubjectV1, CaptureBatchId,
    CaptureSourceReceiptV1, CaptureSourceStateV1, CompletenessV1, CoverageV1, IntegrationId,
    NativeEntityKind, SourceInstanceId, SourcePositionV1,
};

fn reference(kind: AgentObjectKind, byte: u8) -> AgentObjectRef {
    AgentObjectRef::new(kind, AgentObjectId::new([byte; 32]))
}

fn coverage() -> CoverageV1 {
    CoverageV1 {
        completeness: CompletenessV1::Partial,
        gaps: 1,
        redacted_fields: 0,
        truncated: false,
    }
}

#[test]
fn auxiliary_catalog_objects_validate_keys_and_strong_edges() {
    let source_id = SourceInstanceId::new([1; 16]);
    let alias = AgentAliasV1 {
        schema_version: 1,
        integration: IntegrationId::Codex,
        source_instance_id: source_id,
        native_kind: NativeEntityKind::Session,
        native_id: "session-17".into(),
        target: AgentEntityRefV1::Session {
            session_id: jj_agent::AgentSessionId::new([2; 16]),
        },
    };
    assert_eq!(alias.catalog_key().unwrap(), alias.catalog_key().unwrap());
    assert!(AgentObjectV1::Alias(alias).validate().is_ok());

    let subject = AgentRelationSubjectV1::Session {
        session_id: jj_agent::AgentSessionId::new([2; 16]),
    };
    let relation = AgentRelationPageV1 {
        schema_version: 1,
        subject: subject.clone(),
        selections: vec![
            reference(AgentObjectKind::CheckpointVersion, 3),
            reference(AgentObjectKind::Association, 4),
        ],
        next: Some(reference(AgentObjectKind::RelationPage, 5)),
    };
    assert!(subject.catalog_key().is_ok());
    assert_eq!(relation.dependencies().unwrap().len(), 3);

    let source = CaptureSourceStateV1 {
        schema_version: 1,
        integration: IntegrationId::Codex,
        source_instance_id: source_id,
        generation: 7,
        parser_version: "codex-0.156.1/v1".into(),
        native_session_key: Some("session-17".into()),
        position: SourcePositionV1::JsonLines {
            complete_byte_offset: 8192,
            prefix_sha256: [6; 32],
        },
        receipts: vec![CaptureSourceReceiptV1 {
            batch_id: CaptureBatchId::new([7; 16]),
            first_sequence: 10,
            last_sequence: 14,
        }],
        coverage: coverage(),
    };
    assert!(source.catalog_key().is_ok());
    assert!(AgentObjectV1::SourceState(source).validate().is_ok());

    let projection = AgentProjectionCatalogV1 {
        schema_version: 1,
        normalizer_version: "codex-0.156.1/v1".into(),
        activated_at_millis: 1_725_000_000_000,
        sessions: vec![reference(AgentObjectKind::SessionSnapshot, 8)],
        event_batches: vec![reference(AgentObjectKind::EventBatch, 9)],
        source_states: vec![reference(AgentObjectKind::SourceState, 10)],
    };
    assert!(projection.catalog_key().is_ok());
    assert_eq!(projection.dependencies().unwrap().len(), 3);
    assert!(
        AgentObjectV1::ProjectionCatalog(projection)
            .validate()
            .is_ok()
    );

    assert!(matches!(
        AgentCatalogKind::Aliases,
        AgentCatalogKind::Aliases
    ));
    assert!(matches!(
        AgentCatalogKind::Relations,
        AgentCatalogKind::Relations
    ));
    assert!(matches!(
        AgentCatalogKind::Sources,
        AgentCatalogKind::Sources
    ));
    assert!(matches!(
        AgentCatalogKind::Projections,
        AgentCatalogKind::Projections
    ));

    let selections = [
        (AgentCatalogKind::Aliases, AgentObjectKind::Alias),
        (AgentCatalogKind::Relations, AgentObjectKind::RelationPage),
        (AgentCatalogKind::Sources, AgentObjectKind::SourceState),
        (
            AgentCatalogKind::Projections,
            AgentObjectKind::ProjectionCatalog,
        ),
    ];
    let delta = AgentDeltaV1 {
        schema_version: 1,
        edits: selections
            .into_iter()
            .enumerate()
            .map(|(index, (catalog, kind))| AgentCatalogEditV1 {
                catalog,
                key: AgentCatalogKey::derive("index", &[index as u8 + 1]).unwrap(),
                expected: None,
                replacement: Some(reference(kind, index as u8 + 11)),
            })
            .collect(),
    };
    assert!(delta.validate().is_ok());
}

#[test]
fn auxiliary_catalog_objects_reject_wrong_kinds_and_receipt_order() {
    let duplicate = reference(AgentObjectKind::Association, 1);
    let relation = AgentRelationPageV1 {
        schema_version: 1,
        subject: AgentRelationSubjectV1::Checkpoint {
            checkpoint_id: AgentCheckpointId::new([2; 16]),
        },
        selections: vec![duplicate, duplicate],
        next: None,
    };
    assert!(relation.validate().is_err());

    let source = CaptureSourceStateV1 {
        schema_version: 1,
        integration: IntegrationId::Cursor,
        source_instance_id: SourceInstanceId::new([3; 16]),
        generation: 0,
        parser_version: "cursor-3.22.7/v1".into(),
        native_session_key: None,
        position: SourcePositionV1::NativeSequence { sequence: 3 },
        receipts: vec![CaptureSourceReceiptV1 {
            batch_id: CaptureBatchId::new([4; 16]),
            first_sequence: 5,
            last_sequence: 4,
        }],
        coverage: coverage(),
    };
    assert!(source.validate().is_err());

    let projection = AgentProjectionCatalogV1 {
        schema_version: 1,
        normalizer_version: "opencode-1.18.30/v1".into(),
        activated_at_millis: 1,
        sessions: vec![reference(AgentObjectKind::ToolSnapshot, 5)],
        event_batches: vec![],
        source_states: vec![],
    };
    assert!(projection.validate().is_err());
}

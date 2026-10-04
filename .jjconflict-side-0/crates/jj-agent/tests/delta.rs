use jj_agent::{
    AgentCatalogEditV1, AgentCatalogKey, AgentCatalogKind, AgentDeltaV1, AgentObjectId,
    AgentObjectKind, AgentObjectRef,
};

fn reference(kind: AgentObjectKind, byte: u8) -> AgentObjectRef {
    AgentObjectRef::new(kind, AgentObjectId::new([byte; 32]))
}

#[test]
fn typed_deltas_reject_duplicate_keys_wrong_kinds_and_noops() {
    let key = AgentCatalogKey::derive("session", &[1; 16]).unwrap();
    let edit = AgentCatalogEditV1 {
        catalog: AgentCatalogKind::Sessions,
        key,
        expected: None,
        replacement: Some(reference(AgentObjectKind::SessionSnapshot, 2)),
    };
    AgentDeltaV1 {
        schema_version: 1,
        edits: vec![edit.clone()],
    }
    .validate()
    .unwrap();

    assert!(
        AgentDeltaV1 {
            schema_version: 1,
            edits: vec![edit.clone(), edit.clone()],
        }
        .validate()
        .is_err()
    );
    assert!(
        AgentDeltaV1 {
            schema_version: 1,
            edits: vec![AgentCatalogEditV1 {
                replacement: Some(reference(AgentObjectKind::Association, 3)),
                ..edit.clone()
            }],
        }
        .validate()
        .is_err()
    );
    assert!(
        AgentDeltaV1 {
            schema_version: 1,
            edits: vec![AgentCatalogEditV1 {
                expected: edit.replacement,
                ..edit
            }],
        }
        .validate()
        .is_err()
    );
}

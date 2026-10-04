use jj_agent::*;
use jj_review::*;
use jj_store_model::RepositoryId;
use std::collections::{BTreeMap, BTreeSet};

fn reference(kind: AgentObjectKind, value: u8) -> AgentObjectRef {
    AgentObjectRef::new(kind, AgentObjectId::new([value; 32]))
}
fn fixture() -> (ArtifactAnchorV2, BTreeMap<AgentObjectRef, AgentObjectV1>) {
    let session_id = AgentSessionId::new([1; 16]);
    let tool_call_id = ToolCallId::new([2; 16]);
    let session = reference(AgentObjectKind::SessionSnapshot, 3);
    let tool = reference(AgentObjectKind::ToolSnapshot, 4);
    let content = reference(AgentObjectKind::Content, 5);
    let artifact = PortableAgentToolV2 {
        session_id,
        session_snapshot: session,
        tool_call_id,
        tool_snapshot: tool,
    };
    let anchor = ArtifactAnchorV2 {
        schema_version: 2,
        repository_id: RepositoryId::new([9; 16]),
        target: ArtifactTargetV2::AgentToolText(Box::new(PortableAgentToolTextV2 {
            tool: artifact,
            part: AgentToolContentPartV2::Output,
            content,
            text: TextSelectionV1 {
                encoding: TextEncoding::Utf8,
                start_byte: ByteOffset::new(1),
                end_byte: ByteOffset::new(3),
                selected_sha256: ReviewDigest::sha256("é".as_bytes()),
            },
        })),
        created_context: CreatedContextV1 {
            publication_position: None,
            code_state_version: None,
            agent_root: None,
        },
        sensitivity: SensitivityV1::Repository,
    };
    let objects = BTreeMap::from([
        (
            session,
            AgentObjectV1::SessionSnapshot(AgentSessionSnapshotV1 {
                schema_version: 1,
                session_id,
                integration: IntegrationId::Codex,
                state: SessionState::Ended,
                parent_session_id: None,
                evidence: vec![],
                event_batches: vec![],
                transcripts: vec![],
                tools: BTreeMap::from([(tool_call_id, tool)]),
                coverage: CoverageV1 {
                    completeness: CompletenessV1::Partial,
                    gaps: 1,
                    redacted_fields: 0,
                    truncated: false,
                },
            }),
        ),
        (
            tool,
            AgentObjectV1::ToolSnapshot(ToolInvocationSnapshotV1 {
                schema_version: 1,
                session_id,
                tool_call_id,
                turn_id: None,
                native_name: "command".into(),
                kind: ToolKind::Execute,
                state: ToolState::Succeeded,
                attempts: BTreeSet::new(),
                input: None,
                output: Some(content),
                evidence: vec![],
            }),
        ),
        (
            content,
            AgentObjectV1::Content(AgentContentV1 {
                schema_version: 1,
                payload: SanitizedPayloadV1::new(SanitizedMediaType::Text, "aéz".as_bytes())
                    .unwrap(),
            }),
        ),
    ]);
    (anchor, objects)
}
fn validate(
    anchor: &ArtifactAnchorV2,
    objects: &BTreeMap<AgentObjectRef, AgentObjectV1>,
) -> jj_review::Result<()> {
    anchor.validate_agent_records(anchor.repository_id, |reference| {
        objects
            .get(&reference)
            .cloned()
            .ok_or(jj_review::Error::Invalid("unselected agent record"))
    })
}

#[test]
fn portable_tool_anchor_uses_retained_invocation_without_invented_attempts() {
    let (anchor, objects) = fixture();
    validate(&anchor, &objects).unwrap();
    let bytes = anchor.encode().unwrap();
    assert_eq!(ArtifactAnchorV2::decode(&bytes).unwrap(), anchor);
    assert!(jj_review::decode_anchor(&bytes).is_err());
    assert!(
        !String::from_utf8(bytes.clone())
            .unwrap()
            .contains("attempt")
    );
    let mut document: serde_json::Value = serde_json::from_slice(&bytes).unwrap();
    document["target"]["value"]["tool"]["attempt_id"] = serde_json::json!("02".repeat(16));
    assert!(ArtifactAnchorV2::decode(&serde_json::to_vec(&document).unwrap()).is_err());
    assert_eq!(
        anchor.agent_dependencies().unwrap(),
        objects.keys().copied().collect()
    );
    assert!(anchor.code_evidence().unwrap().is_empty());
}

#[test]
fn portable_agent_validation_rejects_unselected_and_wrong_session_records() {
    let (anchor, mut objects) = fixture();
    assert!(
        anchor
            .validate_agent_records(RepositoryId::new([8; 16]), |_| unreachable!())
            .is_err()
    );
    let session = reference(AgentObjectKind::SessionSnapshot, 3);
    let original = objects.remove(&session).unwrap();
    assert!(validate(&anchor, &objects).is_err());
    objects.insert(session, original);
    let AgentObjectV1::SessionSnapshot(value) = objects.get_mut(&session).unwrap() else {
        panic!()
    };
    value.session_id = AgentSessionId::new([6; 16]);
    assert!(validate(&anchor, &objects).is_err());
}

#[test]
fn portable_tool_requires_exact_membership_and_both_stable_ids() {
    let (anchor, objects) = fixture();
    let session = reference(AgentObjectKind::SessionSnapshot, 3);
    let tool = reference(AgentObjectKind::ToolSnapshot, 4);
    for mode in 0..4 {
        let mut replaced = objects.clone();
        if mode < 2 {
            let AgentObjectV1::SessionSnapshot(value) = replaced.get_mut(&session).unwrap() else {
                panic!()
            };
            if mode == 0 {
                value.tools.clear();
            } else {
                value.tools.insert(
                    ToolCallId::new([2; 16]),
                    reference(AgentObjectKind::ToolSnapshot, 7),
                );
            }
        } else {
            let AgentObjectV1::ToolSnapshot(value) = replaced.get_mut(&tool).unwrap() else {
                panic!()
            };
            if mode == 2 {
                value.session_id = AgentSessionId::new([6; 16]);
            } else {
                value.tool_call_id = ToolCallId::new([6; 16]);
            }
        }
        assert!(validate(&anchor, &replaced).is_err(), "mode {mode}");
    }
}

#[test]
fn portable_tool_text_validates_part_exact_content_and_utf8_digest() {
    let (anchor, objects) = fixture();
    for mode in 0..4 {
        let mut changed = anchor.clone();
        let ArtifactTargetV2::AgentToolText(value) = &mut changed.target else {
            panic!()
        };
        match mode {
            0 => value.part = AgentToolContentPartV2::Input,
            1 => value.content = reference(AgentObjectKind::Content, 8),
            2 => value.text.end_byte = ByteOffset::new(2),
            _ => value.text.selected_sha256 = ReviewDigest::sha256(b"xx"),
        }
        assert!(validate(&changed, &objects).is_err(), "mode {mode}");
    }
    let mut changed = objects;
    let AgentObjectV1::Content(content) = changed
        .get_mut(&reference(AgentObjectKind::Content, 5))
        .unwrap()
    else {
        panic!()
    };
    content.payload = SanitizedPayloadV1::new(SanitizedMediaType::Text, b"abcd").unwrap();
    assert!(validate(&anchor, &changed).is_err());
}

#[test]
fn portable_session_and_whole_tool_keep_distinct_artifact_identity() {
    let (mut anchor, objects) = fixture();
    let ArtifactTargetV2::AgentToolText(value) = &anchor.target else {
        panic!()
    };
    let tool = value.tool.clone();
    let text_id = anchor.artifact_id().unwrap();
    anchor.target = ArtifactTargetV2::AgentToolArtifact(tool.clone());
    validate(&anchor, &objects).unwrap();
    assert_ne!(anchor.artifact_id().unwrap(), text_id);
    anchor.target = ArtifactTargetV2::AgentSession(PortableAgentSessionV2 {
        session_id: tool.session_id,
        session_snapshot: tool.session_snapshot,
    });
    validate(&anchor, &objects).unwrap();
    assert_eq!(
        anchor.agent_dependencies().unwrap(),
        BTreeSet::from([tool.session_snapshot])
    );
}

#[test]
fn portable_agent_kinds_and_provider_representations_fail_closed() {
    let (mut anchor, _) = fixture();
    let ArtifactTargetV2::AgentToolText(value) = &mut anchor.target else {
        panic!()
    };
    value.content.kind = AgentObjectKind::ObservationBatch;
    assert!(anchor.validate().is_err());
    let (anchor, mut objects) = fixture();
    let tool = reference(AgentObjectKind::ToolSnapshot, 4);
    let content = objects
        .get(&reference(AgentObjectKind::Content, 5))
        .unwrap()
        .clone();
    objects.insert(tool, content);
    assert!(validate(&anchor, &objects).is_err());
}

#[test]
fn legacy_native_anchor_bytes_and_identity_remain_readable() {
    let bytes = br#"{"schema_version":1,"repository_id":"01010101010101010101010101010101","target":{"target":"repository"},"created_context":{"publication_position":7,"code_state_version":4,"agent_root":null},"sensitivity":"repository"}"#;
    let anchor = jj_review::decode_anchor(bytes).unwrap();
    assert_eq!(jj_review::encode_anchor(&anchor).unwrap(), bytes);
    assert!(ArtifactAnchorV2::decode(bytes).is_err());
    assert_eq!(
        anchor.anchor_id().unwrap().to_string(),
        "4db53878698c6a69888bae9771988514dc3739c3c8a32083c09803c807892de5"
    );
    assert_eq!(anchor.repository_id, RepositoryId::new([1; 16]));
    assert_eq!(anchor.created_context.publication_position, Some(7));
}

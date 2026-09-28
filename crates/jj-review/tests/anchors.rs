use jj_agent::{
    AgentObjectId, AgentObjectKind, AgentObjectRef, AgentSessionId, BlockId, MessageId,
    ToolAttemptId, ToolCallId,
};
use jj_review::{
    AgentContentOwnerV1, AgentContentPart, AgentTextAnchorV1, ArtifactAnchorV1, ArtifactTargetV1,
    ByteOffset, CodeArtifactAnchorV1, CodeTextAnchorV1, CodeValueKind, ComparisonKind,
    CreatedContextV1, DiffContextV1, DiffSide, ReviewDigest, SensitivityV1, SourceCoverageV1,
    TextEncoding, TextSelectionV1, decode_anchor, encode_anchor,
};
use jj_store_model::{ChangeId, CommitId, FileId, ObjectKey, OperationId, RepositoryId};
use std::collections::BTreeSet;

fn agent_ref(kind: AgentObjectKind, byte: u8) -> AgentObjectRef {
    AgentObjectRef::new(kind, AgentObjectId::new([byte; 32]))
}

fn selection(bytes: &[u8], start: u64, end: u64) -> TextSelectionV1 {
    TextSelectionV1 {
        encoding: TextEncoding::Utf8,
        start_byte: ByteOffset::new(start),
        end_byte: ByteOffset::new(end),
        selected_sha256: ReviewDigest::sha256(&bytes[start as usize..end as usize]),
    }
}

fn context() -> CreatedContextV1 {
    CreatedContextV1 {
        publication_position: Some(7),
        code_state_version: Some(4),
        agent_root: None,
    }
}

#[test]
fn code_text_anchor_pins_exact_native_objects_and_utf8_bytes() {
    let bytes = "before 🦀 after".as_bytes();
    let start = "before ".len() as u64;
    let end = start + "🦀".len() as u64;
    let anchor = ArtifactAnchorV1 {
        schema_version: 1,
        repository_id: RepositoryId::new([1; 16]),
        target: ArtifactTargetV1::CodeText(Box::new(CodeTextAnchorV1 {
            operation_id: OperationId::new([2; 64]),
            commit_id: CommitId::new([3; 64]),
            change_id: Some(ChangeId::new([4; 16])),
            path: "src/lib.rs".into(),
            file_id: FileId::new([5; 64]),
            value_kind: CodeValueKind::File,
            text: selection(bytes, start, end),
            diff_context: Some(DiffContextV1 {
                before_operation_id: OperationId::new([6; 64]),
                before_commit_id: CommitId::new([7; 64]),
                before_path: Some("src/old.rs".into()),
                after_operation_id: OperationId::new([2; 64]),
                after_commit_id: CommitId::new([3; 64]),
                after_path: Some("src/lib.rs".into()),
                side: DiffSide::After,
                comparison: ComparisonKind::ExplicitPair,
            }),
        })),
        created_context: context(),
        sensitivity: SensitivityV1::Repository,
    };
    anchor.validate().unwrap();
    anchor.validate_text(bytes).unwrap();
    assert_eq!(
        anchor.native_dependencies().unwrap(),
        BTreeSet::from([
            ObjectKey::Operation(OperationId::new([2; 64])),
            ObjectKey::Commit(CommitId::new([3; 64])),
            ObjectKey::File(FileId::new([5; 64])),
            ObjectKey::Operation(OperationId::new([6; 64])),
            ObjectKey::Commit(CommitId::new([7; 64])),
        ])
    );
    assert_eq!(
        decode_anchor(&encode_anchor(&anchor).unwrap()).unwrap(),
        anchor
    );

    let mut wrong = anchor.clone();
    let ArtifactTargetV1::CodeText(value) = &mut wrong.target else {
        unreachable!()
    };
    value.text.start_byte = ByteOffset::new(start + 1);
    assert!(wrong.validate_text(bytes).is_err());
    let ArtifactTargetV1::CodeText(value) = &mut wrong.target else {
        unreachable!()
    };
    value.text.start_byte = ByteOffset::new(start);
    value.text.selected_sha256 = ReviewDigest::new([9; 32]);
    assert!(wrong.validate_text(bytes).is_err());
}

#[test]
fn agent_text_anchor_keeps_owner_attempt_block_and_content_versions_distinct() {
    let bytes = b"test failed";
    let session_snapshot = agent_ref(AgentObjectKind::SessionSnapshot, 10);
    let tool_snapshot = agent_ref(AgentObjectKind::ToolSnapshot, 11);
    let block_version = agent_ref(AgentObjectKind::Content, 12);
    let content = agent_ref(AgentObjectKind::Content, 13);
    let anchor = ArtifactAnchorV1 {
        schema_version: 1,
        repository_id: RepositoryId::new([1; 16]),
        target: ArtifactTargetV1::AgentText(Box::new(AgentTextAnchorV1 {
            session_id: AgentSessionId::new([14; 16]),
            session_snapshot,
            owner: AgentContentOwnerV1::Tool {
                tool_call_id: ToolCallId::new([15; 16]),
                snapshot: tool_snapshot,
                attempt_id: ToolAttemptId::new([16; 16]),
            },
            block_id: BlockId::new([17; 16]),
            block_version,
            part: AgentContentPart::Output,
            content,
            representation_version: "published-text-v1".into(),
            sanitizer_version: "agent-redaction@0.1.0".into(),
            normalizer_version: "codex@0.156.1/v1".into(),
            source_coverage: SourceCoverageV1::Partial,
            text: selection(bytes, 0, bytes.len() as u64),
        })),
        created_context: context(),
        sensitivity: SensitivityV1::Restricted,
    };
    anchor.validate().unwrap();
    anchor.validate_text(bytes).unwrap();
    assert_eq!(
        anchor.agent_dependencies().unwrap(),
        BTreeSet::from([session_snapshot, tool_snapshot, block_version, content])
    );

    let mut wrong = anchor;
    let ArtifactTargetV1::AgentText(value) = &mut wrong.target else {
        unreachable!()
    };
    value.owner = AgentContentOwnerV1::Message {
        message_id: MessageId::new([18; 16]),
        snapshot: tool_snapshot,
    };
    assert!(wrong.validate().is_err());
}

#[test]
fn anchors_reject_paths_ranges_and_diff_sides_that_lose_exact_identity() {
    let bytes = b"line";
    let invalid = |path: &str, text: TextSelectionV1, side: DiffSide| ArtifactAnchorV1 {
        schema_version: 1,
        repository_id: RepositoryId::new([1; 16]),
        target: ArtifactTargetV1::CodeText(Box::new(CodeTextAnchorV1 {
            operation_id: OperationId::new([2; 64]),
            commit_id: CommitId::new([3; 64]),
            change_id: None,
            path: path.into(),
            file_id: FileId::new([5; 64]),
            value_kind: CodeValueKind::File,
            text,
            diff_context: Some(DiffContextV1 {
                before_operation_id: OperationId::new([6; 64]),
                before_commit_id: CommitId::new([7; 64]),
                before_path: Some("old.rs".into()),
                after_operation_id: OperationId::new([2; 64]),
                after_commit_id: CommitId::new([3; 64]),
                after_path: None,
                side,
                comparison: ComparisonKind::ExplicitPair,
            }),
        })),
        created_context: context(),
        sensitivity: SensitivityV1::Repository,
    };
    assert!(
        invalid("../secret", selection(bytes, 0, 4), DiffSide::Before)
            .validate()
            .is_err()
    );
    assert!(
        invalid("lib.rs", selection(bytes, 1, 1), DiffSide::Before)
            .validate()
            .is_err()
    );
    assert!(
        invalid("lib.rs", selection(bytes, 0, 4), DiffSide::After)
            .validate()
            .is_err()
    );
}

#[test]
fn whole_artifact_anchor_requires_a_matching_native_object_kind() {
    let anchor = |value_kind, object| ArtifactAnchorV1 {
        schema_version: 1,
        repository_id: RepositoryId::new([1; 16]),
        target: ArtifactTargetV1::CodeArtifact(Box::new(CodeArtifactAnchorV1 {
            operation_id: OperationId::new([2; 64]),
            commit_id: CommitId::new([3; 64]),
            change_id: None,
            path: "assets/logo.bin".into(),
            object,
            value_kind,
        })),
        created_context: context(),
        sensitivity: SensitivityV1::Repository,
    };

    assert!(
        anchor(
            CodeValueKind::BinaryFile,
            ObjectKey::File(FileId::new([4; 64]))
        )
        .validate()
        .is_ok()
    );
    assert!(
        anchor(
            CodeValueKind::Symlink,
            ObjectKey::File(FileId::new([4; 64]))
        )
        .validate()
        .is_err()
    );
    assert!(
        anchor(
            CodeValueKind::Gitlink,
            ObjectKey::Commit(CommitId::new([4; 64]))
        )
        .validate()
        .is_err()
    );
}

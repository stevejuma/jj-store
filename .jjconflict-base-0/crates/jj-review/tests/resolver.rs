use jj_agent::{
    AgentObjectId, AgentObjectKind, AgentObjectRef, AgentSessionId, BlockId, ToolAttemptId,
    ToolCallId,
};
use jj_review::{
    AgentBlockLineageV1, AgentContentOwnerV1, AgentContentPart, AgentPlacementTargetV1,
    AgentTextAnchorV1, AnchorResolutionRequestV1, AnchorResolverV1, ArtifactAnchorV1,
    ArtifactTargetV1, ByteOffset, CodeLineageKindV1, CodePlacementTargetV1, CodeTargetArtifactV1,
    CodeTextAnchorV1, CodeValueKind, CreatedContextV1, ExplicitPlacementTargetV1,
    PlacementCandidateV1, PlacementKindV1, ResolverBudgetV1, RetainedTextV1, ReviewDigest,
    SensitivityV1, SourceCoverageV1, TextEncoding, TextSelectionV1, ValidatedCodeLineageV1,
};
use jj_store_model::{ChangeId, CommitId, FileId, OperationId, RepositoryId};

const ORIGINAL: &[u8] = b"fn alpha() {}\nfn omega() {}\n";

fn selection(bytes: &[u8], start: usize, end: usize) -> TextSelectionV1 {
    TextSelectionV1 {
        encoding: TextEncoding::Utf8,
        start_byte: ByteOffset::new(start as u64),
        end_byte: ByteOffset::new(end as u64),
        selected_sha256: ReviewDigest::sha256(&bytes[start..end]),
    }
}

fn code_anchor() -> ArtifactAnchorV1 {
    ArtifactAnchorV1 {
        schema_version: 1,
        repository_id: RepositoryId::new([1; 16]),
        target: ArtifactTargetV1::CodeText(Box::new(CodeTextAnchorV1 {
            operation_id: OperationId::new([2; 64]),
            commit_id: CommitId::new([3; 64]),
            change_id: Some(ChangeId::new([4; 16])),
            path: "src/lib.rs".into(),
            file_id: FileId::new([5; 64]),
            value_kind: CodeValueKind::File,
            text: selection(ORIGINAL, 3, 8),
            diff_context: None,
        })),
        created_context: CreatedContextV1 {
            publication_position: Some(1),
            code_state_version: Some(1),
            agent_root: None,
        },
        sensitivity: SensitivityV1::Repository,
    }
}

fn lineage(path: &str, file: u8, commit: u8, operation: u8) -> ValidatedCodeLineageV1 {
    ValidatedCodeLineageV1 {
        from_operation_id: OperationId::new([2; 64]),
        from_commit_id: CommitId::new([3; 64]),
        from_path: "src/lib.rs".into(),
        from_file_id: FileId::new([5; 64]),
        to_operation_id: OperationId::new([operation; 64]),
        to_commit_id: CommitId::new([commit; 64]),
        to_path: path.into(),
        to_file_id: FileId::new([file; 64]),
        relation: CodeLineageKindV1::Predecessor,
    }
}

fn code_target(
    operation: u8,
    commit: u8,
    artifacts: Vec<CodeTargetArtifactV1>,
    lineage_complete: bool,
) -> ExplicitPlacementTargetV1 {
    ExplicitPlacementTargetV1::Code(CodePlacementTargetV1 {
        operation_id: OperationId::new([operation; 64]),
        commit_id: CommitId::new([commit; 64]),
        change_id: Some(ChangeId::new([4; 16])),
        artifacts,
        lineage_complete,
    })
}

fn code_artifact(
    path: &str,
    file: u8,
    text: RetainedTextV1,
    relation: Option<ValidatedCodeLineageV1>,
) -> CodeTargetArtifactV1 {
    CodeTargetArtifactV1 {
        path: path.into(),
        file_id: FileId::new([file; 64]),
        text,
        lineage: relation,
    }
}

fn request(target: ExplicitPlacementTargetV1) -> AnchorResolutionRequestV1 {
    AnchorResolutionRequestV1 {
        original_text: RetainedTextV1::Available(ORIGINAL.to_vec()),
        target,
    }
}

#[test]
fn exact_identity_and_explicit_insertion_map_without_retargeting_the_anchor() {
    let resolver = AnchorResolverV1::default();
    let anchor = code_anchor();
    let exact = resolver
        .resolve(
            &anchor,
            &request(code_target(
                2,
                3,
                vec![code_artifact(
                    "src/lib.rs",
                    5,
                    RetainedTextV1::Available(ORIGINAL.to_vec()),
                    None,
                )],
                true,
            )),
        )
        .unwrap();
    assert_eq!(exact.placement, PlacementKindV1::Exact);
    assert_eq!(anchor, code_anchor());

    let inserted = b"// header\nfn alpha() {}\nfn omega() {}\n";
    let mapped = resolver
        .resolve(
            &anchor,
            &request(code_target(
                6,
                7,
                vec![code_artifact(
                    "src/lib.rs",
                    8,
                    RetainedTextV1::Available(inserted.to_vec()),
                    Some(lineage("src/lib.rs", 8, 7, 6)),
                )],
                true,
            )),
        )
        .unwrap();
    assert_eq!(mapped.placement, PlacementKindV1::MappedUnchanged);
    assert!(matches!(
        mapped.candidates.as_slice(),
        [PlacementCandidateV1::CodeText { start_byte, end_byte, .. }]
            if start_byte.get() == 13 && end_byte.get() == 18
    ));
}

#[test]
fn changed_deleted_duplicate_and_missing_lineage_are_categorical() {
    let resolver = AnchorResolverV1::default();
    let anchor = code_anchor();
    let changed = b"fn beta() {}\nfn omega() {}\n";
    let modified = resolver
        .resolve(
            &anchor,
            &request(code_target(
                6,
                7,
                vec![code_artifact(
                    "src/lib.rs",
                    8,
                    RetainedTextV1::Available(changed.to_vec()),
                    Some(lineage("src/lib.rs", 8, 7, 6)),
                )],
                true,
            )),
        )
        .unwrap();
    assert_eq!(modified.placement, PlacementKindV1::CandidateModified);

    let deleted = resolver
        .resolve(&anchor, &request(code_target(6, 7, Vec::new(), true)))
        .unwrap();
    assert_eq!(deleted.placement, PlacementKindV1::Deleted);

    let duplicate = resolver
        .resolve(
            &anchor,
            &request(code_target(
                6,
                7,
                vec![
                    code_artifact(
                        "src/lib.rs",
                        8,
                        RetainedTextV1::Available(ORIGINAL.to_vec()),
                        Some(lineage("src/lib.rs", 8, 7, 6)),
                    ),
                    code_artifact(
                        "src/copy.rs",
                        9,
                        RetainedTextV1::Available(ORIGINAL.to_vec()),
                        Some(lineage("src/copy.rs", 9, 7, 6)),
                    ),
                ],
                true,
            )),
        )
        .unwrap();
    assert_eq!(duplicate.placement, PlacementKindV1::Ambiguous);
    assert_eq!(duplicate.candidates.len(), 2);

    let missing_lineage = resolver
        .resolve(
            &anchor,
            &request(code_target(
                6,
                7,
                vec![code_artifact(
                    "src/lib.rs",
                    8,
                    RetainedTextV1::Available(ORIGINAL.to_vec()),
                    None,
                )],
                false,
            )),
        )
        .unwrap();
    assert_eq!(missing_lineage.placement, PlacementKindV1::Unresolved);
}

#[test]
fn budgets_divergence_and_unavailable_content_fail_closed() {
    let anchor = code_anchor();
    let tiny = AnchorResolverV1::new(ResolverBudgetV1 {
        max_bytes: 4,
        max_candidates: 2,
    })
    .unwrap();
    let target = code_target(
        6,
        7,
        vec![code_artifact(
            "src/lib.rs",
            8,
            RetainedTextV1::Available(ORIGINAL.to_vec()),
            Some(lineage("src/lib.rs", 8, 7, 6)),
        )],
        true,
    );
    assert_eq!(
        tiny.resolve(&anchor, &request(target.clone()))
            .unwrap()
            .placement,
        PlacementKindV1::Unresolved
    );

    let divergent = code_target(
        6,
        7,
        vec![code_artifact(
            "src/lib.rs",
            8,
            RetainedTextV1::Available(ORIGINAL.to_vec()),
            None,
        )],
        true,
    );
    assert_eq!(
        AnchorResolverV1::default()
            .resolve(&anchor, &request(divergent))
            .unwrap()
            .placement,
        PlacementKindV1::Unresolved
    );

    let unavailable = AnchorResolutionRequestV1 {
        original_text: RetainedTextV1::Redacted,
        target,
    };
    assert_eq!(
        AnchorResolverV1::default()
            .resolve(&anchor, &unavailable)
            .unwrap()
            .placement,
        PlacementKindV1::Unavailable
    );
}

fn agent_ref(kind: AgentObjectKind, byte: u8) -> AgentObjectRef {
    AgentObjectRef::new(kind, AgentObjectId::new([byte; 32]))
}

fn agent_anchor() -> ArtifactAnchorV1 {
    let bytes = b"running tests";
    ArtifactAnchorV1 {
        schema_version: 1,
        repository_id: RepositoryId::new([1; 16]),
        target: ArtifactTargetV1::AgentText(Box::new(AgentTextAnchorV1 {
            session_id: AgentSessionId::new([10; 16]),
            session_snapshot: agent_ref(AgentObjectKind::SessionSnapshot, 11),
            owner: AgentContentOwnerV1::Tool {
                tool_call_id: ToolCallId::new([12; 16]),
                snapshot: agent_ref(AgentObjectKind::ToolSnapshot, 13),
                attempt_id: ToolAttemptId::new([14; 16]),
            },
            block_id: BlockId::new([15; 16]),
            block_version: agent_ref(AgentObjectKind::Content, 16),
            part: AgentContentPart::Output,
            content: agent_ref(AgentObjectKind::Content, 17),
            representation_version: "published-text-v1".into(),
            sanitizer_version: "agent-redaction@0.1.0".into(),
            normalizer_version: "codex@0.156.1/v1".into(),
            source_coverage: SourceCoverageV1::Partial,
            text: selection(bytes, 0, bytes.len()),
        })),
        created_context: CreatedContextV1 {
            publication_position: Some(1),
            code_state_version: Some(1),
            agent_root: None,
        },
        sensitivity: SensitivityV1::Restricted,
    }
}

#[test]
fn agent_output_maps_only_through_retained_unchanged_prefix_lineage() {
    let anchor = agent_anchor();
    let target_block = agent_ref(AgentObjectKind::Content, 18);
    let target_content = agent_ref(AgentObjectKind::Content, 19);
    let target = ExplicitPlacementTargetV1::Agent(AgentPlacementTargetV1 {
        session_id: AgentSessionId::new([10; 16]),
        block_id: BlockId::new([15; 16]),
        block_version: target_block,
        content: target_content,
        representation_version: "published-text-v1".into(),
        sanitizer_version: "agent-redaction@0.1.0".into(),
        normalizer_version: "codex@0.156.1/v1".into(),
        text: RetainedTextV1::Available(b"running tests\nall passed".to_vec()),
        lineage: Some(AgentBlockLineageV1 {
            from_block_version: agent_ref(AgentObjectKind::Content, 16),
            from_content: agent_ref(AgentObjectKind::Content, 17),
            to_block_version: target_block,
            to_content: target_content,
        }),
    });
    let request = AnchorResolutionRequestV1 {
        original_text: RetainedTextV1::Available(b"running tests".to_vec()),
        target: target.clone(),
    };
    assert_eq!(
        AnchorResolverV1::default()
            .resolve(&anchor, &request)
            .unwrap()
            .placement,
        PlacementKindV1::MappedUnchanged
    );

    let ExplicitPlacementTargetV1::Agent(mut unsupported) = target else {
        unreachable!()
    };
    unsupported.lineage = None;
    assert_eq!(
        AnchorResolverV1::default()
            .resolve(
                &anchor,
                &AnchorResolutionRequestV1 {
                    original_text: RetainedTextV1::Available(b"running tests".to_vec()),
                    target: ExplicitPlacementTargetV1::Agent(unsupported),
                },
            )
            .unwrap()
            .placement,
        PlacementKindV1::Unresolved
    );
}

use jj_review::*;
use jj_store_model::{self as m, portable::*};

fn revision(id: u8) -> PortableRevisionV2 {
    PortableRevisionV2 {
        evidence: CodeEvidenceRefV1 {
            repository: m::RepositoryId::new([1; 16]),
            profile: CodeProfileV1::GitSha1Jj045,
            operation: m::OperationId::new([id; 64]),
            capsule: m::CodeCapsuleId::new([id; 32]),
        },
        commit: CodeCommitIdV1::GitSha1Jj045(m::GitSha1Id::new([id; 20])),
        change: Some(m::ChangeId::new([id; 16])),
    }
}
fn fixture_artifact() -> PortableCodeArtifactV2 {
    PortableCodeArtifactV2 {
        revision: revision(2),
        path: "src/main.rs".into(),
        object: CodeObjectIdV1::GitSha1Jj045 {
            kind: GitObjectKindV1::Blob,
            id: m::GitSha1Id::new([3; 20]),
        },
        value_kind: CodeValueKind::File,
    }
}
fn anchor() -> ArtifactAnchorV2 {
    ArtifactAnchorV2 {
        schema_version: 2,
        repository_id: m::RepositoryId::new([1; 16]),
        target: ArtifactTargetV2::CodeText(Box::new(PortableCodeTextV2 {
            artifact: fixture_artifact(),
            text: TextSelectionV1 {
                encoding: TextEncoding::Utf8,
                start_byte: ByteOffset::new(1),
                end_byte: ByteOffset::new(3),
                selected_sha256: ReviewDigest::sha256("é".as_bytes()),
            },
            diff_context: None,
        })),
        created_context: CreatedContextV1 {
            publication_position: None,
            code_state_version: None,
            agent_root: None,
        },
        sensitivity: SensitivityV1::Repository,
    }
}
#[test]
fn portable_anchors_bind_source_profiles_and_exact_utf8_selections() {
    let anchor = anchor();
    let bytes = anchor.encode().unwrap();
    assert_eq!(ArtifactAnchorV2::decode(&bytes).unwrap(), anchor);
    assert!(decode_anchor(&bytes).is_err());
    anchor.validate_text("aéz".as_bytes()).unwrap();
    assert!(anchor.validate_text(b"abcd").is_err());
    let mut changed = anchor.clone();
    let ArtifactTargetV2::CodeText(value) = &mut changed.target else {
        panic!()
    };
    value.text.end_byte = ByteOffset::new(2);
    assert!(changed.validate_text("aéz".as_bytes()).is_err());
    let mut value: serde_json::Value = serde_json::from_slice(&bytes).unwrap();
    value["target"]["value"]["artifact"]["revision"]["commit"]["id"] =
        serde_json::json!("0".repeat(128));
    assert!(ArtifactAnchorV2::decode(&serde_json::to_vec(&value).unwrap()).is_err());
    assert!("AB".repeat(20).parse::<m::GitSha1Id>().is_err());
}
#[test]
fn mixed_profile_and_invalid_artifact_records_fail_closed() {
    let mut artifact = fixture_artifact();
    artifact.revision.commit = CodeCommitIdV1::NativeV1(m::CommitId::new([2; 64]));
    assert!(artifact.validate().is_err());
    let mut artifact = fixture_artifact();
    artifact.object = CodeObjectIdV1::NativeV1 {
        object: m::ObjectKey::File(m::FileId::new([3; 64])),
    };
    assert!(artifact.validate().is_err());
    let mut artifact = fixture_artifact();
    artifact.path = "../secret".into();
    assert!(artifact.validate().is_err());
    let mut artifact = fixture_artifact();
    artifact.object = CodeObjectIdV1::GitSha1Jj045 {
        kind: GitObjectKindV1::Tree,
        id: m::GitSha1Id::new([3; 20]),
    };
    assert!(artifact.validate().is_err());
    let mut artifact = fixture_artifact();
    artifact.object = CodeObjectIdV1::GitSha1Jj045 {
        kind: GitObjectKindV1::Blob,
        id: m::GitSha1Id::new([0; 20]),
    };
    assert!(artifact.validate().is_err());
}

#[test]
fn portable_diff_selection_requires_the_exact_declared_side() {
    let mut anchor = anchor();
    let ArtifactTargetV2::CodeText(value) = &mut anchor.target else {
        panic!()
    };
    value.diff_context = Some(PortableDiffContextV2 {
        before: revision(4),
        before_path: Some("src/main.rs".into()),
        after: value.artifact.revision.clone(),
        after_path: Some(value.artifact.path.clone()),
        side: DiffSide::After,
    });
    anchor.validate().unwrap();
    let ArtifactTargetV2::CodeText(value) = &mut anchor.target else {
        panic!()
    };
    value.diff_context.as_mut().unwrap().side = DiffSide::Before;
    assert!(anchor.validate().is_err());
}

#[test]
fn portable_code_evidence_binds_the_anchor_repository_and_complete_diff_pair() {
    let mut anchor = anchor();
    anchor.repository_id = m::RepositoryId::new([8; 16]);
    assert!(anchor.validate().is_err());
    anchor.repository_id = m::RepositoryId::new([1; 16]);
    let ArtifactTargetV2::CodeText(value) = &mut anchor.target else {
        panic!()
    };
    value.diff_context = Some(PortableDiffContextV2 {
        before: revision(4),
        before_path: Some("src/main.rs".into()),
        after: value.artifact.revision.clone(),
        after_path: Some(value.artifact.path.clone()),
        side: DiffSide::After,
    });
    assert_eq!(anchor.code_evidence().unwrap().len(), 2);
    let ArtifactTargetV2::CodeText(value) = &mut anchor.target else {
        panic!()
    };
    value
        .diff_context
        .as_mut()
        .unwrap()
        .before
        .evidence
        .repository = m::RepositoryId::new([8; 16]);
    assert!(anchor.code_evidence().is_err());
    for target in [
        ArtifactTargetV2::Revision(revision(2)),
        ArtifactTargetV2::Operation(revision(2).evidence),
    ] {
        anchor.target = target;
        anchor.validate().unwrap();
        anchor.repository_id = m::RepositoryId::new([8; 16]);
        assert!(anchor.validate().is_err());
        anchor.repository_id = m::RepositoryId::new([1; 16]);
    }
}

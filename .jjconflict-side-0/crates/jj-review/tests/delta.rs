use jj_review::{
    ReviewCatalogEditV1, ReviewCatalogKey, ReviewCatalogKind, ReviewDeltaV1, ReviewObjectId,
    ReviewObjectKind, ReviewObjectRef,
};

fn reference(kind: ReviewObjectKind, byte: u8) -> ReviewObjectRef {
    ReviewObjectRef::new(kind, ReviewObjectId::new([byte; 32]))
}

#[test]
fn review_delta_accepts_typed_expected_replacement_edits() {
    let key = ReviewCatalogKey::derive("thread", b"one").unwrap();
    let delta = ReviewDeltaV1 {
        schema_version: 1,
        edits: vec![ReviewCatalogEditV1 {
            catalog: ReviewCatalogKind::Threads,
            key,
            expected: Some(reference(ReviewObjectKind::ThreadSnapshot, 1)),
            replacement: Some(reference(ReviewObjectKind::ThreadSnapshot, 2)),
        }],
    };
    assert!(delta.validate().is_ok());
}

#[test]
fn review_delta_rejects_noops_duplicates_and_semantic_kind_substitution() {
    let key = ReviewCatalogKey::derive("comment", b"one").unwrap();
    let selected = reference(ReviewObjectKind::CommentRevision, 1);
    let no_op = ReviewCatalogEditV1 {
        catalog: ReviewCatalogKind::Comments,
        key,
        expected: Some(selected),
        replacement: Some(selected),
    };
    assert!(
        ReviewDeltaV1 {
            schema_version: 1,
            edits: vec![no_op]
        }
        .validate()
        .is_err()
    );

    let edit = ReviewCatalogEditV1 {
        catalog: ReviewCatalogKind::Comments,
        key,
        expected: None,
        replacement: Some(selected),
    };
    assert!(
        ReviewDeltaV1 {
            schema_version: 1,
            edits: vec![edit.clone(), edit]
        }
        .validate()
        .is_err()
    );
    assert!(
        ReviewDeltaV1 {
            schema_version: 1,
            edits: vec![ReviewCatalogEditV1 {
                catalog: ReviewCatalogKind::ThreadEvents,
                key,
                expected: None,
                replacement: Some(reference(ReviewObjectKind::ThreadSnapshot, 3)),
            }]
        }
        .validate()
        .is_err()
    );
}

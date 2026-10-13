use jj_store_model as m;
use jj_workflow as w;
#[test]
fn declarative_conditions_match_without_an_executable_language() {
    assert!(w::bookmark_matches("release/*", "release/1.0"));
    assert!(w::bookmark_matches("m?in", "main"));
    assert!(!w::bookmark_matches("main", "domain"));
    let mut c = w::Conditions {
        authors: ["alice".into()].into(),
        thread: Some("11".repeat(16)),
        text: Some(w::TextCondition {
            operator: w::TextOperator::Contains,
            value: "deploy".into(),
        }),
        ..Default::default()
    };
    assert!(c.matches_content("alice", "please deploy", Some(&"11".repeat(16)), None, None));
    assert!(!c.matches_content("bob", "please deploy", Some(&"11".repeat(16)), None, None));
    c.excluded_authors.insert("alice".into());
    assert!(!c.matches_content("alice", "please deploy", Some(&"11".repeat(16)), None, None));
    c.thread = Some("invalid".into());
    assert!(c.validate().is_err());
}
#[test]
fn legacy_definitions_keep_their_canonical_bytes_and_missing_proof_fails_closed() {
    let input = serde_json::json!({"name":"build","description":"","enabled":true,"repository":"01".repeat(16),"trigger":{"on":"code_published"},"labels":[],"actions":[{"id":"build","executor":{"type":"command","name":"build"},"inputs":{},"timeout_seconds":300}]});
    let mut definition: w::Definition = serde_json::from_value(input.clone()).unwrap();
    assert_eq!(serde_json::to_value(&definition).unwrap(), input);
    definition.conditions.revset = Some("all()".into());
    let event = w::SourceEvent::new(
        "native/selected".into(),
        w::EventKind::CodePublished,
        w::Source {
            repository: m::RepositoryId::new([1; 16]),
            operation: Some(m::OperationId::new([2; 64])),
            commit: Some(m::CommitId::new([3; 64])),
            bookmark: None,
            conversation: None,
        },
    )
    .unwrap();
    // Published source identities must remain byte-identical across the additive schema.
    use sha2::{Digest, Sha256};
    let legacy = format!(
        r#"["native/selected","code_published",{{"repository":"{}","operation":"{}","commit":"{}","bookmark":null}}]"#,
        event.source.repository,
        event.source.operation.unwrap(),
        event.source.commit.unwrap()
    );
    assert_eq!(
        event.id.as_bytes(),
        &<[u8; 32]>::from(Sha256::digest(legacy.as_bytes()))
    );
    assert!(definition.matches(&event).is_err());
    let mut proven = event.clone();
    proven
        .filter_results
        .insert(definition.conditions.id().unwrap(), true);
    assert_eq!(proven.id, event.id);
    assert!(definition.matches(&proven).unwrap());
    proven.validate().unwrap();
}

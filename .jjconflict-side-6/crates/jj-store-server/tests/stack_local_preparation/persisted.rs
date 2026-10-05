use super::*;

#[tokio::test]
async fn malformed_saved_execution_is_rejected_without_repair_or_source_writes() {
    let f = Fixture::new().await;
    let state = f.witness().await;
    let req = request(&state);
    let private = tempfile::tempdir().unwrap();
    let path = private.path().join("stacks");
    let journal = StackCommandJournal::create(&path, scope(&state)).unwrap();
    let auth = Allowed {
        access: f.access(),
        root: f.root(),
        state,
    };
    stack_preparation::prepare_reword(&journal, &Coordinator::new(1).unwrap(), &auth, actor(), req)
        .await
        .unwrap();
    let file = path.join("state.json");
    let original = std::fs::read(&file).unwrap();
    let original_value: serde_json::Value = serde_json::from_slice(&original).unwrap();
    let before = f.state();
    let mut malformed = Vec::new();
    let mut value = original_value.clone();
    value["version"] = 1.into();
    malformed.push(value);
    for field in ["witness_id", "archive"] {
        let mut value = original_value.clone();
        value["records"][0]["execution"][field] =
            serde_json::to_value(m::RecordDigest::new([0; 32])).unwrap();
        malformed.push(value);
    }
    let mut value = original_value.clone();
    value["records"][0]["execution"]["key"]["prepared"] =
        serde_json::to_value(m::RecordDigest::new([97; 32])).unwrap();
    malformed.push(value);
    let mut value = original_value.clone();
    value["records"][0]["execution"]["retention"] = serde_json::Value::Null;
    malformed.push(value);
    for value in malformed {
        let bytes = serde_json::to_vec(&value).unwrap();
        std::fs::write(&file, &bytes).unwrap();
        assert!(StackCommandJournal::open(&path, scope(&auth.state)).is_err());
        assert_eq!(std::fs::read(&file).unwrap(), bytes);
        assert_eq!(f.state(), before);
    }
    std::fs::write(file, original).unwrap();
    StackCommandJournal::open(&path, scope(&auth.state)).unwrap();
}

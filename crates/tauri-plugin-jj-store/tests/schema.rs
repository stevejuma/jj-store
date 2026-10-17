#[test]
fn checked_schema_matches_owned_native_history_contract() {
    let current = serde_json::to_value(schemars::schema_for!(
        tauri_plugin_jj_store::frames::Contract
    ))
    .unwrap();
    let saved: serde_json::Value = serde_json::from_str(include_str!("../schema.json")).unwrap();
    assert_eq!(current, saved);
}

use jj_history_service::frames::*;
#[test]
fn renderer_frames_cannot_supply_native_authority() {
    for field in [
        "window",
        "url",
        "root",
        "metadata_path",
        "generation",
        "cache",
    ] {
        let mut value = serde_json::json!({"vault":"token"});
        value[field] = serde_json::json!("outside");
        assert!(
            serde_json::from_value::<Vault>(value.clone()).is_err(),
            "{field}"
        );
        value["operation"] = serde_json::json!("00".repeat(64));
        assert!(serde_json::from_value::<Open>(value).is_err(), "{field}");
        let mut selected = serde_json::json!({"vault":"token", "handle":Uuid::new_v4()});
        selected[field] = serde_json::json!("outside");
        assert!(
            serde_json::from_value::<Selection>(selected.clone()).is_err(),
            "{field}"
        );
        selected["query"] = serde_json::json!({"kind":"query_info"});
        assert!(
            serde_json::from_value::<Read>(selected.clone()).is_err(),
            "{field}"
        );
        selected["query"] = serde_json::json!({"commit":"00".repeat(20), "path":"note.md"});
        assert!(
            serde_json::from_value::<Blame>(selected).is_err(),
            "{field}"
        );
    }
    assert!(serde_json::from_value::<Vault>(serde_json::json!({"vault":"token"})).is_ok());
}
use uuid::Uuid;

use jj_store_capture::transcript_sanitization::TranscriptSanitizer;
use serde_json::json;
#[test]
fn opaque_device_credentials_remain_redacted_after_workspace_rebinding() {
    let root = tempfile::tempdir().unwrap();
    let sanitizer = TranscriptSanitizer::new(
        &root.path().join("workspace"),
        &root.path().join("home"),
        &root.path().join("temporary"),
        &root.path().join("cache"),
    )
    .unwrap();
    let secret = "opaque-provider-credential-without-a-known-prefix";
    let sanitizer = sanitizer
        .for_secrets(&[secret.into()])
        .unwrap()
        .for_workspace(&root.path().join("another-workspace"))
        .unwrap();
    let payload=sanitizer.sanitize(json!({"requestId":"bb".repeat(16),"text":format!("Tool returned {secret}"),"nested":{"value":secret}})).unwrap();
    let encoded = std::str::from_utf8(payload.bytes()).unwrap();
    assert!(!encoded.contains(secret));
    assert!(encoded.contains("$SECRET"));
    assert!(encoded.contains(&"bb".repeat(16)));
}

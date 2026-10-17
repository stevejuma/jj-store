use jj_store_capture::{agent_sanitization, transcript_sanitization::TranscriptSanitizer};
use serde_json::json;
use std::path::Path;

#[test]
fn jj_payload_adapters_preserve_independent_native_results() {
    let cache = tempfile::tempdir().unwrap();
    let workspace = Path::new("/workspace/vault");
    let home = Path::new("/home/fixture");
    let temporary = Path::new("/tmp/fixture");
    let native =
        lapis_agent_redaction::TranscriptSanitizer::new(workspace, home, temporary, cache.path())
            .unwrap();
    let adapter = TranscriptSanitizer::new(workspace, home, temporary, cache.path()).unwrap();
    for value in [
        json!({"b": "/workspace/vault/note", "a": ["/home/fixture/private", "/tmp/fixture/a"]}),
        json!({"token": "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"}),
    ] {
        assert_eq!(
            native.sanitize(value.clone()).unwrap().bytes(),
            adapter.sanitize(value).unwrap().bytes()
        );
    }
    let raw = br#"{"z":"/workspace/vault/a","a":true}"#;
    let lapis_agent_redaction::SanitizedHookPayload::Sanitized {
        payload: expected,
        policy,
    } = lapis_agent_redaction::sanitize_hook_payload(raw, workspace, cache.path())
    else {
        panic!("sanitized native fixture")
    };
    let agent_sanitization::SanitizedHookPayload::Sanitized {
        payload: actual,
        policy: actual_policy,
    } = agent_sanitization::sanitize_hook_payload(raw, workspace, cache.path())
    else {
        panic!("sanitized JJ fixture")
    };
    assert_eq!(actual.bytes(), expected.bytes());
    assert_eq!(actual_policy, policy);
}

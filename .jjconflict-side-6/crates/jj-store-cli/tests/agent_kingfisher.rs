use jj_store_cli::agent_sanitization::{
    SanitizedHookPayload, sanitize_hook_payload, warm_kingfisher_cache,
};
use std::{process::Command, time::Instant};

const SECRET: &str = "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";

fn payload(workspace: &std::path::Path) -> Vec<u8> {
    serde_json::to_vec(&serde_json::json!({
        "session_id": "session-1",
        "message": format!("token={SECRET}"),
        "path": workspace.join("src/lib.rs"),
    }))
    .unwrap()
}

#[test]
fn kingfisher_redacts_secrets_and_normalizes_workspace_paths() {
    let workspace = tempfile::tempdir().unwrap();
    let cache = tempfile::tempdir().unwrap();
    warm_kingfisher_cache(cache.path()).unwrap();

    let result = sanitize_hook_payload(&payload(workspace.path()), workspace.path(), cache.path());
    let SanitizedHookPayload::Sanitized { payload, policy } = result else {
        panic!("expected sanitized payload");
    };
    let text = std::str::from_utf8(payload.bytes()).unwrap();
    assert!(!text.contains(SECRET));
    assert!(!text.contains(workspace.path().to_str().unwrap()));
    assert!(text.contains("[REDACTED]"));
    assert!(text.contains("$WORKSPACE/"));
    assert_eq!(policy, "mongodb-kingfisher@2.7.0/builtin-medium/v1");

    let started = Instant::now();
    let status = Command::new(std::env::current_exe().unwrap())
        .args(["--exact", "kingfisher_cached_scan_child"])
        .env("JJ_STORE_KINGFISHER_CHILD_CACHE", cache.path())
        .env("JJ_STORE_KINGFISHER_CHILD_WORKSPACE", workspace.path())
        .status()
        .unwrap();
    assert!(status.success());
    assert!(started.elapsed().as_secs() < 5);
}

#[test]
fn kingfisher_cached_scan_child() {
    let Some(cache) = std::env::var_os("JJ_STORE_KINGFISHER_CHILD_CACHE") else {
        return;
    };
    let workspace = std::env::var_os("JJ_STORE_KINGFISHER_CHILD_WORKSPACE").unwrap();
    assert!(matches!(
        sanitize_hook_payload(
            &payload(std::path::Path::new(&workspace)),
            std::path::Path::new(&workspace),
            std::path::Path::new(&cache),
        ),
        SanitizedHookPayload::Sanitized { .. }
    ));
}

#[test]
fn invalid_or_oversized_hook_payload_is_omitted_without_source_details() {
    let workspace = tempfile::tempdir().unwrap();
    let cache = tempfile::tempdir().unwrap();
    assert_eq!(
        sanitize_hook_payload(b"not-json", workspace.path(), cache.path()),
        SanitizedHookPayload::Omitted
    );
    assert_eq!(
        sanitize_hook_payload(
            &vec![b'a'; 4 * 1024 * 1024 + 1],
            workspace.path(),
            cache.path(),
        ),
        SanitizedHookPayload::Omitted
    );
}

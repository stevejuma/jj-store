use jj_store_capture::{
    opencode_snapshot::{OpenCodeActivity, OpenCodeSessionSnapshot},
    source_registry::{OpenCodeSourceBinding, SourceRegistry},
};
use serde_json::json;
use std::{fs, process::Command};

fn binding(endpoint: u8, id: &str, directory: &str) -> OpenCodeSourceBinding {
    let session = OpenCodeSessionSnapshot::from_api_info(
        &serde_json::to_vec(&json!({"id":id,"version":"1.18.30","directory":directory})).unwrap(),
        id,
    )
    .unwrap();
    OpenCodeSourceBinding::new([endpoint; 32], &session).unwrap()
}

#[test]
fn admitted_session_index_survives_exit_before_enqueue_without_paths_or_allocation() {
    let root = tempfile::tempdir().unwrap();
    let path = root.path().join("registry");
    let registry = SourceRegistry::open(&path, [7; 16], 3).unwrap();
    assert!(
        registry
            .admitted_opencode_sessions([4; 32])
            .unwrap()
            .is_empty()
    );
    assert!(
        registry
            .acquire_opencode_active(
                binding(4, "ses_A", "/workspace/private"),
                OpenCodeActivity::Idle
            )
            .is_err()
    );
    let a = registry
        .acquire_opencode_active(
            binding(4, "ses_A", "/workspace/private"),
            OpenCodeActivity::Busy,
        )
        .unwrap();
    let b = registry
        .acquire_opencode_active(
            binding(4, "ses_b", "/workspace/other"),
            OpenCodeActivity::Retry,
        )
        .unwrap();
    let c = registry
        .acquire_opencode_active(
            binding(5, "ses_other", "/workspace/elsewhere"),
            OpenCodeActivity::Busy,
        )
        .unwrap();
    drop((a, b, c, registry));
    let before = fs::read(path.join("sources.json")).unwrap();
    let text = std::str::from_utf8(&before).unwrap();
    assert!(!text.contains("/workspace"));
    assert!(!text.contains("http"));
    let child = Command::new(std::env::current_exe().unwrap())
        .args(["--exact", "fresh_process_index_child", "--nocapture"])
        .env("JJ_STORE_OPENCODE_INDEX_TEST", &path)
        .output()
        .unwrap();
    assert!(
        child.status.success(),
        "{}",
        String::from_utf8_lossy(&child.stderr)
    );
    assert!(String::from_utf8_lossy(&child.stdout).contains("recovered admitted IDs"));
    assert_eq!(fs::read(path.join("sources.json")).unwrap(), before);
    let registry = SourceRegistry::open_existing(&path, [7; 16], 3)
        .unwrap()
        .unwrap();
    assert!(
        registry
            .lookup_opencode_admitted(&binding(4, "ses_A", "/workspace/rebound"))
            .unwrap()
            .is_none()
    );
    assert!(registry.admitted_opencode_sessions([0; 32]).is_err());
}

#[test]
fn fresh_process_index_child() {
    let Some(path) = std::env::var_os("JJ_STORE_OPENCODE_INDEX_TEST") else {
        return;
    };
    let registry = SourceRegistry::open_existing(std::path::Path::new(&path), [7; 16], 3)
        .unwrap()
        .unwrap();
    assert_eq!(
        registry.admitted_opencode_sessions([4; 32]).unwrap(),
        vec!["ses_A", "ses_b"]
    );
    assert_eq!(
        registry.admitted_opencode_sessions([5; 32]).unwrap(),
        vec!["ses_other"]
    );
    assert!(
        registry
            .admitted_opencode_sessions([6; 32])
            .unwrap()
            .is_empty()
    );
    let lease = registry
        .acquire_opencode_admitted(binding(4, "ses_A", "/workspace/private"))
        .unwrap()
        .unwrap();
    drop(lease);
    println!("recovered admitted IDs");
}

#[test]
fn malformed_or_unbound_index_withholds_registry_and_legacy_empty_index_is_readable() {
    let root = tempfile::tempdir().unwrap();
    let path = root.path().join("registry");
    let registry = SourceRegistry::open(&path, [7; 16], 1).unwrap();
    let mut old: serde_json::Value =
        serde_json::from_slice(&fs::read(path.join("sources.json")).unwrap()).unwrap();
    old.as_object_mut().unwrap().remove("opencode_sessions");
    fs::write(path.join("sources.json"), serde_json::to_vec(&old).unwrap()).unwrap();
    assert!(
        registry
            .admitted_opencode_sessions([4; 32])
            .unwrap()
            .is_empty()
    );
    let lease = registry
        .acquire_opencode_active(
            binding(4, "ses_A", "/workspace/private"),
            OpenCodeActivity::Busy,
        )
        .unwrap();
    drop(lease);
    let valid_bytes = fs::read(path.join("sources.json")).unwrap();
    let valid: serde_json::Value = serde_json::from_slice(&valid_bytes).unwrap();
    let mut old_admission = valid.clone();
    old_admission
        .as_object_mut()
        .unwrap()
        .remove("opencode_sessions");
    fs::write(
        path.join("sources.json"),
        serde_json::to_vec(&old_admission).unwrap(),
    )
    .unwrap();
    let old_registry = SourceRegistry::open_existing(&path, [7; 16], 1)
        .unwrap()
        .unwrap();
    assert!(
        old_registry
            .admitted_opencode_sessions([4; 32])
            .unwrap()
            .is_empty()
    );
    let lease = old_registry
        .acquire_opencode_admitted(binding(4, "ses_A", "/workspace/private"))
        .unwrap()
        .unwrap();
    assert_eq!(
        old_registry.admitted_opencode_sessions([4; 32]).unwrap(),
        vec!["ses_A"]
    );
    assert_eq!(fs::read(path.join("sources.json")).unwrap(), valid_bytes);
    drop((lease, old_registry));
    for field in [
        "native_session",
        "endpoint",
        "witness",
        "source",
        "product_version",
    ] {
        let mut corrupt = valid.clone();
        let entry = &mut corrupt["opencode_sessions"][0];
        entry[field] = match field {
            "native_session" => json!("ses_/unsafe/path"),
            "endpoint" | "witness" => json!(vec![0; 32]),
            "source" => json!("11111111111111111111111111111111"),
            _ => json!("unknown"),
        };
        fs::write(
            path.join("sources.json"),
            serde_json::to_vec(&corrupt).unwrap(),
        )
        .unwrap();
        assert!(
            SourceRegistry::open_existing(&path, [7; 16], 1).is_err(),
            "accepted corrupt {field}"
        );
    }
    let mut duplicate = valid.clone();
    duplicate["opencode_sessions"]
        .as_array_mut()
        .unwrap()
        .push(valid["opencode_sessions"][0].clone());
    fs::write(
        path.join("sources.json"),
        serde_json::to_vec(&duplicate).unwrap(),
    )
    .unwrap();
    assert!(SourceRegistry::open_existing(&path, [7; 16], 1).is_err());
}

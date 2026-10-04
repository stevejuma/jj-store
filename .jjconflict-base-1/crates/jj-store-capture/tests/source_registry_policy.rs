#![cfg(unix)]
use jj_store_capture::{
    codex_discovery::{ApprovedCodexRoot, CodexDirectoryScanner},
    profile::CaptureProfile,
    source_registry::{QualifiedCodexSource, SourceRegistry},
};
use serde_json::json;
use std::{
    fs,
    os::unix::fs::{PermissionsExt as _, symlink},
    path::Path,
};
const INSTALLATION: [u8; 16] = [8; 16];
fn write(path: &Path, desktop: bool) {
    let mut bytes = serde_json::to_vec(&json!({"type":"session_meta","payload":{
        "id":"00000000-0000-0000-0000-000000000001","cwd":"/private/project",
        "cli_version":if desktop { "0.159.2" } else { "0.156.1" },
        "source":if desktop { "vscode" } else { "cli" },
        "originator":if desktop { "Codex Desktop" } else { "codex_cli_rs" },
    }}))
    .unwrap();
    bytes.push(b'\n');
    fs::write(path, bytes).unwrap();
}
fn candidate(root: &Path, path: &Path) -> jj_store_capture::codex_discovery::CodexSourceCandidate {
    let path = fs::canonicalize(path).unwrap();
    let mut scanner =
        CodexDirectoryScanner::new(ApprovedCodexRoot::open(root).unwrap().unwrap()).unwrap();
    scanner
        .step(100, 100)
        .unwrap()
        .candidates
        .into_iter()
        .find(|c| c.path() == path)
        .unwrap()
}
fn qualified(root: &Path, path: &Path, desktop: bool) -> QualifiedCodexSource {
    QualifiedCodexSource::inspect(
        if desktop {
            CaptureProfile::CodexDesktopJsonlV1
        } else {
            CaptureProfile::CodexCliJsonlV1
        },
        candidate(root, path),
    )
    .unwrap()
    .unwrap()
}

#[test]
fn capacity_conflicting_aliases_and_installation_mismatch_preserve_registry() {
    let root = tempfile::tempdir().unwrap();
    let private = tempfile::tempdir().unwrap();
    let state = private.path().join("registry");
    let first = root.path().join("one.jsonl");
    let second = root.path().join("two.jsonl");
    let third = root.path().join("three.jsonl");
    for path in [&first, &second, &third] {
        write(path, false);
    }
    let registry = SourceRegistry::open(&state, INSTALLATION, 2).unwrap();
    let one = registry
        .acquire(qualified(root.path(), &first, false))
        .unwrap();
    let two = registry
        .acquire(qualified(root.path(), &second, false))
        .unwrap();
    assert_ne!(one.source_instance_id(), two.source_instance_id());
    let before = fs::read(state.join("sources.json")).unwrap();
    assert!(
        registry
            .acquire(qualified(root.path(), &third, false))
            .is_err()
    );
    assert_eq!(before, fs::read(state.join("sources.json")).unwrap());
    assert!(SourceRegistry::open(&state, [9; 16], 2).is_err());
    assert!(SourceRegistry::open(&state, INSTALLATION, 1).is_err());
    drop(one);
    drop(two);
    fs::remove_file(&second).unwrap();
    fs::hard_link(&first, &second).unwrap();
    assert!(
        registry
            .acquire(qualified(root.path(), &second, false))
            .is_err()
    );
    assert_eq!(before, fs::read(state.join("sources.json")).unwrap());
    // Distinct channels can be leased concurrently through independent handles.
    let other = SourceRegistry::open(&state, INSTALLATION, 2).unwrap();
    let _one = registry
        .acquire(qualified(root.path(), &first, false))
        .unwrap();
    fs::remove_file(&second).unwrap();
    write(&second, false);
    let _two = other
        .acquire(qualified(root.path(), &second, false))
        .unwrap();
}

#[test]
fn unsupported_or_changed_header_cannot_allocate_or_change_channel() {
    let root = tempfile::tempdir().unwrap();
    let private = tempfile::tempdir().unwrap();
    let state = private.path().join("registry");
    let path = root.path().join("session.jsonl");
    write(&path, false);
    let registry = SourceRegistry::open(&state, INSTALLATION, 4).unwrap();
    let before = fs::read(state.join("sources.json")).unwrap();
    assert!(
        QualifiedCodexSource::inspect(
            CaptureProfile::CodexDesktopJsonlV1,
            candidate(root.path(), &path)
        )
        .is_err()
    );
    let original = qualified(root.path(), &path, false);
    write(&path, true);
    assert!(registry.acquire(original).is_err());
    assert_eq!(before, fs::read(state.join("sources.json")).unwrap());
    let desktop = registry
        .acquire(qualified(root.path(), &path, true))
        .unwrap();
    let source = desktop.source_instance_id();
    let before = fs::read(state.join("sources.json")).unwrap();
    write(&path, false);
    assert!(
        registry
            .acquire(qualified(root.path(), &path, false))
            .is_err()
    );
    assert_eq!(before, fs::read(state.join("sources.json")).unwrap());
    drop(desktop);
    assert_eq!(
        registry
            .acquire(qualified(root.path(), &path, false))
            .unwrap()
            .source_instance_id(),
        source
    );
}

#[test]
fn private_state_rejects_checkouts_symlinks_permissions_and_rebound_leases() {
    let root = tempfile::tempdir().unwrap();
    let private = tempfile::tempdir().unwrap();
    let state = private.path().join("registry");
    let path = root.path().join("session.jsonl");
    write(&path, false);
    let checkout = private.path().join("checkout");
    fs::create_dir(&checkout).unwrap();
    fs::create_dir(checkout.join(".jj")).unwrap();
    assert!(SourceRegistry::open(&checkout.join("state"), INSTALLATION, 4).is_err());
    assert!(!checkout.join("state").exists());
    let insecure = private.path().join("insecure");
    fs::create_dir(&insecure).unwrap();
    fs::set_permissions(&insecure, fs::Permissions::from_mode(0o755)).unwrap();
    assert!(SourceRegistry::open(&insecure, INSTALLATION, 4).is_err());
    let registry = SourceRegistry::open(&state, INSTALLATION, 4).unwrap();
    let alias = private.path().join("alias");
    symlink(&state, &alias).unwrap();
    assert!(SourceRegistry::open(&alias, INSTALLATION, 4).is_err());
    let lease = registry
        .acquire(qualified(root.path(), &path, false))
        .unwrap();
    let source_lock = state.join(format!("source-{}.lock", lease.source_instance_id()));
    let mut reader = lease.open_reader(None).unwrap().unwrap();
    fs::rename(&source_lock, private.path().join("retained.lock")).unwrap();
    fs::write(&source_lock, []).unwrap();
    fs::set_permissions(&source_lock, fs::Permissions::from_mode(0o600)).unwrap();
    assert!(reader.poll().is_err());
    drop(reader);
    fs::set_permissions(
        state.join("sources.json"),
        fs::Permissions::from_mode(0o644),
    )
    .unwrap();
    assert!(SourceRegistry::open(&state, INSTALLATION, 4).is_err());
}

#[test]
fn registry_recovery_is_bounded_and_does_not_evict_history_or_unknown_files() {
    let private = tempfile::tempdir().unwrap();
    let state = private.path().join("registry");
    let registry = SourceRegistry::open(&state, INSTALLATION, 4).unwrap();
    drop(registry);
    let before = fs::read(state.join("sources.json")).unwrap();
    let stale = state.join(".tmpinterrupted");
    fs::write(&stale, b"incomplete safe private state").unwrap();
    fs::set_permissions(&stale, fs::Permissions::from_mode(0o600)).unwrap();
    SourceRegistry::open(&state, INSTALLATION, 4).unwrap();
    assert!(!stale.exists());
    assert_eq!(before, fs::read(state.join("sources.json")).unwrap());
    let unknown = state.join("unknown");
    fs::write(&unknown, []).unwrap();
    fs::set_permissions(&unknown, fs::Permissions::from_mode(0o600)).unwrap();
    assert!(SourceRegistry::open(&state, INSTALLATION, 4).is_err());
    assert!(unknown.exists());
    fs::remove_file(unknown).unwrap();
    fs::write(state.join("sources.json"), vec![b' '; 2 * 1024 * 1024 + 1]).unwrap();
    assert!(SourceRegistry::open(&state, INSTALLATION, 4).is_err());
}

#[test]
fn unknown_alias_of_changed_header_waits_for_known_location_rediscovery() {
    let root = tempfile::tempdir().unwrap();
    let private = tempfile::tempdir().unwrap();
    let state = private.path().join("registry");
    let path = root.path().join("original.jsonl");
    let alias = root.path().join("unknown-alias.jsonl");
    write(&path, false);
    let registry = SourceRegistry::open(&state, INSTALLATION, 4).unwrap();
    let source = registry
        .acquire(qualified(root.path(), &path, false))
        .unwrap()
        .source_instance_id();
    fs::hard_link(&path, &alias).unwrap();
    write(&path, true);
    let before = fs::read(state.join("sources.json")).unwrap();
    assert!(
        registry
            .acquire(qualified(root.path(), &alias, true))
            .is_err()
    );
    assert_eq!(before, fs::read(state.join("sources.json")).unwrap());
    assert_eq!(
        registry
            .acquire(qualified(root.path(), &path, true))
            .unwrap()
            .source_instance_id(),
        source
    );
    assert_eq!(
        registry
            .acquire(qualified(root.path(), &alias, true))
            .unwrap()
            .source_instance_id(),
        source
    );
}

#[test]
fn missing_catalog_and_rebound_directory_never_reallocate_source_ids() {
    let root = tempfile::tempdir().unwrap();
    let private = tempfile::tempdir().unwrap();
    let state = private.path().join("registry");
    let path = root.path().join("session.jsonl");
    write(&path, false);
    let registry = SourceRegistry::open(&state, INSTALLATION, 4).unwrap();
    drop(
        registry
            .acquire(qualified(root.path(), &path, false))
            .unwrap(),
    );
    let json = state.join("sources.json");
    let bytes = fs::read(&json).unwrap();
    fs::remove_file(&json).unwrap();
    assert!(
        registry
            .acquire(qualified(root.path(), &path, false))
            .is_err()
    );
    assert!(SourceRegistry::open(&state, INSTALLATION, 4).is_err());
    assert!(!json.exists());
    fs::write(&json, &bytes).unwrap();
    fs::set_permissions(&json, fs::Permissions::from_mode(0o600)).unwrap();
    let mut corrupt: serde_json::Value = serde_json::from_slice(&bytes).unwrap();
    corrupt["next"] = json!(1);
    fs::write(&json, serde_json::to_vec(&corrupt).unwrap()).unwrap();
    assert!(SourceRegistry::open(&state, INSTALLATION, 4).is_err());
    fs::write(&json, &bytes).unwrap();
    fs::rename(&state, private.path().join("retained-state")).unwrap();
    let replacement = SourceRegistry::open(&state, INSTALLATION, 4).unwrap();
    assert!(
        registry
            .acquire(qualified(root.path(), &path, false))
            .is_err()
    );
    drop(replacement);
}

#[test]
fn header_rewrite_requires_new_registration_before_returning_batches() {
    let root = tempfile::tempdir().unwrap();
    let private = tempfile::tempdir().unwrap();
    let state = private.path().join("registry");
    let path = root.path().join("session.jsonl");
    write(&path, false);
    let registry = SourceRegistry::open(&state, INSTALLATION, 4).unwrap();
    let registered = registry
        .acquire(qualified(root.path(), &path, false))
        .unwrap();
    let source = registered.source_instance_id();
    let rewrite = |suffix: char| {
        let mut header: serde_json::Value =
            serde_json::from_slice(&fs::read(&path).unwrap()).unwrap();
        header["payload"]["id"] = json!(format!("00000000-0000-0000-0000-00000000000{suffix}"));
        let mut bytes = serde_json::to_vec(&header).unwrap();
        bytes.push(b'\n');
        fs::write(&path, bytes).unwrap();
    };
    rewrite('2');
    assert!(registered.open_reader(None).is_err());
    let registered = registry
        .acquire(qualified(root.path(), &path, false))
        .unwrap();
    assert_eq!(registered.source_instance_id(), source);
    let mut reader = registered.open_reader(None).unwrap().unwrap();
    rewrite('3');
    assert!(reader.poll().is_err());
    assert!(!reader.revalidate().unwrap());
    assert!(reader.poll().is_err());
    drop(reader);
    assert_eq!(
        registry
            .acquire(qualified(root.path(), &path, false))
            .unwrap()
            .source_instance_id(),
        source
    );
}

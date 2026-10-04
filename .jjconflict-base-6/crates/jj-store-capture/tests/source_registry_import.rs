#![cfg(unix)]
use jj_store_capture::{
    codex_discovery::{ApprovedCodexRoot, CodexDirectoryScanner},
    codex_import::{CodexImportBinding, CodexImportPoll, CodexImportProbe},
    profile::CaptureProfile,
    source_registry::{QualifiedCodexSource, SourceRegistry},
};
use serde_json::json;
use std::{
    fs,
    io::Write as _,
    path::{Path, PathBuf},
};
#[path = "source_registry_import/leases.rs"]
mod leases;
#[path = "source_registry_import/loss.rs"]
mod loss;
#[path = "source_registry_import/restart.rs"]
mod restart;

struct Fixture {
    _root: tempfile::TempDir,
    transcripts: PathBuf,
    path: PathBuf,
    state: PathBuf,
    registry: SourceRegistry,
}
impl Fixture {
    fn new(capacity: usize) -> Self {
        let root = tempfile::tempdir().unwrap();
        let transcripts = root.path().join("transcripts");
        fs::create_dir(&transcripts).unwrap();
        let path = transcripts.join("session.jsonl");
        write(&path, CaptureProfile::CodexCliJsonlV1);
        let state = root.path().join("sources");
        let registry = SourceRegistry::open(&state, [8; 16], capacity).unwrap();
        Self {
            _root: root,
            transcripts,
            path,
            state,
            registry,
        }
    }
    fn qualified(&self) -> QualifiedCodexSource {
        qualify(&self.path, CaptureProfile::CodexCliJsonlV1)
    }
    fn prepared(&self) -> (CodexImportProbe, CodexImportBinding) {
        let mut probe = CodexImportProbe::select(self.qualified()).unwrap();
        let binding = ready(&mut probe);
        (probe, binding)
    }
    fn catalog(&self) -> Vec<u8> {
        fs::read(self.state.join("sources.json")).unwrap()
    }
}
fn write(path: &Path, profile: CaptureProfile) {
    let desktop = profile == CaptureProfile::CodexDesktopJsonlV1;
    fs::write(path, format!("{}\n", json!({"type":"session_meta","payload":{
        "id":"00000000-0000-0000-0000-000000000001", "cwd":"/private/import",
        "cli_version":profile.product_version(), "source":if desktop {"vscode"} else {"cli"},
        "originator":if desktop {"Codex Desktop"} else {"codex_cli_rs"}
    }}))).unwrap();
}
fn qualify(path: &Path, profile: CaptureProfile) -> QualifiedCodexSource {
    let mut scanner = CodexDirectoryScanner::new(
        ApprovedCodexRoot::open(path.parent().unwrap())
            .unwrap()
            .unwrap(),
    )
    .unwrap();
    let physical = fs::canonicalize(path).unwrap();
    let candidate = scanner
        .step(100, 100)
        .unwrap()
        .candidates
        .into_iter()
        .find(|c| c.path() == physical)
        .unwrap();
    QualifiedCodexSource::inspect(profile, candidate)
        .unwrap()
        .unwrap()
}
fn ready(probe: &mut CodexImportProbe) -> CodexImportBinding {
    for _ in 0..20 {
        if let CodexImportPoll::Ready(snapshot) = probe.poll().unwrap() {
            return snapshot.binding();
        }
    }
    panic!("bounded import fixture did not settle");
}
fn append(path: &Path) {
    writeln!(
        fs::OpenOptions::new().append(true).open(path).unwrap(),
        "{}",
        json!({"type":"turn_context","payload":{"cwd":"/private/current"}})
    )
    .unwrap();
}

#[test]
fn import_requires_completed_exact_preparation_and_current_consent_before_allocation() {
    let fixture = Fixture::new(4);
    let (probe, binding) = fixture.prepared();
    let before = fixture.catalog();
    let pending = CodexImportProbe::select(fixture.qualified()).unwrap();
    assert!(
        fixture
            .registry
            .acquire_import(&pending, binding, || panic!(
                "unprepared selection reached consent"
            ))
            .is_err()
    );
    let mut changed = binding;
    changed.complete_byte_offset += 1;
    assert!(
        fixture
            .registry
            .acquire_import(&probe, changed, || panic!("changed intent reached consent"))
            .is_err()
    );
    assert!(
        fixture
            .registry
            .acquire_import(&probe, binding, || Err(
                "durable consent/current authority unavailable".into()
            ))
            .is_err()
    );
    assert_eq!(fixture.catalog(), before);
    assert_eq!(fs::read_dir(&fixture.state).unwrap().count(), 2);
    assert!(
        fixture
            .registry
            .lookup_admitted(&fixture.qualified())
            .unwrap()
            .is_none()
    );
}

#[test]
fn explicit_enrollment_preserves_registered_identity_and_exact_replay() {
    let fixture = Fixture::new(4);
    let source = fixture
        .registry
        .acquire(fixture.qualified())
        .unwrap()
        .source_instance_id();
    assert!(
        fixture
            .registry
            .lookup_admitted(&fixture.qualified())
            .unwrap()
            .is_none()
    );
    let (probe, binding) = fixture.prepared();
    let mut checks = 0;
    let imported = fixture
        .registry
        .acquire_import(&probe, binding, || {
            checks += 1;
            Ok(())
        })
        .unwrap();
    assert_eq!(checks, 3);
    assert_eq!(source, imported.source_instance_id());
    assert_eq!(
        fixture
            .registry
            .lookup_import_admitted(&probe, binding, || Ok(()))
            .unwrap(),
        Some(source)
    );
    assert!(
        fixture
            .registry
            .lookup_admitted(&fixture.qualified())
            .unwrap()
            .is_none()
    );
    assert!(
        fixture
            .registry
            .acquire_admitted(fixture.qualified())
            .unwrap()
            .is_none()
    );
    drop(imported);
    let before = fixture.catalog();
    let repeated = fixture
        .registry
        .acquire_import(&probe, binding, || Ok(()))
        .unwrap();
    assert_eq!(repeated.source_instance_id(), source);
    assert_eq!(fixture.catalog(), before);
    let stored: serde_json::Value = serde_json::from_slice(&before).unwrap();
    assert_eq!(stored["version"], 1);
    assert_eq!(stored["sources"].as_array().unwrap().len(), 1);
    assert_eq!(
        stored["sources"][0]["admitted_sessions"]
            .as_array()
            .unwrap()
            .len(),
        1
    );
    let text = String::from_utf8(before).unwrap();
    for private in [
        "/private/import",
        "00000000-0000-0000-0000-000000000001",
        "session_meta",
        "session.jsonl",
    ] {
        assert!(!text.contains(private));
    }
}

#[test]
fn appended_snapshot_requires_reconciliation_without_extending_accepted_history() {
    let fixture = Fixture::new(4);
    let (probe, binding) = fixture.prepared();
    let before = fixture.catalog();
    append(&fixture.path);
    assert!(
        fixture
            .registry
            .acquire_import(&probe, binding, || Ok(()))
            .is_err()
    );
    assert_eq!(fixture.catalog(), before);
    let mut reconciled = CodexImportProbe::reconcile(fixture.qualified(), binding).unwrap();
    assert_eq!(ready(&mut reconciled), binding);
    let _imported = fixture
        .registry
        .acquire_import(&reconciled, binding, || Ok(()))
        .unwrap();
    assert!(
        fixture
            .registry
            .lookup_import_admitted(&reconciled, binding, || Ok(()))
            .unwrap()
            .is_some()
    );
    assert!(fs::metadata(&fixture.path).unwrap().len() > binding.complete_byte_offset);
}

#[test]
fn replacement_or_changed_header_cannot_inherit_import_only_consent() {
    let fixture = Fixture::new(4);
    let (probe, binding) = fixture.prepared();
    let source = fixture
        .registry
        .acquire_import(&probe, binding, || Ok(()))
        .unwrap()
        .source_instance_id();
    let original = fs::read(&fixture.path).unwrap();
    fs::rename(&fixture.path, fixture.transcripts.join("original")).unwrap();
    fs::write(&fixture.path, &original).unwrap();
    let before = fixture.catalog();
    assert!(
        fixture
            .registry
            .acquire_import(&probe, binding, || Ok(()))
            .is_err()
    );
    assert!(CodexImportProbe::reconcile(fixture.qualified(), binding).is_err());
    assert!(
        fixture
            .registry
            .acquire_admitted(fixture.qualified())
            .unwrap()
            .is_none()
    );
    assert_eq!(fixture.catalog(), before);
    // Ordinary physical registration preserves aliases but never extends consent.
    assert_eq!(
        fixture
            .registry
            .acquire(fixture.qualified())
            .unwrap()
            .source_instance_id(),
        source
    );
    assert!(
        fixture
            .registry
            .lookup_admitted(&fixture.qualified())
            .unwrap()
            .is_none()
    );
    let changed = String::from_utf8(original)
        .unwrap()
        .replace("/private/import", "/private/changed");
    fs::write(&fixture.path, changed).unwrap();
    assert!(
        fixture
            .registry
            .acquire_admitted(fixture.qualified())
            .unwrap()
            .is_none()
    );
    write(&fixture.path, CaptureProfile::CodexDesktopJsonlV1);
    let desktop = qualify(&fixture.path, CaptureProfile::CodexDesktopJsonlV1);
    assert!(
        fixture
            .registry
            .lookup_admitted(&desktop)
            .unwrap()
            .is_none()
    );
    let mut fresh = CodexImportProbe::select(desktop).unwrap();
    let desktop_binding = ready(&mut fresh);
    assert_ne!(desktop_binding, binding);
    assert_eq!(
        fixture
            .registry
            .acquire_import(&fresh, desktop_binding, || Ok(()))
            .unwrap()
            .source_instance_id(),
        source
    );
}

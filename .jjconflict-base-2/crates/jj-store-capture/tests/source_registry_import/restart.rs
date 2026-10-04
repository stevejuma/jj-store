use super::*;
use std::process::Command;

#[test]
fn import_only_marker_never_promotes_unverified_prefix_into_ordinary_resume() {
    let fixture = Fixture::new(4);
    append(&fixture.path);
    let (probe, binding) = fixture.prepared();
    fixture
        .registry
        .acquire_import(&probe, binding, || Ok(()))
        .unwrap();
    let before = fixture.catalog();
    // Rewrite only accepted body/context, preserving the physical/header/session.
    let bytes = fs::read_to_string(&fixture.path).unwrap();
    fs::write(
        &fixture.path,
        bytes.replace("/private/current", "/private/rewrite"),
    )
    .unwrap();
    let reopened = SourceRegistry::open(&fixture.state, [8; 16], 4).unwrap();
    assert!(
        reopened
            .lookup_admitted(&fixture.qualified())
            .unwrap()
            .is_none()
    );
    assert!(
        reopened
            .acquire_admitted(fixture.qualified())
            .unwrap()
            .is_none()
    );
    assert!(
        reopened
            .lookup_import_admitted(&probe, binding, || Ok(()))
            .is_err()
    );
    let mut reconcile = CodexImportProbe::reconcile(fixture.qualified(), binding).unwrap();
    assert!(reconcile.poll().is_err());
    assert!(
        reopened
            .acquire_import(&reconcile, binding, || Ok(()))
            .is_err()
    );
    assert_eq!(fixture.catalog(), before);
}

#[test]
fn fresh_process_reconciles_retained_import_marker_without_retargeting_replacement() {
    let fixture = Fixture::new(4);
    let (probe, binding) = fixture.prepared();
    let source = fixture
        .registry
        .acquire_import(&probe, binding, || Ok(()))
        .unwrap()
        .source_instance_id();
    let metadata = json!({
        "physical":binding.physical_digest,"header":binding.header_digest,
        "session":binding.session_digest,"prefix":binding.prefix_digest,
        "offset":binding.complete_byte_offset,"records":binding.complete_records
    });
    let child = |replacement: bool| {
        let output = Command::new(std::env::current_exe().unwrap())
            .args([
                "--exact",
                "restart::import_registry_process_probe",
                "--nocapture",
            ])
            .env("JJ_STORE_IMPORT_REGISTRY", &fixture.state)
            .env("JJ_STORE_IMPORT_TRANSCRIPT", &fixture.path)
            .env("JJ_STORE_IMPORT_BINDING", metadata.to_string())
            .env("JJ_STORE_IMPORT_SOURCE", source.to_string())
            .env(
                "JJ_STORE_IMPORT_REPLACEMENT",
                if replacement { "1" } else { "0" },
            )
            .output()
            .unwrap();
        assert!(
            output.status.success(),
            "{}\n{}",
            String::from_utf8_lossy(&output.stdout),
            String::from_utf8_lossy(&output.stderr)
        );
    };
    append(&fixture.path);
    child(false);
    let original = fs::read(&fixture.path).unwrap();
    fs::rename(&fixture.path, fixture.transcripts.join("retained")).unwrap();
    fs::write(&fixture.path, original).unwrap();
    let before = fixture.catalog();
    child(true);
    assert_eq!(fixture.catalog(), before);
}

#[test]
fn import_registry_process_probe() {
    let Some(state) = std::env::var_os("JJ_STORE_IMPORT_REGISTRY") else {
        return;
    };
    let path = PathBuf::from(std::env::var_os("JJ_STORE_IMPORT_TRANSCRIPT").unwrap());
    let metadata: serde_json::Value =
        serde_json::from_str(&std::env::var("JJ_STORE_IMPORT_BINDING").unwrap()).unwrap();
    let array = |field: &str| serde_json::from_value(metadata[field].clone()).unwrap();
    let binding = CodexImportBinding {
        profile: CaptureProfile::CodexCliJsonlV1,
        physical_digest: array("physical"),
        header_digest: array("header"),
        session_digest: array("session"),
        prefix_digest: array("prefix"),
        complete_byte_offset: metadata["offset"].as_u64().unwrap(),
        complete_records: metadata["records"].as_u64().unwrap(),
    };
    let registry = SourceRegistry::open_existing(Path::new(&state), [8; 16], 4)
        .unwrap()
        .unwrap();
    let qualified = qualify(&path, CaptureProfile::CodexCliJsonlV1);
    assert!(registry.lookup_admitted(&qualified).unwrap().is_none());
    let reconcile = CodexImportProbe::reconcile(qualified, binding);
    if std::env::var("JJ_STORE_IMPORT_REPLACEMENT").unwrap() == "1" {
        assert!(reconcile.is_err());
        return;
    }
    let mut reconcile = reconcile.unwrap();
    assert_eq!(ready(&mut reconcile), binding);
    let source = registry
        .lookup_import_admitted(&reconcile, binding, || Ok(()))
        .unwrap()
        .unwrap();
    assert_eq!(
        source.to_string(),
        std::env::var("JJ_STORE_IMPORT_SOURCE").unwrap()
    );
    assert_eq!(
        registry
            .acquire_import(&reconcile, binding, || Ok(()))
            .unwrap()
            .source_instance_id(),
        source
    );
}

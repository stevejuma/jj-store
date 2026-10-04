use super::*;
use std::process::Command;

#[test]
fn accepted_header_digest_matches_only_its_exact_reader_generation_witness() {
    let root = tempfile::tempdir().unwrap();
    let path = root.path().join("session.jsonl");
    header(&path, false);
    let source = qualified(&path, CaptureProfile::CodexCliJsonlV1);
    let accepted = ready(&mut CodexImportProbe::select(source.clone()).unwrap()).binding();
    assert!(accepted.matches_generation_digest(source.generation_digest()));
    let mut changed = source.generation_digest();
    changed[0] ^= 1;
    assert!(!accepted.matches_generation_digest(changed));
    assert!(!accepted.matches_generation_digest([0; 32]));
    let bytes = fs::read(&path).unwrap();
    fs::rename(&path, root.path().join("original")).unwrap();
    fs::write(&path, bytes).unwrap();
    assert!(!accepted.matches_generation_digest(
        qualified(&path, CaptureProfile::CodexCliJsonlV1).generation_digest()
    ));
}

#[test]
fn binding_rejects_byte_identical_physical_replacement_and_changed_header_or_profile() {
    let root = tempfile::tempdir().unwrap();
    let path = root.path().join("session.jsonl");
    header(&path, false);
    let accepted = ready(&mut select(&path)).binding();
    let bytes = fs::read(&path).unwrap();
    fs::rename(&path, root.path().join("held-original")).unwrap();
    fs::write(&path, &bytes).unwrap();
    assert!(
        CodexImportProbe::reconcile(qualified(&path, CaptureProfile::CodexCliJsonlV1), accepted)
            .is_err()
    );
    fs::remove_file(&path).unwrap();
    fs::rename(root.path().join("held-original"), &path).unwrap();
    let changed = String::from_utf8(bytes.clone())
        .unwrap()
        .replace("/workspace/a", "/workspace/b");
    fs::write(&path, changed).unwrap();
    assert!(
        CodexImportProbe::reconcile(qualified(&path, CaptureProfile::CodexCliJsonlV1), accepted)
            .is_err()
    );
    fs::write(&path, &bytes).unwrap();
    let mut invalid = accepted;
    invalid.profile = CaptureProfile::CodexDesktopJsonlV1;
    assert!(
        CodexImportProbe::reconcile(qualified(&path, CaptureProfile::CodexCliJsonlV1), invalid)
            .is_err()
    );
    for field in 0..5 {
        let mut invalid = accepted;
        match field {
            0 => invalid.physical_digest = [0; 32],
            1 => invalid.header_digest = [0; 32],
            2 => invalid.session_digest = [0; 32],
            3 => invalid.complete_records = 0,
            _ => invalid.complete_byte_offset = 0,
        }
        assert!(
            CodexImportProbe::reconcile(qualified(&path, CaptureProfile::CodexCliJsonlV1), invalid)
                .is_err()
        );
    }
}

#[test]
fn qualified_desktop_surface_has_the_same_complete_record_preparation_bounds() {
    let root = tempfile::tempdir().unwrap();
    let path = root.path().join("desktop.jsonl");
    header(&path, true);
    append(
        &path,
        json!({"type":"event_msg","payload":{"type":"task_started"}}),
    );
    let mut probe =
        CodexImportProbe::select(qualified(&path, CaptureProfile::CodexDesktopJsonlV1)).unwrap();
    let snapshot = ready(&mut probe);
    assert_eq!(
        snapshot.binding().profile,
        CaptureProfile::CodexDesktopJsonlV1
    );
    assert_eq!(snapshot.binding().complete_records, 2);
    assert_eq!(
        snapshot.binding().complete_byte_offset,
        fs::metadata(&path).unwrap().len()
    );
}

#[test]
fn accepted_binding_reconciles_in_a_fresh_process_without_retargeting_appended_history() {
    const CHILD_PATH: &str = "JJ_STORE_IMPORT_PROBE_COLD_PATH";
    const CHILD_BINDING: &str = "JJ_STORE_IMPORT_PROBE_COLD_BINDING";
    if let Some(path) = std::env::var_os(CHILD_PATH) {
        let encoded = std::env::var(CHILD_BINDING).unwrap();
        let parts: Vec<_> = encoded.split(':').collect();
        let digest = |i: usize| -> [u8; 32] {
            (0..32)
                .map(|n| u8::from_str_radix(&parts[i][n * 2..n * 2 + 2], 16).unwrap())
                .collect::<Vec<_>>()
                .try_into()
                .unwrap()
        };
        let binding = CodexImportBinding {
            profile: CaptureProfile::CodexCliJsonlV1,
            physical_digest: digest(0),
            header_digest: digest(1),
            session_digest: digest(2),
            prefix_digest: digest(3),
            complete_byte_offset: parts[4].parse().unwrap(),
            complete_records: parts[5].parse().unwrap(),
        };
        let mut probe = CodexImportProbe::reconcile(
            qualified(Path::new(&path), CaptureProfile::CodexCliJsonlV1),
            binding,
        )
        .unwrap();
        let snapshot = ready(&mut probe);
        assert_eq!(snapshot.binding(), binding);
        assert_eq!(snapshot.directory(), Some(Path::new("/workspace/b")));
        println!("import-probe-cold-ok");
        return;
    }
    let root = tempfile::tempdir().unwrap();
    let path = root.path().join("session.jsonl");
    header(&path, false);
    let binding = ready(&mut select(&path)).binding();
    append(
        &path,
        json!({"type":"turn_context","payload":{"cwd":"/workspace/b"}}),
    );
    let digest = |bytes: [u8; 32]| bytes.iter().map(|b| format!("{b:02x}")).collect::<String>();
    let encoded = format!(
        "{}:{}:{}:{}:{}:{}",
        digest(binding.physical_digest),
        digest(binding.header_digest),
        digest(binding.session_digest),
        digest(binding.prefix_digest),
        binding.complete_byte_offset,
        binding.complete_records
    );
    let output=Command::new(std::env::current_exe().unwrap()).args(["--exact","identity::accepted_binding_reconciles_in_a_fresh_process_without_retargeting_appended_history","--nocapture"])
        .env(CHILD_PATH,&path).env(CHILD_BINDING,encoded).output().unwrap();
    assert!(
        output.status.success(),
        "{}",
        String::from_utf8_lossy(&output.stderr)
    );
    assert!(String::from_utf8_lossy(&output.stdout).contains("import-probe-cold-ok"));
}

use super::*;

#[test]
fn accepted_boundary_survives_append_with_current_context_and_excludes_new_tail() {
    let root = tempfile::tempdir().unwrap();
    let path = root.path().join("session.jsonl");
    header(&path, false);
    append(
        &path,
        json!({"type":"turn_context","payload":{"cwd":"/workspace/b"}}),
    );
    let mut initial = select(&path);
    let accepted = ready(&mut initial).binding();
    append(
        &path,
        json!({"type":"turn_context","payload":{"cwd":"/workspace/c"}}),
    );
    assert!(!initial.revalidate().unwrap());
    let mut restored =
        CodexImportProbe::reconcile(qualified(&path, CaptureProfile::CodexCliJsonlV1), accepted)
            .unwrap();
    let snapshot = ready(&mut restored);
    assert_eq!(snapshot.binding(), accepted);
    assert_eq!(snapshot.directory(), Some(Path::new("/workspace/c")));
    assert!(snapshot.binding().complete_byte_offset < fs::metadata(&path).unwrap().len());
    // New selection may see appended history; it cannot replace accepted metadata.
    assert!(
        ready(&mut select(&path)).binding().complete_byte_offset > accepted.complete_byte_offset
    );
    fs::OpenOptions::new()
        .append(true)
        .open(&path)
        .unwrap()
        .write_all(b"partial tail")
        .unwrap();
    let snapshot = ready(&mut restored);
    assert_eq!(snapshot.binding(), accepted);
    assert_eq!(snapshot.directory(), Some(Path::new("/workspace/c")));
}

#[test]
fn selected_prefix_rewrite_truncation_count_or_nonrecord_boundary_is_withheld() {
    let root = tempfile::tempdir().unwrap();
    let path = root.path().join("session.jsonl");
    header(&path, false);
    append(
        &path,
        json!({"type":"turn_context","payload":{"cwd":"/workspace/b"}}),
    );
    let accepted = ready(&mut select(&path)).binding();
    let bytes = fs::read(&path).unwrap();
    for fault in 0..5 {
        let mut binding = accepted;
        let content = match fault {
            0 => String::from_utf8(bytes.clone())
                .unwrap()
                .replace("/workspace/b", "/workspace/c")
                .into_bytes(),
            1 => bytes[..bytes.len() - 1].to_vec(),
            2 => {
                binding.complete_records += 1;
                bytes.clone()
            }
            3 => {
                binding.complete_byte_offset -= 1;
                bytes.clone()
            }
            _ => {
                binding.prefix_digest = [8; 32];
                bytes.clone()
            }
        };
        fs::write(&path, &content).unwrap();
        let mut restored =
            CodexImportProbe::reconcile(qualified(&path, CaptureProfile::CodexCliJsonlV1), binding);
        let withheld = match restored.as_mut() {
            Err(_) => true,
            Ok(probe) => probe.poll().is_err(),
        };
        assert!(withheld, "fault {fault}");
        fs::write(&path, &bytes).unwrap();
    }
    let mut restored =
        CodexImportProbe::reconcile(qualified(&path, CaptureProfile::CodexCliJsonlV1), accepted)
            .unwrap();
    assert_eq!(ready(&mut restored).binding(), accepted);
}

#[test]
fn multi_step_snapshots_restart_preparation_but_preserve_accepted_prefix_on_reconcile() {
    let root = tempfile::tempdir().unwrap();
    let path = root.path().join("session.jsonl");
    header(&path, false);
    for _ in 0..7 {
        append(&path, json!({"padding":"x".repeat(800*1024)}));
    }
    append(
        &path,
        json!({"type":"turn_context","payload":{"cwd":"/workspace/b"}}),
    );
    let mut probe = select(&path);
    assert!(matches!(probe.poll().unwrap(), CodexImportPoll::Verifying));
    append(
        &path,
        json!({"type":"turn_context","payload":{"cwd":"/workspace/c"}}),
    );
    assert!(matches!(
        probe.poll().unwrap(),
        CodexImportPoll::ChangedDuringRead
    ));
    let accepted = ready(&mut probe).binding();
    let mut restored =
        CodexImportProbe::reconcile(qualified(&path, CaptureProfile::CodexCliJsonlV1), accepted)
            .unwrap();
    assert!(matches!(
        restored.poll().unwrap(),
        CodexImportPoll::Verifying
    ));
    append(
        &path,
        json!({"type":"turn_context","payload":{"cwd":"/workspace/d"}}),
    );
    assert!(matches!(
        restored.poll().unwrap(),
        CodexImportPoll::ChangedDuringRead
    ));
    let snapshot = ready(&mut restored);
    assert_eq!(snapshot.binding(), accepted);
    assert_eq!(snapshot.directory(), Some(Path::new("/workspace/d")));
}

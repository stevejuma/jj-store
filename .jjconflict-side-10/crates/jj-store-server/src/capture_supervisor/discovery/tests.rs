use super::*;
use std::fs;

fn file(root: &std::path::Path, id: u8) {
    let header = serde_json::json!({"type":"session_meta","payload":{
        "id":format!("00000000-0000-0000-0000-{id:012}"),
        "cwd":root.join("workspace"),"cli_version":"0.156.1",
        "source":"cli","originator":"codex_cli_rs"
    }});
    fs::write(root.join(format!("{id}.jsonl")), format!("{header}\n")).unwrap();
}

#[test]
fn busy_rescan_preserves_progress_and_coalesces_one_followup_before_cooldown() {
    let root = tempfile::tempdir().unwrap();
    for id in 1..=3 {
        file(root.path(), id);
    }
    let mut work = DiscoveryWork::new(
        vec![(CaptureProfile::CodexCliJsonlV1, root.path().to_owned())],
        Duration::from_secs(30),
    );
    let first = work.advance().unwrap().unwrap().fingerprint();
    assert!(work.roots[0].scanner.is_some());
    for _ in 0..100 {
        work.rescan();
    }
    assert!(work.coverage().rescan_pending);
    let mut seen = BTreeSet::from([first]);
    while work.roots[0].scanner.is_some() {
        if let Some(source) = work.advance().unwrap() {
            assert!(
                seen.insert(source.fingerprint()),
                "rescan restarted active scanner"
            );
        }
    }
    assert_eq!(seen.len(), 3);
    assert!(work.roots[0].next <= Instant::now());
    assert!(!work.coverage().rescan_pending);
    let mut followup = BTreeSet::new();
    loop {
        if let Some(source) = work.advance().unwrap() {
            followup.insert(source.fingerprint());
        }
        if work.roots[0].scanner.is_none() {
            break;
        }
    }
    assert!(
        followup == seen,
        "fresh pass changed the qualified source set"
    );
    assert!(work.roots[0].next > Instant::now() + Duration::from_secs(29));
    file(root.path(), 4);
    assert!(work.advance().unwrap().is_none());
    work.rescan();
    let mut fresh = BTreeSet::new();
    loop {
        if let Some(source) = work.advance().unwrap() {
            fresh.insert(source.fingerprint());
        }
        if work.roots[0].scanner.is_none() {
            break;
        }
    }
    assert_eq!(fresh.len(), 4);
    assert!(seen.is_subset(&fresh));
}

#[test]
fn missing_root_remains_cached_until_requested_recovery() {
    let device = tempfile::tempdir().unwrap();
    let root = device.path().join("missing");
    let mut work = DiscoveryWork::new(
        vec![(CaptureProfile::CodexCliJsonlV1, root.clone())],
        Duration::from_secs(30),
    );
    assert_eq!(work.coverage().unchecked, 1);
    assert!(work.advance().unwrap().is_none());
    assert_eq!(work.coverage().unavailable, 1);
    fs::create_dir(&root).unwrap();
    file(&root, 1);
    assert!(work.advance().unwrap().is_none());
    assert_eq!(work.coverage().unavailable, 1);
    work.rescan();
    assert!(work.advance().unwrap().is_some());
    assert_eq!(work.coverage().available, 1);
    assert_eq!(work.coverage().unavailable, 0);
    fs::rename(&root, device.path().join("retained")).unwrap();
    assert_eq!(
        work.coverage().available,
        1,
        "cached observation touched filesystem"
    );
    assert!(work.advance().is_err());
    assert_eq!(work.coverage().unavailable, 1);
}

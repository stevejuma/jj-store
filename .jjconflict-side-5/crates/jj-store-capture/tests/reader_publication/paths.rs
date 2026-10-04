use super::*;
#[test]
fn ingress_capacity_refusal_does_not_select_a_publication_requirement() {
    let root = tempfile::tempdir().unwrap();
    let mut spool = AgentCaptureSpool::open(root.path(), 128).unwrap();
    assert!(spool.enqueue_from_automatic_reader(input(1, REPO)).is_err());
    assert!(spool.pending().unwrap().is_empty());
    assert!(!root.path().join("reader-publication").exists());
    drop(spool);
    let mut spool = AgentCaptureSpool::open(root.path(), 4 * 1024 * 1024).unwrap();
    spool.enqueue_from_automatic_reader(input(1, REPO)).unwrap();
    assert!(
        spool
            .reader_publication_progress(IntegrationId::Codex, SOURCE, [1; 32])
            .unwrap()
            .unwrap()
            .history_available
    );
}
#[test]
fn oversized_or_foreign_catalog_files_withhold_without_reading_transcripts_or_repair() {
    for foreign in [false, true] {
        let root = tempfile::tempdir().unwrap();
        let mut spool = AgentCaptureSpool::open(root.path(), 4 * 1024 * 1024).unwrap();
        spool.enqueue_from_automatic_reader(input(1, REPO)).unwrap();
        let directory = root.path().join("reader-publication");
        let path = directory.join("catalog.json");
        if foreign {
            fs::write(directory.join("foreign.json"), b"{}").unwrap();
        } else {
            fs::OpenOptions::new()
                .write(true)
                .open(&path)
                .unwrap()
                .set_len(2 * 1024 * 1024 + 1)
                .unwrap();
        }
        let before = fs::read(&path).unwrap();
        assert!(spool.enqueue_from_automatic_reader(input(2, REPO)).is_err());
        assert_eq!(spool.pending().unwrap().len(), 1);
        drop(spool);
        assert!(AgentCaptureSpool::open(root.path(), 4 * 1024 * 1024).is_err());
        assert_eq!(fs::read(&path).unwrap(), before);
    }
}
#[cfg(unix)]
#[test]
fn symlinked_or_public_selected_evidence_cannot_authorize_coverage() {
    use std::os::unix::fs::{PermissionsExt as _, symlink};
    for symlinked in [false, true] {
        let root = tempfile::tempdir().unwrap();
        let mut spool = AgentCaptureSpool::open(root.path(), 4 * 1024 * 1024).unwrap();
        spool.enqueue_from_automatic_reader(input(1, REPO)).unwrap();
        let path = root.path().join("reader-publication/catalog.json");
        let bytes = fs::read(&path).unwrap();
        if symlinked {
            let target = root.path().join("foreign-private.json");
            fs::rename(&path, &target).unwrap();
            symlink(&target, &path).unwrap();
        } else {
            fs::set_permissions(&path, fs::Permissions::from_mode(0o644)).unwrap();
        }
        assert!(
            spool
                .reader_publication_progress(IntegrationId::Codex, SOURCE, [1; 32])
                .is_err()
        );
        assert!(spool.enqueue_from_automatic_reader(input(2, REPO)).is_err());
        assert_eq!(fs::read(path).unwrap(), bytes);
    }
}

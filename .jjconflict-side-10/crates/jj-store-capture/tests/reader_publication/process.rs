use super::*;
#[test]
fn committed_frontier_survives_a_separate_process_exit() {
    const ENV: &str = "JJ_STORE_READER_PUBLICATION_TEST_CHILD";
    if let Some(path) = std::env::var_os(ENV) {
        let mut spool =
            AgentCaptureSpool::open(std::path::Path::new(&path), 4 * 1024 * 1024).unwrap();
        spool.enqueue_from_automatic_reader(input(1, REPO)).unwrap();
        let entry = spool.pending().unwrap().remove(0);
        ready(drain_capture_spool_entry(&mut spool, &entry, &Publisher)).unwrap();
        // No destructor or in-process handle carries this receipt back to the parent.
        std::process::exit(0);
    }
    let root = tempfile::tempdir().unwrap();
    let result = std::process::Command::new(std::env::current_exe().unwrap())
        .args([
            "--exact",
            "process::committed_frontier_survives_a_separate_process_exit",
            "--test-threads=1",
        ])
        .env(ENV, root.path())
        .output()
        .unwrap();
    assert!(
        result.status.success(),
        "{}",
        String::from_utf8_lossy(&result.stderr)
    );
    let spool = AgentCaptureSpool::open(root.path(), 4 * 1024 * 1024).unwrap();
    assert!(spool.pending().unwrap().is_empty());
    let progress = spool
        .reader_publication_progress(IntegrationId::Codex, SOURCE, [1; 32])
        .unwrap()
        .unwrap();
    assert!(progress.history_available);
    assert_eq!(progress.committed_byte_offset, 100);
    assert_eq!(progress.receipts, 1);
}

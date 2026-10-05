use super::*;
use std::process::Command;

#[test]
fn import_receipt_survives_fresh_process_exit_and_exact_retry() {
    const CHILD_PATH: &str = "JJ_STORE_IMPORT_JOURNAL_COLD_PATH";
    if let Some(path) = std::env::var_os(CHILD_PATH) {
        let journal =
            CaptureImportJournal::open(std::path::Path::new(&path), INSTALLATION, GENERATION)
                .unwrap();
        assert!(!journal.record(&intent(1), &source()).unwrap().replayed);
        // Exit without destructors; OS teardown must release the journal lock.
        std::process::exit(0);
    }
    let temporary = tempfile::tempdir().unwrap();
    let path = temporary.path().join("imports");
    let journal = CaptureImportJournal::create(&path, INSTALLATION, GENERATION).unwrap();
    let status = Command::new(std::env::current_exe().unwrap())
        .args([
            "--exact",
            "cold::import_receipt_survives_fresh_process_exit_and_exact_retry",
        ])
        .env(CHILD_PATH, &path)
        .status()
        .unwrap();
    assert!(status.success());
    let selected = fs::read(path.join("state.json")).unwrap();
    let receipt = journal.resolve(&intent(1)).unwrap().unwrap();
    let reopened = CaptureImportJournal::open(&path, INSTALLATION, GENERATION).unwrap();
    assert_eq!(
        reopened.record(&intent(1), &source()).unwrap(),
        CaptureImportSelection {
            receipt,
            replayed: true
        }
    );
    assert_eq!(fs::read(path.join("state.json")).unwrap(), selected);
}

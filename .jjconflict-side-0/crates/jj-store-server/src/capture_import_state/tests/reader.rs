use super::*;
use jj_store_service::collaboration::AgentSourceInstanceIdentity as SourceInstanceId;
pub(super) fn accepted() -> CaptureImportSourceBinding {
    let mut binding = source();
    // Independent pinned vector: header/v1 length-framed generation witness [4;32].
    binding.header_digest =
        hex::decode("c0c83c836b39fc9f69e00a5d38319e301f9624f46b824c5ce32d7e71a674288d")
            .unwrap()
            .try_into()
            .unwrap();
    binding
}
pub(super) fn reader() -> CaptureImportReaderBinding {
    CaptureImportReaderBinding {
        ordinal: 1,
        source: SourceInstanceId::new([7; 16]),
        generation: 3,
        generation_digest: [4; 32],
        prefix_verified: true,
    }
}
#[test]
fn synchronized_reader_selection_refreshes_the_held_stamp_without_changing_its_receipt() {
    let root = tempfile::tempdir().unwrap();
    let path = root.path().join("imports");
    let journal = CaptureImportJournal::create(&path, INSTALLATION, GENERATION).unwrap();
    let second = CaptureImportJournal::open(&path, INSTALLATION, GENERATION).unwrap();
    let mut selected = journal.select_owned(&intent(), &accepted()).unwrap();
    let original = selected.selection().receipt.clone();
    let stamp = selected.stamp;
    assert_eq!(selected.bind_reader(reader()).unwrap(), reader());
    assert!(selected.stamp != stamp);
    selected.check().unwrap();
    assert_eq!(selected.selection().receipt, original);
    assert_eq!(second.reader_binding(&intent()), Err(Error::Unavailable));
    drop(selected);
    assert_eq!(second.reader_binding(&intent()).unwrap(), Some(reader()));
    assert_eq!(second.entries().unwrap(), vec![original]);
}
#[test]
fn numeric_continuation_reuses_original_association_and_changed_reuse_keeps_selected_bytes() {
    let root = tempfile::tempdir().unwrap();
    let path = root.path().join("imports");
    let journal = CaptureImportJournal::create(&path, INSTALLATION, GENERATION).unwrap();
    let mut selected = journal.select_owned(&intent(), &accepted()).unwrap();
    selected.bind_reader(reader()).unwrap();
    drop(selected);
    let before = fs::read(path.join("state.json")).unwrap();
    let reopened = CaptureImportJournal::open(&path, INSTALLATION, GENERATION).unwrap();
    let mut selected = reopened.select_owned(&intent(), &accepted()).unwrap();
    let mut later = reader();
    later.generation = 6;
    assert_eq!(selected.bind_reader(later).unwrap(), reader());
    for mode in 0..4 {
        let mut changed = reader();
        let expected = match mode {
            0 => {
                changed.source = SourceInstanceId::new([8; 16]);
                Error::Conflict
            }
            1 => {
                changed.generation = 2;
                Error::Conflict
            }
            2 => {
                changed.generation_digest = [9; 32];
                Error::Invalid
            }
            _ => {
                changed.ordinal = 2;
                Error::Invalid
            }
        };
        assert_eq!(selected.bind_reader(changed), Err(expected));
        selected.check().unwrap();
        assert_eq!(fs::read(path.join("state.json")).unwrap(), before);
    }
}
#[test]
fn duplicate_foreign_invalid_or_oversized_reader_catalogs_fail_without_repair() {
    for mode in 0..8 {
        let root = tempfile::tempdir().unwrap();
        let path = root.path().join("imports");
        let journal = CaptureImportJournal::create(&path, INSTALLATION, GENERATION).unwrap();
        let mut selected = journal.select_owned(&intent(), &accepted()).unwrap();
        selected.bind_reader(reader()).unwrap();
        drop(selected);
        let file = path.join("state.json");
        let mut value: serde_json::Value =
            serde_json::from_slice(&fs::read(&file).unwrap()).unwrap();
        match mode {
            0 => {
                let extra = value["readers"][0].clone();
                value["readers"].as_array_mut().unwrap().push(extra);
            }
            1 => value["readers"][0]["ordinal"] = 2.into(),
            2 => value["readers"][0]["generation"] = 0.into(),
            3 => value["readers"][0]["generation_digest"] = "00".repeat(32).into(),
            4 => {
                value["readers"][0]["source"] =
                    serde_json::to_value(SourceInstanceId::new([0; 16])).unwrap()
            }
            5 => value["readers"][0]["generation_digest"] = "09".repeat(32).into(),
            6 => value["readers"][0]["extra"] = true.into(),
            _ => {
                let extra = value["readers"][0].clone();
                value["readers"] = serde_json::to_value(vec![extra; MAX_RECEIPTS + 1]).unwrap();
            }
        }
        let mut bytes = serde_json::to_vec(&value).unwrap();
        if mode == 7 {
            bytes.resize(MAX_BYTES + 1, b' ');
        }
        fs::write(&file, &bytes).unwrap();
        assert_eq!(journal.reader_binding(&intent()), Err(Error::Unavailable));
        assert!(matches!(
            CaptureImportJournal::open(&path, INSTALLATION, GENERATION),
            Err(Error::Unavailable)
        ));
        assert_eq!(fs::read(file).unwrap(), bytes);
    }
}
#[test]
fn original_reader_link_survives_process_exit_while_selection_gate_is_held() {
    const PATH: &str = "JJ_STORE_IMPORT_READER_CHILD_PATH";
    if let Some(path) = std::env::var_os(PATH) {
        let journal =
            CaptureImportJournal::open(std::path::Path::new(&path), INSTALLATION, GENERATION)
                .unwrap();
        let mut selected = journal.select_owned(&intent(), &accepted()).unwrap();
        selected.bind_reader(reader()).unwrap();
        selected.check().unwrap();
        std::process::exit(0);
    }
    let root = tempfile::tempdir().unwrap();
    let path = root.path().join("imports");
    let journal = CaptureImportJournal::create(&path, INSTALLATION, GENERATION).unwrap();
    let original = journal.record(&intent(), &accepted()).unwrap().receipt;
    let output = std::process::Command::new(std::env::current_exe().unwrap())
        .args(["--exact", "capture_import_state::tests::reader::original_reader_link_survives_process_exit_while_selection_gate_is_held", "--test-threads=1"])
        .env(PATH, &path).output().unwrap();
    assert!(
        output.status.success(),
        "{}",
        String::from_utf8_lossy(&output.stderr)
    );
    let reopened = CaptureImportJournal::open(&path, INSTALLATION, GENERATION).unwrap();
    assert_eq!(reopened.entries().unwrap(), vec![original]);
    assert_eq!(reopened.reader_binding(&intent()).unwrap(), Some(reader()));
}

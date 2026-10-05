use super::reader::*;
use super::*;
#[test]
fn old_reader_link_needs_explicit_prefix_proof_and_upgrade_retains_original_generation() {
    let root = tempfile::tempdir().unwrap();
    let path = root.path().join("imports");
    let journal = CaptureImportJournal::create(&path, INSTALLATION, GENERATION).unwrap();
    let mut selected = journal.select_owned(&intent(), &accepted()).unwrap();
    selected.bind_reader(reader()).unwrap();
    let receipt = selected.selection().receipt.clone();
    drop(selected);
    let mut value: serde_json::Value =
        serde_json::from_slice(&fs::read(path.join("state.json")).unwrap()).unwrap();
    value["readers"][0]
        .as_object_mut()
        .unwrap()
        .remove("prefix_verified");
    fs::write(path.join("state.json"), serde_json::to_vec(&value).unwrap()).unwrap();
    let original = journal.reader_binding(&intent()).unwrap().unwrap();
    assert!(!original.prefix_verified);
    let mut selected = journal.select_owned(&intent(), &accepted()).unwrap();
    let stamp = selected.stamp;
    let mut continuation = reader();
    continuation.generation = 6;
    let verified = selected.bind_reader(continuation).unwrap();
    assert!(verified.prefix_verified);
    assert_eq!(verified.generation, original.generation);
    assert!(selected.stamp != stamp);
    selected.check().unwrap();
    assert_eq!(selected.selection().receipt, receipt);
    drop(selected);
    assert_eq!(journal.reader_binding(&intent()).unwrap(), Some(verified));
}

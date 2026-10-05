use super::*;
use std::os::unix::fs::{PermissionsExt as _, symlink};

#[test]
fn import_open_never_initializes_or_repairs_missing_corrupt_foreign_or_checkout_state() {
    let temporary = tempfile::tempdir().unwrap();
    let path = temporary.path().join("imports");
    assert!(matches!(
        CaptureImportJournal::open(&path, INSTALLATION, GENERATION),
        Err(Error::Unavailable)
    ));
    assert!(!path.exists());
    let journal = CaptureImportJournal::create(&path, INSTALLATION, GENERATION).unwrap();
    let bytes = fs::read(path.join("state.json")).unwrap();
    for (installation, generation) in [([1; 16], GENERATION), (INSTALLATION, [2; 16])] {
        assert!(matches!(
            CaptureImportJournal::open(&path, installation, generation),
            Err(Error::Unavailable)
        ));
        assert_eq!(fs::read(path.join("state.json")).unwrap(), bytes);
    }
    fs::remove_file(path.join("state.json")).unwrap();
    assert_eq!(journal.entries(), Err(Error::Unavailable));
    assert!(matches!(
        CaptureImportJournal::open(&path, INSTALLATION, GENERATION),
        Err(Error::Unavailable)
    ));
    assert!(!path.join("state.json").exists());
    for bad in [
        b"{}".to_vec(),
        b"broken".to_vec(),
        vec![b' '; MAX_BYTES + 1],
    ] {
        fs::write(path.join("state.json"), &bad).unwrap();
        fs::set_permissions(path.join("state.json"), fs::Permissions::from_mode(0o600)).unwrap();
        assert_eq!(journal.entries(), Err(Error::Unavailable));
        assert_eq!(fs::read(path.join("state.json")).unwrap(), bad);
    }
    let checkout = temporary.path().join("checkout");
    fs::create_dir(&checkout).unwrap();
    fs::create_dir(checkout.join(".jj")).unwrap();
    assert!(matches!(
        CaptureImportJournal::create(&checkout.join("imports"), INSTALLATION, GENERATION),
        Err(Error::Unavailable)
    ));
    assert!(!checkout.join("imports").exists());
}

#[test]
fn import_handles_refuse_rebound_directories_locks_permissions_symlinks_and_unknown_files() {
    let temporary = tempfile::tempdir().unwrap();
    let path = temporary.path().join("imports");
    let journal = CaptureImportJournal::create(&path, INSTALLATION, GENERATION).unwrap();
    let command = intent(1);
    journal.record(&command, &source()).unwrap();
    let bytes = fs::read(path.join("state.json")).unwrap();
    for target in [&path, &path.join("state.json"), &path.join("import.lock")] {
        let mode = fs::metadata(target).unwrap().permissions().mode();
        fs::set_permissions(target, fs::Permissions::from_mode(mode | 0o044)).unwrap();
        assert_eq!(journal.resolve(&command), Err(Error::Unavailable));
        assert_ne!(
            fs::metadata(target).unwrap().permissions().mode() & 0o044,
            0
        );
        fs::set_permissions(target, fs::Permissions::from_mode(mode)).unwrap();
    }
    for name in ["state.json", "import.lock"] {
        let target = path.join(name);
        let held = temporary.path().join(name);
        fs::rename(&target, &held).unwrap();
        symlink(&held, &target).unwrap();
        assert_eq!(journal.resolve(&command), Err(Error::Unavailable));
        fs::remove_file(target).unwrap();
        fs::rename(held, path.join(name)).unwrap();
    }
    let lock = path.join("import.lock");
    let held = temporary.path().join("held-lock");
    fs::rename(&lock, &held).unwrap();
    fs::write(&lock, []).unwrap();
    fs::set_permissions(&lock, fs::Permissions::from_mode(0o600)).unwrap();
    assert_eq!(
        journal.record(&intent(2), &source()),
        Err(Error::Unavailable)
    );
    fs::remove_file(lock).unwrap();
    fs::rename(held, path.join("import.lock")).unwrap();
    let original = temporary.path().join("original");
    fs::rename(&path, &original).unwrap();
    fs::create_dir(&path).unwrap();
    fs::set_permissions(&path, fs::Permissions::from_mode(0o700)).unwrap();
    for entry in fs::read_dir(&original).unwrap() {
        let entry = entry.unwrap();
        fs::copy(entry.path(), path.join(entry.file_name())).unwrap();
    }
    assert_eq!(journal.resolve(&command), Err(Error::Unavailable));
    fs::remove_dir_all(&path).unwrap();
    fs::rename(original, &path).unwrap();
    fs::write(path.join("unknown"), []).unwrap();
    fs::set_permissions(path.join("unknown"), fs::Permissions::from_mode(0o600)).unwrap();
    assert_eq!(journal.entries(), Err(Error::Unavailable));
    fs::remove_file(path.join("unknown")).unwrap();
    assert!(journal.resolve(&command).unwrap().is_some());
    assert_eq!(fs::read(path.join("state.json")).unwrap(), bytes);
}

#[test]
fn import_contends_across_handles_and_preserves_one_unselected_candidate() {
    let temporary = tempfile::tempdir().unwrap();
    let path = temporary.path().join("imports");
    let first = CaptureImportJournal::create(&path, INSTALLATION, GENERATION).unwrap();
    let second = CaptureImportJournal::open(&path, INSTALLATION, GENERATION).unwrap();
    let lock = fs::OpenOptions::new()
        .read(true)
        .write(true)
        .open(path.join("import.lock"))
        .unwrap();
    lock.try_lock().unwrap();
    assert_eq!(first.record(&intent(1), &source()), Err(Error::Unavailable));
    assert_eq!(second.entries(), Err(Error::Unavailable));
    lock.unlock().unwrap();
    let candidate = path.join(".capture-import-unused");
    fs::write(&candidate, b"unselected").unwrap();
    fs::set_permissions(&candidate, fs::Permissions::from_mode(0o600)).unwrap();
    assert!(first.entries().unwrap().is_empty());
    let extra = path.join(".capture-import-extra");
    fs::write(&extra, []).unwrap();
    fs::set_permissions(&extra, fs::Permissions::from_mode(0o600)).unwrap();
    assert_eq!(first.record(&intent(1), &source()), Err(Error::Unavailable));
    assert!(candidate.exists());
    fs::remove_file(extra).unwrap();
    first.record(&intent(1), &source()).unwrap();
    assert!(!candidate.exists());
    assert!(second.record(&intent(1), &source()).unwrap().replayed);
    assert_eq!(first.entries().unwrap().len(), 1);
}

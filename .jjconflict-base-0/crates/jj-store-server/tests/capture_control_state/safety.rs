use super::*;
use std::{
    fs,
    os::unix::fs::{PermissionsExt as _, symlink},
};

#[test]
fn ordinary_open_never_recreates_missing_or_repairs_corrupt_foreign_state() {
    let temporary = tempfile::tempdir().unwrap();
    let path = temporary.path().join("controls");
    assert!(matches!(
        CaptureControlJournal::open(&path, INSTALLATION),
        Err(Error::Unavailable)
    ));
    assert!(!path.exists());
    let journal = CaptureControlJournal::create(&path, INSTALLATION).unwrap();
    let bytes = fs::read(path.join("state.json")).unwrap();
    assert!(matches!(
        CaptureControlJournal::open(&path, [0xcd; 16]),
        Err(Error::Unavailable)
    ));
    assert_eq!(fs::read(path.join("state.json")).unwrap(), bytes);
    fs::remove_file(path.join("state.json")).unwrap();
    assert_eq!(journal.current(), Err(Error::Unavailable));
    assert!(matches!(
        CaptureControlJournal::open(&path, INSTALLATION),
        Err(Error::Unavailable)
    ));
    assert!(!path.join("state.json").exists());
    for corruption in [
        b"{}".to_vec(),
        b"not json".to_vec(),
        vec![b' '; 1024 * 1024 + 1],
    ] {
        fs::write(path.join("state.json"), &corruption).unwrap();
        fs::set_permissions(path.join("state.json"), fs::Permissions::from_mode(0o600)).unwrap();
        assert_eq!(journal.current(), Err(Error::Unavailable));
        assert_eq!(fs::read(path.join("state.json")).unwrap(), corruption);
    }
    fs::write(path.join("state.json"), &bytes).unwrap();
    assert!(journal.current().is_ok());
    assert!(matches!(
        CaptureControlJournal::create(&path, INSTALLATION),
        Err(Error::Unavailable)
    ));
    assert_eq!(fs::read(path.join("state.json")).unwrap(), bytes);
}

#[test]
fn an_open_handle_refuses_a_replaced_lock_or_directory() {
    let temporary = tempfile::tempdir().unwrap();
    let path = temporary.path().join("controls");
    let journal = CaptureControlJournal::create(&path, INSTALLATION).unwrap();
    let initial = journal.current().unwrap();
    let bytes = fs::read(path.join("state.json")).unwrap();
    let held = temporary.path().join("original-lock");
    fs::rename(path.join("control.lock"), &held).unwrap();
    fs::write(path.join("control.lock"), []).unwrap();
    fs::set_permissions(path.join("control.lock"), fs::Permissions::from_mode(0o600)).unwrap();
    assert_eq!(journal.current(), Err(Error::Unavailable));
    assert_eq!(
        journal.record(&change(initial.generation, 1, 0, true)),
        Err(Error::Unavailable)
    );
    assert_eq!(fs::read(path.join("state.json")).unwrap(), bytes);
    fs::remove_file(path.join("control.lock")).unwrap();
    fs::rename(&held, path.join("control.lock")).unwrap();
    assert_eq!(journal.current().unwrap(), initial);
    let original = temporary.path().join("original-directory");
    fs::rename(&path, &original).unwrap();
    let replacement = CaptureControlJournal::create(&path, INSTALLATION).unwrap();
    assert_ne!(
        replacement.current().unwrap().generation,
        initial.generation
    );
    assert_eq!(journal.current(), Err(Error::Unavailable));
    assert_eq!(
        replacement.record(&change(initial.generation, 1, 0, true)),
        Err(Error::ForeignGeneration)
    );
    fs::remove_dir_all(&path).unwrap();
    fs::rename(original, path).unwrap();
    assert_eq!(journal.current().unwrap(), initial);
}

#[test]
fn unsafe_permissions_symlinks_and_unknown_files_are_refused_without_repair() {
    let temporary = tempfile::tempdir().unwrap();
    let path = temporary.path().join("controls");
    let journal = CaptureControlJournal::create(&path, INSTALLATION).unwrap();
    let initial = journal.current().unwrap();
    let bytes = fs::read(path.join("state.json")).unwrap();
    for target in [&path, &path.join("state.json"), &path.join("control.lock")] {
        let mode = fs::metadata(target).unwrap().permissions().mode();
        fs::set_permissions(target, fs::Permissions::from_mode(mode | 0o044)).unwrap();
        assert_eq!(journal.current(), Err(Error::Unavailable));
        assert_eq!(
            fs::metadata(target).unwrap().permissions().mode() & 0o044,
            0o044
        );
        fs::set_permissions(target, fs::Permissions::from_mode(mode)).unwrap();
        assert_eq!(journal.current().unwrap(), initial);
    }
    for name in ["state.json", "control.lock"] {
        let target = path.join(name);
        let held = temporary.path().join(name);
        fs::rename(&target, &held).unwrap();
        symlink(&held, &target).unwrap();
        assert_eq!(journal.current(), Err(Error::Unavailable));
        assert_eq!(
            journal.record(&change(initial.generation, 1, 0, true)),
            Err(Error::Unavailable)
        );
        fs::remove_file(&target).unwrap();
        fs::rename(held, target).unwrap();
    }
    let held = temporary.path().join("original");
    fs::rename(&path, &held).unwrap();
    symlink(&held, &path).unwrap();
    assert_eq!(journal.current(), Err(Error::Unavailable));
    assert!(matches!(
        CaptureControlJournal::open(&path, INSTALLATION),
        Err(Error::Unavailable)
    ));
    fs::remove_file(&path).unwrap();
    fs::rename(held, &path).unwrap();
    fs::write(path.join("unknown"), []).unwrap();
    fs::set_permissions(path.join("unknown"), fs::Permissions::from_mode(0o600)).unwrap();
    assert_eq!(journal.current(), Err(Error::Unavailable));
    fs::remove_file(path.join("unknown")).unwrap();
    assert_eq!(journal.current().unwrap(), initial);
    assert_eq!(fs::read(path.join("state.json")).unwrap(), bytes);
}

#[test]
fn independent_handles_contend_and_never_merge_stale_changes() {
    let temporary = tempfile::tempdir().unwrap();
    let path = temporary.path().join("controls");
    let first = CaptureControlJournal::create(&path, INSTALLATION).unwrap();
    let second = CaptureControlJournal::open(&path, INSTALLATION).unwrap();
    let initial = first.current().unwrap();
    let lock = fs::OpenOptions::new()
        .read(true)
        .write(true)
        .open(path.join("control.lock"))
        .unwrap();
    lock.try_lock().unwrap();
    assert_eq!(first.current(), Err(Error::Unavailable));
    assert_eq!(
        second.record(&change(initial.generation, 1, 0, true)),
        Err(Error::Unavailable)
    );
    lock.unlock().unwrap();
    let a = change(initial.generation, 1, 0, true);
    let b = change(initial.generation, 2, 0, false);
    std::thread::scope(|scope| {
        let one = scope.spawn(|| first.record(&a));
        let two = scope.spawn(|| second.record(&b));
        let outcomes = [one.join().unwrap(), two.join().unwrap()];
        assert_eq!(outcomes.iter().filter(|r| r.is_ok()).count(), 1);
        assert!(
            outcomes
                .iter()
                .filter_map(|r| r.as_ref().err())
                .all(|e| matches!(e, Error::Unavailable | Error::StaleRevision))
        );
    });
    assert_eq!(first.current().unwrap(), second.current().unwrap());
    assert_eq!(first.current().unwrap().revision, 1);
}

#[test]
fn unpublished_candidate_is_not_selected_and_only_one_can_be_cleaned() {
    let temporary = tempfile::tempdir().unwrap();
    let path = temporary.path().join("controls");
    let journal = CaptureControlJournal::create(&path, INSTALLATION).unwrap();
    let initial = journal.current().unwrap();
    let candidate = path.join(".capture-control-unused");
    fs::write(&candidate, b"unpublished bytes").unwrap();
    fs::set_permissions(&candidate, fs::Permissions::from_mode(0o600)).unwrap();
    assert_eq!(journal.current().unwrap(), initial);
    let extra = path.join(".capture-control-extra");
    fs::write(&extra, []).unwrap();
    fs::set_permissions(&extra, fs::Permissions::from_mode(0o600)).unwrap();
    assert_eq!(
        journal.record(&change(initial.generation, 1, 0, true)),
        Err(Error::Unavailable)
    );
    assert!(candidate.exists() && extra.exists());
    fs::remove_file(extra).unwrap();
    assert!(
        journal
            .record(&change(initial.generation, 1, 0, true))
            .unwrap()
            .current
            .paused
    );
    assert!(!candidate.exists());
}

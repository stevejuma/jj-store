//! Real filesystem publication/GC proofs; no native RustFS claims.
use gix::objs::Write as _;
use jj_store_jj::local_metadata::*;
use jj_store_model as m;
use std::{collections::BTreeMap, fs, path::Path, process::Command, time::Duration};

const REPOSITORY: m::RepositoryId = m::RepositoryId::new([41; 16]);
const ACTOR: m::PrincipalId = m::PrincipalId::new([42; 16]);
fn fixture() -> (tempfile::TempDir, MetadataStore) {
    let root = tempfile::tempdir().unwrap();
    gix::init_bare(root.path()).unwrap();
    let store = MetadataStore::open(root.path(), Limits::default()).unwrap();
    (root, store)
}
fn update(id: u8, domain: Domain, expected: Option<Selection>) -> Update {
    Update {
        repository: REPOSITORY,
        actor: ACTOR,
        request: m::RequestId::new([id; 16]),
        intent: m::IntentDigest::new([id; 32]),
        domain,
        expected,
        bytes: format!("{{\"fixture\":{id}}}").into_bytes(),
        result: vec![id],
        retained: BTreeMap::new(),
    }
}
fn candidate(store: &MetadataStore, base: Option<&Snapshot>, update: &Update) -> Candidate {
    match store.prepare(base, update).unwrap() {
        Preparation::Ready(candidate) => *candidate,
        Preparation::Committed(_) => panic!("expected a new candidate"),
    }
}
fn publish(store: &MetadataStore, base: Option<&Snapshot>, update: &Update) -> Receipt {
    match store.publish(candidate(store, base, update)).unwrap() {
        PublishOutcome::Committed(receipt) => receipt,
        other => panic!("expected publication, got {other:?}"),
    }
}
fn git(path: &Path, args: &[&str]) {
    let output = Command::new("git")
        .arg("--git-dir")
        .arg(path)
        .args(args)
        .env("GIT_CONFIG_NOSYSTEM", "1")
        .env("GIT_CONFIG_GLOBAL", "/dev/null")
        .output()
        .unwrap();
    assert!(
        output.status.success(),
        "{}",
        String::from_utf8_lossy(&output.stderr)
    );
}

#[test]
fn reads_and_staged_candidates_never_initialize_or_publish() {
    let (_root, store) = fixture();
    assert!(store.read().unwrap().is_none());
    let staged = candidate(&store, None, &update(1, Domain::Work, None));
    assert!(store.read().unwrap().is_none());
    assert_eq!(store.read_at(staged.commit()).unwrap().sequence(), 1);
    let receipt = store.publish(staged).unwrap();
    assert!(matches!(receipt, PublishOutcome::Committed(_)));
    let snapshot = store.read().unwrap().unwrap();
    assert_eq!(snapshot.repository(), REPOSITORY);
    assert_eq!(
        store.read_domain(&snapshot, Domain::Work).unwrap(),
        Some(update(1, Domain::Work, None).bytes)
    );
}

#[test]
fn racing_initializers_adopt_winning_identity_and_do_not_replace_siblings() {
    let (_root, store) = fixture();
    let first = candidate(&store, None, &update(2, Domain::Work, None));
    let mut rival = update(3, Domain::Review, None);
    rival.repository = m::RepositoryId::new([99; 16]);
    let second = candidate(&store, None, &rival);
    assert!(matches!(
        store.publish(first).unwrap(),
        PublishOutcome::Committed(_)
    ));
    assert_eq!(store.publish(second).unwrap(), PublishOutcome::Conflict);
    let selected = store.read().unwrap().unwrap();
    assert_eq!(selected.repository(), REPOSITORY);
    assert!(matches!(
        store.prepare(Some(&selected), &rival),
        Err(Error::Invalid(_))
    ));
    rival.repository = selected.repository();
    publish(&store, Some(&selected), &rival);
    let latest = store.read().unwrap().unwrap();
    assert_eq!(
        latest.selection(Domain::Work),
        selected.selection(Domain::Work)
    );
    assert!(latest.selection(Domain::Review).is_some());
}

#[test]
fn receipts_survive_lost_response_restart_and_later_domain_publication() {
    let (root, store) = fixture();
    let original = update(4, Domain::Work, None);
    let receipt = publish(&store, None, &original);
    let first = store.read().unwrap().unwrap();
    let left = candidate(&store, Some(&first), &update(5, Domain::Agent, None));
    let right = candidate(&store, Some(&first), &update(6, Domain::Review, None));
    assert!(matches!(
        store.publish(left).unwrap(),
        PublishOutcome::Committed(_)
    ));
    assert_eq!(store.publish(right).unwrap(), PublishOutcome::Conflict);
    let restarted = MetadataStore::open(root.path(), Limits::default()).unwrap();
    let selected = restarted.read().unwrap().unwrap();
    match restarted.prepare(Some(&selected), &original).unwrap() {
        Preparation::Committed(replayed) => assert_eq!(replayed, receipt),
        _ => panic!("lost response must replay the original receipt"),
    }
    let mut changed = update(4, Domain::Work, None);
    changed.intent = m::IntentDigest::new([88; 32]);
    assert!(matches!(
        restarted.prepare(Some(&selected), &changed),
        Err(Error::IntentConflict)
    ));
    assert!(matches!(
        restarted.prepare(Some(&selected), &update(7, Domain::Work, None)),
        Err(Error::Stale)
    ));
}

#[test]
fn stock_git_gc_retains_history_code_and_nonstandard_dependencies() {
    let (root, store) = fixture();
    let repo = gix::open(root.path()).unwrap();
    let file = repo.write_blob(b"immutable evidence\n").unwrap().detach();
    let extra = repo
        .write_blob(b"JJ operation/view/extra evidence\n")
        .unwrap()
        .detach();
    let tree = repo
        .write_object(gix::objs::Tree {
            entries: vec![gix::objs::tree::Entry {
                mode: gix::objs::tree::EntryKind::Blob.into(),
                filename: "file".into(),
                oid: file,
            }],
        })
        .unwrap()
        .detach();
    let code = repo.objects.write_buf(gix::objs::Kind::Commit, format!("tree {tree}\nauthor A <a@example.invalid> 1 +0000\ncommitter A <a@example.invalid> 1 +0000\n\ncode\n").as_bytes()).unwrap();
    let mut first = update(8, Domain::Work, None);
    first
        .retained
        .insert(GitOid::from_git(code).unwrap(), RetainedKind::Commit);
    first
        .retained
        .insert(GitOid::from_git(extra).unwrap(), RetainedKind::Blob);
    first
        .retained
        .insert(GitOid::from_git(tree).unwrap(), RetainedKind::Tree);
    let receipt = publish(&store, None, &first);
    let old = store.read().unwrap().unwrap();
    publish(
        &store,
        Some(&old),
        &update(9, Domain::Work, old.selection(Domain::Work).cloned()),
    );
    drop(repo);
    git(root.path(), &["gc", "--prune=now"]);
    git(root.path(), &["fsck", "--full", "--no-reflogs"]);
    let restarted = MetadataStore::open(root.path(), Limits::default()).unwrap();
    let latest = restarted.read().unwrap().unwrap();
    let history = restarted.history(&latest).unwrap();
    assert_eq!(history.len(), 2);
    assert_eq!(history[1].commit(), old.commit());
    assert_eq!(history[1].receipt(ACTOR, first.request), Some(&receipt));
    assert_eq!(
        restarted.read_domain(&history[1], Domain::Work).unwrap(),
        Some(first.bytes)
    );
    let repo = gix::open(root.path()).unwrap();
    for id in [file, extra, tree, code] {
        assert!(repo.find_object(id).is_ok());
    }
    assert!(
        repo.try_find_reference("refs/heads/lapis")
            .unwrap()
            .is_none()
    );
}

#[test]
fn lock_failures_remain_unknown_and_retry_uses_the_exact_candidate() {
    let (root, store) = fixture();
    let staged = candidate(&store, None, &update(10, Domain::Work, None));
    let id = staged.commit();
    let lock = root.path().join(format!("{STATE_REF}.lock"));
    fs::create_dir_all(lock.parent().unwrap()).unwrap();
    fs::write(&lock, b"held by another writer").unwrap();
    assert_eq!(store.publish(staged).unwrap(), PublishOutcome::Unknown);
    assert!(store.read().unwrap().is_none());
    fs::remove_file(lock).unwrap();
    let staged = candidate(&store, None, &update(10, Domain::Work, None));
    assert_eq!(staged.commit(), id);
    assert!(matches!(
        store.publish(staged).unwrap(),
        PublishOutcome::Committed(_)
    ));
}

#[test]
fn admission_and_missing_selected_objects_fail_without_partial_reads() {
    let (root, store) = fixture();
    let mut oversized = update(11, Domain::Work, None);
    oversized.result = vec![0; 1024 * 1024 + 1];
    assert!(matches!(store.prepare(None, &oversized), Err(Error::Limit)));
    assert!(store.read().unwrap().is_none());
    publish(&store, None, &update(12, Domain::Work, None));
    let selected = store.read().unwrap().unwrap();
    let hex = selected.selection(Domain::Work).unwrap().object.to_string();
    fs::remove_file(root.path().join("objects").join(&hex[..2]).join(&hex[2..])).unwrap();
    assert!(store.read().is_err());
}

#[test]
fn crash_child_helper() {
    let Some(path) = std::env::var_os("JJ_STORE_METADATA_CRASH_DIR") else {
        return;
    };
    let store = MetadataStore::open(Path::new(&path), Limits::default()).unwrap();
    let staged = candidate(&store, None, &update(13, Domain::Work, None));
    if std::env::var_os("JJ_STORE_METADATA_CRASH_PUBLISHED").is_some() {
        assert!(matches!(
            store.publish(staged).unwrap(),
            PublishOutcome::Committed(_)
        ));
    }
    fs::write(Path::new(&path).join("boundary-ready"), b"ready").unwrap();
    loop {
        std::thread::park();
    }
}

#[test]
fn hard_process_exit_before_and_after_ref_publication_recovers_correctly() {
    for published in [false, true] {
        let (root, store) = fixture();
        let mut command = Command::new(std::env::current_exe().unwrap());
        command
            .args(["--exact", "crash_child_helper", "--nocapture"])
            .env("JJ_STORE_METADATA_CRASH_DIR", root.path());
        if published {
            command.env("JJ_STORE_METADATA_CRASH_PUBLISHED", "1");
        }
        let mut child = command.spawn().unwrap();
        let ready = root.path().join("boundary-ready");
        for _ in 0..100 {
            if ready.exists() {
                break;
            }
            if let Some(status) = child.try_wait().unwrap() {
                panic!("child exited before boundary: {status}");
            }
            std::thread::sleep(Duration::from_millis(20));
        }
        assert!(ready.exists(), "child did not reach crash boundary");
        child.kill().unwrap();
        child.wait().unwrap();
        assert_eq!(store.read().unwrap().is_some(), published);
        let selected = store.read().unwrap();
        if published {
            assert!(matches!(
                store
                    .prepare(selected.as_ref(), &update(13, Domain::Work, None))
                    .unwrap(),
                Preparation::Committed(_)
            ));
        } else {
            publish(&store, None, &update(13, Domain::Work, None));
        }
    }
}

#[test]
fn jj_conflict_header_terms_get_real_git_reachability() {
    let (root, store) = fixture();
    let repo = gix::open(root.path()).unwrap();
    let mut trees = Vec::new();
    for body in [b"left".as_slice(), b"base", b"right"] {
        let blob = repo.write_blob(body).unwrap().detach();
        trees.push(
            repo.write_object(gix::objs::Tree {
                entries: vec![gix::objs::tree::Entry {
                    mode: gix::objs::tree::EntryKind::Blob.into(),
                    filename: "file".into(),
                    oid: blob,
                }],
            })
            .unwrap()
            .detach(),
        );
    }
    let empty = repo
        .write_object(gix::objs::Tree::empty())
        .unwrap()
        .detach();
    let bytes = format!(
        "tree {empty}\nauthor A <a@example.invalid> 1 +0000\ncommitter A <a@example.invalid> 1 +0000\njj:trees {} {} {}\n\nconflict\n",
        trees[0], trees[1], trees[2]
    );
    let code = repo
        .objects
        .write_buf(gix::objs::Kind::Commit, bytes.as_bytes())
        .unwrap();
    let mut first = update(14, Domain::Review, None);
    first
        .retained
        .insert(GitOid::from_git(code).unwrap(), RetainedKind::Commit);
    publish(&store, None, &first);
    drop(repo);
    git(root.path(), &["gc", "--prune=now"]);
    let selected = store.read().unwrap().unwrap();
    assert_eq!(
        store.read_domain(&selected, Domain::Review).unwrap(),
        Some(first.bytes)
    );
    let repo = gix::open(root.path()).unwrap();
    for tree in trees {
        assert!(repo.find_object(tree).is_ok());
    }
}

#[test]
fn broken_nested_dependencies_and_budget_exhaustion_never_publish() {
    let (_root, store) = fixture();
    let repo = gix::open(store.git_dir()).unwrap();
    let missing = gix::ObjectId::from_hex(b"1111111111111111111111111111111111111111").unwrap();
    let tree = repo
        .write_object(gix::objs::Tree {
            entries: vec![gix::objs::tree::Entry {
                mode: gix::objs::tree::EntryKind::Blob.into(),
                filename: "missing".into(),
                oid: missing,
            }],
        })
        .unwrap()
        .detach();
    let mut change = update(15, Domain::Work, None);
    change
        .retained
        .insert(GitOid::from_git(tree).unwrap(), RetainedKind::Tree);
    assert!(store.prepare(None, &change).is_err());
    assert!(store.read().unwrap().is_none());
    let blob = repo.write_blob(b"retained evidence").unwrap().detach();
    change.retained.clear();
    change
        .retained
        .insert(GitOid::from_git(blob).unwrap(), RetainedKind::Blob);
    let small = MetadataStore::open(
        store.git_dir(),
        Limits {
            closure_bytes: 1,
            ..Limits::default()
        },
    )
    .unwrap();
    assert!(matches!(small.prepare(None, &change), Err(Error::Limit)));
    assert!(store.read().unwrap().is_none());
}

#[test]
fn stock_jj_import_gc_and_undo_preserve_companion_metadata() {
    let root = tempfile::tempdir().unwrap();
    let checkout = root.path().join("repo");
    let jj = |args: &[&str]| {
        let output = Command::new("/opt/homebrew/bin/jj")
            .args([
                "--no-pager",
                "--config",
                "user.name='Metadata Test'",
                "--config",
                "user.email='metadata@example.invalid'",
            ])
            .args(args)
            .env("JJ_CONFIG", "/dev/null")
            .env("GIT_CONFIG_NOSYSTEM", "1")
            .env("GIT_CONFIG_GLOBAL", "/dev/null")
            .current_dir(root.path())
            .output()
            .unwrap();
        assert!(
            output.status.success(),
            "{}",
            String::from_utf8_lossy(&output.stderr)
        );
        String::from_utf8(output.stdout).unwrap()
    };
    jj(&["git", "init", "--colocate", checkout.to_str().unwrap()]);
    let folder = checkout.to_str().unwrap();
    let before = jj(&[
        "-R",
        folder,
        "log",
        "--no-graph",
        "-r",
        "all()",
        "-T",
        "commit_id ++ '\\n'",
    ]);
    let store = MetadataStore::open(&checkout.join(".git"), Limits::default()).unwrap();
    let receipt = publish(&store, None, &update(16, Domain::Work, None));
    let selected = store.read().unwrap().unwrap();
    jj(&["-R", folder, "git", "import"]);
    assert_eq!(
        jj(&[
            "-R",
            folder,
            "log",
            "--no-graph",
            "-r",
            "all()",
            "-T",
            "commit_id ++ '\\n'"
        ]),
        before
    );
    jj(&["-R", folder, "describe", "-m", "ordinary code description"]);
    jj(&["-R", folder, "undo"]);
    jj(&["-R", folder, "util", "gc", "--expire", "now"]);
    git(&checkout.join(".git"), &["gc", "--prune=now"]);
    let after = store.read().unwrap().unwrap();
    assert_eq!(after.commit(), selected.commit());
    assert_eq!(
        after.receipt(ACTOR, m::RequestId::new([16; 16])),
        Some(&receipt)
    );
}

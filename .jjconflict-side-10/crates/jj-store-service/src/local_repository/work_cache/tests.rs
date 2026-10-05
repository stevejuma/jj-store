use super::*;
use crate::work::*;
use jj_store_jj::local_metadata::{Limits, MetadataStore};
use jj_store_model as m;
use std::{fs, process::Command};

fn fixture() -> (tempfile::TempDir, LocalRepository) {
    let root = tempfile::tempdir().unwrap();
    let output = Command::new("/opt/homebrew/bin/jj")
        .args(["--no-pager", "git", "init", "--colocate"])
        .arg(root.path().join("repo"))
        .env("JJ_CONFIG", "/dev/null")
        .env("GIT_CONFIG_NOSYSTEM", "1")
        .env("GIT_CONFIG_GLOBAL", "/dev/null")
        .output()
        .unwrap();
    assert!(
        output.status.success(),
        "{}",
        String::from_utf8_lossy(&output.stderr)
    );
    let path = root.path().join("repo");
    let store = MetadataStore::open(&path.join(".git"), Limits::default()).unwrap();
    let repository = LocalRepository::new(&path.join(".jj/repo"), store);
    repository
        .apply_work_command(&Request {
            version: 1,
            request_id: m::RequestId::new([1; 16]),
            command: WorkCommandV1::CreateProject {
                id: ProjectId::new([1; 16]),
                title: "Project".into(),
                body: "Body".into(),
            },
        })
        .unwrap();
    repository
        .apply_work_command(&Request {
            version: 1,
            request_id: m::RequestId::new([2; 16]),
            command: WorkCommandV1::CreateTask {
                id: TaskId::new([2; 16]),
                project: ProjectId::new([1; 16]),
                expected_project_version: 1,
                title: "Task".into(),
                body: "Needle".into(),
                repository: None,
                parent: None,
            },
        })
        .unwrap();
    (root, repository)
}

#[test]
fn local_work_first_pages_share_data_across_repository_handles() {
    let (_root, repository) = fixture();
    let selected = repository.metadata.read().unwrap().unwrap();
    let first = repository.work_data(Some(&selected)).unwrap();
    let reopened = LocalRepository::new(
        &repository.repo_path,
        MetadataStore::open(repository.metadata.git_dir(), Limits::default()).unwrap(),
    );
    let admitted = reopened.selected_metadata(selected.commit()).unwrap();
    let second = reopened.work_data(Some(&admitted)).unwrap();
    assert_eq!(first.state, second.state);
    assert_eq!(first.projection.activity[0].tasks, 1);
    assert!(
        Arc::ptr_eq(&first, &second),
        "unchanged root was parsed/projected again after reopening"
    );
    let query = WorkBrowseQuery {
        kind: WorkBrowseKind::Tasks,
        q: "needle".into(),
        limit: 1,
        ..Default::default()
    };
    let page = repository
        .work_browse_at(selected.commit(), &query)
        .unwrap();
    assert_eq!(page.tasks[0].id, TaskId::new([2; 16]));
    assert_eq!(
        reopened
            .work_browse_at(selected.commit(), &query)
            .unwrap()
            .snapshot,
        page.snapshot
    );
}

fn admitted(repository: &LocalRepository) -> (Snapshot, Vec<u8>, Key) {
    let selected = repository.metadata.read().unwrap().unwrap();
    let bytes = repository
        .metadata
        .read_domain(&selected, Domain::Work)
        .unwrap()
        .unwrap();
    let key = Key {
        repository: Some(selected.repository()),
        object: Some(selected.selection(Domain::Work).unwrap().object),
        bytes: Sha256::digest(&bytes).into(),
    };
    (selected, bytes, key)
}

#[test]
fn local_work_query_keys_are_reused_and_evicted_without_rehydrating_other_rows() {
    let (_root, repository) = fixture();
    let (_, bytes, key) = admitted(&repository);
    let cache = Cache::default();
    let data = cache.load(key, &bytes).unwrap();
    let query = WorkBrowseQuery {
        kind: WorkBrowseKind::Tasks,
        limit: 1,
        ..Default::default()
    };
    let first = data.select(&query).unwrap();
    let resumed = data
        .select(&WorkBrowseQuery {
            cursor: Some("continued".into()),
            ..query.clone()
        })
        .unwrap();
    assert!(Arc::ptr_eq(&first, &resumed));
    let rows = browse_selected(&data.state, &data.projection, &first.keys, 1, 0).unwrap();
    assert_eq!(rows.hydrated, 1);
    assert_eq!(rows.total_matches, 1);
    let deep = browse_selected(&data.state, &data.projection, &first.keys, 1, 1).unwrap();
    assert_eq!(deep.hydrated, 0);
    for n in 0..8 {
        data.select(&WorkBrowseQuery {
            q: format!("other {n}"),
            ..query.clone()
        })
        .unwrap();
    }
    let evicted_used = cache.budget.used();
    let rebuilt = data.select(&query).unwrap();
    assert!(!Arc::ptr_eq(&first, &rebuilt));
    assert_eq!(rebuilt.keys.len(), first.keys.len());
    drop(first);
    drop(resumed);
    assert!(cache.budget.used() < evicted_used);
    assert!(matches!(
        data.select(&WorkBrowseQuery {
            project: Some(ProjectId::new([9; 16])),
            ..query
        }),
        Err(Error::NotFound)
    ));
}

#[test]
fn local_work_eviction_expiry_and_request_local_fallback_preserve_live_charges() {
    let (_root, repository) = fixture();
    let (_, bytes, key) = admitted(&repository);
    let mut cache = Cache::default();
    let first = cache.load(key, &bytes).unwrap();
    let selected = first.select(&WorkBrowseQuery::default()).unwrap();
    assert!(cache.lookup(key, Instant::now() + cache.ttl).is_none());
    let expired_charge = cache.budget.used();
    assert!(expired_charge > 0);
    let rebuilt = cache.load(key, &bytes).unwrap();
    assert!(!Arc::ptr_eq(&first, &rebuilt));
    cache.entries.lock().unwrap().clear();
    drop(first);
    drop(rebuilt);
    assert!(
        cache.budget.used() > 0,
        "evicted selection lost its live charge"
    );
    drop(selected);
    assert_eq!(cache.budget.used(), 0);
    cache.retained = 1;
    let oversized = cache.load(key, &bytes).unwrap();
    assert!(cache.entries.lock().unwrap().is_empty());
    assert_eq!(oversized.state.tasks.len(), 1);
    drop(oversized);
    assert_eq!(cache.budget.used(), 0);
}

#[test]
fn local_work_live_admission_rejects_growth_and_releases_failed_builds() {
    let (_root, repository) = fixture();
    let (_, bytes, key) = admitted(&repository);
    let allowance = memory::decoding(bytes.len()).unwrap();
    let cache = Cache {
        budget: Arc::new(Budget::new(allowance)),
        retained: 0,
        ..Default::default()
    };
    let occupied = cache.budget.reserve(allowance).unwrap();
    assert!(matches!(cache.load(key, &bytes), Err(Error::Busy)));
    drop(occupied);
    let data = cache.load(key, &bytes).unwrap();
    drop(data);
    assert_eq!(cache.budget.used(), 0);
    let bad_key = Key {
        bytes: Sha256::digest(b"broken").into(),
        ..key
    };
    assert!(matches!(
        cache.load(bad_key, b"broken"),
        Err(Error::Corrupt)
    ));
    assert_eq!(cache.budget.used(), 0);
    let too_small = Cache {
        budget: Arc::new(Budget::new(1)),
        ..Default::default()
    };
    assert!(matches!(too_small.load(key, &bytes), Err(Error::Limit)));
    assert_eq!(too_small.budget.used(), 0);
    let mut lease = cache.budget.reserve(10).unwrap();
    lease.resize(20).unwrap();
    assert_eq!(cache.budget.used(), 20);
    let occupied = cache.budget.reserve(allowance - 20).unwrap();
    assert_eq!(lease.resize(21), Err(Error::Busy));
    assert_eq!(cache.budget.used(), allowance);
    drop(occupied);
    drop(lease);
    assert_eq!(cache.budget.used(), 0);
}

#[test]
fn local_work_warm_hits_do_not_wait_for_cold_builds_and_duplicate_builds_share_data() {
    let (_root, repository) = fixture();
    let (_, bytes, key) = admitted(&repository);
    let cache = Arc::new(Cache::default());
    let warm = cache.load(key, &bytes).unwrap();
    let builder = cache.build.lock().unwrap();
    let warm_cache = cache.clone();
    let (sender, receiver) = std::sync::mpsc::channel();
    let worker = std::thread::spawn(move || {
        sender
            .send(warm_cache.lookup(key, Instant::now()).unwrap())
            .unwrap()
    });
    let hit = receiver.recv_timeout(Duration::from_secs(2));
    drop(builder);
    worker.join().unwrap();
    assert!(Arc::ptr_eq(&warm, &hit.unwrap()));
    cache.entries.lock().unwrap().clear();
    let barrier = Arc::new(std::sync::Barrier::new(2));
    let handles: Vec<_> = (0..2)
        .map(|_| {
            let (cache, bytes, barrier) = (cache.clone(), bytes.clone(), barrier.clone());
            std::thread::spawn(move || {
                barrier.wait();
                cache.load(key, &bytes).unwrap()
            })
        })
        .collect();
    let mut results = handles.into_iter().map(|handle| handle.join().unwrap());
    assert!(Arc::ptr_eq(
        &results.next().unwrap(),
        &results.next().unwrap()
    ));
}

#[test]
fn local_work_warm_cache_cannot_hide_missing_objects_or_tighter_source_limits() {
    let (_root, repository) = fixture();
    let (selected, _, _) = admitted(&repository);
    let warm = repository.work_data(Some(&selected)).unwrap();
    let object = selected.selection(Domain::Work).unwrap().object.to_string();
    let path = repository
        .metadata
        .git_dir()
        .join("objects")
        .join(&object[..2])
        .join(&object[2..]);
    let loose = fs::read(&path).unwrap();
    fs::remove_file(&path).unwrap();
    let missing = repository.work_browse_at(selected.commit(), &WorkBrowseQuery::default());
    // Restore before assertions/fixture cleanup, regardless of the result.
    fs::write(&path, loose).unwrap();
    assert!(missing.is_err());
    let restricted = LocalRepository::new(
        &repository.repo_path,
        MetadataStore::open(
            repository.metadata.git_dir(),
            Limits {
                object_bytes: 1,
                ..Default::default()
            },
        )
        .unwrap(),
    );
    assert!(restricted.metadata.identity().unwrap().is_some());
    assert!(matches!(
        restricted.work_data(Some(&selected)),
        Err(Error::Limit)
    ));
    assert!(
        restricted
            .work_browse_at(selected.commit(), &WorkBrowseQuery::default())
            .is_err()
    );
    assert!(Arc::ptr_eq(
        &warm,
        &repository.work_data(Some(&selected)).unwrap()
    ));
}

#[test]
fn local_work_reused_data_keeps_fresh_envelopes_and_changed_roots_distinct() {
    use jj_store_jj::local_metadata::{Preparation, PublishOutcome, Update};
    let (_root, repository) = fixture();
    let (selected, bytes, _) = admitted(&repository);
    let first = repository.work_data(Some(&selected)).unwrap();
    let old_page = repository
        .work_browse_at(selected.commit(), &WorkBrowseQuery::default())
        .unwrap();
    let update = Update {
        repository: selected.repository(),
        actor: repository.actor().unwrap().unwrap(),
        request: m::RequestId::new([3; 16]),
        intent: m::IntentDigest::new([3; 32]),
        domain: Domain::Work,
        expected: selected.selection(Domain::Work).cloned(),
        bytes,
        result: vec![],
        retained: Default::default(),
    };
    let Preparation::Ready(candidate) = repository
        .metadata
        .prepare(Some(&selected), &update)
        .unwrap()
    else {
        panic!("candidate")
    };
    assert!(matches!(
        repository.metadata.publish(*candidate).unwrap(),
        PublishOutcome::Committed(_)
    ));
    let current = repository.metadata.read().unwrap().unwrap();
    assert!(Arc::ptr_eq(
        &first,
        &repository.work_data(Some(&current)).unwrap()
    ));
    let fresh = repository
        .work_browse_at(current.commit(), &WorkBrowseQuery::default())
        .unwrap();
    assert_ne!(
        fresh.snapshot.metadata_commit,
        old_page.snapshot.metadata_commit
    );
    assert!(fresh.snapshot.work_state_version > old_page.snapshot.work_state_version);
    repository
        .apply_work_command(&Request {
            version: 1,
            request_id: m::RequestId::new([4; 16]),
            command: WorkCommandV1::CreateTask {
                id: TaskId::new([4; 16]),
                project: ProjectId::new([1; 16]),
                expected_project_version: 1,
                title: "Next".into(),
                body: "Changed".into(),
                repository: None,
                parent: None,
            },
        })
        .unwrap();
    let changed = repository.metadata.read().unwrap().unwrap();
    let data = repository.work_data(Some(&changed)).unwrap();
    assert!(!Arc::ptr_eq(&first, &data));
    assert_eq!(data.projection.activity[0].tasks, 2);
    let old = repository
        .work_activity_at(selected.commit(), &WorkActivityQuery::default())
        .unwrap();
    assert_eq!(old.groups[0].tasks, 1);
    let detail = repository
        .work_page_at(
            changed.commit(),
            &WorkQuery {
                project: Some(ProjectId::new([1; 16])),
                limit: 1,
                ..Default::default()
            },
        )
        .unwrap();
    assert_eq!(detail.tasks[0].id, TaskId::new([2; 16]));
    let next = repository
        .work_page_at(
            changed.commit(),
            &WorkQuery {
                project: Some(ProjectId::new([1; 16])),
                limit: 1,
                cursor: detail.next_cursor,
                ..Default::default()
            },
        )
        .unwrap();
    assert_eq!(next.tasks[0].id, TaskId::new([4; 16]));
}

#[test]
fn local_work_root_count_and_retention_limits_evict_old_roots() {
    let (_root_a, repository_a) = fixture();
    let (_root_b, repository_b) = fixture();
    let (_, bytes_a, key_a) = admitted(&repository_a);
    let (_, bytes_b, key_b) = admitted(&repository_b);
    let cache = Cache {
        roots: 1,
        ..Default::default()
    };
    let first = cache.load(key_a, &bytes_a).unwrap();
    let second = cache.load(key_b, &bytes_b).unwrap();
    assert_eq!(cache.entries.lock().unwrap().len(), 1);
    assert!(cache.lookup(key_a, Instant::now()).is_none());
    assert!(Arc::ptr_eq(
        &second,
        &cache.lookup(key_b, Instant::now()).unwrap()
    ));
    let charge = cache.budget.used();
    drop(first);
    assert!(cache.budget.used() < charge);
    let retained = Cache {
        retained: second.retention,
        ..Default::default()
    };
    let old = retained.load(key_a, &bytes_a).unwrap();
    let new = retained.load(key_b, &bytes_b).unwrap();
    assert_eq!(retained.entries.lock().unwrap().len(), 1);
    assert!(retained.lookup(key_a, Instant::now()).is_none());
    assert_eq!(old.state.tasks.len(), new.state.tasks.len());
}

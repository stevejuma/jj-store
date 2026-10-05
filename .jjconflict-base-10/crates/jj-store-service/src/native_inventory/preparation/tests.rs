use super::*;
mod cancellation;
use crate::{
    auth::{Permission, SystemClock},
    native::{SessionLimits, create_repository_local_trusted},
};
use jj_store_jj::{codec::NativeFile, objects::NativeObjects, policy::Protection, roots};
use jj_store_model as m;
use jj_store_object::{Condition, Key, ObjectStore as _, StagedUpload, WriteOutcome};
use jj_store_wal::{
    catalog::Catalog,
    inventory::DependencySource as _,
    records::{RecordKind, Records},
};
use sha2::{Digest as _, Sha256};
use std::{
    collections::{BTreeMap, BTreeSet},
    sync::Arc,
    time::Duration,
};
use tokio::time::Instant;

struct Fixture {
    store: Arc<jj_store_object::memory::MemoryStore>,
    objects: Arc<NativeObjects>,
    session: Arc<crate::native::NativeSession>,
    repository: m::RepositoryId,
    operation: m::ObjectKey,
}
async fn fixture(files: u64) -> Fixture {
    let repository = m::RepositoryId::new([74; 16]);
    let store = Arc::new(jj_store_object::memory::MemoryStore::default());
    let objects = Arc::new(NativeObjects::new(store.clone(), repository, 8 * 1024 * 1024).unwrap());
    create_repository_local_trusted(
        store.clone(),
        objects.clone(),
        Permission::Admin,
        m::EpochId::new([1; 16]),
        jj_store_wal::publication::Limits::default(),
    )
    .await
    .unwrap();
    let mut operation = roots::operation();
    operation.metadata.description = "resumable kind construction".into();
    let (operation, _) = objects.put(&operation).await.unwrap();
    let session = Arc::new(
        crate::native::NativeSession::new_local_trusted(
            store.clone(),
            objects.clone(),
            m::PrincipalId::new([1; 16]),
            Permission::Read,
            Arc::new(SystemClock),
            Protection::default(),
            SessionLimits::default(),
        )
        .unwrap(),
    );
    let f = Fixture {
        store,
        objects,
        session,
        repository,
        operation,
    };
    let mut keys = vec![operation];
    for n in 0..files {
        keys.push(
            f.objects
                .put(&NativeFile(n.to_be_bytes().to_vec()))
                .await
                .unwrap()
                .0,
        );
    }
    select(&f, keys).await;
    f
}
// Test-only selection of actual verified bodies; no serving API accepts these
// caller-built descriptors or manifests, and this is not RustFS qualification.
async fn select(f: &Fixture, keys: Vec<m::ObjectKey>) {
    let records = Records::new(f.store.clone(), f.repository);
    let mut entries = BTreeMap::new();
    for key in keys {
        let node = f.objects.load_verified(key, 8 * 1024 * 1024).await.unwrap();
        let reference = records
            .put(RecordKind::Inventory, None, &node)
            .await
            .unwrap();
        let mut hash = Sha256::new_with_prefix(b"jj-store/object/v1");
        hash.update([key.kind() as u8]);
        hash.update(key.as_bytes());
        entries.insert(m::RecordDigest::new(hash.finalize().into()), reference);
    }
    let mut manifest = f.session.snapshot().await.unwrap().manifest().clone();
    manifest.inventory = Catalog::new(records)
        .update(manifest.inventory, entries)
        .await
        .unwrap();
    manifest.revision += 1;
    replace(f, &manifest).await;
}
async fn replace(f: &Fixture, manifest: &jj_store_wal::publication::Manifest) {
    let path = Key::new(format!("repos/{}/manifest.json", f.repository)).unwrap();
    let version = f.store.head(&path).await.unwrap().version;
    let upload = StagedUpload::from_bytes(&serde_json::to_vec(manifest).unwrap(), 128 * 1024)
        .await
        .unwrap();
    assert!(matches!(
        f.store
            .put(&path, Condition::Matches(version), &upload)
            .await
            .unwrap(),
        WriteOutcome::Applied(_)
    ));
}
fn deadline() -> Instant {
    Instant::now() + Duration::from_secs(10)
}

#[tokio::test]
async fn native_kind_preparation_resumes_exact_windows_and_promotes_only_complete_ids() {
    let f = fixture(600).await;
    let expected_scanned = f
        .session
        .snapshot()
        .await
        .unwrap()
        .inventory(f.store.clone(), 1000)
        .await
        .unwrap()
        .len();
    let mut task = f
        .session
        .begin_kind_preparation(false, Limits::default())
        .await
        .unwrap();
    let mut concurrent = f
        .session
        .begin_kind_preparation(false, Limits::default())
        .await
        .unwrap();
    assert!(matches!(
        f.session.finish_kind_preparation(&task, deadline()).await,
        Err(jj_store_jj::query::Error::IndexRebuildRequired)
    ));
    let first = f
        .session
        .advance_kind_preparation(&mut task, deadline())
        .await
        .unwrap();
    assert!(!first.complete);
    assert_eq!(first.stats.scanned, 256);
    assert!(
        !f.session
            .advance_kind_preparation(&mut concurrent, deadline())
            .await
            .unwrap()
            .complete
    );
    let partial = concurrent.stats;
    assert!(matches!(
        f.session.finish_kind_preparation(&task, deadline()).await,
        Err(jj_store_jj::query::Error::IndexRebuildRequired)
    ));
    // Continuation retains the old checked root after a later selection.
    let mut later = roots::operation();
    later.metadata.description = "later selection".into();
    let (later, _) = f.objects.put(&later).await.unwrap();
    select(&f, vec![later]).await;
    let second = f
        .session
        .advance_kind_preparation(&mut task, deadline())
        .await
        .unwrap();
    assert!(!second.complete);
    assert_eq!(second.stats.scanned, 512);
    let third = f
        .session
        .advance_kind_preparation(&mut task, deadline())
        .await
        .unwrap();
    assert!(third.complete);
    assert_eq!(third.stats.scanned, expected_scanned);
    assert_eq!(third.stats.windows, 3);
    assert!(
        f.session
            .advance_kind_preparation(&mut concurrent, deadline())
            .await
            .unwrap()
            .complete
    );
    assert_eq!(
        concurrent.stats, partial,
        "a concurrently completed root requires no further scanning"
    );
    assert!(third.stats.record_reads >= third.stats.scanned);
    assert!(third.stats.record_bytes > 0);
    let expected = BTreeSet::from([m::ObjectKey::Operation(roots::ROOT_OPERATION), f.operation]);
    assert_eq!(
        f.session
            .finish_kind_preparation(&task, deadline())
            .await
            .unwrap(),
        expected
    );
    assert_eq!(
        f.session
            .advance_kind_preparation(&mut task, deadline())
            .await
            .unwrap()
            .stats,
        third.stats,
        "ready calls must not rescan"
    );
    let mut current = f
        .session
        .begin_kind_preparation(false, Limits::default())
        .await
        .unwrap();
    while !f
        .session
        .advance_kind_preparation(&mut current, deadline())
        .await
        .unwrap()
        .complete
    {}
    let mut extended = expected;
    extended.insert(later);
    assert_eq!(
        f.session
            .finish_kind_preparation(&current, deadline())
            .await
            .unwrap(),
        extended
    );
}

#[tokio::test]
async fn native_kind_preparation_keeps_total_bounds_after_failed_windows() {
    use jj_store_jj::query::Error;
    let f = fixture(0).await;
    for limits in [
        Limits {
            max_record_reads: WINDOW_READS - 1,
            ..Limits::default()
        },
        Limits {
            max_record_bytes: WINDOW_BYTES - 1,
            ..Limits::default()
        },
    ] {
        let mut task = f
            .session
            .begin_kind_preparation(false, limits)
            .await
            .unwrap();
        assert!(matches!(
            f.session
                .advance_kind_preparation(&mut task, deadline())
                .await,
            Err(Error::IndexBudget)
        ));
        assert_eq!(task.progress().stats, Stats::default());
        assert!(task.after.is_none() && task.ids.is_empty() && task.ready.is_none());
    }
    for limits in [
        Limits {
            max_ids: 1,
            ..Limits::default()
        },
        Limits {
            max_bytes: HEADER,
            ..Limits::default()
        },
    ] {
        let mut task = f
            .session
            .begin_kind_preparation(false, limits)
            .await
            .unwrap();
        assert!(matches!(
            f.session
                .advance_kind_preparation(&mut task, deadline())
                .await,
            Err(Error::Limit)
        ));
        assert_eq!(
            task.stats.descriptor_work, 256,
            "failed result admission retains reserved scan work"
        );
        assert!(task.after.is_none() && task.ids.is_empty() && task.ready.is_none());
    }
    let mut small = f
        .session
        .begin_kind_preparation(
            false,
            Limits {
                max_descriptors: 1,
                ..Limits::default()
            },
        )
        .await
        .unwrap();
    assert!(
        !f.session
            .advance_kind_preparation(&mut small, deadline())
            .await
            .unwrap()
            .complete
    );
    assert_eq!(small.stats.scanned, 1);
    assert!(matches!(
        f.session
            .advance_kind_preparation(&mut small, deadline())
            .await,
        Err(Error::IndexBudget)
    ));
    assert!(matches!(
        f.session.finish_kind_preparation(&small, deadline()).await,
        Err(Error::IndexRebuildRequired)
    ));
    // A prior failed/unknown window may have consumed the ordinary complete
    // ceiling without moving the cursor. The explicit total budget is separate.
    let mut larger = f
        .session
        .begin_kind_preparation(false, Limits::default())
        .await
        .unwrap();
    larger.stats.descriptor_work = super::super::MAX_SCANNED;
    assert!(
        f.session
            .advance_kind_preparation(&mut larger, deadline())
            .await
            .unwrap()
            .complete
    );
    assert!(larger.stats.descriptor_work > super::super::MAX_SCANNED);
}

#[tokio::test]
async fn native_kind_preparation_authorization_and_deadlines_precede_window_admission() {
    use jj_store_jj::query::Error;
    use jj_store_object::memory::Fault;
    let f = fixture(0).await;
    let mut task = f
        .session
        .begin_kind_preparation(false, Limits::default())
        .await
        .unwrap();
    // Authorization reads the manifest before any window admission.
    f.store.inject(Fault::ReadCorrupt);
    assert!(
        f.session
            .advance_kind_preparation(&mut task, deadline())
            .await
            .is_err()
    );
    assert_eq!(
        task.stats,
        Stats::default(),
        "admission authorization failure spends no window"
    );
    assert!(task.after.is_none() && task.ids.is_empty());
    assert!(
        f.session
            .advance_kind_preparation(&mut task, deadline())
            .await
            .unwrap()
            .stats
            .scanned
            > 0
    );
    let old = task.progress().stats;
    assert!(matches!(
        f.session
            .advance_kind_preparation(&mut task, Instant::now())
            .await,
        Err(Error::Limit)
    ));
    assert_eq!(task.progress().stats, old);
    task.expires = Instant::now();
    assert!(matches!(
        f.session
            .advance_kind_preparation(&mut task, deadline())
            .await,
        Err(Error::Limit)
    ));
    assert!(matches!(
        f.session.finish_kind_preparation(&task, deadline()).await,
        Err(Error::Limit)
    ));
    assert_eq!(task.progress().stats, old);
}

#[tokio::test]
async fn native_kind_preparation_rejects_foreign_sources_and_epoch_changes_after_promotion() {
    use jj_store_jj::query::Error;
    let f = fixture(0).await;
    let mut task = f
        .session
        .begin_kind_preparation(false, Limits::default())
        .await
        .unwrap();
    let objects =
        Arc::new(NativeObjects::new(f.store.clone(), f.repository, 8 * 1024 * 1024).unwrap());
    let foreign = crate::native::NativeSession::new_local_trusted(
        f.store.clone(),
        objects,
        m::PrincipalId::new([1; 16]),
        Permission::Read,
        Arc::new(SystemClock),
        Protection::default(),
        SessionLimits::default(),
    )
    .unwrap();
    assert!(matches!(
        foreign
            .advance_kind_preparation(&mut task, deadline())
            .await,
        Err(Error::Denied)
    ));
    assert!(
        f.session
            .advance_kind_preparation(&mut task, deadline())
            .await
            .unwrap()
            .complete
    );
    let ready = task.ready.clone().unwrap();
    let cached = cache::shared()
        .lookup(
            cache::Key::new(task.scope, &task.selected, false),
            task.selected.manifest().revision,
        )
        .0
        .unwrap();
    assert!(
        Arc::ptr_eq(&ready, &cached),
        "ordinary complete reads use the promoted allocation"
    );
    let warm = f
        .session
        .begin_kind_preparation(false, Limits::default())
        .await
        .unwrap();
    assert!(warm.progress().complete && warm.stats == Stats::default());
    assert_eq!(
        f.session
            .finish_kind_preparation(&warm, deadline())
            .await
            .unwrap()
            .len(),
        2
    );
    let mut manifest = f.session.snapshot().await.unwrap().manifest().clone();
    manifest.epoch = m::EpochId::new([2; 16]);
    replace(&f, &manifest).await;
    assert!(matches!(
        f.session
            .advance_kind_preparation(&mut task, deadline())
            .await,
        Err(Error::EpochChanged)
    ));
    assert!(matches!(
        f.session.finish_kind_preparation(&warm, deadline()).await,
        Err(Error::EpochChanged)
    ));
}

#[tokio::test]
async fn pinned_kind_preparation_outlives_the_initiating_page_and_empty_kind_is_complete() {
    use crate::reads::{OpenRead, ReadLimits};
    let f = fixture(0).await;
    let OpenRead::Pinned(read) = f
        .session
        .clone()
        .open_read(
            None,
            ReadLimits {
                max_objects: 1,
                timeout: Duration::from_millis(50),
            },
        )
        .await
        .unwrap()
    else {
        panic!("single head")
    };
    let mut task = read
        .begin_kind_preparation(
            false,
            Limits {
                max_ids: 8,
                ..Limits::default()
            },
        )
        .await
        .unwrap();
    tokio::time::sleep(Duration::from_millis(60)).await;
    assert!(
        f.session
            .advance_kind_preparation(&mut task, deadline())
            .await
            .unwrap()
            .complete
    );
    assert_eq!(
        f.session
            .finish_kind_preparation(&task, deadline())
            .await
            .unwrap()
            .len(),
        2
    );
    let mut copies = f
        .session
        .begin_kind_preparation(true, Limits::default())
        .await
        .unwrap();
    assert!(
        f.session
            .advance_kind_preparation(&mut copies, deadline())
            .await
            .unwrap()
            .complete
    );
    assert!(
        f.session
            .finish_kind_preparation(&copies, deadline())
            .await
            .unwrap()
            .is_empty()
    );
}

#[tokio::test]
async fn prepared_kind_progress_and_ready_ids_reauthorize_after_token_revocation() {
    use crate::auth::{Scope, TokenPolicy};
    use jj_store_jj::query::Error;
    let f = fixture(0).await;
    let mut policy = TokenPolicy::new(Scope::Repository(f.repository));
    let token = policy
        .issue(m::PrincipalId::new([9; 16]), Permission::Read, None)
        .unwrap();
    let token_id = token.id();
    async fn settings(f: &Fixture, policy: &TokenPolicy) {
        // Test-only selected policy replacement. Only verifier bytes enter
        // in-memory native records; the generated bearer stays in memory.
        let records = Records::new(f.store.clone(), f.repository);
        let selected = records
            .put(
                RecordKind::Settings,
                None,
                &serde_json::json!({"data": policy.encode().unwrap()}),
            )
            .await
            .unwrap();
        let mut manifest = f.session.snapshot().await.unwrap().manifest().clone();
        manifest.settings = selected;
        manifest.revision += 1;
        replace(f, &manifest).await;
    }
    settings(&f, &policy).await;
    let session = crate::native::NativeSession::new(
        f.store.clone(),
        f.objects.clone(),
        token,
        Arc::new(SystemClock),
        Protection::default(),
        SessionLimits::default(),
    )
    .unwrap();
    let mut task = session
        .begin_kind_preparation(false, Limits::default())
        .await
        .unwrap();
    assert!(
        session
            .advance_kind_preparation(&mut task, deadline())
            .await
            .unwrap()
            .complete
    );
    assert_eq!(
        session
            .finish_kind_preparation(&task, deadline())
            .await
            .unwrap()
            .len(),
        2
    );
    assert!(policy.revoke(token_id));
    settings(&f, &policy).await;
    let old = task.stats;
    assert!(matches!(
        session
            .advance_kind_preparation(&mut task, deadline())
            .await,
        Err(Error::Denied)
    ));
    assert!(matches!(
        session.finish_kind_preparation(&task, deadline()).await,
        Err(Error::Denied)
    ));
    assert_eq!(task.stats, old);
}

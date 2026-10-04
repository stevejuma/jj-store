use super::*;
use jj_store_jj::query::{self as q, copy_index};
use tokio::{sync::Notify, time::Instant};
use tokio_util::sync::CancellationToken;

struct FamilyAuthority {
    base: Arc<Authority>,
    index: copy_index::Index,
    entered: Notify,
    resume: Notify,
    pause: AtomicBool,
}
#[async_trait::async_trait]
impl BackendAuthority for FamilyAuthority {
    async fn authorize_read(&self) -> AuthorityResult<()> {
        self.base.authorize_read().await
    }
    async fn authorize_write(&self) -> AuthorityResult<()> {
        self.base.authorize_write().await
    }
    async fn copy_ids(&self, _: usize) -> AuthorityResult<BTreeSet<m::CopyId>> {
        panic!("projected family must not enumerate inventory")
    }
    async fn copy_family(
        &self,
        seeds: &BTreeSet<m::CopyId>,
        required: &BTreeSet<m::CopyId>,
        limits: copy_index::LookupLimits,
        deadline: Instant,
    ) -> AuthorityResult<Option<copy_index::Family>> {
        if self.pause.swap(false, Ordering::SeqCst) {
            self.entered.notify_one();
            self.resume.notified().await;
        }
        let mut selected = BTreeSet::new();
        for seed in seeds {
            if self.index.parents(*seed).is_some() {
                selected.insert(*seed);
            } else if required.contains(seed) {
                return Err(q::Error::NotCommitted.into());
            }
        }
        Ok(Some(
            self.index
                .family_with_work(&selected, limits, deadline, &CancellationToken::new())
                .await?,
        ))
    }
}
fn owned(id: &CopyId) -> m::CopyId {
    m::CopyId::from_slice(id.as_bytes()).unwrap()
}
async fn authority(f: &Fixture, nodes: &[(&CopyId, Vec<&CopyId>)]) -> Arc<FamilyAuthority> {
    let mut builder = copy_index::Builder::new(copy_index::Limits::default()).unwrap();
    for (id, parents) in nodes {
        builder
            .insert(owned(id), parents.iter().map(|id| owned(id)).collect())
            .unwrap();
    }
    Arc::new(FamilyAuthority {
        base: f.authority.clone(),
        index: builder
            .finish(
                Instant::now() + Duration::from_secs(10),
                &CancellationToken::new(),
            )
            .await
            .unwrap(),
        entered: Notify::new(),
        resume: Notify::new(),
        pause: AtomicBool::new(false),
    })
}
fn ids(values: &[RelatedCopy]) -> BTreeSet<CopyId> {
    values.iter().map(|value| value.id.clone()).collect()
}

#[tokio::test]
async fn projected_family_merges_committed_and_acknowledged_relations_only() {
    let f = Fixture::new();
    let a = copy(&f.backend, "a", vec![]).await;
    let sibling = copy(&f.backend, "sibling", vec![a.clone()]).await;
    let foreign = copy(&f.backend, "foreign", vec![a.clone()]).await;
    let auth = authority(&f, &[(&a, vec![]), (&sibling, vec![&a])]).await;
    let backend = NativeBackend::new(f.objects.clone(), auth, BackendLimits::default()).unwrap();
    // Writing a selected identity again must not hide its committed descendants.
    assert_eq!(copy(&backend, "a", vec![]).await, a);
    let requested = copy(&backend, "requested", vec![a.clone()]).await;
    let unrelated = copy(&backend, "unrelated", vec![]).await;
    let merge = copy(
        &backend,
        "merge",
        vec![requested.clone(), unrelated.clone()],
    )
    .await;
    let leaf = copy(&backend, "leaf", vec![merge.clone()]).await;
    let other = copy(&backend, "other", vec![unrelated.clone()]).await;
    let first = backend.get_related_copies(&requested).await.unwrap();
    assert_eq!(
        ids(&first),
        BTreeSet::from([
            a.clone(),
            sibling.clone(),
            requested.clone(),
            merge.clone(),
            leaf.clone()
        ])
    );
    assert!(!ids(&first).contains(&foreign));
    assert!(!ids(&first).contains(&unrelated));
    assert!(!ids(&first).contains(&other));
    assert_eq!(backend.get_related_copies(&requested).await.unwrap(), first);
    assert_eq!(
        ids(&backend.get_related_copies(&a).await.unwrap()),
        ids(&first)
    );
    assert!(backend.get_related_copies(&foreign).await.is_err());
    let failed = CopyHistory {
        current_path: path("failed"),
        parents: vec![a.clone()],
        salt: vec![],
    };
    let failed_id = CopyId::from_bytes(failed.key().as_bytes());
    f.store.inject(Fault::AfterWrite);
    f.store.inject(Fault::ReadMissing);
    assert!(
        backend.write_copy(&failed).await.is_err(),
        "unknown upload must not acknowledge a relation"
    );
    assert!(!ids(&backend.get_related_copies(&a).await.unwrap()).contains(&failed_id));
    assert!(backend.get_related_copies(&failed_id).await.is_err());
    assert_eq!(backend.write_copy(&failed).await.unwrap(), failed_id);
    assert!(ids(&backend.get_related_copies(&a).await.unwrap()).contains(&failed_id));
}

#[tokio::test]
async fn projected_family_captures_staging_before_provider_await() {
    let f = Fixture::new();
    let a = copy(&f.backend, "a", vec![]).await;
    let auth = authority(&f, &[(&a, vec![])]).await;
    let backend = Arc::new(
        NativeBackend::new(f.objects.clone(), auth.clone(), BackendLimits::default()).unwrap(),
    );
    let requested = copy(&backend, "requested", vec![a.clone()]).await;
    auth.pause.store(true, Ordering::SeqCst);
    let running = {
        let backend = backend.clone();
        let id = requested.clone();
        tokio::spawn(async move { backend.get_related_copies(&id).await })
    };
    tokio::time::timeout(Duration::from_secs(1), auth.entered.notified())
        .await
        .unwrap();
    let later = copy(&backend, "later", vec![requested.clone()]).await;
    auth.resume.notify_one();
    let old = tokio::time::timeout(Duration::from_secs(1), running)
        .await
        .unwrap()
        .unwrap()
        .unwrap();
    assert_eq!(ids(&old), BTreeSet::from([a.clone(), requested.clone()]));
    assert_eq!(
        ids(&backend.get_related_copies(&requested).await.unwrap()),
        BTreeSet::from([a, requested, later])
    );
}

#[tokio::test]
async fn staged_copy_edges_and_bytes_are_admitted_before_upload() {
    let f = Fixture::new();
    let root = copy(&f.backend, "root", vec![]).await;
    let other = copy(&f.backend, "other", vec![]).await;
    for limits in [
        BackendLimits {
            max_work: 1,
            ..BackendLimits::default()
        },
        BackendLimits {
            max_bytes: 256 + 1024 + 2 * 512 - 1,
            ..BackendLimits::default()
        },
    ] {
        let backend = f.reload(limits);
        let value = CopyHistory {
            current_path: path("rejected"),
            parents: vec![root.clone(), other.clone()],
            salt: vec![],
        };
        let id = CopyId::from_bytes(value.key().as_bytes());
        assert!(backend.write_copy(&value).await.is_err());
        assert!(matches!(
            backend.read_copy(&id).await,
            Err(BackendError::ObjectNotFound { .. })
        ));
        assert!(
            backend
                .write_copy(&CopyHistory {
                    current_path: path("accepted"),
                    parents: vec![CopyId::placeholder(), root.clone(), root.clone()],
                    salt: vec![]
                })
                .await
                .is_ok(),
            "placeholder and duplicate edges do not inflate staging"
        );
    }
}

use super::*;

#[tokio::test(flavor = "multi_thread", worker_threads = 2)]
async fn remote_copy_family_ignores_unrelated_inventory_and_body_pressure() {
    let mut f = Harness::new().await;
    retain_files(&f, 0, 96).await;
    let a = copy_node(&f, "a", &[]).await;
    let requested = copy_node(&f, "b", &[a]).await;
    let sibling = copy_node(&f, "c", &[a]).await;
    let unrelated = copy_node(&f, "unrelated", &[]).await;
    let merge = copy_node(&f, "merge", &[a, unrelated]).await;
    let leaf = copy_node(&f, "leaf", &[merge]).await;
    let other = copy_node(&f, "other", &[unrelated]).await;
    let mut keys = vec![a, requested, sibling, unrelated, merge, leaf, other];
    for n in 0..96 {
        keys.push(copy_node(&f, &format!("unrelated-{n}"), &[]).await);
    }
    select_keys(&f, keys.into_iter().map(m::ObjectKey::Copy).collect()).await;
    let counted = CountedInventory::new(f.store.clone());
    f.stop().await;
    f.start_on_store(counted.clone(), Protection::default(), RpcLimits::default())
        .await;
    let stores = remote::stores(
        f.client(&f.reader).await,
        MAX_BLOB,
        BackendLimits {
            max_nodes: 5,
            max_work: 64,
            max_bytes: 64 * 1024,
            ..BackendLimits::default()
        },
        OperationLimits::default(),
    )
    .unwrap();
    let cache = tempfile::tempdir().unwrap();
    let loader = stores
        .initialize_cache(&settings(), cache.path())
        .await
        .unwrap();
    let repo = stores
        .load_at(&loader, roots::ROOT_OPERATION)
        .await
        .unwrap();
    let backend = repo.store().backend();
    let expected: BTreeSet<_> = [a, requested, sibling, merge, leaf].into_iter().collect();
    let request = b::CopyId::from_bytes(requested.as_bytes());
    let related = backend
        .get_related_copies(&request)
        .await
        .expect("small copy family must not charge unrelated selected copies");
    let actual = related
        .iter()
        .map(|copy| m::CopyId::from_slice(copy.id.as_bytes()).unwrap())
        .collect();
    assert_eq!(expected, actual);
    assert_eq!(counted.copy_bodies.load(Ordering::Relaxed), 5);
    let position = |id: m::CopyId| {
        related
            .iter()
            .position(|copy| copy.id.as_bytes() == id.as_bytes())
            .unwrap()
    };
    assert!(position(leaf) < position(merge) && position(merge) < position(a));
    assert!(position(requested) < position(a) && position(sibling) < position(a));
    counted.reset();
    assert_eq!(backend.get_related_copies(&request).await.unwrap(), related);
    assert_eq!(counted.descriptors.load(Ordering::Relaxed), 0);
    assert_eq!(counted.copy_bodies.load(Ordering::Relaxed), 5);
    f.revoke_reader().await;
    assert!(backend.get_related_copies(&request).await.is_err());
    f.stop().await;
}

#[tokio::test(flavor = "multi_thread", worker_threads = 2)]
async fn rpc_copy_family_preserves_limits_selection_and_fresh_authorization() {
    use jj_store_service::transport::CopyFamilyLimits;
    use tokio::time::Instant;
    let mut f = Harness::new().await;
    retain_files(&f, 0, 96).await;
    let root = copy_node(&f, "root", &[]).await;
    let child = copy_node(&f, "child", &[root]).await;
    let staged = copy_node(&f, "foreign-stage", &[root]).await;
    select_keys(
        &f,
        vec![m::ObjectKey::Copy(root), m::ObjectKey::Copy(child)],
    )
    .await;
    f.stop().await;
    let counted = CountedInventory::new(f.store.clone());
    f.start_on_store(counted.clone(), Protection::default(), RpcLimits::default())
        .await;
    let client = f.client(&f.reader).await;
    let seeds = BTreeSet::from([child]);
    let limits = CopyFamilyLimits {
        max_nodes: 4,
        max_work: 64,
        max_bytes: 64 * 1024,
    };
    let deadline = || Instant::now() + Duration::from_secs(10);
    let first = client
        .copy_family(&seeds, &seeds, limits, deadline())
        .await
        .unwrap();
    assert_eq!(first.ids, BTreeSet::from([root, child]));
    assert!(first.work > 0 && first.work <= 64);
    assert_eq!(counted.copy_bodies.load(Ordering::Relaxed), 0);
    counted.reset();
    assert_eq!(
        client
            .copy_family(&seeds, &seeds, limits, deadline())
            .await
            .unwrap()
            .ids,
        first.ids
    );
    counted.warm("RPC selected-family warm root");
    for tighter in [
        CopyFamilyLimits {
            max_nodes: 1,
            ..limits
        },
        CopyFamilyLimits {
            max_work: 1,
            ..limits
        },
        CopyFamilyLimits {
            max_bytes: 512,
            ..limits
        },
    ] {
        assert_eq!(
            client
                .copy_family(&seeds, &seeds, tighter, deadline())
                .await
                .unwrap_err()
                .code(),
            Code::ResourceExhausted
        );
    }
    counted.reset();
    assert_eq!(
        client
            .copy_family(&seeds, &seeds, limits, Instant::now())
            .await
            .unwrap_err()
            .code(),
        Code::DeadlineExceeded
    );
    assert_eq!(
        counted.reads.load(Ordering::Relaxed),
        0,
        "expired client must perform no I/O"
    );
    let missing = BTreeSet::from([staged]);
    assert_eq!(
        client
            .copy_family(&missing, &missing, limits, deadline())
            .await
            .unwrap_err()
            .code(),
        Code::FailedPrecondition
    );
    assert!(
        client
            .copy_family(&missing, &BTreeSet::new(), limits, deadline())
            .await
            .unwrap()
            .ids
            .is_empty()
    );
    assert!(
        client
            .copy_family(&BTreeSet::new(), &BTreeSet::new(), limits, deadline())
            .await
            .unwrap()
            .ids
            .is_empty()
    );
    assert_eq!(
        client
            .copy_family(&seeds, &missing, limits, deadline())
            .await
            .unwrap_err()
            .code(),
        Code::InvalidArgument
    );
    publish_operation(&f, "RPC selected family later publication").await;
    let leaf = copy_node(&f, "leaf", &[child]).await;
    select_keys(&f, vec![m::ObjectKey::Copy(leaf)]).await;
    counted.reset();
    assert_eq!(
        client
            .copy_family(&seeds, &seeds, limits, deadline())
            .await
            .unwrap()
            .ids,
        BTreeSet::from([root, child, leaf])
    );
    assert_eq!(
        counted.descriptors.load(Ordering::Relaxed),
        3,
        "one seed membership read plus operation and copy additions"
    );
    assert_eq!(counted.copy_bodies.load(Ordering::Relaxed), 0);
    // Server session ceilings cannot be raised by a client's larger request.
    f.stop().await;
    f.start_with_session_limits(
        counted.clone(),
        Protection::default(),
        RpcLimits::default(),
        SessionLimits {
            inventory_nodes: 2,
            ..SessionLimits::default()
        },
    )
    .await;
    assert_eq!(
        f.client(&f.reader)
            .await
            .copy_family(&seeds, &seeds, limits, deadline())
            .await
            .unwrap_err()
            .code(),
        Code::ResourceExhausted
    );
    f.revoke_reader().await;
    assert_eq!(
        f.client(&f.reader)
            .await
            .copy_family(&seeds, &seeds, limits, deadline())
            .await
            .unwrap_err()
            .code(),
        Code::PermissionDenied
    );
    f.stop().await;
}

#[tokio::test(flavor = "multi_thread", worker_threads = 2)]
async fn rpc_copy_family_rejects_invalid_requests_before_storage() {
    let mut f = Harness::new().await;
    f.stop().await;
    let counted = CountedInventory::new(f.store.clone());
    f.start_on_store(counted.clone(), Protection::default(), RpcLimits::default())
        .await;
    let body = || wire::CopyFamilyRequest {
        repository: REPO.as_bytes().to_vec(),
        seeds: vec![wire::CopyFamilySeed {
            id: vec![2; 64],
            required: true,
        }],
        max_nodes: 2,
        max_work: 64,
        max_bytes: 64 * 1024,
        timeout_millis: 1000,
    };
    let mut variants = vec![];
    let mut bad = body();
    bad.seeds[0].id.pop();
    variants.push((bad, Code::InvalidArgument));
    let mut bad = body();
    bad.seeds.push(bad.seeds[0].clone());
    variants.push((bad, Code::InvalidArgument));
    let mut bad = body();
    bad.seeds.push(wire::CopyFamilySeed {
        id: vec![1; 64],
        required: false,
    });
    variants.push((bad, Code::InvalidArgument));
    let mut bad = body();
    bad.max_nodes = 0;
    variants.push((bad, Code::ResourceExhausted));
    let mut bad = body();
    bad.max_work = 0;
    variants.push((bad, Code::ResourceExhausted));
    let mut bad = body();
    bad.max_bytes = 511;
    variants.push((bad, Code::ResourceExhausted));
    let mut bad = body();
    bad.timeout_millis = 0;
    variants.push((bad, Code::ResourceExhausted));
    let mut raw = f.raw().await;
    for (bad, code) in variants {
        counted.reset();
        assert_eq!(
            raw.copy_family(req(&f.reader, bad))
                .await
                .unwrap_err()
                .code(),
            code
        );
        assert_eq!(
            counted.reads.load(Ordering::Relaxed),
            0,
            "reject before storage access"
        );
    }
    f.stop().await;
    f.start_with_session_limits(
        counted.clone(),
        Protection::default(),
        RpcLimits::default(),
        SessionLimits {
            inventory_nodes: 1,
            ..SessionLimits::default()
        },
    )
    .await;
    let mut two = body();
    two.seeds.push(wire::CopyFamilySeed {
        id: vec![3; 64],
        required: false,
    });
    counted.reset();
    assert_eq!(
        f.raw()
            .await
            .copy_family(req(&f.reader, two))
            .await
            .unwrap_err()
            .code(),
        Code::ResourceExhausted
    );
    assert_eq!(counted.reads.load(Ordering::Relaxed), 0);
    f.stop().await;
}

#[tokio::test(flavor = "multi_thread", worker_threads = 2)]
async fn remote_copy_family_merges_only_acknowledged_client_staging() {
    let mut f = Harness::new().await;
    let root = copy_node(&f, "root", &[]).await;
    let sibling = copy_node(&f, "sibling", &[root]).await;
    let foreign = copy_node(&f, "foreign-stage", &[root]).await;
    select_keys(
        &f,
        vec![m::ObjectKey::Copy(root), m::ObjectKey::Copy(sibling)],
    )
    .await;
    let stores = remote::stores(
        f.client(&f.admin).await,
        MAX_BLOB,
        BackendLimits::default(),
        OperationLimits::default(),
    )
    .unwrap();
    let cache = tempfile::tempdir().unwrap();
    let loader = stores
        .initialize_cache(&settings(), cache.path())
        .await
        .unwrap();
    let repo = stores
        .load_at(&loader, roots::ROOT_OPERATION)
        .await
        .unwrap();
    let backend = repo.store().backend();
    let owned = |id: m::CopyId| b::CopyId::from_bytes(id.as_bytes());
    let write = |name: &str, parents: Vec<b::CopyId>| b::CopyHistory {
        current_path: jj_lib::repo_path::RepoPathBuf::from_internal_string(name).unwrap(),
        parents,
        salt: name.as_bytes().to_vec(),
    };
    // Reacknowledging a selected identity must preserve committed descendants.
    let original = f
        .objects
        .get::<b::CopyHistory>(m::ObjectKey::Copy(root))
        .await
        .unwrap();
    assert_eq!(backend.write_copy(&original).await.unwrap(), owned(root));
    let requested = backend
        .write_copy(&write("requested", vec![owned(root)]))
        .await
        .unwrap();
    let unrelated = backend
        .write_copy(&write("unrelated", vec![]))
        .await
        .unwrap();
    let merge = backend
        .write_copy(&write("merge", vec![requested.clone(), unrelated.clone()]))
        .await
        .unwrap();
    let leaf = backend
        .write_copy(&write("leaf", vec![merge.clone()]))
        .await
        .unwrap();
    let other = backend
        .write_copy(&write("other", vec![unrelated.clone()]))
        .await
        .unwrap();
    let family = backend.get_related_copies(&requested).await.unwrap();
    let ids = |family: Vec<b::RelatedCopy>| {
        family
            .into_iter()
            .map(|copy| copy.id)
            .collect::<BTreeSet<_>>()
    };
    assert_eq!(
        ids(family),
        BTreeSet::from([owned(root), owned(sibling), requested.clone(), merge, leaf])
    );
    assert!(!ids(backend.get_related_copies(&requested).await.unwrap()).contains(&other));
    assert!(backend.get_related_copies(&owned(foreign)).await.is_err());
    let failed = write("unknown", vec![owned(root)]);
    let failed_id = b::CopyId::from_bytes(failed.key().as_bytes());
    f.store.inject(Fault::AfterWrite);
    f.store.inject(Fault::ReadMissing);
    assert!(backend.write_copy(&failed).await.is_err());
    assert!(!ids(backend.get_related_copies(&requested).await.unwrap()).contains(&failed_id));
    assert!(backend.get_related_copies(&failed_id).await.is_err());
    assert_eq!(backend.write_copy(&failed).await.unwrap(), failed_id);
    assert!(ids(backend.get_related_copies(&requested).await.unwrap()).contains(&failed_id));
    f.stop().await;
}

// An actual older-peer boundary: suppress only the new method while keeping
// ordinary authorization, complete inventory and object calls on the real server.
#[derive(Clone)]
struct FamilyPeer<S> {
    inner: S,
    code: Code,
    inventory_calls: Arc<AtomicUsize>,
}
impl<S, B> tower::Service<axum::http::Request<B>> for FamilyPeer<S>
where
    S: tower::Service<axum::http::Request<B>, Response = axum::http::Response<tonic::body::Body>>,
    S::Future: Send + 'static,
{
    type Response = S::Response;
    type Error = S::Error;
    type Future = std::pin::Pin<
        Box<
            dyn std::future::Future<Output = std::result::Result<Self::Response, Self::Error>>
                + Send,
        >,
    >;
    fn poll_ready(
        &mut self,
        cx: &mut std::task::Context<'_>,
    ) -> std::task::Poll<std::result::Result<(), Self::Error>> {
        self.inner.poll_ready(cx)
    }
    fn call(&mut self, request: axum::http::Request<B>) -> Self::Future {
        if request.uri().path().ends_with("/CopyFamily") {
            let response = tonic::Status::new(self.code, "selected-family test peer").into_http();
            return Box::pin(async move { Ok(response) });
        }
        if request.uri().path().ends_with("/Inventory") {
            self.inventory_calls.fetch_add(1, Ordering::Relaxed);
        }
        Box::pin(self.inner.call(request))
    }
}
#[tokio::test(flavor = "multi_thread", worker_threads = 2)]
async fn remote_copy_family_falls_back_only_for_unimplemented_peers() {
    let mut f = Harness::new().await;
    let root = copy_node(&f, "root", &[]).await;
    let child = copy_node(&f, "child", &[root]).await;
    select_keys(
        &f,
        vec![m::ObjectKey::Copy(root), m::ObjectKey::Copy(child)],
    )
    .await;
    f.stop().await;
    for code in [
        Code::Unimplemented,
        Code::PermissionDenied,
        Code::ResourceExhausted,
        Code::DeadlineExceeded,
        Code::DataLoss,
        Code::Unavailable,
        Code::FailedPrecondition,
    ] {
        let calls = Arc::new(AtomicUsize::new(0));
        let endpoint = NativeEndpoint::new(
            REPO,
            f.store.clone(),
            Arc::new(SystemClock),
            Protection::default(),
            SessionLimits::default(),
            MAX_BLOB,
        )
        .unwrap();
        let rpc = NativeRpc::new(vec![endpoint], RpcLimits::default()).unwrap();
        let listener = tokio::net::TcpListener::bind("127.0.0.1:0").await.unwrap();
        f.address = format!("http://{}", listener.local_addr().unwrap());
        let (tx, rx) = tokio::sync::oneshot::channel();
        f.shutdown = Some(tx);
        let layer_calls = calls.clone();
        f.server = Some(tokio::spawn(async move {
            tonic::transport::Server::builder()
                .layer(tower::layer::layer_fn(move |inner| FamilyPeer {
                    inner,
                    code,
                    inventory_calls: layer_calls.clone(),
                }))
                .add_service(rpc.service())
                .serve_with_incoming_shutdown(
                    tokio_stream::wrappers::TcpListenerStream::new(listener),
                    async {
                        let _ = rx.await;
                    },
                )
                .await
                .unwrap();
        }));
        let stores = remote::stores(
            f.client(&f.reader).await,
            MAX_BLOB,
            BackendLimits::default(),
            OperationLimits::default(),
        )
        .unwrap();
        let cache = tempfile::tempdir().unwrap();
        let loader = stores
            .initialize_cache(&settings(), cache.path())
            .await
            .unwrap();
        let repo = stores
            .load_at(&loader, roots::ROOT_OPERATION)
            .await
            .unwrap();
        let result = repo
            .store()
            .backend()
            .get_related_copies(&b::CopyId::from_bytes(child.as_bytes()))
            .await;
        if code == Code::Unimplemented {
            assert_eq!(
                result
                    .unwrap()
                    .into_iter()
                    .map(|copy| m::CopyId::from_slice(copy.id.as_bytes()).unwrap())
                    .collect::<BTreeSet<_>>(),
                BTreeSet::from([root, child])
            );
            assert_eq!(calls.load(Ordering::Relaxed), 1);
        } else {
            assert!(result.is_err(), "{code:?} must stay an error");
            assert_eq!(
                calls.load(Ordering::Relaxed),
                0,
                "{code:?} must not fall back to complete inventory"
            );
        }
        f.stop().await;
    }
}

struct PendingFamilyRead {
    inner: Arc<MemoryStore>,
    entered: tokio::sync::Notify,
    active: Arc<AtomicUsize>,
}
struct FamilyReadGuard(Arc<AtomicUsize>);
impl Drop for FamilyReadGuard {
    fn drop(&mut self) {
        self.0.fetch_sub(1, Ordering::SeqCst);
    }
}
#[async_trait::async_trait]
impl jj_store_object::ObjectStore for PendingFamilyRead {
    async fn head(&self, key: &Key) -> jj_store_object::Result<jj_store_object::Metadata> {
        self.inner.head(key).await
    }
    async fn put(
        &self,
        key: &Key,
        condition: Condition,
        upload: &StagedUpload,
    ) -> jj_store_object::Result<WriteOutcome> {
        self.inner.put(key, condition, upload).await
    }
    async fn get(
        &self,
        key: &Key,
        range: Option<jj_store_object::ByteRange>,
    ) -> jj_store_object::Result<jj_store_object::Download> {
        if key.as_str().contains("/inventories/") {
            self.active.fetch_add(1, Ordering::SeqCst);
            let _guard = FamilyReadGuard(self.active.clone());
            self.entered.notify_one();
            std::future::pending::<()>().await;
        }
        self.inner.get(key, range).await
    }
}
#[tokio::test(flavor = "multi_thread", worker_threads = 2)]
async fn rpc_copy_family_deadline_cancels_pending_storage_and_releases_capacity() {
    use jj_store_service::transport::CopyFamilyLimits;
    use tokio::time::Instant;
    let mut f = Harness::new().await;
    let root = copy_node(&f, "root", &[]).await;
    select_keys(&f, vec![m::ObjectKey::Copy(root)]).await;
    f.stop().await;
    let pending = Arc::new(PendingFamilyRead {
        inner: f.store.clone(),
        entered: tokio::sync::Notify::new(),
        active: Arc::new(AtomicUsize::new(0)),
    });
    f.start_on_store(
        pending.clone(),
        Protection::default(),
        RpcLimits {
            concurrent: 1,
            ..RpcLimits::default()
        },
    )
    .await;
    let client = f.client(&f.reader).await;
    let running = {
        let client = client.clone();
        tokio::spawn(async move {
            let seeds = BTreeSet::from([root]);
            client
                .copy_family(
                    &seeds,
                    &seeds,
                    CopyFamilyLimits::default(),
                    Instant::now() + Duration::from_millis(100),
                )
                .await
        })
    };
    tokio::time::timeout(Duration::from_secs(1), pending.entered.notified())
        .await
        .unwrap();
    assert_eq!(pending.active.load(Ordering::SeqCst), 1);
    let error = tokio::time::timeout(Duration::from_secs(1), running)
        .await
        .unwrap()
        .unwrap()
        .unwrap_err();
    assert!(
        matches!(
            error.code(),
            Code::DeadlineExceeded | Code::ResourceExhausted
        ),
        "unexpected deadline status: {error}"
    );
    tokio::time::timeout(Duration::from_secs(1), async {
        while pending.active.load(Ordering::SeqCst) != 0 {
            tokio::task::yield_now().await;
        }
    })
    .await
    .unwrap();
    client
        .authorize(false)
        .await
        .expect("expired lookup released sole RPC slot");
    f.stop().await;
}

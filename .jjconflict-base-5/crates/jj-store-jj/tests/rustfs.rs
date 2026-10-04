//! Explicit deployment lane. Only the owned disposable Python harness supplies
//! these credentials/endpoints; normal cargo tests never start external services.
use jj_lib::backend::Tree;
use jj_store_jj::objects::{NativeObjects, UploadStatus};
use jj_store_model::RepositoryId;
use jj_store_object::{
    s3::{Config, Credentials, S3Store},
    *,
};
use std::{sync::Arc, time::Duration};
use tokio::io::{AsyncBufReadExt as _, AsyncReadExt as _, AsyncWriteExt as _};

fn client(endpoint: &str) -> S3Store {
    S3Store::new(Config {
        endpoint: endpoint.into(),
        bucket: "jj-store-contract".into(),
        region: "us-east-1".into(),
        credentials: Credentials {
            access_key: std::env::var("JST_TEST_ACCESS_KEY").unwrap(),
            secret_key: std::env::var("JST_TEST_SECRET_KEY").unwrap(),
            session_token: None,
        },
        allow_http_loopback: true,
        max_object_bytes: 16 * 1024 * 1024,
        timeout: Duration::from_secs(10),
    })
    .unwrap()
}
fn winner(outcome: WriteOutcome) -> VersionToken {
    if let WriteOutcome::Applied(token) = outcome {
        token
    } else {
        panic!("expected applied: {outcome:?}")
    }
}

#[tokio::test]
#[ignore = "requires isolated pinned RustFS; run scripts/rustfs-contract.py --run --rust-client"]
async fn real_rustfs_transport_and_native_blob_contract() {
    let endpoint = std::env::var("JST_TEST_ENDPOINT").expect("use disposable RustFS harness");
    let store = Arc::new(client(&endpoint));
    let key = Key::new("rust-client/manifest").unwrap();
    let upload = StagedUpload::from_bytes(b"initial", 100).await.unwrap();
    let first = winner(store.put(&key, Condition::Absent, &upload).await.unwrap());
    assert_eq!(store.head(&key).await.unwrap().version, first);
    assert_eq!(
        store.put(&key, Condition::Absent, &upload).await.unwrap(),
        WriteOutcome::PreconditionFailed
    );
    let replacement = StagedUpload::from_bytes(b"replacement", 100).await.unwrap();
    winner(
        store
            .put(&key, Condition::Matches(first.clone()), &replacement)
            .await
            .unwrap(),
    );
    assert_eq!(
        store
            .put(&key, Condition::Matches(first), &upload)
            .await
            .unwrap(),
        WriteOutcome::PreconditionFailed
    );
    assert_eq!(
        store
            .get(&key, Some(ByteRange::new(1, 5).unwrap()))
            .await
            .unwrap()
            .read_bounded(4)
            .await
            .unwrap(),
        b"epla"
    );
    for round in 0..8 {
        let key = Key::new(format!("rust-client/race/{round}")).unwrap();
        for create in [true, false] {
            let condition = if create {
                Condition::Absent
            } else {
                Condition::Matches(store.head(&key).await.unwrap().version)
            };
            let mut tasks = Vec::new();
            let gate = Arc::new(tokio::sync::Barrier::new(8));
            for n in 0..8 {
                let store = Arc::new(client(&endpoint));
                let key = key.clone();
                let condition = condition.clone();
                let gate = gate.clone();
                tasks.push(tokio::spawn(async move {
                    let upload = StagedUpload::from_bytes(
                        format!("round-{round}-{create}-{n}").as_bytes(),
                        100,
                    )
                    .await
                    .unwrap();
                    gate.wait().await;
                    store.put(&key, condition, &upload).await.unwrap()
                }));
            }
            let mut wins = 0;
            for task in tasks {
                match task.await.unwrap() {
                    WriteOutcome::Applied(_) => wins += 1,
                    WriteOutcome::PreconditionFailed => {}
                    WriteOutcome::Unknown => panic!("race result unknown"),
                }
            }
            assert_eq!(wins, 1);
        }
    }
    let native =
        NativeObjects::new(store.clone(), RepositoryId::new([17; 16]), 9 * 1024 * 1024).unwrap();
    let (tree, status) = native.put(&Tree::default()).await.unwrap();
    assert_eq!(status, UploadStatus::Stored);
    assert_eq!(
        native.put(&Tree::default()).await.unwrap().1,
        UploadStatus::AlreadyPresent
    );
    assert_eq!(native.get::<Tree>(tree).await.unwrap(), Tree::default());
    let (file, status) = native
        .put_file(tokio::io::repeat(37).take(8 * 1024 * 1024))
        .await
        .unwrap();
    assert_eq!(status, UploadStatus::Stored);
    let raw = native.get_file(file).await.unwrap();
    assert_eq!(raw.len(), 8 * 1024 * 1024);
    let mut reader = raw.reader().unwrap();
    let mut bytes = vec![0; 64 * 1024];
    let mut length = 0;
    loop {
        let n = reader.read(&mut bytes).await.unwrap();
        if n == 0 {
            break;
        }
        assert!(bytes[..n].iter().all(|b| *b == 37));
        length += n;
    }
    assert_eq!(length, 8 * 1024 * 1024);
    assert_eq!(
        native
            .put_file(tokio::io::repeat(37).take(8 * 1024 * 1024))
            .await
            .unwrap()
            .1,
        UploadStatus::AlreadyPresent
    );
    // Proxy forwards a complete write and reads the storage acknowledgement,
    // then closes the client connection without returning any response bytes.
    let listener = tokio::net::TcpListener::bind("127.0.0.1:0").await.unwrap();
    let proxy_endpoint = format!("http://{}", listener.local_addr().unwrap());
    let target = endpoint.strip_prefix("http://").unwrap().to_owned();
    let proxy = tokio::spawn(async move {
        let (client, _) = listener.accept().await.unwrap();
        let mut reader = tokio::io::BufReader::new(client);
        let mut headers = Vec::new();
        loop {
            let start = headers.len();
            assert!(start < 16384);
            reader.read_until(b'\n', &mut headers).await.unwrap();
            if &headers[start..] == b"\r\n" {
                break;
            }
        }
        let text = std::str::from_utf8(&headers).unwrap();
        let len = text
            .lines()
            .find_map(|l| {
                l.to_ascii_lowercase()
                    .strip_prefix("content-length:")
                    .map(|v| v.trim().parse::<u64>().unwrap())
            })
            .unwrap();
        let mut upstream = tokio::net::TcpStream::connect(target).await.unwrap();
        upstream.write_all(&headers).await.unwrap();
        assert_eq!(
            tokio::io::copy(&mut (&mut reader).take(len), &mut upstream)
                .await
                .unwrap(),
            len
        );
        upstream.flush().await.unwrap();
        let mut response = tokio::io::BufReader::new(upstream);
        let mut line = String::new();
        response.read_line(&mut line).await.unwrap();
        assert!(line.contains("200"), "{line}");
    });
    let lost = Key::new("rust-client/lost-ack").unwrap();
    let outcome = client(&proxy_endpoint)
        .put(&lost, Condition::Absent, &upload)
        .await
        .unwrap();
    proxy.await.unwrap();
    assert_eq!(outcome, WriteOutcome::Unknown);
    assert_eq!(
        store
            .get(&lost, None)
            .await
            .unwrap()
            .read_bounded(100)
            .await
            .unwrap(),
        b"initial"
    );
    println!(
        "Rust adapter: 8x8 create/CAS races, stale tokens, range, native metadata, 8 MiB streamed binary duplicate and verified GET, real dropped acknowledgement: passed"
    );
}

#[tokio::test]
#[ignore = "requires isolated pinned RustFS; run scripts/rustfs-contract.py --run --rust-client"]
async fn real_rustfs_manifest_publication_and_cold_recovery() {
    use jj_lib::merge::Merge;
    use jj_lib::repo_path::RepoPathComponentBuf;
    use jj_lib::{backend as b, op_store as o};
    use jj_store_jj::{
        codec::{NativeCodec as _, NativeFile},
        roots,
    };
    use jj_store_model::{self as m, ObjectKey};
    use jj_store_wal::publication::{
        Coordinator, Limits, ManifestSnapshot, PublicationGuard, PublishOutcome, Request,
        SemanticKind, Snapshot, Transition,
    };
    use std::collections::{BTreeMap, BTreeSet};
    use std::sync::atomic::{AtomicBool, Ordering};
    struct Permit;
    #[async_trait::async_trait]
    impl PublicationGuard for Permit {
        async fn authorize(&self, _: &ManifestSnapshot, _: &Request) -> jj_store_wal::Result<()> {
            Ok(())
        }
        async fn validate_transition(
            &self,
            _: &ManifestSnapshot,
            _: &Request,
            _: &Snapshot,
        ) -> jj_store_wal::Result<()> {
            Ok(())
        }
    }
    struct LoseAck {
        inner: S3Store,
        drop_next: AtomicBool,
    }
    #[async_trait::async_trait]
    impl ObjectStore for LoseAck {
        async fn head(&self, key: &Key) -> Result<Metadata> {
            self.inner.head(key).await
        }
        async fn get(&self, key: &Key, range: Option<ByteRange>) -> Result<Download> {
            self.inner.get(key, range).await
        }
        async fn put(
            &self,
            key: &Key,
            condition: Condition,
            upload: &StagedUpload,
        ) -> Result<WriteOutcome> {
            let result = self.inner.put(key, condition, upload).await?;
            if key.as_str().ends_with("/manifest.json")
                && matches!(result, WriteOutcome::Applied(_))
                && self.drop_next.swap(false, Ordering::SeqCst)
            {
                Ok(WriteOutcome::Unknown)
            } else {
                Ok(result)
            }
        }
    }
    struct NativeFixture {
        file: NativeFile,
        tree: b::Tree,
        commit: b::Commit,
        view: o::View,
        operation: o::Operation,
    }
    fn fixture(n: u8, parents: &[&NativeFixture]) -> NativeFixture {
        let file = NativeFile(vec![n; 4096]);
        let tree = b::Tree::from_sorted_entries(vec![(
            RepoPathComponentBuf::new("data.bin").unwrap(),
            b::TreeValue::File {
                id: b::FileId::from_bytes(file.key().as_bytes()),
                executable: n == 2,
                copy_id: b::CopyId::placeholder(),
            },
        )]);
        let mut commit = roots::commit();
        commit.description = format!("native publication {n}");
        commit.root_tree = Merge::resolved(b::TreeId::from_bytes(tree.key().as_bytes()));
        commit.parents = if parents.is_empty() {
            vec![b::CommitId::from_bytes(roots::ROOT_COMMIT.as_bytes())]
        } else {
            parents
                .iter()
                .map(|f| b::CommitId::from_bytes(f.commit.key().as_bytes()))
                .collect()
        };
        let mut view = roots::view();
        view.head_ids = [b::CommitId::from_bytes(commit.key().as_bytes())]
            .into_iter()
            .collect();
        view.wc_commit_ids.insert(
            "fixture".into(),
            b::CommitId::from_bytes(commit.key().as_bytes()),
        );
        let mut operation = roots::operation();
        operation.view_id = o::ViewId::from_bytes(view.key().as_bytes());
        operation.metadata.description = format!("native operation {n}");
        operation.parents = if parents.is_empty() {
            vec![o::OperationId::from_bytes(roots::ROOT_OPERATION.as_bytes())]
        } else {
            parents
                .iter()
                .map(|f| o::OperationId::from_bytes(f.operation.key().as_bytes()))
                .collect()
        };
        operation.commit_predecessors = Some(BTreeMap::from([(
            b::CommitId::from_bytes(commit.key().as_bytes()),
            parents
                .iter()
                .map(|f| b::CommitId::from_bytes(f.commit.key().as_bytes()))
                .collect(),
        )]));
        NativeFixture {
            file,
            tree,
            commit,
            view,
            operation,
        }
    }
    fn req(n: u8, fixture: &NativeFixture, remove: &[m::OperationId]) -> Request {
        Request {
            principal: m::PrincipalId::new([18; 16]),
            request_id: m::RequestId::new([n; 16]),
            publication_id: m::PublicationId::new([n; 16]),
            transition: Some(Transition::Native {
                remove: remove.iter().copied().collect(),
                add: m::OperationId::from_slice(fixture.operation.key().as_bytes()).unwrap(),
            }),
            settings: None,
            result: vec![n],
            kind: SemanticKind::OperationPublished,
        }
    }
    async fn stage(objects: &NativeObjects, f: &NativeFixture) {
        objects.put(&f.file).await.unwrap();
        objects.put(&f.tree).await.unwrap();
        objects.put(&f.commit).await.unwrap();
        objects.put(&f.view).await.unwrap();
        objects.put(&f.operation).await.unwrap();
    }
    let endpoint = std::env::var("JST_TEST_ENDPOINT").expect("use disposable RustFS harness");
    let repo = m::RepositoryId::new([18; 16]);
    let store = Arc::new(LoseAck {
        inner: client(&endpoint),
        drop_next: AtomicBool::new(false),
    });
    let objects = Arc::new(NativeObjects::new(store.clone(), repo, 1024 * 1024).unwrap());
    let coordinator = Coordinator::new(
        store.clone(),
        repo,
        objects.clone(),
        Arc::new(Permit),
        Limits::default(),
    )
    .unwrap();
    let a = fixture(1, &[]);
    let b = fixture(2, &[]);
    let merged = fixture(3, &[&a, &b]);
    let extra = (5..=20).map(|n| fixture(n, &[&merged])).collect::<Vec<_>>();
    let mut expected_heads =
        BTreeSet::from([m::OperationId::from_slice(merged.operation.key().as_bytes()).unwrap()]);
    let mut extra_requests = Vec::new();
    let mut random = 0x21acfe93848u64;
    for (n, f) in (5..=20).zip(&extra) {
        random = random.wrapping_mul(6364136223846793005).wrapping_add(1);
        let remove = if random & 1 == 0 {
            expected_heads.iter().copied().collect::<Vec<_>>()
        } else {
            Vec::new()
        };
        let request = req(n, f, &remove);
        for id in &remove {
            expected_heads.remove(id);
        }
        expected_heads.insert(m::OperationId::from_slice(f.operation.key().as_bytes()).unwrap());
        extra_requests.push(request);
    }
    let root = roots::ROOT_OPERATION;
    let ra = req(1, &a, &[root]);
    let rb = req(2, &b, &[root]);
    let a_id = m::OperationId::from_slice(a.operation.key().as_bytes()).unwrap();
    let b_id = m::OperationId::from_slice(b.operation.key().as_bytes()).unwrap();
    if std::env::var_os("JST_TEST_RECOVER_ONLY").is_none() {
        coordinator
            .initialize(m::EpochId::new([18; 16]), b"fixture policy".to_vec())
            .await
            .unwrap();
        stage(&objects, &a).await;
        stage(&objects, &b).await;
        stage(&objects, &merged).await;
        let other_store = Arc::new(client(&endpoint));
        let other_source =
            Arc::new(NativeObjects::new(other_store.clone(), repo, 1024 * 1024).unwrap());
        let other = Coordinator::new(
            other_store,
            repo,
            other_source,
            Arc::new(Permit),
            Limits::default(),
        )
        .unwrap();
        store.drop_next.store(true, Ordering::SeqCst);
        let (first, second) = tokio::join!(coordinator.publish(&ra), other.publish(&rb));
        assert!(matches!(first.unwrap(), PublishOutcome::Committed(_)));
        assert!(matches!(second.unwrap(), PublishOutcome::Committed(_)));
        assert!(!store.drop_next.load(Ordering::SeqCst));
        let forked = coordinator.read().await.unwrap();
        assert_eq!(forked.manifest().heads, BTreeSet::from([a_id, b_id]));
        assert!(matches!(
            coordinator
                .publish(&req(3, &merged, &[a_id, b_id]))
                .await
                .unwrap(),
            PublishOutcome::Committed(_)
        ));
        coordinator.checkpoint().await.unwrap();
        let mut missing = fixture(4, &[]);
        missing.operation.view_id = o::ViewId::from_bytes(&[250; 64]);
        objects.put(&missing.operation).await.unwrap();
        assert!(coordinator.publish(&req(4, &missing, &[])).await.is_err());
        for (i, (f, request)) in extra.iter().zip(&extra_requests).enumerate() {
            stage(&objects, f).await;
            if i % 4 == 0 {
                store.drop_next.store(true, Ordering::SeqCst);
            }
            assert!(matches!(
                coordinator.publish(request).await.unwrap(),
                PublishOutcome::Committed(_)
            ));
            if i % 5 == 0 {
                coordinator.checkpoint().await.unwrap();
            }
            if i % 3 == 0 {
                assert!(matches!(
                    coordinator.publish(&ra).await.unwrap(),
                    PublishOutcome::Committed(_)
                ));
            }
        }
        // Simulate out-of-band corruption under owned test credentials. Normal
        // native writes expose immutable creation, never this mutable raw path.
        let key = Key::new(a.file.key().storage_path(repo)).unwrap();
        let response = store.get(&key, None).await.unwrap();
        let version = response.metadata.version.clone();
        let original = response.read_bounded(65536).await.unwrap();
        let mut corrupt = original.clone();
        *corrupt.last_mut().unwrap() ^= 1;
        let token = winner(
            store
                .put(
                    &key,
                    Condition::Matches(version),
                    &StagedUpload::from_bytes(&corrupt, 65536).await.unwrap(),
                )
                .await
                .unwrap(),
        );
        assert!(coordinator.recover(1000).await.is_err());
        winner(
            store
                .put(
                    &key,
                    Condition::Matches(token),
                    &StagedUpload::from_bytes(&original, 65536).await.unwrap(),
                )
                .await
                .unwrap(),
        );
    }
    // This branch also runs in a fresh test process after Docker kill/start,
    // with new clients/source instances and no reusable in-process caches.
    let recovered = coordinator.recover(1000).await.unwrap();
    assert_eq!(recovered.manifest().published, m::WalPosition(19));
    assert_eq!(recovered.manifest().heads, expected_heads);
    assert!(recovered.manifest().checkpoint.is_some());
    assert_eq!(
        coordinator
            .committed(&recovered, m::WalPosition(0), 100)
            .await
            .unwrap()
            .len(),
        19
    );
    assert_eq!(
        coordinator.status(&ra).await.unwrap().unwrap().result,
        vec![1]
    );
    assert!(matches!(
        coordinator.publish(&ra).await.unwrap(),
        PublishOutcome::Committed(_)
    ));
    assert_eq!(
        coordinator.read().await.unwrap().manifest().published,
        m::WalPosition(19)
    );
    for request in &extra_requests {
        let original = coordinator.status(request).await.unwrap().unwrap();
        assert_eq!(
            coordinator.publish(request).await.unwrap(),
            PublishOutcome::Committed(original)
        );
    }
    assert_eq!(
        objects.get::<b::Commit>(merged.commit.key()).await.unwrap(),
        merged.commit
    );
    assert!(
        coordinator
            .inventory(&recovered, 1000)
            .await
            .unwrap()
            .iter()
            .any(|n| n.key == a.commit.key())
    );
    let ObjectKey::File(file) = a.file.key() else {
        unreachable!()
    };
    assert_eq!(objects.get_file(file).await.unwrap().len(), 4096);
    println!(
        "Native manifest publication: concurrent siblings, lost ACK, reconciliation head update, durable receipts/checkpoint, missing dependency rejection, corruption detection, cache-free retained history, randomized retries/checkpoints and evolution dependency recovery passed"
    );
}

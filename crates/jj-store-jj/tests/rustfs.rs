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

use jj_store_object::{
    memory::{Fault, MemoryStore},
    *,
};
use std::sync::Arc;
use tokio::io::AsyncReadExt as _;

async fn upload(bytes: &[u8]) -> StagedUpload {
    StagedUpload::from_bytes(bytes, 1024 * 1024).await.unwrap()
}
fn applied(outcome: WriteOutcome) -> VersionToken {
    match outcome {
        WriteOutcome::Applied(token) => token,
        other => panic!("expected applied: {other:?}"),
    }
}

#[tokio::test]
async fn conditional_create_and_cas_have_one_winner() {
    let store = Arc::new(MemoryStore::default());
    let key = Key::new("repo/manifest").unwrap();
    for create in [true, false] {
        let condition = if create {
            Condition::Absent
        } else {
            Condition::Matches(store.head(&key).await.unwrap().version)
        };
        let mut tasks = Vec::new();
        for n in 0..16 {
            let store = store.clone();
            let key = key.clone();
            let condition = condition.clone();
            tasks.push(tokio::spawn(async move {
                store
                    .put(
                        &key,
                        condition,
                        &upload(format!("unique-{create}-{n}").as_bytes()).await,
                    )
                    .await
                    .unwrap()
            }));
        }
        let mut winners = 0;
        for task in tasks {
            match task.await.unwrap() {
                WriteOutcome::Applied(_) => winners += 1,
                WriteOutcome::PreconditionFailed => {}
                WriteOutcome::Unknown => panic!("unexpected unknown"),
            }
        }
        assert_eq!(winners, 1);
    }
}
#[tokio::test]
async fn unknown_before_and_after_apply_are_distinct_from_conflicts() {
    let store = MemoryStore::default();
    let key = Key::new("repo/manifest").unwrap();
    let data = upload(b"first").await;
    store.inject(Fault::BeforeWrite);
    assert_eq!(
        store.put(&key, Condition::Absent, &data).await.unwrap(),
        WriteOutcome::Unknown
    );
    assert!(matches!(store.head(&key).await, Err(Error::NotFound)));
    store.inject(Fault::AfterWrite);
    assert_eq!(
        store.put(&key, Condition::Absent, &data).await.unwrap(),
        WriteOutcome::Unknown
    );
    let first = store.head(&key).await.unwrap();
    assert_eq!(
        store
            .get(&key, None)
            .await
            .unwrap()
            .read_bounded(100)
            .await
            .unwrap(),
        b"first"
    );
    assert_eq!(
        store.put(&key, Condition::Absent, &data).await.unwrap(),
        WriteOutcome::PreconditionFailed
    );
    let next = applied(
        store
            .put(
                &key,
                Condition::Matches(first.version.clone()),
                &upload(b"second").await,
            )
            .await
            .unwrap(),
    );
    assert_ne!(next, first.version);
    assert_eq!(
        store
            .put(&key, Condition::Matches(first.version), &data)
            .await
            .unwrap(),
        WriteOutcome::PreconditionFailed
    );
}
#[tokio::test]
async fn checked_uploads_and_ranges_enforce_exact_limits() {
    assert!(matches!(
        StagedUpload::from_reader(tokio::io::repeat(1), 65535).await,
        Err(Error::Limit)
    ));
    let blob = StagedUpload::from_reader(tokio::io::repeat(9).take(65536), 65536)
        .await
        .unwrap();
    assert_eq!(blob.len(), 65536);
    let store = MemoryStore::default();
    let key = Key::new("repo/blob").unwrap();
    store.put(&key, Condition::Absent, &blob).await.unwrap();
    assert!(matches!(
        store
            .get(&key, None)
            .await
            .unwrap()
            .read_bounded(65535)
            .await,
        Err(Error::Limit)
    ));
    assert_eq!(
        store
            .get(&key, Some(ByteRange::new(123, 456).unwrap()))
            .await
            .unwrap()
            .read_bounded(333)
            .await
            .unwrap(),
        vec![9; 333]
    );
    assert!(
        store
            .get(&key, Some(ByteRange::new(65535, 65537).unwrap()))
            .await
            .is_err()
    );
    assert!(ByteRange::new(5, 5).is_err());
    for bad in ["", "/absolute", "a/../b", "a//b", "a?query", "a\nheader"] {
        assert!(Key::new(bad).is_err());
    }
}
#[test]
fn storage_credentials_and_endpoint_policy_fail_closed() {
    use jj_store_object::s3::{Config, Credentials, S3Store};
    for (endpoint, allow, valid) in [
        ("http://127.0.0.1:9000", false, false),
        ("http://storage.example.invalid", true, false),
        ("https://storage.example.invalid", false, true),
        ("https://user:secret@storage.example.invalid", false, false),
        ("https://storage.example.invalid/prefix", false, false),
        ("http://127.0.0.1:9000", true, true),
    ] {
        let config = Config {
            endpoint: endpoint.into(),
            bucket: "bucket".into(),
            region: "us-east-1".into(),
            credentials: Credentials {
                access_key: "access".into(),
                secret_key: "secret-token".into(),
                session_token: None,
            },
            allow_http_loopback: allow,
            max_object_bytes: 1048576,
            timeout: std::time::Duration::from_secs(5),
        };
        assert_eq!(S3Store::new(config).is_ok(), valid, "{endpoint}");
    }
}

#[tokio::test]
async fn private_spool_ranges_seek_and_end_at_exact_boundaries() {
    let bytes: Vec<u8> = (0..200_000).map(|n| n as u8).collect();
    let spool = StagedUpload::from_bytes(&bytes, bytes.len() as u64)
        .await
        .unwrap();
    for (start, end) in [(0, 1), (65_535, 65_537), (199_999, 200_000), (0, 200_000)] {
        let mut reader = spool
            .reader_range(ByteRange::new(start, end).unwrap())
            .await
            .unwrap();
        let mut actual = vec![];
        reader.read_to_end(&mut actual).await.unwrap();
        assert_eq!(actual, bytes[start as usize..end as usize]);
        assert_eq!(reader.read(&mut [0; 1]).await.unwrap(), 0);
    }
    assert!(matches!(
        spool
            .reader_range(ByteRange::new(1, 200_001).unwrap())
            .await,
        Err(Error::Range)
    ));
    let empty = StagedUpload::from_bytes(&[], 0).await.unwrap();
    assert!(matches!(
        empty.reader_range(ByteRange::new(0, 1).unwrap()).await,
        Err(Error::Range)
    ));
}

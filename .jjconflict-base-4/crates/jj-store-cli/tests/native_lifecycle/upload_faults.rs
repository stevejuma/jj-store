use super::*;
use std::sync::atomic::{AtomicBool, Ordering};
use tokio::io::{AsyncRead, AsyncWriteExt as _};
struct LostUploadAck {
    inner: Arc<MemoryStore>,
    armed: AtomicBool,
    hide: AtomicBool,
}
#[async_trait::async_trait]
impl jj_store_object::ObjectStore for LostUploadAck {
    async fn head(&self, key: &Key) -> jj_store_object::Result<jj_store_object::Metadata> {
        self.inner.head(key).await
    }
    async fn get(
        &self,
        key: &Key,
        range: Option<ByteRange>,
    ) -> jj_store_object::Result<jj_store_object::Download> {
        if key.as_str().contains("/uploads/v1/") && self.hide.swap(false, Ordering::SeqCst) {
            return Err(jj_store_object::Error::Transport);
        }
        self.inner.get(key, range).await
    }
    async fn put(
        &self,
        key: &Key,
        condition: Condition,
        upload: &StagedUpload,
    ) -> jj_store_object::Result<WriteOutcome> {
        let r = self.inner.put(key, condition, upload).await?;
        if key.as_str().contains("/uploads/v1/") && self.armed.swap(false, Ordering::SeqCst) {
            self.hide.store(true, Ordering::SeqCst);
            return Ok(WriteOutcome::Unknown);
        }
        Ok(r)
    }
}
#[tokio::test(flavor = "multi_thread", worker_threads = 4)]
async fn conflict_upload_lost_ack_repeats_same_record_after_restart_and_stale_binding_rejects() {
    let mut h = History::new().await;
    let (read, _) = complex(&mut h).await;
    let ds = read
        .conflicts(
            &reader(),
            cf::Request {
                commit: h.commit,
                path: "text".into(),
            },
        )
        .await
        .unwrap();
    let expected = h.f.session.snapshot().await.unwrap().manifest().snapshot();
    let binding = binding(&h, expected.clone(), &ds.conflicts[0]);
    let store = Arc::new(LostUploadAck {
        inner: h.f.store.clone(),
        armed: AtomicBool::new(true),
        hide: AtomicBool::new(false),
    });
    let session = scoped_session(&h, store, h.f.token.clone(), SessionLimits::default());
    assert_eq!(
        Uploads::new(Default::default())
            .unwrap()
            .stage(session, binding.clone(), b"lost ack".as_slice(), None)
            .await
            .unwrap_err(),
        uploads::Error::Unavailable
    );
    let first = Uploads::new(Default::default())
        .unwrap()
        .stage(
            h.f.session.clone(),
            binding.clone(),
            b"lost ack".as_slice(),
            None,
        )
        .await
        .unwrap();
    let second = Uploads::new(Default::default())
        .unwrap()
        .stage(
            h.f.session(h.f.token.clone(), Protection::default()),
            binding.clone(),
            b"lost ack".as_slice(),
            None,
        )
        .await
        .unwrap();
    assert_eq!(first, second);
    assert_eq!(
        h.f.session.snapshot().await.unwrap().manifest().snapshot(),
        expected
    );
    let mut stale = binding.clone();
    stale.expected.epoch = m::EpochId::new([89; 16]);
    assert_eq!(
        Uploads::new(Default::default())
            .unwrap()
            .stage(h.f.session.clone(), stale, b"lost ack".as_slice(), None)
            .await
            .unwrap_err(),
        uploads::Error::Stale
    );
}
struct NotifiedReader {
    inner: tokio::io::DuplexStream,
    started: Arc<tokio::sync::Notify>,
}
impl AsyncRead for NotifiedReader {
    fn poll_read(
        mut self: std::pin::Pin<&mut Self>,
        cx: &mut std::task::Context<'_>,
        buf: &mut tokio::io::ReadBuf<'_>,
    ) -> std::task::Poll<std::io::Result<()>> {
        self.started.notify_one();
        std::pin::Pin::new(&mut self.inner).poll_read(cx, buf)
    }
}
#[tokio::test(flavor = "multi_thread", worker_threads = 4)]
async fn conflict_upload_revocation_during_body_and_busy_deadline_reject_before_remote_writes() {
    use jj_store_service::administration::{TokenChange, TokenRequest};
    let mut h = History::new().await;
    let writer = BearerToken::generate().unwrap();
    h.f.session
        .change_token(&TokenRequest {
            request_id: m::RequestId::new([181; 16]),
            expected: h.f.session.snapshot().await.unwrap().manifest().snapshot(),
            change: TokenChange::Grant {
                credential: writer.clone(),
                principal: m::PrincipalId::new([181; 16]),
                permission: Permission::Write,
                expires_at: None,
            },
        })
        .await
        .unwrap();
    let (read, _) = complex(&mut h).await;
    let ds = read
        .conflicts(
            &reader(),
            cf::Request {
                commit: h.commit,
                path: "text".into(),
            },
        )
        .await
        .unwrap();
    let expected = h.f.session.snapshot().await.unwrap().manifest().snapshot();
    let bind = binding(&h, expected, &ds.conflicts[0]);
    let counter = Arc::new(CountWrites {
        inner: h.f.store.clone(),
        writes: Default::default(),
    });
    let session = scoped_session(
        &h,
        counter.clone(),
        writer.clone(),
        SessionLimits::default(),
    );
    let uploader = Uploads::new(uploads::Limits {
        max_file_bytes: 8,
        concurrency: 1,
    })
    .unwrap();
    let started = Arc::new(tokio::sync::Notify::new());
    let (mut writer_stream, reader_stream) = tokio::io::duplex(8);
    let job = tokio::spawn({
        let uploader = uploader.clone();
        let session = session.clone();
        let bind = bind.clone();
        let started = started.clone();
        async move {
            uploader
                .stage(
                    session,
                    bind,
                    NotifiedReader {
                        inner: reader_stream,
                        started,
                    },
                    None,
                )
                .await
        }
    });
    tokio::time::timeout(Duration::from_secs(2), started.notified())
        .await
        .unwrap();
    assert_eq!(
        uploader
            .stage(session, bind.clone(), b"".as_slice(), None)
            .await
            .unwrap_err(),
        uploads::Error::Busy
    );
    h.f.session
        .change_token(&TokenRequest {
            request_id: m::RequestId::new([182; 16]),
            expected: h.f.session.snapshot().await.unwrap().manifest().snapshot(),
            change: TokenChange::Revoke { token: writer.id() },
        })
        .await
        .unwrap();
    writer_stream.write_all(b"data").await.unwrap();
    writer_stream.shutdown().await.unwrap();
    assert_eq!(job.await.unwrap().unwrap_err(), uploads::Error::Denied);
    assert_eq!(counter.writes.load(Ordering::SeqCst), 0);
    let mut fresh = bind;
    fresh.expected = h.f.session.snapshot().await.unwrap().manifest().snapshot();
    let short = scoped_session(
        &h,
        counter.clone(),
        h.f.token.clone(),
        SessionLimits {
            timeout: Duration::from_millis(50),
            ..Default::default()
        },
    );
    let (_hold, pending) = tokio::io::duplex(8);
    assert_eq!(
        uploader
            .stage(short, fresh.clone(), pending, None)
            .await
            .unwrap_err(),
        uploads::Error::Limit
    );
    assert_eq!(counter.writes.load(Ordering::SeqCst), 0);
    assert!(
        uploader
            .stage(h.f.session.clone(), fresh, b"ok".as_slice(), None)
            .await
            .is_ok(),
        "deadline released upload admission"
    );
}

fn unpolled_body() -> Body {
    Body::from_stream(futures::stream::poll_fn(
        |_| -> std::task::Poll<Option<std::io::Result<axum::body::Bytes>>> {
            panic!("body consumed before admission")
        },
    ))
}
#[tokio::test(flavor = "multi_thread", worker_threads = 4)]
async fn conflict_upload_http_bounds_bodies_and_authorizes_before_consumption() {
    let mut h = History::new().await;
    let (read, _) = complex(&mut h).await;
    let ds = read
        .conflicts(
            &reader(),
            cf::Request {
                commit: h.commit,
                path: "text".into(),
            },
        )
        .await
        .unwrap();
    let descriptor = &ds.conflicts[0];
    let expected = h.f.session.snapshot().await.unwrap().manifest().snapshot();
    let cache = tempfile::tempdir().unwrap();
    let router = api(
        &h.f,
        cache.path(),
        jj_store_http::Config {
            uploads: uploads::Limits {
                max_file_bytes: 8,
                concurrency: 1,
            },
            ..Default::default()
        },
        Protection::default(),
    );
    for case in 0..6 {
        let mut req = upload_request(
            if case == 0 { &h.f.reader } else { &h.f.token },
            &expected,
            h.commit,
            descriptor,
            vec![1],
        );
        *req.body_mut() = unpolled_body();
        let expected_status = match case {
            0 => StatusCode::FORBIDDEN,
            1 => {
                let uri = req.uri().to_string().replace(
                    &REPO.to_string(),
                    &m::RepositoryId::new([98; 16]).to_string(),
                );
                *req.uri_mut() = uri.parse().unwrap();
                StatusCode::FORBIDDEN
            }
            2 => {
                let uri = format!("{}&unexpected=true", req.uri());
                *req.uri_mut() = uri.parse().unwrap();
                StatusCode::BAD_REQUEST
            }
            3 => {
                req.headers_mut()
                    .insert("content-type", "application/json".parse().unwrap());
                StatusCode::UNSUPPORTED_MEDIA_TYPE
            }
            4 => {
                req.headers_mut()
                    .insert("content-length", "9".parse().unwrap());
                StatusCode::PAYLOAD_TOO_LARGE
            }
            _ => {
                req.headers_mut()
                    .insert("content-encoding", "gzip".parse().unwrap());
                StatusCode::UNSUPPORTED_MEDIA_TYPE
            }
        };
        assert_eq!(send(&router, req).await.0, expected_status, "case {case}");
    }
    let mut overflow = upload_request(&h.f.token, &expected, h.commit, descriptor, vec![1; 9]);
    overflow.headers_mut().remove("content-length");
    assert_eq!(
        send(&router, overflow).await.0,
        StatusCode::PAYLOAD_TOO_LARGE
    );
    let mut short = upload_request(&h.f.token, &expected, h.commit, descriptor, vec![1]);
    short
        .headers_mut()
        .insert("content-length", "3".parse().unwrap());
    assert_eq!(send(&router, short).await.0, StatusCode::BAD_REQUEST);
    let mut broken = upload_request(&h.f.token, &expected, h.commit, descriptor, vec![]);
    broken.headers_mut().remove("content-length");
    *broken.body_mut() = Body::from_stream(futures::stream::iter([
        Ok(axum::body::Bytes::from_static(b"part")),
        Err(std::io::Error::other("truncated")),
    ]));
    assert_eq!(
        send(&router, broken).await.0,
        StatusCode::SERVICE_UNAVAILABLE
    );
    assert_eq!(
        h.f.session.snapshot().await.unwrap().manifest().snapshot(),
        expected
    );
    assert_eq!(
        send(
            &router,
            upload_request(&h.f.token, &expected, h.commit, descriptor, vec![0; 8])
        )
        .await
        .0,
        StatusCode::OK
    );
    let limited = api(
        &h.f,
        cache.path(),
        jj_store_http::Config {
            max_json_bytes: 64,
            ..Default::default()
        },
        Protection::default(),
    );
    assert_eq!(
        send(
            &limited,
            upload_request(&h.f.token, &expected, h.commit, descriptor, vec![0; 8])
        )
        .await
        .0,
        StatusCode::PAYLOAD_TOO_LARGE
    );
    assert_eq!(
        send(
            &router,
            upload_request(&h.f.token, &expected, h.commit, descriptor, vec![0; 8])
        )
        .await
        .0,
        StatusCode::OK
    );
    assert_eq!(
        h.f.session.snapshot().await.unwrap().manifest().snapshot(),
        expected
    );
}

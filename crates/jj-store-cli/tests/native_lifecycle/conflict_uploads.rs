use super::*;
use crate::http_mutation_acceptance::{api, send};
use axum::{
    body::Body,
    http::{Request as HttpRequest, StatusCode},
};
fn upload_request(
    token: &BearerToken,
    expected: &Snapshot,
    commit: m::CommitId,
    descriptor: &cf::Descriptor,
    bytes: Vec<u8>,
) -> HttpRequest<Body> {
    let query = url::form_urlencoded::Serializer::new(String::new())
        .append_pair("epoch", &expected.epoch.to_string())
        .append_pair("position", &expected.position.0.to_string())
        .append_pair("operation", &expected.heads.first().unwrap().to_string())
        .append_pair("commit", &commit.to_string())
        .append_pair("path", &descriptor.path)
        .append_pair("fingerprint", &descriptor.fingerprint.to_string())
        .finish();
    HttpRequest::builder()
        .method("POST")
        .uri(format!("/v1/repos/{REPO}/conflict-uploads?{query}"))
        .header(
            "authorization",
            format!("Bearer {}", token.expose().as_str()),
        )
        .header("content-type", "application/octet-stream")
        .header("content-length", bytes.len())
        .body(Body::from(bytes))
        .unwrap()
}
#[tokio::test(flavor = "multi_thread", worker_threads = 4)]
async fn conflict_upload_http_stages_verified_binary_without_publication_then_resolves() {
    tokio::task::LocalSet::new().run_until(async {
        let mut h=History::new().await;
        let (read,_)=complex(&mut h).await;
        let descriptors=read.conflicts(&reader(),cf::Request{commit:h.commit,path:String::new()}).await.unwrap();
        let descriptor=descriptors.conflicts.iter().find(|d|d.path=="binary").unwrap();
        let expected=h.f.session.snapshot().await.unwrap().manifest().snapshot();
        let cache=tempfile::tempdir().unwrap();
        let router=api(&h.f,cache.path(),jj_store_http::Config::default(),Protection::default());
        let bytes:Vec<u8>=(0..600_000).map(|n|n as u8).collect();
        let (status,upload)=send(&router,upload_request(&h.f.token,&expected,h.commit,descriptor,bytes.clone())).await;
        assert_eq!(status,StatusCode::OK,"{upload}");
        assert_eq!(upload["bytes"],bytes.len());
        assert_eq!(h.f.session.snapshot().await.unwrap().manifest().snapshot(),expected);
        let native:m::FileId=upload["file"].as_str().unwrap().parse().unwrap();
        let raw=h.f.objects.get_file(native).await.unwrap();
        let mut checked=vec![];
        tokio::io::AsyncReadExt::read_to_end(&mut raw.reader().unwrap(),&mut checked).await.unwrap();
        assert_eq!(checked,bytes);
        let request=input(expected.clone(),h.commit,json!([{"path":descriptor.path,"fingerprint":descriptor.fingerprint,"resolution":{"type":"uploaded_file","upload":upload["id"],"executable":true,"copy_from_term":null}}]));
        let body=serde_json::to_value(&request).unwrap();
        let (status,preview)=send(&router,crate::http_mutation_acceptance::request(&h.f.token,"mutations/preview",Some(&body))).await;
        assert_eq!(status,StatusCode::OK,"{preview}");
        assert_eq!(preview["status"],"prepared");
        assert_eq!(preview["preview"]["result"]["conflict_resolution"]["selected_paths"],json!(["binary"]));
        assert_eq!(h.f.session.snapshot().await.unwrap().manifest().snapshot(),expected);
        let (status,accepted)=send(&router,crate::http_mutation_acceptance::request(&h.f.token,"mutations",Some(&body))).await;
        assert_eq!(status,StatusCode::OK,"{accepted}");
        let Outcome::Committed{receipt}=service().apply(h.f.session.clone(),&request).await.unwrap() else {panic!("receipt")};
        let report=receipt.result.conflict_resolution.as_ref().unwrap();
        assert_eq!(report.target.before,h.commit);
        assert_eq!(report.selected_paths,["binary"]);
        assert_eq!(report.remaining.len(),1);
        assert_eq!(report.remaining[&report.target.after].len(),7);
        assert_eq!(accepted["receipt"]["result"]["conflict_resolution"],json!(report));
        let new=h.native_store().get_commit_async(&b::CommitId::from_bytes(receipt.result.rewrites[0].commit.as_bytes())).await.unwrap();
        assert_eq!(new.tree().path_value(&RepoPathBuf::from_internal_string("binary").unwrap()).await.unwrap(),Merge::resolved(Some(b::TreeValue::File{id:b::FileId::from_bytes(native.as_bytes()),executable:true,copy_id:b::CopyId::placeholder()})));
        assert_eq!(service().apply(h.f.session.clone(),&request).await.unwrap(),Outcome::Committed{receipt:receipt.clone()});
        if let Some(path)=std::env::var_os("JJ_STORE_UPLOAD_CONTRACT_CAPTURE") {
            let samples=[json!({"path":"/v1/repos/{repo}/conflict-uploads","method":"post","status":200,"media":"application/json","data":upload}),json!({"path":"/v1/repos/{repo}/mutations/preview","method":"post","status":200,"media":"application/json","data":preview}),json!({"path":"/v1/repos/{repo}/mutations","method":"post","status":200,"media":"application/json","data":accepted})];
            std::fs::write(path,serde_json::to_vec(&samples).unwrap()).unwrap();
        }
    }).await;
}

use jj_store_service::uploads::{self as uploads, Binding, Uploads};
fn binding(h: &History, expected: Snapshot, descriptor: &cf::Descriptor) -> Binding {
    Binding {
        expected,
        commit: h.commit,
        path: descriptor.path.clone(),
        fingerprint: descriptor.fingerprint,
    }
}
fn uploaded(
    descriptor: &cf::Descriptor,
    id: m::ConflictUploadId,
    copy: Option<u32>,
) -> serde_json::Value {
    json!({"path":descriptor.path,"fingerprint":descriptor.fingerprint,"resolution":{"type":"uploaded_file","upload":id,"executable":false,"copy_from_term":copy}})
}
fn scoped_session(
    _h: &History,
    store: Arc<dyn jj_store_object::ObjectStore>,
    token: BearerToken,
    limits: SessionLimits,
) -> Arc<NativeSession> {
    Arc::new(
        NativeSession::new(
            store.clone(),
            Arc::new(NativeObjects::new(store, REPO, 32 * 1024 * 1024).unwrap()),
            token,
            Arc::new(SystemClock),
            Protection::default(),
            limits,
        )
        .unwrap(),
    )
}
fn record_path(binding: &Binding, principal: m::PrincipalId, id: m::ConflictUploadId) -> Key {
    Key::new(format!(
        "repos/{REPO}/uploads/v1/{}/{principal}/{id}",
        binding.expected.epoch
    ))
    .unwrap()
}
#[tokio::test(flavor = "multi_thread", worker_threads = 4)]
async fn conflict_upload_reuse_checks_principal_snapshot_path_and_verified_bytes() {
    use jj_store_service::{
        administration::{TokenChange, TokenRequest},
        commands::Error as C,
        mutations::Error as E,
    };
    tokio::task::LocalSet::new()
        .run_until(async {
            let mut h = History::new().await;
            let writer = BearerToken::generate().unwrap();
            h.f.session
                .change_token(&TokenRequest {
                    request_id: m::RequestId::new([171; 16]),
                    expected: h.f.session.snapshot().await.unwrap().manifest().snapshot(),
                    change: TokenChange::Grant {
                        credential: writer.clone(),
                        principal: m::PrincipalId::new([171; 16]),
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
                        path: String::new(),
                    },
                )
                .await
                .unwrap();
            let descriptor = ds.conflicts.iter().find(|d| d.path == "text").unwrap();
            let other = ds.conflicts.iter().find(|d| d.path == "copy").unwrap();
            let expected = h.f.session.snapshot().await.unwrap().manifest().snapshot();
            let binding = binding(&h, expected.clone(), descriptor);
            let uploader = Uploads::new(Default::default()).unwrap();
            let upload = uploader
                .stage(
                    h.f.session.clone(),
                    binding.clone(),
                    b"replacement".as_slice(),
                    None,
                )
                .await
                .unwrap();
            // Restarted upload service reconstructs the same durable scoped identity.
            assert_eq!(
                Uploads::new(Default::default())
                    .unwrap()
                    .stage(
                        h.f.session(h.f.token.clone(), Protection::default()),
                        binding.clone(),
                        b"replacement".as_slice(),
                        None
                    )
                    .await
                    .unwrap(),
                upload
            );
            let valid = input(
                expected.clone(),
                h.commit,
                json!([uploaded(descriptor, upload.id, None)]),
            );
            let foreign = h.f.session(writer, Protection::default());
            assert_eq!(
                service().preview(foreign, &valid).await.unwrap_err(),
                E::Preparation(C::InvalidUpload)
            );
            let counter = Arc::new(CountWrites {
                inner: h.f.store.clone(),
                writes: Default::default(),
            });
            let session = scoped_session(
                &h,
                counter.clone(),
                h.f.token.clone(),
                SessionLimits::default(),
            );
            for selection in [
                uploaded(other, upload.id, None),
                uploaded(descriptor, m::ConflictUploadId::new([33; 32]), None),
                uploaded(descriptor, upload.id, Some(99)),
            ] {
                let request = input(expected.clone(), h.commit, json!([selection]));
                assert!(service().preview(session.clone(), &request).await.is_err());
            }
            assert_eq!(counter.writes.load(std::sync::atomic::Ordering::SeqCst), 0);
            // An upload handle never adds its content to an ordinary read capability.
            assert_eq!(
                read.object(&reader(), m::ObjectKey::File(upload.file))
                    .await
                    .unwrap_err(),
                q::Error::NotCommitted
            );
            let path = record_path(&binding, ACTOR, upload.id);
            let stored = h.f.store.get(&path, None).await.unwrap();
            let version = stored.metadata.version.clone();
            let original = stored.read_bounded(16 * 1024).await.unwrap();
            let mut corrupt = original.clone();
            *corrupt.last_mut().unwrap() ^= 1;
            h.f.store
                .put(
                    &path,
                    Condition::Matches(version),
                    &StagedUpload::from_bytes(&corrupt, 16 * 1024).await.unwrap(),
                )
                .await
                .unwrap();
            assert_eq!(
                service()
                    .preview(session.clone(), &valid)
                    .await
                    .unwrap_err(),
                E::Preparation(C::Query(q::Error::Corrupt))
            );
            let version = h.f.store.head(&path).await.unwrap().version;
            h.f.store
                .put(
                    &path,
                    Condition::Matches(version),
                    &StagedUpload::from_bytes(&original, 16 * 1024)
                        .await
                        .unwrap(),
                )
                .await
                .unwrap();
            let file_path = Key::new(m::ObjectKey::File(upload.file).storage_path(REPO)).unwrap();
            let missing = Arc::new(MissingNative {
                inner: h.f.store.clone(),
                missing: file_path.clone(),
            });
            assert!(
                service()
                    .preview(
                        scoped_session(&h, missing, h.f.token.clone(), SessionLimits::default()),
                        &valid
                    )
                    .await
                    .is_err()
            );
            let stored = h.f.store.get(&file_path, None).await.unwrap();
            let version = stored.metadata.version.clone();
            let mut data = stored.read_bounded(1024 * 1024).await.unwrap();
            *data.last_mut().unwrap() ^= 1;
            h.f.store
                .put(
                    &file_path,
                    Condition::Matches(version),
                    &StagedUpload::from_bytes(&data, 1024 * 1024).await.unwrap(),
                )
                .await
                .unwrap();
            assert!(service().preview(session, &valid).await.is_err());
            assert_eq!(counter.writes.load(std::sync::atomic::Ordering::SeqCst), 0);
            assert_eq!(
                h.f.session.snapshot().await.unwrap().manifest().snapshot(),
                expected
            );
        })
        .await;
}

#[tokio::test(flavor = "multi_thread", worker_threads = 4)]
async fn conflict_upload_atomic_mixed_resolution_retains_copy_history_and_other_conflicts() {
    tokio::task::LocalSet::new()
        .run_until(async {
            let mut h = History::new().await;
            let (read, _) = complex(&mut h).await;
            let ds = read
                .conflicts(
                    &reader(),
                    cf::Request {
                        commit: h.commit,
                        path: String::new(),
                    },
                )
                .await
                .unwrap();
            let find = |name: &str| ds.conflicts.iter().find(|d| d.path == name).unwrap();
            let expected = h.f.session.snapshot().await.unwrap().manifest().snapshot();
            let uploader = Uploads::new(Default::default()).unwrap();
            let content = uploader
                .stage(
                    h.f.session.clone(),
                    binding(&h, expected.clone(), find("copy")),
                    b"".as_slice(),
                    Some(0),
                )
                .await
                .unwrap();
            let structure = uploader
                .stage(
                    h.f.session.clone(),
                    binding(&h, expected.clone(), find("structural")),
                    b"plain replacement\n".as_slice(),
                    None,
                )
                .await
                .unwrap();
            let request = input(
                expected.clone(),
                h.commit,
                json!([
                    uploaded(find("copy"), content.id, Some(2)),
                    uploaded(find("structural"), structure.id, None),
                    selection(find("text"), 2)
                ]),
            );
            let mut bad = serde_json::to_value(&request).unwrap();
            bad["command"]["paths"][2]["fingerprint"] = json!(m::ConflictFingerprint::new([7; 64]));
            assert_eq!(
                service()
                    .apply(h.f.session.clone(), &serde_json::from_value(bad).unwrap())
                    .await
                    .unwrap_err(),
                jj_store_service::mutations::Error::Preparation(
                    jj_store_service::commands::Error::StaleConflict
                )
            );
            assert_eq!(
                h.f.session.snapshot().await.unwrap().manifest().snapshot(),
                expected
            );
            let Outcome::Committed { receipt } = service()
                .apply(h.f.session.clone(), &request)
                .await
                .unwrap()
            else {
                panic!("receipt")
            };
            let new = h
                .native_store()
                .get_commit_async(&b::CommitId::from_bytes(
                    receipt.result.rewrites[0].commit.as_bytes(),
                ))
                .await
                .unwrap();
            let value = new
                .tree()
                .path_value(&RepoPathBuf::from_internal_string("copy").unwrap())
                .await
                .unwrap();
            let dto::TreeValue::File { copy, .. } = find("copy").terms[2].value.as_ref().unwrap()
            else {
                panic!("file")
            };
            assert_eq!(
                value
                    .as_resolved()
                    .unwrap()
                    .as_ref()
                    .map(|v| dto::TreeValue::try_from(v).unwrap()),
                Some(dto::TreeValue::File {
                    id: content.file,
                    executable: false,
                    copy: *copy
                })
            );
            assert_eq!(
                new.tree()
                    .path_value(&RepoPathBuf::from_internal_string("structural").unwrap())
                    .await
                    .unwrap(),
                Merge::resolved(Some(b::TreeValue::File {
                    id: b::FileId::from_bytes(structure.file.as_bytes()),
                    executable: false,
                    copy_id: b::CopyId::placeholder()
                }))
            );
            assert_eq!(new.tree().conflicts().count(), 5);
            let report = receipt.result.conflict_resolution.as_ref().unwrap();
            assert_eq!(report.target.after, cid(new.id()));
            assert_eq!(report.selected_paths, ["copy", "structural", "text"]);
            assert_eq!(
                report.remaining[&report.target.after],
                new.tree()
                    .conflicts()
                    .map(|(p, v)| {
                        v.unwrap();
                        p.as_internal_file_string().to_owned()
                    })
                    .collect::<Vec<_>>()
            );
            assert_eq!(
                uploader
                    .stage(
                        h.f.session.clone(),
                        content.binding.clone(),
                        b"".as_slice(),
                        None
                    )
                    .await
                    .unwrap_err(),
                uploads::Error::Stale
            );
            // Selected receipts no longer depend on a staging record, even after a
            // checkpoint and a service restart. The published file is in native inventory.
            struct NoPublication;
            #[async_trait::async_trait]
            impl PublicationGuard for NoPublication {
                async fn authorize(&self, _: &ManifestSnapshot, _: &Request) -> wal::Result<()> {
                    Err(wal::Error::Denied)
                }
                async fn validate_transition(
                    &self,
                    _: &ManifestSnapshot,
                    _: &Request,
                    _: &Snapshot,
                ) -> wal::Result<()> {
                    Err(wal::Error::Denied)
                }
            }
            Coordinator::new(
                h.f.store.clone(),
                REPO,
                h.f.objects.clone(),
                Arc::new(NoPublication),
                Limits::default(),
            )
            .unwrap()
            .checkpoint()
            .await
            .unwrap();
            let missing = Arc::new(MissingNative {
                inner: h.f.store.clone(),
                missing: record_path(&content.binding, ACTOR, content.id),
            });
            let restarted =
                scoped_session(&h, missing, h.f.token.clone(), SessionLimits::default());
            assert_eq!(
                service().apply(restarted.clone(), &request).await.unwrap(),
                Outcome::Committed {
                    receipt: receipt.clone()
                }
            );
            assert_eq!(
                service()
                    .status(restarted, request.request_id)
                    .await
                    .unwrap(),
                jj_store_service::mutations::Status::Committed { receipt }
            );
        })
        .await;
}

#[path = "upload_faults.rs"]
mod faults;

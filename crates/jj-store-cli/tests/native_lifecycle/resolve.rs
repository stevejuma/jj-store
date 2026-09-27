use super::*;
use jj_store_service::{
    commands::{Executor, Limits as CommandLimits},
    mutations::{CommandService, Outcome, Preview, Request as MutationRequest},
};
use serde_json::json;

fn service() -> CommandService {
    CommandService::new(Executor::new(CommandLimits::default(), 2).unwrap())
}
fn input(expected: Snapshot, commit: m::CommitId, paths: serde_json::Value) -> MutationRequest {
    serde_json::from_value(json!({
        "version":1, "request_id":m::RequestId::new([161;16]), "expected":expected,
        "command":{"type":"resolve_conflicts", "commit":commit,"paths":paths}
    }))
    .expect("typed conflict resolution must be accepted")
}
fn selection(descriptor: &cf::Descriptor, term: u32) -> serde_json::Value {
    // Public consumers can construct the intent entirely through service-owned
    // exports without adding a direct dependency on the native integration crate.
    use jj_store_service::commands::{ConflictResolution, Resolution};
    json!(ConflictResolution {
        path: descriptor.path.clone(),
        fingerprint: descriptor.fingerprint,
        resolution: Resolution::SelectTerm { term },
    })
}

#[tokio::test(flavor = "multi_thread", worker_threads = 4)]
async fn typed_resolve_native_terms_are_atomic_preserve_other_conflicts_and_retry_receipts() {
    tokio::task::LocalSet::new().run_until(async {
        let mut h = History::new().await;
        let (read, _) = complex(&mut h).await;
        let response = read.conflicts(&reader(), cf::Request {commit:h.commit,path:String::new()}).await.unwrap();
        let by_path = |path: &str| response.conflicts.iter().find(|d| d.path == path).unwrap();
        let expected = h.f.session.snapshot().await.unwrap().manifest().snapshot();
        let request = input(expected.clone(), h.commit, json!([
            selection(by_path("text"), 2), selection(by_path("binary"), 0),
            selection(by_path("mode"), 0), selection(by_path("copy"), 2),
            selection(by_path("submodule"), 0), selection(by_path("structural"), 2),
            {"path":"delete-modify", "fingerprint":by_path("delete-modify").fingerprint,"resolution":{"type":"delete"}}
        ]));
        let commands = service();
        let Preview::Prepared { preview } = commands.preview(h.f.session.clone(), &request).await.unwrap() else {panic!("preview")};
        assert!(preview.result.rewrites.iter().any(|r| r.predecessors == [h.commit]));
        assert_eq!(h.f.session.snapshot().await.unwrap().manifest().snapshot(), expected);
        let Outcome::Committed { receipt } = commands.apply(h.f.session.clone(), &request).await.unwrap() else {panic!("receipt")};
        let rewrite = receipt.result.rewrites.iter().find(|r| r.predecessors == [h.commit]).unwrap();
        assert_eq!(rewrite.change, response.change);
        let new = h.native_store().get_commit_async(&b::CommitId::from_bytes(rewrite.commit.as_bytes())).await.unwrap();
        for (path, index) in [("text",2),("binary",0),("mode",0),("copy",2),("submodule",0),("structural",2)] {
            let value = new.tree().path_value(&RepoPathBuf::from_internal_string(path).unwrap()).await.unwrap();
            assert_eq!(value.as_resolved().unwrap().as_ref().map(|v| dto::TreeValue::try_from(v).unwrap()),by_path(path).terms[index].value);
        }
        assert!(new.tree().path_value(&RepoPathBuf::from_internal_string("delete-modify").unwrap()).await.unwrap().is_absent());
        let remaining = new.tree().conflicts().collect::<Vec<_>>();
        assert_eq!(remaining.len(),1);
        assert_eq!(remaining[0].0.as_internal_file_string(),"dir/x");
        assert_eq!(remaining[0].1.as_ref().unwrap().iter().map(|v|v.as_ref().map(|v|dto::TreeValue::try_from(v).unwrap())).collect::<Vec<_>>(), by_path("dir/x").terms.iter().map(|t|t.value.clone()).collect::<Vec<_>>());
        assert_eq!(new.tree().labels_by_term("").iter().cloned().collect::<Vec<_>>(), vec!["left","base","right"]);
        assert_eq!(receipt.accepted.position.0,expected.position.0+1);
        let reopened = h.f.session(h.f.token.clone(),Protection::default());
        assert_eq!(service().apply(reopened.clone(), &request).await.unwrap(), Outcome::Committed {receipt:receipt.clone()});
        assert_eq!(service().status(reopened,request.request_id).await.unwrap(),jj_store_service::mutations::Status::Committed {receipt});
    }).await;
}

#[tokio::test(flavor = "multi_thread", worker_threads = 4)]
async fn typed_resolve_rejects_the_whole_stale_or_invalid_batch_before_staging() {
    use jj_store_service::{commands::Error as C, mutations::Error as E};
    tokio::task::LocalSet::new()
        .run_until(async {
            let mut h = History::new().await;
            let (read, _) = complex(&mut h).await;
            let response = read
                .conflicts(
                    &reader(),
                    cf::Request {
                        commit: h.commit,
                        path: String::new(),
                    },
                )
                .await
                .unwrap();
            let by_path = |path: &str| response.conflicts.iter().find(|d| d.path == path).unwrap();
            let expected = h.f.session.snapshot().await.unwrap().manifest().snapshot();
            let valid = selection(by_path("binary"), 0);
            let mut stale = selection(by_path("text"), 2);
            stale["fingerprint"] = json!(m::ConflictFingerprint::new([77; 64]));
            let mut non_conflict = valid.clone();
            non_conflict["path"] = json!("trivial");
            let mut absent = valid.clone();
            absent["path"] = json!("missing");
            let mut term = valid.clone();
            term["resolution"]["term"] = json!(9);
            let mut root = valid.clone();
            root["path"] = json!("");
            let mut escape = valid.clone();
            escape["path"] = json!("../binary");
            let mut overlap = valid.clone();
            overlap["path"] = json!("binary/child");
            let counter = Arc::new(CountWrites {
                inner: h.f.store.clone(),
                writes: Default::default(),
            });
            let session = Arc::new(
                NativeSession::new(
                    counter.clone(),
                    Arc::new(NativeObjects::new(counter.clone(), REPO, 4 * 1024 * 1024).unwrap()),
                    h.f.token.clone(),
                    Arc::new(SystemClock),
                    Protection::default(),
                    SessionLimits::default(),
                )
                .unwrap(),
            );
            for (paths, error) in [
                (json!([valid, stale]), C::StaleConflict),
                (json!([valid, non_conflict]), C::StaleConflict),
                (json!([valid, absent]), C::StaleConflict),
                (json!([term]), C::Invalid),
                (json!([valid, root]), C::Invalid),
                (json!([valid, escape]), C::Invalid),
                (json!([valid, overlap]), C::Invalid),
                (json!([valid, valid]), C::Invalid),
                (json!([]), C::Invalid),
                (json!(vec![valid.clone(); 257]), C::Invalid),
            ] {
                let request = input(expected.clone(), h.commit, paths);
                assert_eq!(
                    service()
                        .preview(session.clone(), &request)
                        .await
                        .unwrap_err(),
                    E::Preparation(error.clone())
                );
                assert_eq!(
                    service()
                        .apply(session.clone(), &request)
                        .await
                        .unwrap_err(),
                    E::Preparation(error)
                );
                assert_eq!(
                    h.f.session.snapshot().await.unwrap().manifest().snapshot(),
                    expected
                );
            }
            assert_eq!(counter.writes.load(std::sync::atomic::Ordering::SeqCst), 0);
            for path in ["/absolute", "a//b", "a/", "a/./b", "a/../b", "a\0b"] {
                let mut selected = valid.clone();
                selected["path"] = json!(path);
                assert!(
                    service()
                        .preview(
                            session.clone(),
                            &input(expected.clone(), h.commit, json!([selected]))
                        )
                        .await
                        .is_err(),
                    "{path:?}"
                );
            }
            assert_eq!(counter.writes.load(std::sync::atomic::Ordering::SeqCst), 0);
            let mut request = input(expected.clone(), h.commit, json!([valid]));
            request.expected.position.0 += 1;
            assert_eq!(
                service()
                    .apply(session.clone(), &request)
                    .await
                    .unwrap_err(),
                E::Stale
            );
            request = input(expected.clone(), roots::ROOT_COMMIT, json!([valid]));
            assert_eq!(
                service()
                    .apply(session.clone(), &request)
                    .await
                    .unwrap_err(),
                E::Preparation(C::Root)
            );
            let restricted = CommandService::new(
                Executor::new(
                    CommandLimits {
                        query: q::Limits {
                            max_work: 1,
                            ..Default::default()
                        },
                        ..Default::default()
                    },
                    1,
                )
                .unwrap(),
            );
            assert_eq!(
                restricted
                    .preview(session, &input(expected.clone(), h.commit, json!([valid])))
                    .await
                    .unwrap_err(),
                E::Limit
            );
            assert_eq!(
                h.f.session.snapshot().await.unwrap().manifest().snapshot(),
                expected
            );
        })
        .await;
}

struct CountWrites {
    inner: Arc<MemoryStore>,
    writes: std::sync::atomic::AtomicUsize,
}
#[async_trait::async_trait]
impl jj_store_object::ObjectStore for CountWrites {
    async fn head(&self, key: &Key) -> jj_store_object::Result<jj_store_object::Metadata> {
        self.inner.head(key).await
    }
    async fn get(
        &self,
        key: &Key,
        range: Option<ByteRange>,
    ) -> jj_store_object::Result<jj_store_object::Download> {
        self.inner.get(key, range).await
    }
    async fn put(
        &self,
        key: &Key,
        condition: Condition,
        upload: &StagedUpload,
    ) -> jj_store_object::Result<WriteOutcome> {
        self.writes
            .fetch_add(1, std::sync::atomic::Ordering::SeqCst);
        self.inner.put(key, condition, upload).await
    }
}

#[tokio::test(flavor = "multi_thread", worker_threads = 4)]
async fn typed_resolve_accepts_multisided_base_absent_and_directory_terms() {
    tokio::task::LocalSet::new()
        .run_until(async {
            for (labels, term) in [(true, 4), (true, 1), (false, 2)] {
                let mut h = History::new().await;
                let mut trees = vec![];
                let mut files = vec![];
                for bytes in [b"left".as_slice(), b"base", b"right", b"base", b"third"] {
                    let file = h.file(bytes).await;
                    files.push(file.clone());
                    trees.push(h.tree(vec![("file", file)]).await);
                }
                let read = h
                    .publish(
                        trees,
                        if labels {
                            vec!["left", "base", "right", "base", "third"]
                        } else {
                            vec![""; 5]
                        },
                    )
                    .await;
                let response = read
                    .conflicts(
                        &reader(),
                        cf::Request {
                            commit: h.commit,
                            path: String::new(),
                        },
                    )
                    .await
                    .unwrap();
                let descriptor = &response.conflicts[0];
                assert_eq!(descriptor.terms.len(), 5);
                assert_eq!(descriptor.terms[0].label.is_some(), labels);
                let request = input(
                    h.f.session.snapshot().await.unwrap().manifest().snapshot(),
                    h.commit,
                    json!([selection(descriptor, term)]),
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
                assert!(!new.tree().has_conflict());
                assert_eq!(
                    new.tree()
                        .path_value(&RepoPathBuf::from_internal_string("file").unwrap())
                        .await
                        .unwrap(),
                    Merge::resolved(Some(files[term as usize].clone()))
                );
            }
            let mut h = History::new().await;
            let (read, _) = complex(&mut h).await;
            let response = read
                .conflicts(
                    &reader(),
                    cf::Request {
                        commit: h.commit,
                        path: String::new(),
                    },
                )
                .await
                .unwrap();
            let by_path = |path: &str| response.conflicts.iter().find(|d| d.path == path).unwrap();
            let request = input(
                h.f.session.snapshot().await.unwrap().manifest().snapshot(),
                h.commit,
                json!([
                    selection(by_path("structural"), 0),
                    selection(by_path("delete-modify"), 0)
                ]),
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
            assert!(
                new.tree()
                    .path_value(&RepoPathBuf::from_internal_string("delete-modify").unwrap())
                    .await
                    .unwrap()
                    .is_absent()
            );
            assert_eq!(
                new.tree()
                    .path_value(&RepoPathBuf::from_internal_string("structural").unwrap())
                    .await
                    .unwrap()
                    .as_resolved()
                    .unwrap()
                    .as_ref()
                    .map(|v| dto::TreeValue::try_from(v).unwrap()),
                by_path("structural").terms[0].value
            );
            assert!(
                new.tree()
                    .path_value(&RepoPathBuf::from_internal_string("structural/x").unwrap())
                    .await
                    .unwrap()
                    .is_resolved()
            );
        })
        .await;
}

#[tokio::test(flavor = "multi_thread", worker_threads = 4)]
async fn typed_resolve_rebases_descendants_updates_references_and_preserves_evolution() {
    use jj_lib::merged_tree_builder::MergedTreeBuilder;
    tokio::task::LocalSet::new().run_until(async {
        let mut h = History::new().await;
        let (read,_) = complex(&mut h).await;
        let response = read.conflicts(&reader(),cf::Request {commit:h.commit,path:String::new()}).await.unwrap();
        let descriptor = response.conflicts.iter().find(|d|d.path=="text").unwrap();
        let cache = tempfile::tempdir().unwrap();
        let loader = h.f.stores.initialize_cache(&settings(),cache.path()).await.unwrap();
        let before = h.f.stores.load_at(&loader,h.operation).await.unwrap();
        let old = before.store().get_commit(&b::CommitId::from_bytes(h.commit.as_bytes())).unwrap();
        let extra = h.file(b"child data\n").await;
        let mut tree = MergedTreeBuilder::new(old.tree());
        tree.set_or_remove(RepoPathBuf::from_internal_string("extra").unwrap(),Merge::resolved(Some(extra.clone())));
        let mut tx = before.start_transaction();
        let child = tx.repo_mut().new_commit(vec![old.id().clone()],tree.write_tree().await.unwrap()).set_description("child").write().await.unwrap();
        tx.repo_mut().set_local_bookmark_target("target".as_ref(),o::RefTarget::normal(old.id().clone()));
        tx.repo_mut().set_local_bookmark_target("child".as_ref(),o::RefTarget::normal(child.id().clone()));
        for name in ["target-a","target-b"] {tx.repo_mut().set_wc_commit(name.into(),old.id().clone()).unwrap();}
        tx.repo_mut().set_wc_commit("child".into(),child.id().clone()).unwrap();
        tx.commit("descendant fixture").await.unwrap();
        let expected = h.f.session.snapshot().await.unwrap().manifest().snapshot();
        let request = input(expected.clone(),h.commit,json!([selection(descriptor,2)]));
        let Outcome::Committed {receipt} = service().apply(h.f.session.clone(),&request).await.unwrap() else {panic!("receipt")};
        let after = h.f.stores.load_at(&loader,receipt.result.operation).await.unwrap();
        let rewritten = after.store().get_commit(after.view().get_local_bookmark("target".as_ref()).as_normal().unwrap()).unwrap();
        let descendant = after.store().get_commit(after.view().get_local_bookmark("child".as_ref()).as_normal().unwrap()).unwrap();
        assert_eq!(rewritten.change_id(),old.change_id());
        assert_eq!(descendant.change_id(),child.change_id());
        assert_eq!(descendant.parent_ids(),&[rewritten.id().clone()]);
        assert_eq!(descendant.tree().path_value(&RepoPathBuf::from_internal_string("text").unwrap()).await.unwrap(),rewritten.tree().path_value(&RepoPathBuf::from_internal_string("text").unwrap()).await.unwrap());
        assert_eq!(descendant.tree().path_value(&RepoPathBuf::from_internal_string("extra").unwrap()).await.unwrap(),Merge::resolved(Some(extra)));
        for name in ["target-a","target-b"] {assert_eq!(after.view().get_wc_commit_id(name.as_ref()),Some(rewritten.id()));}
        assert_eq!(after.view().get_wc_commit_id("child".as_ref()),Some(descendant.id()));
        assert_eq!(receipt.result.rewrites.len(),2);
        for (new,old) in [(&rewritten,&old),(&descendant,&child)] {
            assert!(receipt.result.rewrites.iter().any(|r|r.commit==cid(new.id()) && r.predecessors==[cid(old.id())]));
            assert_eq!(after.operation().store_operation().commit_predecessors.as_ref().unwrap()[new.id()],vec![old.id().clone()]);
            assert_eq!(after.store().get_commit(old.id()).unwrap().tree_ids(),old.tree_ids());
        }
        assert_eq!(receipt.accepted.position.0,expected.position.0+1);
        let protected = h.f.session(h.f.token.clone(),Protection {commits:BTreeSet::from([cid(descendant.id())]),..Default::default()});
        let OpenRead::Pinned(read) = h.f.session.open_read(None,ReadLimits::default()).await.unwrap() else {panic!("read")};
        let response = Arc::new(read).conflicts(&reader(),cf::Request {commit:cid(rewritten.id()),path:"mode".into()}).await.unwrap();
        let mut forbidden = input(receipt.accepted.clone(),cid(rewritten.id()),json!([selection(&response.conflicts[0],0)]));
        forbidden.request_id=m::RequestId::new([162;16]);
        for preview in [true,false] {
            let error = if preview {service().preview(protected.clone(),&forbidden).await.unwrap_err()} else {service().apply(protected.clone(),&forbidden).await.unwrap_err()};
            assert_eq!(error,jj_store_service::mutations::Error::Denied);
        }
        assert_eq!(h.f.session.snapshot().await.unwrap().manifest().snapshot(),receipt.accepted);
        let mut changed = request.clone();
        changed.command = serde_json::from_value(json!({"type":"resolve_conflicts","commit":h.commit,"paths":[selection(descriptor,0)]})).unwrap();
        assert_eq!(service().apply(h.f.session.clone(),&changed).await.unwrap_err(),jj_store_service::mutations::Error::IntentConflict);
    }).await;
}

#[tokio::test(flavor = "multi_thread", worker_threads = 4)]
async fn http_resolve_validates_atomic_fingerprints_permissions_and_selected_receipts() {
    use crate::http_mutation_acceptance::{api, request, send};
    use axum::http::StatusCode;
    use jj_store_service::administration::{TokenChange, TokenRequest};
    tokio::task::LocalSet::new().run_until(async {
        let mut h = History::new().await;
        let writer = BearerToken::generate().unwrap();
        h.f.session.change_token(&TokenRequest {request_id:m::RequestId::new([164;16]),expected:h.f.session.snapshot().await.unwrap().manifest().snapshot(),change:TokenChange::Grant {credential:writer.clone(),principal:m::PrincipalId::new([164;16]),permission:Permission::Write,expires_at:None}}).await.unwrap();
        let (read,_) = complex(&mut h).await;
        let descriptors = read.conflicts(&reader(),cf::Request {commit:h.commit,path:String::new()}).await.unwrap();
        let first = descriptors.conflicts.iter().find(|d|d.path=="binary").unwrap();
        let last = descriptors.conflicts.iter().find(|d|d.path=="text").unwrap();
        let expected=h.f.session.snapshot().await.unwrap().manifest().snapshot();
        let valid=input(expected.clone(),h.commit,json!([selection(first,0),selection(last,2)]));
        let body=serde_json::to_value(&valid).unwrap();
        let cache=tempfile::tempdir().unwrap();
        let router=api(&h.f,cache.path(),jj_store_http::Config::default(),Protection::default());
        for path in ["mutations/preview","mutations"] {
            assert_eq!(send(&router,request(&h.f.reader,path,Some(&body))).await.0,StatusCode::FORBIDDEN);
            let mut stale=body.clone(); stale["command"]["paths"][1]["fingerprint"]=json!(first.fingerprint);
            let (status,error)=send(&router,request(&h.f.token,path,Some(&stale))).await;
            assert_eq!(status,StatusCode::CONFLICT,"{error}");
            assert_eq!(error["code"],"stale_conflict");
        }
        let mut unsupported=body.clone(); unsupported["command"]["paths"][0]["resolution"]=json!({"type":"file","uploaded_file_id":m::FileId::new([1;64]),"executable":false});
        assert_eq!(send(&router,request(&h.f.token,"mutations",Some(&unsupported))).await.0,StatusCode::BAD_REQUEST);
        let (status,preview)=send(&router,request(&writer,"mutations/preview",Some(&body))).await;
        assert_eq!(status,StatusCode::OK,"{preview}");
        assert_eq!(preview["status"],"prepared");
        assert_eq!(h.f.session.snapshot().await.unwrap().manifest().snapshot(),expected);
        let (status,receipt)=send(&router,request(&writer,"mutations",Some(&body))).await;
        assert_eq!(status,StatusCode::OK,"{receipt}");
        assert_eq!(receipt["status"],"committed");
        assert_eq!(receipt["receipt"]["accepted"]["position"],json!(expected.position.0+1));
        let accepted=h.f.session.snapshot().await.unwrap().manifest().snapshot();
        let status_path=format!("mutations/{}",valid.request_id);
        let restarted=api(&h.f,cache.path(),jj_store_http::Config::default(),Protection::default());
        assert_eq!(send(&restarted,request(&writer,&status_path,None)).await.1,receipt);
        assert_eq!(send(&restarted,request(&writer,"mutations",Some(&body))).await.1,receipt);
        let (status,not_known)=send(&restarted,request(&h.f.token,&status_path,None)).await;
        assert_eq!(status,StatusCode::OK); assert_eq!(not_known["status"],"not_known");
        let mut stale=body.clone(); stale["request_id"]=json!(m::RequestId::new([165;16]));
        let (status,error)=send(&restarted,request(&writer,"mutations",Some(&stale))).await;
        assert_eq!(status,StatusCode::CONFLICT); assert_eq!(error["code"],"stale_snapshot");
        assert_eq!(h.f.session.snapshot().await.unwrap().manifest().snapshot(),accepted);
        h.f.session.change_token(&TokenRequest {request_id:m::RequestId::new([166;16]),expected:accepted,change:TokenChange::Revoke {token:writer.id()}}).await.unwrap();
        let revoked=h.f.session.snapshot().await.unwrap().manifest().snapshot();
        assert_eq!(send(&restarted,request(&writer,&status_path,None)).await.0,StatusCode::FORBIDDEN);
        assert_eq!(send(&restarted,request(&writer,"mutations",Some(&body))).await.0,StatusCode::FORBIDDEN);
        assert_eq!(h.f.session.snapshot().await.unwrap().manifest().snapshot(),revoked);
        if let Some(path)=std::env::var_os("JJ_STORE_RESOLVE_CONTRACT_CAPTURE") {
            let samples=[
                json!({"path":"/v1/repos/{repo}/mutations/preview","method":"post","status":200,"media":"application/json","data":preview}),
                json!({"path":"/v1/repos/{repo}/mutations","method":"post","status":200,"media":"application/json","data":receipt}),
                json!({"path":"/v1/repos/{repo}/mutations","method":"post","status":409,"media":"application/json","data":error})
            ];
            std::fs::write(path,serde_json::to_vec(&samples).unwrap()).unwrap();
        }
    }).await;
}

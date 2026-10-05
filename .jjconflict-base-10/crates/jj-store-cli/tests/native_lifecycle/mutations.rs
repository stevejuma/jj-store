use super::*;
use jj_store_service::{
    commands::{Command, Executor, Limits as CommandLimits},
    mutations::{CommandService, Outcome, Preview, Request as MutationRequest, Status},
};

fn service() -> CommandService {
    CommandService::new(Executor::new(CommandLimits::default(), 2).unwrap())
}
fn mutation(expected: Snapshot, n: u8) -> MutationRequest {
    MutationRequest {
        version: 1,
        request_id: m::RequestId::new([n; 16]),
        expected,
        command: Command::New {
            parents: vec![roots::ROOT_COMMIT],
            description: "first native command".into(),
            workspace: None,
        },
    }
}
fn receipt(outcome: Outcome) -> jj_store_service::mutations::Receipt {
    match outcome {
        Outcome::Committed { receipt } => *receipt,
        other => panic!("expected receipt: {other:?}"),
    }
}

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

#[tokio::test(flavor = "multi_thread", worker_threads = 2)]
async fn typed_apply_returns_original_native_result_after_restart_and_later_publication() {
    let f = Fixture::new().await;
    let commands = service();
    let expected = f.session.snapshot().await.unwrap().manifest().snapshot();
    let request = mutation(expected.clone(), 101);
    let preview = commands.preview(f.session.clone(), &request).await.unwrap();
    let Preview::Prepared { preview } = preview else {
        panic!("expected unpublished preview")
    };
    assert_eq!(
        f.session.snapshot().await.unwrap().manifest().snapshot(),
        expected
    );
    assert_eq!(preview.result.created.len(), 1);
    let first = receipt(commands.apply(f.session.clone(), &request).await.unwrap());
    assert_eq!(first.result.created.len(), 1);
    assert_eq!(first.request_digest, preview.request_digest);
    assert_eq!(first.accepted.position, m::WalPosition(1));
    let later = MutationRequest {
        version: 1,
        request_id: m::RequestId::new([102; 16]),
        expected: first.accepted.clone(),
        command: Command::Describe {
            commit: first.result.created[0],
            description: "later description".into(),
        },
    };
    let second = receipt(commands.apply(f.session.clone(), &later).await.unwrap());
    Coordinator::new(
        f.store.clone(),
        REPO,
        f.objects.clone(),
        Arc::new(NoPublication),
        Limits::default(),
    )
    .unwrap()
    .checkpoint()
    .await
    .unwrap();
    let reopened = f.session(f.token.clone(), Protection::default());
    let restarted = service();
    assert_eq!(
        receipt(restarted.apply(reopened.clone(), &request).await.unwrap()),
        first
    );
    assert_eq!(
        restarted
            .status(reopened.clone(), request.request_id)
            .await
            .unwrap(),
        Status::Committed {
            receipt: Box::new(first.clone())
        }
    );
    assert_eq!(
        restarted.preview(reopened.clone(), &request).await.unwrap(),
        Preview::Committed { receipt: first }
    );
    assert_eq!(
        reopened.snapshot().await.unwrap().manifest().snapshot(),
        second.accepted
    );
    assert!(matches!(
        restarted
            .status(reopened, m::RequestId::new([103; 16]))
            .await
            .unwrap(),
        Status::NotKnown { .. }
    ));
}

#[tokio::test(flavor = "multi_thread", worker_threads = 2)]
async fn typed_mutations_reject_stale_intent_conflicts_readers_and_protected_previews() {
    use jj_store_service::mutations::Error;
    let f = Fixture::new().await;
    let commands = service();
    let initial = f.session.snapshot().await.unwrap().manifest().snapshot();
    let request = mutation(initial.clone(), 104);
    commands.preview(f.session.clone(), &request).await.unwrap();
    let accepted = receipt(
        commands
            .apply(f.session.clone(), &mutation(initial.clone(), 105))
            .await
            .unwrap(),
    );
    assert_eq!(
        commands
            .apply(f.session.clone(), &request)
            .await
            .unwrap_err(),
        Error::Stale
    );
    let mut conflict = mutation(initial, 105);
    conflict.command = Command::New {
        parents: vec![roots::ROOT_COMMIT],
        description: "changed intent".into(),
        workspace: None,
    };
    assert_eq!(
        commands
            .apply(f.session.clone(), &conflict)
            .await
            .unwrap_err(),
        Error::IntentConflict
    );
    let reader = f.session(f.reader.clone(), Protection::default());
    assert_eq!(
        commands
            .preview(reader.clone(), &request)
            .await
            .unwrap_err(),
        Error::Denied
    );
    assert_eq!(
        commands.apply(reader.clone(), &request).await.unwrap_err(),
        Error::Denied
    );
    assert_eq!(
        commands
            .status(reader, request.request_id)
            .await
            .unwrap_err(),
        Error::Denied
    );
    let protected = f.session(
        f.token.clone(),
        Protection {
            commits: BTreeSet::from([accepted.result.created[0]]),
            bookmarks: BTreeSet::new(),
        },
    );
    let rewrite = MutationRequest {
        version: 1,
        request_id: m::RequestId::new([106; 16]),
        expected: accepted.accepted.clone(),
        command: Command::Describe {
            commit: accepted.result.created[0],
            description: "forbidden".into(),
        },
    };
    assert_eq!(
        commands
            .preview(protected.clone(), &rewrite)
            .await
            .unwrap_err(),
        Error::Protected
    );
    assert_eq!(
        commands.apply(protected, &rewrite).await.unwrap_err(),
        Error::Protected
    );
    assert_eq!(
        f.session.snapshot().await.unwrap().manifest().snapshot(),
        accepted.accepted
    );
    assert_eq!(
        commands
            .status(f.session.clone(), rewrite.request_id)
            .await
            .unwrap(),
        Status::NotKnown {
            request_id: rewrite.request_id
        }
    );
}

#[tokio::test(flavor = "multi_thread", worker_threads = 2)]
async fn typed_mutation_status_is_principal_scoped_and_preview_does_not_authorize_apply() {
    use jj_store_service::{
        administration::{TokenChange, TokenRequest},
        mutations::Error,
    };
    let f = Fixture::new().await;
    let commands = service();
    let first_request = mutation(
        f.session.snapshot().await.unwrap().manifest().snapshot(),
        107,
    );
    let first = receipt(
        commands
            .apply(f.session.clone(), &first_request)
            .await
            .unwrap(),
    );
    let writer = BearerToken::generate().unwrap();
    f.session
        .change_token(&TokenRequest {
            request_id: m::RequestId::new([108; 16]),
            expected: first.accepted,
            change: TokenChange::Grant {
                credential: writer.clone(),
                principal: m::PrincipalId::new([108; 16]),
                permission: Permission::Write,
                expires_at: None,
            },
        })
        .await
        .unwrap();
    let writer_session = f.session(writer.clone(), Protection::default());
    assert_eq!(
        commands
            .status(writer_session.clone(), first_request.request_id)
            .await
            .unwrap(),
        Status::NotKnown {
            request_id: first_request.request_id
        }
    );
    let before = f.session.snapshot().await.unwrap().manifest().snapshot();
    let request = mutation(before.clone(), 109);
    commands
        .preview(writer_session.clone(), &request)
        .await
        .unwrap();
    f.session
        .change_token(&TokenRequest {
            request_id: m::RequestId::new([110; 16]),
            expected: before,
            change: TokenChange::Revoke { token: writer.id() },
        })
        .await
        .unwrap();
    let after = f.session.snapshot().await.unwrap().manifest().snapshot();
    assert_eq!(
        commands
            .apply(writer_session.clone(), &request)
            .await
            .unwrap_err(),
        Error::Denied
    );
    assert_eq!(
        commands
            .status(writer_session, request.request_id)
            .await
            .unwrap_err(),
        Error::Denied
    );
    assert_eq!(
        f.session.snapshot().await.unwrap().manifest().snapshot(),
        after
    );
    let other_objects = Arc::new(
        NativeObjects::new(
            f.store.clone(),
            m::RepositoryId::new([111; 16]),
            4 * 1024 * 1024,
        )
        .unwrap(),
    );
    let other = Arc::new(
        NativeSession::new(
            f.store.clone(),
            other_objects,
            f.token.clone(),
            Arc::new(SystemClock),
            Protection::default(),
            SessionLimits::default(),
        )
        .unwrap(),
    );
    assert_eq!(
        commands
            .status(other, first_request.request_id)
            .await
            .unwrap_err(),
        Error::Denied
    );
}

use jj_store_object::{
    ByteRange, Condition, Download, Key, Metadata, ObjectStore, StagedUpload, WriteOutcome,
};
use std::sync::atomic::{AtomicBool, Ordering};
#[derive(Clone, Copy)]
enum Fault {
    HoldBefore,
    DropBefore,
    DropAfter,
}
struct Store {
    inner: Arc<MemoryStore>,
    path: &'static str,
    fault: Fault,
    armed: AtomicBool,
    unreadable: AtomicBool,
    entered: tokio::sync::Notify,
    release: tokio::sync::Notify,
}
impl Store {
    fn session(f: &Fixture, path: &'static str, fault: Fault) -> (Arc<Self>, Arc<NativeSession>) {
        let store = Arc::new(Self {
            inner: f.store.clone(),
            path,
            fault,
            armed: AtomicBool::new(true),
            unreadable: AtomicBool::new(false),
            entered: Default::default(),
            release: Default::default(),
        });
        let objects = Arc::new(NativeObjects::new(store.clone(), REPO, 4 * 1024 * 1024).unwrap());
        let session = Arc::new(
            NativeSession::new(
                store.clone(),
                objects,
                f.token.clone(),
                Arc::new(SystemClock),
                Protection::default(),
                SessionLimits::default(),
            )
            .unwrap(),
        );
        (store, session)
    }
}
#[async_trait::async_trait]
impl ObjectStore for Store {
    async fn head(&self, key: &Key) -> jj_store_object::Result<Metadata> {
        self.inner.head(key).await
    }
    async fn get(&self, key: &Key, range: Option<ByteRange>) -> jj_store_object::Result<Download> {
        if key.as_str().ends_with("/manifest.json") && self.unreadable.load(Ordering::SeqCst) {
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
        let fault = key.as_str().contains(self.path) && self.armed.swap(false, Ordering::SeqCst);
        if fault {
            match self.fault {
                Fault::HoldBefore => {
                    self.entered.notify_one();
                    self.release.notified().await;
                }
                Fault::DropBefore => return Ok(WriteOutcome::Unknown),
                Fault::DropAfter => {}
            }
        }
        let outcome = self.inner.put(key, condition, upload).await?;
        if fault && matches!(self.fault, Fault::DropAfter) {
            self.unreadable.store(true, Ordering::SeqCst);
            return Ok(WriteOutcome::Unknown);
        }
        Ok(outcome)
    }
}

#[tokio::test(flavor = "multi_thread", worker_threads = 4)]
async fn typed_identical_apply_races_return_one_result_during_preparation_or_manifest_cas() {
    for path in ["/objects/v1/", "/manifest.json"] {
        let f = Fixture::new().await;
        let commands = service();
        let request = mutation(
            f.session.snapshot().await.unwrap().manifest().snapshot(),
            112,
        );
        let (store, paused) = Store::session(&f, path, Fault::HoldBefore);
        let first = commands.clone();
        let input = request.clone();
        let pending = tokio::spawn(async move { first.apply(paused, &input).await });
        tokio::time::timeout(std::time::Duration::from_secs(3), store.entered.notified())
            .await
            .unwrap();
        let selected = receipt(commands.apply(f.session.clone(), &request).await.unwrap());
        store.release.notify_one();
        assert_eq!(receipt(pending.await.unwrap().unwrap()), selected);
        assert_eq!(
            f.session.snapshot().await.unwrap().manifest().snapshot(),
            selected.accepted
        );
        assert_eq!(selected.accepted.position, m::WalPosition(1));
    }
}

#[tokio::test(flavor = "multi_thread", worker_threads = 2)]
async fn typed_apply_unknown_status_and_retry_recover_the_original_selected_result() {
    for fault in [Fault::DropBefore, Fault::DropAfter] {
        let f = Fixture::new().await;
        let commands = service();
        let request = mutation(
            f.session.snapshot().await.unwrap().manifest().snapshot(),
            113,
        );
        let (store, session) = Store::session(&f, "/manifest.json", fault);
        assert_eq!(
            commands.apply(session.clone(), &request).await.unwrap(),
            Outcome::OutcomeUnknown {
                request_id: request.request_id
            }
        );
        if matches!(fault, Fault::DropBefore) {
            assert_eq!(
                commands
                    .status(session.clone(), request.request_id)
                    .await
                    .unwrap(),
                Status::NotKnown {
                    request_id: request.request_id
                }
            );
        } else {
            assert_eq!(
                commands
                    .status(session.clone(), request.request_id)
                    .await
                    .unwrap(),
                Status::OutcomeUnknown {
                    request_id: request.request_id
                }
            );
        }
        store.unreadable.store(false, Ordering::SeqCst);
        let recovered = receipt(service().apply(session.clone(), &request).await.unwrap());
        assert_eq!(
            commands.status(session, request.request_id).await.unwrap(),
            Status::Committed {
                receipt: Box::new(recovered.clone())
            }
        );
        assert_eq!(
            f.session.snapshot().await.unwrap().manifest().snapshot(),
            recovered.accepted
        );
        assert_eq!(recovered.accepted.position, m::WalPosition(1));
    }
}

#[tokio::test(flavor = "multi_thread", worker_threads = 2)]
async fn typed_bookmark_receipts_replay_after_move_and_delete_without_resurrection() {
    use jj_store_jj::native_dto::RefTarget;
    let f = Fixture::new().await;
    let commands = service();
    let first = receipt(
        commands
            .apply(
                f.session.clone(),
                &mutation(
                    f.session.snapshot().await.unwrap().manifest().snapshot(),
                    114,
                ),
            )
            .await
            .unwrap(),
    );
    let second = receipt(
        commands
            .apply(f.session.clone(), &mutation(first.accepted.clone(), 115))
            .await
            .unwrap(),
    );
    let create = MutationRequest {
        version: 1,
        request_id: m::RequestId::new([116; 16]),
        expected: second.accepted,
        command: Command::BookmarkCreate {
            name: "topic".into(),
            target: first.result.created[0],
        },
    };
    let created = receipt(commands.apply(f.session.clone(), &create).await.unwrap());
    let moving = MutationRequest {
        version: 1,
        request_id: m::RequestId::new([117; 16]),
        expected: created.accepted.clone(),
        command: Command::BookmarkMove {
            name: "topic".into(),
            expected: RefTarget(vec![Some(first.result.created[0])]),
            target: second.result.created[0],
        },
    };
    let moved = receipt(commands.apply(f.session.clone(), &moving).await.unwrap());
    let delete = MutationRequest {
        version: 1,
        request_id: m::RequestId::new([118; 16]),
        expected: moved.accepted.clone(),
        command: Command::BookmarkDelete {
            name: "topic".into(),
            expected: RefTarget(vec![Some(second.result.created[0])]),
        },
    };
    let deleted = receipt(commands.apply(f.session.clone(), &delete).await.unwrap());
    assert_eq!(
        receipt(commands.apply(f.session.clone(), &create).await.unwrap()),
        created
    );
    assert_eq!(
        receipt(commands.apply(f.session.clone(), &moving).await.unwrap()),
        moved
    );
    assert_eq!(
        f.session.snapshot().await.unwrap().manifest().snapshot(),
        deleted.accepted
    );
    let operation: o::Operation = f
        .objects
        .get(m::ObjectKey::Operation(deleted.result.operation))
        .await
        .unwrap();
    let view: o::View = f
        .objects
        .get(m::ObjectKey::View(
            m::ViewId::from_slice(operation.view_id.as_bytes()).unwrap(),
        ))
        .await
        .unwrap();
    assert!(
        !view
            .local_bookmarks
            .contains_key(jj_lib::ref_name::RefName::new("topic"))
    );
}

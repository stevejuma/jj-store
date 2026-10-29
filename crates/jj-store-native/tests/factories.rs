#[path = "factories/store_capture.rs"]
mod store_capture;
mod support;
use jj_store_core::*;
use std::{collections::BTreeSet, sync::atomic::Ordering};
use support::*;

#[tokio::test]
async fn loading_and_empty_domain_reads_do_not_initialize_or_stage() {
    let (storage, c, _, stores) = fixture().await;
    let before = c.read().await.unwrap();
    let writes = storage.puts.load(Ordering::SeqCst);
    assert_eq!(stores.agent().unwrap().snapshot().await.unwrap().version, 0);
    assert_eq!(
        stores.review().unwrap().snapshot().await.unwrap().version,
        0
    );
    assert!(
        stores
            .work()
            .unwrap()
            .snapshot()
            .await
            .unwrap()
            .data
            .projects
            .is_empty()
    );
    assert_eq!(
        stores.thread().unwrap().snapshot().await.unwrap().version,
        0
    );
    assert_eq!(
        stores.reaction().unwrap().snapshot().await.unwrap().version,
        0
    );
    assert_eq!(storage.puts.load(Ordering::SeqCst), writes);
    assert_eq!(c.read().await.unwrap().manifest(), before.manifest());
}
#[tokio::test]
async fn original_domain_receipts_survive_later_writes_checkpoint_and_reopen() {
    let (storage, c, host, stores) = fixture().await;
    let work = committed(stores.work().unwrap().execute(&project(1)).await.unwrap());
    assert_eq!(
        work.result
            .project
            .as_ref()
            .unwrap()
            .actor
            .created_at_millis,
        12345
    );
    let thread = committed(stores.thread().unwrap().execute(&thread(2)).await.unwrap());
    let review = request(
        3,
        ReviewCommand::RepositoryThread {
            body: jj_review::ReviewCommentBodyV1 {
                schema_version: 1,
                format: jj_review::ReviewBodyFormatV1::PlainText,
                text: "Review".into(),
                sanitizer_version: "plain@1".into(),
            },
        },
    );
    let review_receipt = committed(stores.review().unwrap().execute(&review).await.unwrap());
    committed(stores.work().unwrap().execute(&project(4)).await.unwrap());
    committed(
        stores
            .thread()
            .unwrap()
            .execute(&support::thread(5))
            .await
            .unwrap(),
    );
    assert_ne!(
        stores.work().unwrap().snapshot().await.unwrap().selection,
        work.selection
    );
    c.checkpoint().await.unwrap();
    c.recover(100).await.unwrap();
    let reopened = open(coordinator(storage), host, |_, _, _| {});
    let old = committed(
        reopened
            .work()
            .unwrap()
            .status(work.request_id)
            .await
            .unwrap(),
    );
    assert_eq!(old.version, 1);
    assert_eq!(old.selection, work.selection);
    assert_eq!(old.provider_receipt, work.provider_receipt);
    assert_eq!(
        committed(
            reopened
                .thread()
                .unwrap()
                .status(thread.request_id)
                .await
                .unwrap()
        )
        .provider_receipt,
        thread.provider_receipt
    );
    assert_eq!(
        committed(reopened.review().unwrap().execute(&review).await.unwrap()).provider_receipt,
        review_receipt.provider_receipt
    );
    assert!(matches!(
        reopened.thread().unwrap().status(work.request_id).await,
        Err(Error::IntentConflict)
    ));
}
#[tokio::test]
async fn prepared_bound_capture_keeps_exact_native_request_and_publishes_atomically() {
    let (storage, c, host, stores) = fixture().await;
    let created = committed(stores.thread().unwrap().execute(&thread(10)).await.unwrap());
    let jj_collab::CollabCommandResultV1::Thread { result: t } = created.result else {
        panic!()
    };
    let raw = request(
        11,
        AgentCommand::Capture {
            profile: jj_agent::NormalizerProfileV1::codex_0_156_1(),
            batch: Box::new(batch()),
            binding: Some(jj_collab::ThreadBindingV1 {
                thread_id: t.thread_id,
                observed_thread: t.version,
                observed_assignment: None,
            }),
        },
    );
    let prepared = stores.prepare_agent(&raw).await.unwrap();
    let mut native = jj_store_native::to_agent_delta(ACTOR, &prepared).unwrap();
    native.result = b"opaque existing RPC result".to_vec();
    let prepared = jj_store_native::from_agent_delta(&native).unwrap();
    assert_eq!(
        jj_store_native::to_agent_delta(ACTOR, &prepared).unwrap(),
        native
    );
    let before = c.read().await.unwrap();
    storage.mode.store(1, Ordering::SeqCst);
    assert!(
        matches!(stores.execute_agent(&prepared).await.unwrap(),StoreOutcome::Unknown{request_id} if request_id==prepared.request_id)
    );
    assert_eq!(before.manifest(), c.read().await.unwrap().manifest());
    let accepted = committed(stores.execute_agent(&prepared).await.unwrap());
    let after = c.read().await.unwrap();
    assert_eq!(
        after.manifest().published.0,
        before.manifest().published.0 + 1
    );
    assert_eq!(after.manifest().agent_state_version, 1);
    assert_eq!(after.manifest().thread_state_version, 2);
    let receipt: jj_store_wal::publication::Receipt =
        serde_json::from_slice(&accepted.provider_receipt).unwrap();
    assert_eq!(receipt.publication_id, native.publication_id);
    assert_eq!(receipt.result, native.result);
    c.checkpoint().await.unwrap();
    let reopened = open(coordinator(storage), host, |_, _, config| {
        config.disable(StoreKind::Thread)
    });
    assert_eq!(
        committed(reopened.execute_agent(&prepared).await.unwrap()).provider_receipt,
        accepted.provider_receipt
    );
    let mut changed = prepared.clone();
    changed.publication_id = Some(jj_store_model::PublicationId::new([88; 16]));
    assert!(matches!(
        reopened.execute_agent(&changed).await,
        Err(Error::IntentConflict)
    ));
}
#[tokio::test]
async fn cross_domain_sources_must_be_selected_compatible_and_authorized_before_staging() {
    for replaced in [false, true] {
        let (storage, c, host, stores) = fixture().await;
        committed(stores.thread().unwrap().execute(&thread(20)).await.unwrap());
        let selected = open(c.clone(), host.clone(), |_, factories, config| {
            if replaced {
                jj_store_native::NativeProvider::new(c.clone(), "fixture")
                    .unwrap()
                    .add_thread_store(factories, "external")
                    .unwrap();
                config.enable(StoreKind::Thread, "external");
            } else {
                config.disable(StoreKind::Thread);
            }
        });
        let command = reaction(21, 20);
        let writes = storage.puts.load(Ordering::SeqCst);
        assert!(matches!(
            selected.reaction().unwrap().execute(&command).await,
            Err(Error::Unsupported(_))
        ));
        assert_eq!(storage.puts.load(Ordering::SeqCst), writes);
        host.denied_source.store(true, Ordering::SeqCst);
        assert!(matches!(
            stores.reaction().unwrap().execute(&command).await,
            Err(Error::Denied)
        ));
        assert_eq!(storage.puts.load(Ordering::SeqCst), writes);
    }
}
fn reaction(id: u8, thread: u8) -> StoreRequest<ReactionCommand> {
    request(
        id,
        ReactionCommand::Reaction(jj_collab::ReactionCommandV1::Set {
            emoji: None,
            target: jj_collab::ReactionTargetV1::Thread {
                thread_id: jj_collab::ThreadId::new([thread; 16]),
            },
            value: jj_collab::ReactionValueV1::Like,
            observed: BTreeSet::new(),
        }),
    )
}
#[tokio::test]
async fn unknown_after_apply_retains_original_receipt_and_replays_without_live_sources() {
    for fault in [2, 3, 4] {
        let (storage, c, host, stores) = fixture().await;
        committed(stores.thread().unwrap().execute(&thread(30)).await.unwrap());
        let command = reaction(31, 30);
        storage.mode.store(fault, Ordering::SeqCst);
        assert!(matches!(
            stores.reaction().unwrap().execute(&command).await.unwrap(),
            StoreOutcome::Unknown { .. }
        ));
        storage.read_blocked.store(false, Ordering::SeqCst);
        let reopened = open(coordinator(storage), host.clone(), |_, _, config| {
            config.disable(StoreKind::Thread)
        });
        let status = committed(
            reopened
                .reaction()
                .unwrap()
                .status(command.request_id)
                .await
                .unwrap(),
        );
        assert_eq!(
            committed(
                reopened
                    .reaction()
                    .unwrap()
                    .execute(&command)
                    .await
                    .unwrap()
            )
            .provider_receipt,
            status.provider_receipt
        );
        host.denied.store(true, Ordering::SeqCst);
        assert!(matches!(
            reopened
                .reaction()
                .unwrap()
                .status(command.request_id)
                .await,
            Err(Error::Denied)
        ));
        assert_eq!(c.read().await.unwrap().manifest().reaction_state_version, 1);
    }
}

#[tokio::test]
async fn native_review_state_errors_keep_their_public_meaning() {
    let (_, _, _, stores) = fixture().await;

    let review = stores.review().unwrap();
    let body = jj_review::ReviewCommentBodyV1 {
        schema_version: 1,
        format: jj_review::ReviewBodyFormatV1::PlainText,
        text: "Review".into(),
        sanitizer_version: "plain@1".into(),
    };
    let created = committed(
        review
            .execute(&request(
                61,
                ReviewCommand::RepositoryThread { body: body.clone() },
            ))
            .await
            .unwrap(),
    )
    .result
    .unwrap();
    let expected =
        jj_review::ThreadSnapshotId::from_slice(created.thread_snapshot.id.as_bytes()).unwrap();
    let apply = |id, command| {
        request(
            id,
            ReviewCommand::Apply {
                command: Box::new(command),
                evidence: vec![],
            },
        )
    };
    assert!(matches!(
        review
            .execute(&apply(
                62,
                jj_review::ReviewCommandV1::ReopenThread {
                    thread_id: created.thread_id,
                    expected_thread_version: expected,
                    reason: None
                }
            ))
            .await,
        Err(Error::ThreadOpen)
    ));
    committed(
        review
            .execute(&apply(
                63,
                jj_review::ReviewCommandV1::ResolveThread {
                    thread_id: created.thread_id,
                    expected_thread_version: expected,
                    reason: None,
                },
            ))
            .await
            .unwrap(),
    );
    assert!(matches!(
        review
            .execute(&apply(
                64,
                jj_review::ReviewCommandV1::AppendReply {
                    thread_id: created.thread_id,
                    expected_workflow_version: 2,
                    parent_comment_id: None,
                    body
                }
            ))
            .await,
        Err(Error::ThreadResolved)
    ));
}

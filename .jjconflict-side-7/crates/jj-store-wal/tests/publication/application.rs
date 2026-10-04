use super::*;

fn intent(expected: Snapshot, payload: &[u8]) -> ApplicationIntent {
    ApplicationIntent::new(
        PrincipalId::new([3; 16]),
        RequestId::new([51; 16]),
        PublicationId::new([51; 16]),
        expected,
        SemanticKind::OperationPublished,
        payload.to_vec(),
    )
    .unwrap()
}

#[tokio::test]
async fn application_receipt_survives_regenerated_candidates_restart_and_checkpoint() {
    let (store, c) = fixture().await;
    let original = c.read().await.unwrap().manifest().snapshot();
    let action = intent(original.clone(), b"canonical typed command");
    // Frozen V1 bytes/hash independently calculated with Python compact JSON + SHA-256.
    assert_eq!(
        action.digest(REPO).unwrap().to_string(),
        "b94a00b4bbe92f7812ec8d1f0cf275729acbe5a0bf5691013e0e88cde6fdc794"
    );
    let accepted = committed(
        c.publish_application(
            &action,
            BTreeSet::from([op(1)]),
            b"first native result".to_vec(),
        )
        .await
        .unwrap(),
    );
    committed(c.publish(&request(52, &[1], 3)).await.unwrap());
    c.checkpoint().await.unwrap();
    let restarted = coordinator(store);
    assert_eq!(
        restarted.status_application(&action).await.unwrap(),
        Some(accepted.clone())
    );
    assert_eq!(
        committed(
            restarted
                .publish_application(
                    &action,
                    BTreeSet::from([op(2)]),
                    b"regenerated result".to_vec()
                )
                .await
                .unwrap()
        ),
        accepted
    );
    assert!(matches!(
        restarted
            .publish_application(
                &intent(original, b"different typed command"),
                BTreeSet::from([op(4)]),
                vec![]
            )
            .await,
        Err(wal::Error::IntentConflict)
    ));
    let current = restarted.read().await.unwrap();
    assert_eq!(current.manifest().published, WalPosition(2));
    assert_eq!(current.manifest().heads, BTreeSet::from([op(3)]));
    assert_eq!(
        restarted
            .receipt_at(&current, accepted.principal, accepted.request_id)
            .await
            .unwrap(),
        Some(accepted)
    );
    assert!(
        restarted
            .receipt_at(
                &current,
                PrincipalId::new([99; 16]),
                RequestId::new([51; 16])
            )
            .await
            .unwrap()
            .is_none()
    );
    assert!(
        restarted
            .receipt_at(
                &current,
                PrincipalId::new([3; 16]),
                RequestId::new([99; 16])
            )
            .await
            .unwrap()
            .is_none()
    );
}

#[tokio::test]
async fn application_competing_candidates_return_only_the_manifest_selected_result() {
    let (store, c) = fixture().await;
    let action = intent(
        c.read().await.unwrap().manifest().snapshot(),
        b"same command",
    );
    store.arm("/manifest.json", Fault::HoldBefore);
    let first = c.clone();
    let first_action = action.clone();
    let pending = tokio::spawn(async move {
        first
            .publish_application(
                &first_action,
                BTreeSet::from([op(1)]),
                b"losing result".to_vec(),
            )
            .await
    });
    tokio::time::timeout(Duration::from_secs(3), store.entered.notified())
        .await
        .unwrap();
    let selected = committed(
        c.publish_application(
            &action,
            BTreeSet::from([op(2)]),
            b"selected result".to_vec(),
        )
        .await
        .unwrap(),
    );
    store.release.notify_one();
    assert_eq!(committed(pending.await.unwrap().unwrap()), selected);
    let snapshot = c.read().await.unwrap();
    assert_eq!(snapshot.manifest().published, WalPosition(1));
    assert_eq!(selected.result, b"selected result");
    assert_eq!(snapshot.manifest().heads, BTreeSet::from([op(2)]));
    let committed_objects = c.inventory(&snapshot, 100).await.unwrap();
    assert!(
        !committed_objects
            .iter()
            .any(|node| node.key == ObjectKey::Operation(op(1)))
    );
    assert_eq!(
        c.committed(&snapshot, WalPosition(0), 10)
            .await
            .unwrap()
            .len(),
        1
    );
}

#[tokio::test]
async fn application_unknown_outcomes_retain_identity_without_claiming_rejection() {
    for fault in [Fault::Before, Fault::After] {
        let (store, c) = fixture().await;
        let action = intent(
            c.read().await.unwrap().manifest().snapshot(),
            b"command after lost response",
        );
        store.arm("/manifest.json", fault);
        let result = c
            .publish_application(&action, BTreeSet::from([op(1)]), b"first".to_vec())
            .await
            .unwrap();
        match fault {
            Fault::Before => {
                assert!(
                    matches!(result, PublishOutcome::OutcomeUnknown { request_id, publication_id } if request_id == RequestId::new([51;16]) && publication_id == PublicationId::new([51;16]))
                );
                assert!(c.status_application(&action).await.unwrap().is_none());
                let retried = committed(
                    c.publish_application(&action, BTreeSet::from([op(2)]), b"retry".to_vec())
                        .await
                        .unwrap(),
                );
                assert_eq!(retried.result, b"retry");
            }
            Fault::After => {
                let selected = committed(result);
                assert_eq!(selected.result, b"first");
                assert_eq!(
                    committed(
                        c.publish_application(&action, BTreeSet::from([op(2)]), b"retry".to_vec())
                            .await
                            .unwrap()
                    ),
                    selected
                );
            }
            _ => unreachable!(),
        }
        assert_eq!(c.read().await.unwrap().manifest().published, WalPosition(1));
    }
}

#[tokio::test]
async fn application_identity_changes_conflict_and_native_digest_profile_is_unchanged() {
    use sha2::{Digest as _, Sha256};
    let (_, c) = fixture().await;
    let expected = c.read().await.unwrap().manifest().snapshot();
    let action = intent(expected.clone(), b"original");
    let receipt = committed(
        c.publish_application(&action, BTreeSet::from([op(1)]), vec![1])
            .await
            .unwrap(),
    );
    let changed = |principal, request, publication, expected, kind, payload| {
        ApplicationIntent::new(principal, request, publication, expected, kind, payload).unwrap()
    };
    let mut position = expected.clone();
    position.position = WalPosition(1);
    let mut epoch = expected.clone();
    epoch.epoch = EpochId::new([9; 16]);
    let mut heads = expected.clone();
    heads.heads = BTreeSet::from([op(9)]);
    for other in [
        changed(
            receipt.principal,
            receipt.request_id,
            receipt.publication_id,
            position,
            SemanticKind::OperationPublished,
            b"original".to_vec(),
        ),
        changed(
            receipt.principal,
            receipt.request_id,
            receipt.publication_id,
            epoch,
            SemanticKind::OperationPublished,
            b"original".to_vec(),
        ),
        changed(
            receipt.principal,
            receipt.request_id,
            receipt.publication_id,
            heads,
            SemanticKind::OperationPublished,
            b"original".to_vec(),
        ),
        changed(
            receipt.principal,
            receipt.request_id,
            receipt.publication_id,
            expected.clone(),
            SemanticKind::HeadsNormalized,
            b"original".to_vec(),
        ),
        changed(
            PrincipalId::new([9; 16]),
            receipt.request_id,
            receipt.publication_id,
            expected.clone(),
            SemanticKind::OperationPublished,
            b"original".to_vec(),
        ),
        changed(
            receipt.principal,
            RequestId::new([9; 16]),
            receipt.publication_id,
            expected.clone(),
            SemanticKind::OperationPublished,
            b"original".to_vec(),
        ),
        changed(
            receipt.principal,
            receipt.request_id,
            PublicationId::new([9; 16]),
            expected.clone(),
            SemanticKind::OperationPublished,
            b"original".to_vec(),
        ),
        intent(expected.clone(), b"different"),
    ] {
        assert!(matches!(
            c.status_application(&other).await,
            Err(wal::Error::IntentConflict)
        ));
        assert!(matches!(
            c.publish_application(&other, BTreeSet::from([op(2)]), vec![])
                .await,
            Err(wal::Error::IntentConflict)
        ));
    }
    assert_ne!(
        action.digest(REPO).unwrap(),
        action.digest(RepositoryId::new([9; 16])).unwrap()
    );
    let native = request(52, &[1], 2);
    let native_receipt = committed(c.publish(&native).await.unwrap());
    assert_eq!(
        native_receipt.intent,
        IntentDigest::new(Sha256::digest(serde_json::to_vec(&native).unwrap()).into())
    );
    let mut collision = native.clone();
    collision.request_id = receipt.request_id;
    collision.publication_id = receipt.publication_id;
    assert!(matches!(
        c.status(&collision).await,
        Err(wal::Error::IntentConflict)
    ));
    assert_eq!(c.read().await.unwrap().manifest().published, WalPosition(2));
}

#[tokio::test]
async fn application_guard_checks_staging_and_reauthorizes_receipt_replay() {
    use std::sync::atomic::{AtomicU8, Ordering};
    struct Guard(AtomicU8);
    #[async_trait::async_trait]
    impl PublicationGuard for Guard {
        async fn authorize(&self, _: &ManifestSnapshot, _: &Request) -> wal::Result<()> {
            if self.0.load(Ordering::SeqCst) == 1 {
                Err(wal::Error::Denied)
            } else {
                Ok(())
            }
        }
        async fn validate_transition(
            &self,
            _: &ManifestSnapshot,
            _: &Request,
            _: &Snapshot,
        ) -> wal::Result<()> {
            if self.0.load(Ordering::SeqCst) == 2 {
                Err(wal::Error::Denied)
            } else {
                Ok(())
            }
        }
    }
    let (store, original) = fixture().await;
    let action = intent(
        original.read().await.unwrap().manifest().snapshot(),
        b"guarded command",
    );
    let guard = Arc::new(Guard(AtomicU8::new(2)));
    let c = Coordinator::new(
        store.clone(),
        REPO,
        Arc::new(Source::new()),
        guard.clone(),
        Limits::default(),
    )
    .unwrap();
    let before = store.writes.lock().unwrap().len();
    assert!(matches!(
        c.validate_application(&action, BTreeSet::from([op(1)]))
            .await,
        Err(wal::Error::Denied)
    ));
    assert!(matches!(
        c.publish_application(&action, BTreeSet::from([op(1)]), vec![])
            .await,
        Err(wal::Error::Denied)
    ));
    assert_eq!(store.writes.lock().unwrap().len(), before);
    guard.0.store(0, Ordering::SeqCst);
    c.validate_application(&action, BTreeSet::from([op(1)]))
        .await
        .unwrap();
    assert_eq!(store.writes.lock().unwrap().len(), before);
    assert!(
        c.validate_application(&action, BTreeSet::from([op(254)]))
            .await
            .is_err()
    );
    assert_eq!(store.writes.lock().unwrap().len(), before);
    committed(
        c.publish_application(&action, BTreeSet::from([op(1)]), vec![])
            .await
            .unwrap(),
    );
    guard.0.store(1, Ordering::SeqCst);
    assert!(matches!(
        c.status_application(&action).await,
        Err(wal::Error::Denied)
    ));
    assert!(matches!(
        c.publish_application(&action, BTreeSet::from([op(2)]), vec![])
            .await,
        Err(wal::Error::Denied)
    ));
    assert_eq!(
        original.read().await.unwrap().manifest().published,
        WalPosition(1)
    );
}

#[tokio::test]
async fn application_receipt_lookup_rejects_selected_identity_position_and_index_corruption() {
    use sha2::{Digest as _, Sha256};
    for field in ["principal", "epoch", "position", "companion"] {
        let (store, c) = fixture().await;
        let action = intent(c.read().await.unwrap().manifest().snapshot(), b"original");
        let receipt = committed(
            c.publish_application(&action, BTreeSet::from([op(1)]), vec![])
                .await
                .unwrap(),
        );
        let records = Records::new(store.clone(), REPO);
        let catalog = Catalog::new(records.clone());
        let mut damaged = receipt.clone();
        match field {
            "principal" => damaged.principal = PrincipalId::new([9; 16]),
            "epoch" => damaged.accepted.epoch = EpochId::new([9; 16]),
            "position" => damaged.accepted.position = WalPosition(2),
            _ => {}
        }
        let reference = records
            .put(RecordKind::Receipt, None, &damaged)
            .await
            .unwrap();
        let mut request_key = Sha256::new_with_prefix(b"jj-store/request/v1");
        request_key.update(REPO.as_bytes());
        request_key.update(receipt.principal.as_bytes());
        request_key.update(receipt.request_id.as_bytes());
        let mut manifest = c.read().await.unwrap().manifest().clone();
        manifest.receipts = catalog
            .update(
                None,
                BTreeMap::from([(RecordDigest::new(request_key.finalize().into()), reference)]),
            )
            .await
            .unwrap();
        let key = c.manifest_key();
        let old = store.head(&key).await.unwrap();
        let bytes = serde_json::to_vec(&manifest).unwrap();
        let upload = StagedUpload::from_bytes(&bytes, 1024 * 1024).await.unwrap();
        store
            .inner
            .put(&key, Condition::Matches(old.version), &upload)
            .await
            .unwrap();
        let current = c.read().await.unwrap();
        assert!(
            matches!(
                c.receipt_at(&current, receipt.principal, receipt.request_id)
                    .await,
                Err(wal::Error::Corrupt(_))
            ),
            "accepted {field} corruption"
        );
    }
}

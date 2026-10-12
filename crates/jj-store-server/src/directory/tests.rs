use super::*;
use jj_store_object::memory::{Fault, MemoryStore};
use jj_store_service::auth::BearerToken;
fn fixture() -> (Arc<MemoryStore>, Manager, Caller, Caller) {
    let store = Arc::new(MemoryStore::default());
    let local = PrincipalId::new([1; 16]);
    let manager = Manager::new(store.clone(), "ab".repeat(16), Some(local)).unwrap();
    (
        store,
        manager,
        Caller::Local(local),
        Caller::Operator(PrincipalId::new([9; 16])),
    )
}
fn request(id: u8, command: DirectoryCommand) -> DirectoryRequest {
    DirectoryRequest {
        request_id: RequestId::new([id; 16]),
        command,
    }
}
fn profile(expected: Option<String>, name: &str) -> DirectoryCommand {
    DirectoryCommand::SaveProfile {
        expected,
        details: ProfileDetails {
            display_name: name.into(),
            description: "A **profile**".into(),
            email: Some(" OWNER@example.com ".into()),
            avatar: None,
        },
    }
}
#[tokio::test]
async fn profiles_are_content_addressed_private_conditional_and_restart_safe() {
    let (store, manager, local, _) = fixture();
    assert!(manager.profile(&local).await.unwrap().revision.is_none());
    assert!(
        store
            .head(&manager.key("manifest.json").unwrap())
            .await
            .is_err()
    );
    let command = request(1, profile(None, "Owner"));
    let receipt = manager.execute(&local, command.clone()).await.unwrap();
    let DirectoryResult::Profile { profile: p } = receipt.result else {
        panic!()
    };
    assert_eq!(p.details.email.as_deref(), Some("owner@example.com"));
    assert_eq!(
        manager.execute(&local, command).await.unwrap().result,
        DirectoryResult::Profile { profile: p.clone() }
    );
    assert!(matches!(
        manager
            .execute(&local, request(2, profile(None, "stale")))
            .await,
        Err(Error::Stale)
    ));
    let restarted = Manager::new(store, "ab".repeat(16), Some(p.principal)).unwrap();
    assert_eq!(restarted.profile(&local).await.unwrap(), p);
    let public = restarted.profiles(&local, vec![p.principal]).await.unwrap();
    let json = serde_json::to_string(&public).unwrap();
    assert!(!json.contains("email"));
    assert!(!json.contains("owner@example.com"));
    assert!(
        public[0]
            .avatar
            .as_ref()
            .unwrap()
            .starts_with("https://www.gravatar.com/")
    );
    restarted
        .execute(&local, request(3, profile(p.revision, "New")))
        .await
        .unwrap();
    assert!(
        restarted
            .receipt(&local, RequestId::new([1; 16]))
            .await
            .unwrap()
            .is_some()
    );
}
#[tokio::test]
async fn enrolled_credentials_are_independent_of_repository_principals_and_revocable() {
    let (_, manager, local, operator) = fixture();
    let token = BearerToken::generate().unwrap();
    let user = PrincipalId::new([2; 16]);
    assert!(matches!(
        manager.profile(&Caller::Token(token.clone())).await,
        Err(Error::Denied)
    ));
    assert!(matches!(
        manager
            .enroll(&local, RequestId::new([1; 16]), user, &token, None)
            .await,
        Err(Error::Denied)
    ));
    manager
        .enroll(&operator, RequestId::new([1; 16]), user, &token, None)
        .await
        .unwrap();
    assert_eq!(
        manager
            .profile(&Caller::Token(token.clone()))
            .await
            .unwrap()
            .principal,
        user
    );
    assert!(matches!(
        manager.profile(&Caller::Local(user)).await,
        Err(Error::Denied)
    ));
    assert!(matches!(
        manager
            .enroll(
                &operator,
                RequestId::new([2; 16]),
                PrincipalId::new([3; 16]),
                &token,
                None
            )
            .await,
        Err(Error::Denied)
    ));
    manager
        .revoke(&operator, RequestId::new([3; 16]), token.id())
        .await
        .unwrap();
    assert!(matches!(
        manager.profile(&Caller::Token(token)).await,
        Err(Error::Denied)
    ));
}
fn png() -> Vec<u8> {
    use base64::Engine as _;
    base64::engine::general_purpose::STANDARD.decode("iVBORw0KGgoAAAANSUhEUgAAAAEAAAABCAYAAAAfFcSJAAAADUlEQVR4nGP4z8DwHwAFAAH/iZk9HQAAAABJRU5ErkJggg==").unwrap()
}
#[tokio::test]
async fn avatar_upload_receipts_survive_lost_acknowledgement_and_keep_source_identity() {
    let store = Arc::new(LostAcknowledgement {
        inner: Default::default(),
        lost: true.into(),
        unreadable: false.into(),
    });
    let actor = PrincipalId::new([1; 16]);
    let manager = Manager::new(store.clone(), "ab".repeat(16), Some(actor)).unwrap();
    let command = request(71, DirectoryCommand::UploadImage { bytes: png() });
    assert!(matches!(
        manager
            .execute(&Caller::Local(actor), command.clone())
            .await,
        Err(Error::Unknown)
    ));
    let restarted = Manager::new(store, "ab".repeat(16), Some(actor)).unwrap();
    let receipt = restarted
        .execute(&Caller::Local(actor), command)
        .await
        .unwrap();
    assert_eq!(
        receipt.result,
        restarted
            .receipt(&Caller::Local(actor), RequestId::new([71; 16]))
            .await
            .unwrap()
            .unwrap()
            .result
    );
    assert!(matches!(
        restarted
            .execute(
                &Caller::Local(actor),
                request(
                    71,
                    DirectoryCommand::UploadImage {
                        bytes: b"changed source".to_vec()
                    }
                )
            )
            .await,
        Err(Error::Invalid)
    ));
    let DirectoryResult::Media { asset, .. } = receipt.result else {
        panic!()
    };
    assert_eq!(
        restarted
            .media(&Caller::Local(actor), &asset)
            .await
            .unwrap()
            .bytes,
        png()
    );
}
#[tokio::test]
async fn oversized_emoji_uploads_normalize_before_explicit_library_save_and_restart() {
    let (store, manager, local, _) = fixture();
    let mut bytes = std::io::Cursor::new(Vec::new());
    image::DynamicImage::ImageRgb8(image::RgbImage::from_pixel(
        2500,
        1800,
        image::Rgb([60, 90, 150]),
    ))
    .write_to(&mut bytes, image::ImageFormat::Jpeg)
    .unwrap();
    let mut bytes = bytes.into_inner();
    bytes.resize(2 * 1024 * 1024 + 1, 0);
    assert!(matches!(
        manager
            .execute(
                &local,
                request(
                    74,
                    DirectoryCommand::Upload {
                        bytes: bytes.clone()
                    }
                )
            )
            .await,
        Err(Error::Invalid)
    ));
    let receipt = manager
        .execute(&local, request(75, DirectoryCommand::UploadImage { bytes }))
        .await
        .unwrap();
    let DirectoryResult::Media { asset, .. } = receipt.result else {
        panic!()
    };
    assert!(manager.emojis(&local).await.unwrap().is_empty());
    manager
        .execute(
            &local,
            request(
                76,
                DirectoryCommand::SaveEmoji {
                    id: RequestId::new([77; 16]),
                    expected: None,
                    shortcode: "large-party".into(),
                    asset: asset.clone(),
                },
            ),
        )
        .await
        .unwrap();
    let restarted = Manager::new(store, "ab".repeat(16), Some(PrincipalId::new([1; 16]))).unwrap();
    let emojis = restarted.emojis(&local).await.unwrap();
    assert_eq!(emojis.len(), 1);
    assert_eq!(emojis[0].asset, asset);
    let media = restarted.media(&local, &asset).await.unwrap();
    let image = image::load_from_memory(&media.bytes).unwrap();
    assert_eq!((image.width(), image.height()), (512, 369));
    restarted.audit().await.unwrap();
}
#[tokio::test]
async fn unselected_records_are_invisible_and_corruption_fails_closed() {
    let (store, manager, local, _) = fixture();
    manager
        .stage(
            "profile",
            &SavedProfile {
                principal: PrincipalId::new([1; 16]),
                details: ProfileDetails::default(),
            },
        )
        .await
        .unwrap();
    assert!(manager.profile(&local).await.unwrap().revision.is_none());
    manager
        .execute(&local, request(1, profile(None, "Owner")))
        .await
        .unwrap();
    store.inject(Fault::ReadCorrupt);
    assert!(matches!(manager.profile(&local).await, Err(Error::Corrupt)));
}
#[tokio::test]
async fn emoji_uploads_are_owned_and_retirement_preserves_bytes() {
    let (_, manager, local, operator) = fixture();
    let token = BearerToken::generate().unwrap();
    manager
        .enroll(
            &operator,
            RequestId::new([11; 16]),
            PrincipalId::new([2; 16]),
            &token,
            None,
        )
        .await
        .unwrap();
    let other = Caller::Token(token);
    let upload = manager
        .execute(
            &local,
            request(1, DirectoryCommand::Upload { bytes: png() }),
        )
        .await
        .unwrap();
    let DirectoryResult::Media { asset, preview: _ } = upload.result else {
        panic!()
    };
    assert!(matches!(
        manager.media(&other, &asset).await,
        Err(Error::Denied)
    ));
    let id = RequestId::new([5; 16]);
    let result = manager
        .execute(
            &local,
            request(
                2,
                DirectoryCommand::SaveEmoji {
                    id,
                    expected: None,
                    shortcode: "party".into(),
                    asset: asset.clone(),
                },
            ),
        )
        .await
        .unwrap();
    let DirectoryResult::Emoji { emoji } = result.result else {
        panic!()
    };
    assert!(manager.media(&other, &asset).await.is_ok());
    assert!(matches!(
        manager
            .execute(
                &other,
                request(
                    3,
                    DirectoryCommand::RetireEmoji {
                        id,
                        expected: emoji.revision.clone()
                    }
                )
            )
            .await,
        Err(Error::Denied)
    ));
    manager
        .execute(
            &local,
            request(
                4,
                DirectoryCommand::RetireEmoji {
                    id,
                    expected: emoji.revision,
                },
            ),
        )
        .await
        .unwrap();
    assert!(manager.emojis(&other).await.unwrap().is_empty());
    assert!(manager.emojis(&local).await.unwrap()[0].retired);
    assert!(manager.media(&other, &asset).await.is_ok());
}

struct LostAcknowledgement {
    inner: Arc<MemoryStore>,
    lost: std::sync::atomic::AtomicBool,
    unreadable: std::sync::atomic::AtomicBool,
}
#[async_trait::async_trait]
impl ObjectStore for LostAcknowledgement {
    async fn head(&self, key: &Key) -> jj_store_object::Result<jj_store_object::Metadata> {
        self.inner.head(key).await
    }
    async fn get(
        &self,
        key: &Key,
        range: Option<jj_store_object::ByteRange>,
    ) -> jj_store_object::Result<jj_store_object::Download> {
        if key.as_str().ends_with("manifest.json")
            && self
                .unreadable
                .swap(false, std::sync::atomic::Ordering::SeqCst)
        {
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
        let result = self.inner.put(key, condition, upload).await?;
        if key.as_str().ends_with("manifest.json")
            && self.lost.swap(false, std::sync::atomic::Ordering::SeqCst)
        {
            self.unreadable
                .store(true, std::sync::atomic::Ordering::SeqCst);
            return Ok(WriteOutcome::Unknown);
        }
        Ok(result)
    }
}
#[tokio::test]
async fn manifest_lost_acknowledgement_recovers_original_receipt_after_restart() {
    let store = Arc::new(LostAcknowledgement {
        inner: Default::default(),
        lost: true.into(),
        unreadable: false.into(),
    });
    let actor = PrincipalId::new([1; 16]);
    let manager = Manager::new(store.clone(), "ab".repeat(16), Some(actor)).unwrap();
    let command = request(1, profile(None, "Original"));
    assert!(matches!(
        manager
            .execute(&Caller::Local(actor), command.clone())
            .await,
        Err(Error::Unknown)
    ));
    let restarted = Manager::new(store, "ab".repeat(16), Some(actor)).unwrap();
    let selected = restarted.profile(&Caller::Local(actor)).await.unwrap();
    let receipt = restarted
        .execute(&Caller::Local(actor), command)
        .await
        .unwrap();
    assert_eq!(
        receipt.result,
        DirectoryResult::Profile { profile: selected }
    );
}
#[tokio::test]
async fn revoked_and_expired_tokens_cannot_rebind_another_identity() {
    let (_, manager, _, operator) = fixture();
    let token = BearerToken::generate().unwrap();
    let user = PrincipalId::new([2; 16]);
    manager
        .enroll(&operator, RequestId::new([31; 16]), user, &token, None)
        .await
        .unwrap();
    manager
        .revoke(&operator, RequestId::new([32; 16]), token.id())
        .await
        .unwrap();
    assert!(matches!(
        manager
            .enroll(
                &operator,
                RequestId::new([33; 16]),
                PrincipalId::new([3; 16]),
                &token,
                None
            )
            .await,
        Err(Error::Denied)
    ));
    manager
        .enroll(&operator, RequestId::new([34; 16]), user, &token, Some(1))
        .await
        .unwrap();
    assert!(matches!(
        manager.profile(&Caller::Token(token)).await,
        Err(Error::Denied)
    ));
}
#[tokio::test]
async fn shortcode_collisions_and_cross_owner_edits_never_publish() {
    let (_, manager, local, operator) = fixture();
    let token = BearerToken::generate().unwrap();
    let owner = PrincipalId::new([2; 16]);
    manager
        .enroll(&operator, RequestId::new([30; 16]), owner, &token, None)
        .await
        .unwrap();
    let other = Caller::Token(token);
    let bytes = png();
    let receipt = manager
        .execute(&local, request(31, DirectoryCommand::Upload { bytes }))
        .await
        .unwrap();
    let DirectoryResult::Media { asset, .. } = receipt.result else {
        panic!()
    };
    assert!(matches!(
        manager.media(&other, &asset).await,
        Err(Error::Denied)
    ));
    let emoji = manager
        .execute(
            &local,
            request(
                32,
                DirectoryCommand::SaveEmoji {
                    id: RequestId::new([41; 16]),
                    expected: None,
                    shortcode: "party".into(),
                    asset: asset.clone(),
                },
            ),
        )
        .await
        .unwrap();
    let DirectoryResult::Emoji { emoji } = emoji.result else {
        panic!()
    };
    assert!(matches!(
        manager
            .execute(
                &other,
                request(
                    33,
                    DirectoryCommand::RetireEmoji {
                        id: emoji.id,
                        expected: emoji.revision.clone()
                    }
                )
            )
            .await,
        Err(Error::Denied)
    ));
    assert!(matches!(
        manager
            .execute(
                &other,
                request(
                    34,
                    DirectoryCommand::SaveEmoji {
                        id: RequestId::new([42; 16]),
                        expected: None,
                        shortcode: "party".into(),
                        asset
                    }
                )
            )
            .await,
        Err(Error::Stale)
    ));
    let snapshot = manager
        .emoji_snapshot(&other, emoji.id, &emoji.revision)
        .await
        .unwrap();
    manager
        .execute(
            &local,
            request(
                35,
                DirectoryCommand::RetireEmoji {
                    id: emoji.id,
                    expected: emoji.revision.clone(),
                },
            ),
        )
        .await
        .unwrap();
    assert!(matches!(
        manager
            .emoji_snapshot(&other, emoji.id, &emoji.revision)
            .await,
        Err(Error::Stale)
    ));
    assert!(
        snapshot
            .snapshot
            .stage(&mut jj_collab::CollabGraphV1::default())
            .is_ok()
    );
    manager.audit().await.unwrap();
}
#[tokio::test]
async fn malformed_avatars_images_and_missing_selected_closure_fail_closed() {
    let (store, manager, local, _) = fixture();
    for avatar in [
        "http://example.com/a.png",
        "https://user:password@example.com/a.png",
        "data:image/svg+xml,%3Csvg%20onload%3D%22alert(1)%22%3E",
    ] {
        let mut details = ProfileDetails::default();
        details.avatar = Some(avatar.into());
        assert!(matches!(
            manager
                .execute(
                    &local,
                    request(
                        40,
                        DirectoryCommand::SaveProfile {
                            expected: None,
                            details
                        }
                    )
                )
                .await,
            Err(Error::Invalid)
        ));
    }
    assert!(matches!(
        manager
            .execute(
                &local,
                request(
                    41,
                    DirectoryCommand::Upload {
                        bytes: b"<svg/>".to_vec()
                    }
                )
            )
            .await,
        Err(Error::Invalid)
    ));
    manager
        .execute(&local, request(42, profile(None, "Owner")))
        .await
        .unwrap();
    let selected = manager.selection().await.unwrap();
    let reference = selected.root.profiles.values().next().unwrap();
    let key = manager.key(&format!("objects/{reference}.json")).unwrap();
    let version = store.head(&key).await.unwrap().version;
    let corrupt = StagedUpload::from_bytes(b"corrupt", 100).await.unwrap();
    store
        .put(&key, Condition::Matches(version), &corrupt)
        .await
        .unwrap();
    assert!(manager.audit().await.is_err());
}

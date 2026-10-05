#[path = "support/review.rs"]
mod support;
use jj_review::{ReviewDeltaV1, ReviewRootV1};
use jj_store_wal::publication::{ReviewCatalogEditor, ReviewObjectIo, ReviewSnapshotRead};
struct ForeignIo(std::sync::atomic::AtomicUsize);
#[async_trait::async_trait]
impl ReviewObjectIo for ForeignIo {
    fn repository(&self) -> jj_store_model::RepositoryId {
        jj_store_model::RepositoryId::new([9; 16])
    }
    async fn read_review_object(
        &self,
        _: jj_review::ReviewObjectRef,
    ) -> jj_store_wal::Result<jj_review::ReviewObjectV1> {
        self.0.fetch_add(1, std::sync::atomic::Ordering::Relaxed);
        Err(jj_store_wal::Error::Corrupt("unexpected foreign read"))
    }
    async fn stage_review_object(
        &self,
        _: &jj_review::ReviewObjectV1,
    ) -> jj_store_wal::Result<jj_review::ReviewObjectRef> {
        self.0.fetch_add(1, std::sync::atomic::Ordering::Relaxed);
        Err(jj_store_wal::Error::Corrupt("unexpected foreign stage"))
    }
}

#[tokio::test]
async fn shared_review_catalog_preserves_native_root_and_publication_boundaries() {
    let coordinator = support::fixture().await;
    let manifest = coordinator.read().await.unwrap();
    let initial = coordinator
        .review_store(manifest.manifest().review_root.unwrap())
        .await
        .unwrap();
    let thread = support::stage_thread(&coordinator, 30, None).await;
    let delta = ReviewDeltaV1 {
        schema_version: 1,
        edits: support::edits(&thread),
    };
    let root: ReviewRootV1 = ReviewCatalogEditor::new(&coordinator)
        .apply_delta(&initial, &delta)
        .await
        .unwrap();
    let native = coordinator
        .stage_review_delta(&initial, &delta)
        .await
        .unwrap();
    assert_eq!(
        &root,
        coordinator.review_store(native).await.unwrap().root()
    );
    assert_eq!(
        ReviewSnapshotRead::repository(&initial),
        support::REPOSITORY
    );
    assert_eq!(
        ReviewObjectIo::repository(&coordinator),
        support::REPOSITORY
    );
    let after = coordinator.read().await.unwrap();
    assert_eq!(
        after.manifest().review_root,
        manifest.manifest().review_root
    );
    assert_eq!(after.manifest().published, manifest.manifest().published);
    let foreign = ForeignIo(std::sync::atomic::AtomicUsize::new(0));
    assert!(matches!(
        ReviewCatalogEditor::new(&foreign)
            .apply_delta(&initial, &delta)
            .await,
        Err(jj_store_wal::Error::Corrupt("review store repository"))
    ));
    assert_eq!(foreign.0.load(std::sync::atomic::Ordering::Relaxed), 0);
}

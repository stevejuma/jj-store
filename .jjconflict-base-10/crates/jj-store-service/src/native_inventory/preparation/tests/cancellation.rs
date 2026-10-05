use super::*;
use jj_store_object as object;
use std::sync::atomic::{AtomicU8, Ordering};
use tokio::sync::Notify;

struct Interrupted {
    inner: Arc<jj_store_object::memory::MemoryStore>,
    mode: AtomicU8,
    entered: Notify,
}
#[async_trait::async_trait]
impl object::ObjectStore for Interrupted {
    async fn head(&self, key: &Key) -> object::Result<object::Metadata> {
        self.inner.head(key).await
    }
    async fn put(
        &self,
        key: &Key,
        condition: Condition,
        upload: &StagedUpload,
    ) -> object::Result<WriteOutcome> {
        self.inner.put(key, condition, upload).await
    }
    async fn get(
        &self,
        key: &Key,
        range: Option<object::ByteRange>,
    ) -> object::Result<object::Download> {
        if key.as_str().contains("/inventories/") {
            match self.mode.load(Ordering::SeqCst) {
                1 => return Err(object::Error::Transport),
                2 => {
                    self.entered.notify_one();
                    std::future::pending::<()>().await;
                }
                _ => {}
            }
        }
        self.inner.get(key, range).await
    }
}

#[tokio::test]
async fn interrupted_kind_windows_retain_conservative_work_without_advancing_or_promoting() {
    let f = fixture(0).await;
    let store = Arc::new(Interrupted {
        inner: f.store.clone(),
        mode: AtomicU8::new(1),
        entered: Notify::new(),
    });
    let session = crate::native::NativeSession::new_local_trusted(
        store.clone(),
        f.objects.clone(),
        m::PrincipalId::new([1; 16]),
        Permission::Read,
        Arc::new(SystemClock),
        Protection::default(),
        SessionLimits::default(),
    )
    .unwrap();
    let mut task = session
        .begin_kind_preparation(false, Limits::default())
        .await
        .unwrap();
    assert!(
        session
            .advance_kind_preparation(&mut task, deadline())
            .await
            .is_err()
    );
    assert_eq!(task.stats.descriptor_work, 256);
    assert_eq!(task.stats.record_reads, WINDOW_READS);
    assert_eq!(task.stats.record_bytes, WINDOW_BYTES);
    assert!(task.after.is_none() && task.ids.is_empty() && task.ready.is_none());
    store.mode.store(2, Ordering::SeqCst);
    let mut advancing = Box::pin(session.advance_kind_preparation(&mut task, deadline()));
    tokio::select! {
        _ = tokio::time::sleep(Duration::from_secs(1)) => panic!("pending descriptor read not reached"),
        result = &mut advancing => panic!("pending window unexpectedly completed: {}", result.is_ok()),
        _ = store.entered.notified() => {},
    }
    drop(advancing);
    assert_eq!(task.stats.descriptor_work, 512);
    assert_eq!(task.stats.record_reads, 2 * WINDOW_READS);
    assert_eq!(task.stats.record_bytes, 2 * WINDOW_BYTES);
    assert!(task.after.is_none() && task.ids.is_empty() && task.ready.is_none());
    assert!(matches!(
        session.finish_kind_preparation(&task, deadline()).await,
        Err(q::Error::IndexRebuildRequired)
    ));
    store.mode.store(0, Ordering::SeqCst);
    assert!(
        session
            .advance_kind_preparation(&mut task, deadline())
            .await
            .unwrap()
            .complete
    );
    assert_eq!(task.stats.windows, 3);
    assert_eq!(task.stats.descriptor_work, 512 + task.stats.scanned);
    assert!(
        task.stats.record_reads > 2 * WINDOW_READS && task.stats.record_bytes > 2 * WINDOW_BYTES
    );
    assert_eq!(
        session
            .finish_kind_preparation(&task, deadline())
            .await
            .unwrap()
            .len(),
        2
    );
}

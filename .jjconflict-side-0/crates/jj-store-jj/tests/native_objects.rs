use jj_lib::backend::Tree;
use jj_store_jj::{
    codec::NativeCodec as _,
    objects::{NativeObjects, UploadStatus},
};
use jj_store_model::{self as m, envelope};
use jj_store_object::{
    memory::{Fault, MemoryStore},
    *,
};
use std::sync::Arc;
use tokio::io::AsyncReadExt as _;
fn objects(store: Arc<MemoryStore>) -> NativeObjects {
    NativeObjects::new(store, m::RepositoryId::new([1; 16]), 10 * 1024 * 1024).unwrap()
}
#[tokio::test]
async fn immutable_metadata_handles_duplicates_unknowns_corruption_and_absence() {
    let store = Arc::new(MemoryStore::default());
    let native = objects(store.clone());
    let tree = Tree::default();
    store.inject(Fault::BeforeWrite);
    assert_eq!(native.put(&tree).await.unwrap().1, UploadStatus::Unknown);
    store.inject(Fault::AfterWrite);
    assert_eq!(
        native.put(&tree).await.unwrap().1,
        UploadStatus::AlreadyPresent
    );
    assert_eq!(
        native.put(&tree).await.unwrap().1,
        UploadStatus::AlreadyPresent
    );
    assert_eq!(native.get::<Tree>(tree.key()).await.unwrap(), tree);
    store.inject(Fault::ReadMissing);
    assert!(native.get::<Tree>(tree.key()).await.is_err());
    store.inject(Fault::ReadCorrupt);
    assert!(native.get::<Tree>(tree.key()).await.is_err());
}
#[tokio::test]
async fn streamed_binary_blobs_match_codec_and_resolve_lost_acknowledgements() {
    let store = Arc::new(MemoryStore::default());
    let native = objects(store.clone());
    let input: Vec<_> = (0..2 * 1024 * 1024).map(|i| i as u8).collect();
    let (id, status) = native.put_file(input.as_slice()).await.unwrap();
    assert_eq!(status, UploadStatus::Stored);
    assert_eq!(
        m::ObjectKey::File(id),
        jj_store_jj::codec::NativeFile(input.clone()).key()
    );
    let raw = native.get_file(id).await.unwrap();
    assert_eq!(raw.len(), input.len() as u64);
    let mut bytes = Vec::new();
    raw.reader().unwrap().read_to_end(&mut bytes).await.unwrap();
    assert_eq!(bytes, input);
    assert_eq!(
        native.put_file(input.as_slice()).await.unwrap().1,
        UploadStatus::AlreadyPresent
    );
    let encoded = store
        .get(
            &Key::new(m::ObjectKey::File(id).storage_path(m::RepositoryId::new([1; 16]))).unwrap(),
            None,
        )
        .await
        .unwrap()
        .read_bounded(3 * 1024 * 1024)
        .await
        .unwrap();
    assert_eq!(
        envelope::decode(&encoded, m::ObjectKey::File(id), 3 * 1024 * 1024).unwrap(),
        input
    );
    store.inject(Fault::AfterWrite);
    assert_eq!(
        native.put_file(b"lost ack".as_slice()).await.unwrap().1,
        UploadStatus::AlreadyPresent
    );
    store.inject(Fault::BeforeWrite);
    assert_eq!(
        native.put_file(b"before apply".as_slice()).await.unwrap().1,
        UploadStatus::Unknown
    );
    store.inject(Fault::ReadCorrupt);
    assert!(native.get_file(id).await.is_err());
}
#[tokio::test]
async fn collision_with_forged_native_identity_is_not_a_successful_duplicate() {
    let store = Arc::new(MemoryStore::default());
    let native = objects(store.clone());
    let tree = Tree::default();
    let key = tree.key();
    let path = Key::new(key.storage_path(m::RepositoryId::new([1; 16]))).unwrap();
    let forged = envelope::encode(
        key,
        b"{\"entries\":[[\"module\",{\"type\":\"submodule\",\"id\":[1]}]]}",
        1024,
    )
    .unwrap();
    let upload = StagedUpload::from_bytes(&forged, 1024).await.unwrap();
    store.put(&path, Condition::Absent, &upload).await.unwrap();
    assert!(native.put(&tree).await.is_err());
    let limit = NativeObjects::new(store, m::RepositoryId::new([1; 16]), 4).unwrap();
    assert!(limit.put_file(b"12345".as_slice()).await.is_err());
}

#[tokio::test]
async fn streamed_reader_rejects_late_corruption_truncation_and_forged_identity() {
    use jj_store_jj::codec::{self, NativeFile};
    let value = NativeFile(b"verified bytes".to_vec());
    let (key, valid) = codec::encode(&value, 100).unwrap();
    let m::ObjectKey::File(id) = key else {
        unreachable!()
    };
    let mut corrupt = valid.clone();
    *corrupt.last_mut().unwrap() ^= 1;
    let mut extra = valid.clone();
    extra.push(0);
    let forged = envelope::encode(key, b"different data", 100).unwrap();
    for bytes in [corrupt, valid[..valid.len() - 1].to_vec(), extra, forged] {
        let store = Arc::new(MemoryStore::default());
        let path = Key::new(key.storage_path(m::RepositoryId::new([1; 16]))).unwrap();
        store
            .put(
                &path,
                Condition::Absent,
                &StagedUpload::from_bytes(&bytes, 1024).await.unwrap(),
            )
            .await
            .unwrap();
        assert!(objects(store).get_file(id).await.is_err());
    }
}

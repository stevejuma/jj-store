use super::*;
use crate::{
    auth::{Permission, SystemClock},
    native::{SessionLimits, create_repository_local_trusted},
};
use jj_store_jj::{objects::NativeObjects, policy::Protection, roots};
use jj_store_object::{Condition, Key, ObjectStore as _, StagedUpload, WriteOutcome};
use query::{InventoryKind as K, ReadAccess as _};
use sha2::{Digest as _, Sha256};
use wal::{
    catalog::Catalog,
    inventory::DependencySource as _,
    records::{RecordKind, Records},
};

#[tokio::test]
async fn prepared_kind_lookup_outlives_page_deadline_without_full_inventory() {
    let repository = m::RepositoryId::new([69; 16]);
    let store = Arc::new(jj_store_object::memory::MemoryStore::default());
    let objects = Arc::new(NativeObjects::new(store.clone(), repository, 8 * 1024 * 1024).unwrap());
    create_repository_local_trusted(
        store.clone(),
        objects.clone(),
        Permission::Admin,
        m::EpochId::new([1; 16]),
        wal::publication::Limits::default(),
    )
    .await
    .unwrap();
    // Test-only descriptor selection isolates provider result admission from
    // publication policy, using a real verified native operation body.
    let mut operation = roots::operation();
    operation.metadata.description = "preparation matching bound".into();
    let (key, _) = objects.put(&operation).await.unwrap();
    let node = objects.load_verified(key, 8 * 1024 * 1024).await.unwrap();
    let records = Records::new(store.clone(), repository);
    let reference = records
        .put(RecordKind::Inventory, None, &node)
        .await
        .unwrap();
    let mut hash = Sha256::new_with_prefix(b"jj-store/object/v1");
    hash.update([key.kind() as u8]);
    hash.update(key.as_bytes());
    let mut manifest = wal::publication::ManifestSnapshot::load(store.clone(), repository)
        .await
        .unwrap()
        .manifest()
        .clone();
    manifest.inventory = Catalog::new(records)
        .update(
            manifest.inventory,
            std::collections::BTreeMap::from([(
                m::RecordDigest::new(hash.finalize().into()),
                reference,
            )]),
        )
        .await
        .unwrap();
    manifest.revision += 1;
    let path = Key::new(format!("repos/{repository}/manifest.json")).unwrap();
    let version = store.head(&path).await.unwrap().version;
    let upload = StagedUpload::from_bytes(&serde_json::to_vec(&manifest).unwrap(), 128 * 1024)
        .await
        .unwrap();
    assert!(matches!(
        store
            .put(&path, Condition::Matches(version), &upload)
            .await
            .unwrap(),
        WriteOutcome::Applied(_)
    ));
    let session = Arc::new(
        NativeSession::new_local_trusted(
            store.clone(),
            objects,
            m::PrincipalId::new([1; 16]),
            Permission::Read,
            Arc::new(SystemClock),
            Protection::default(),
            SessionLimits::default(),
        )
        .unwrap(),
    );
    let OpenRead::Pinned(mut read) = session
        .open_read(
            None,
            ReadLimits {
                max_objects: 1,
                ..ReadLimits::default()
            },
        )
        .await
        .unwrap()
    else {
        panic!("single head")
    };
    let task = Access::capture(&read).await.unwrap();
    let deadline = || Instant::now() + Duration::from_secs(10);
    // Optional acknowledged staged seeds need no committed descriptor. The
    // preparation capability uses its caller's admission rather than the page's
    // one-object bound; neither capability can claim these as selected copies.
    let staged = BTreeSet::from([m::CopyId::new([7; 64]), m::CopyId::new([8; 64])]);
    let family_limits = query::copy_index::LookupLimits {
        max_nodes: 8,
        ..Default::default()
    };
    assert!(matches!(
        read.copy_family(&staged, &BTreeSet::new(), family_limits, deadline())
            .await,
        Err(query::Error::Limit)
    ));
    assert!(
        task.copy_family(&staged, &BTreeSet::new(), family_limits, deadline())
            .await
            .unwrap()
            .unwrap()
            .ids
            .is_empty()
    );
    let expected = BTreeSet::from([m::ObjectKey::Operation(roots::ROOT_OPERATION), key]);
    assert_eq!(
        read.inventory_kind(K::Operations, 8, deadline())
            .await
            .unwrap_err(),
        query::Error::Limit
    );
    assert_eq!(
        task.inventory_kind(K::Operations, 8, deadline())
            .await
            .unwrap(),
        expected
    );
    read.deadline = Instant::now();
    assert!(matches!(
        read.copy_family(&staged, &BTreeSet::new(), family_limits, deadline())
            .await,
        Err(query::Error::Limit)
    ));
    assert!(
        task.copy_family(&staged, &BTreeSet::new(), family_limits, deadline())
            .await
            .unwrap()
            .unwrap()
            .ids
            .is_empty()
    );
    assert_eq!(
        read.inventory_kind(K::Operations, 8, deadline())
            .await
            .unwrap_err(),
        query::Error::Limit
    );
    assert_eq!(
        task.inventory().await.unwrap_err(),
        query::Error::IndexBudget
    );
    assert_eq!(
        task.inventory_kind(K::Operations, 8, deadline())
            .await
            .unwrap(),
        expected
    );
    assert!(
        task.inventory_kind(K::Copies, 1, deadline())
            .await
            .unwrap()
            .is_empty()
    );
    assert_eq!(
        task.inventory_kind(K::Operations, 0, deadline())
            .await
            .unwrap_err(),
        query::Error::Limit
    );
    assert_eq!(
        task.inventory_kind(K::Operations, 1, deadline())
            .await
            .unwrap_err(),
        query::Error::Limit
    );
    assert_eq!(
        task.inventory_kind(K::Operations, 8, Instant::now())
            .await
            .unwrap_err(),
        query::Error::Limit
    );

    // Test-only epoch replacement exercises retained reader invalidation;
    // neither public browser input nor this provider can select a manifest.
    let mut manifest = session.snapshot().await.unwrap().manifest().clone();
    manifest.epoch = m::EpochId::new([2; 16]);
    let path = Key::new(format!("repos/{repository}/manifest.json")).unwrap();
    let version = store.head(&path).await.unwrap().version;
    let upload = StagedUpload::from_bytes(&serde_json::to_vec(&manifest).unwrap(), 128 * 1024)
        .await
        .unwrap();
    assert!(matches!(
        store
            .put(&path, Condition::Matches(version), &upload)
            .await
            .unwrap(),
        WriteOutcome::Applied(_)
    ));
    read.deadline = deadline();
    assert!(matches!(
        task.copy_family(&staged, &BTreeSet::new(), family_limits, deadline())
            .await,
        Err(query::Error::EpochChanged)
    ));
    assert_eq!(
        read.inventory_kind(K::Operations, 1, deadline())
            .await
            .unwrap_err(),
        query::Error::EpochChanged
    );
    assert_eq!(
        task.inventory_kind(K::Operations, 1, deadline())
            .await
            .unwrap_err(),
        query::Error::EpochChanged
    );
}

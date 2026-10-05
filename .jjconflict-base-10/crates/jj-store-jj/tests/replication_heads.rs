use jj_lib::op_store as native;
use jj_store_jj::{objects::NativeObjects, replication::NativeOperationHeadReducer, roots};
use jj_store_model::{ObjectKey, OperationId, RepositoryId};
use jj_store_object::memory::MemoryStore;
use jj_store_wal::publication::OperationHeadReducer;
use std::{collections::BTreeSet, sync::Arc};

fn operation(parent: OperationId, label: &str) -> native::Operation {
    let mut operation = roots::operation();
    operation.parents = vec![native::OperationId::from_bytes(parent.as_bytes())];
    operation.metadata.description = label.to_owned();
    operation
}

async fn put_operation(objects: &NativeObjects, operation: &native::Operation) -> OperationId {
    let (ObjectKey::Operation(id), _) = objects.put(operation).await.unwrap() else {
        panic!("operation codec returned another object kind");
    };
    id
}

#[tokio::test]
async fn imported_heads_use_pinned_operation_ancestry_and_keep_siblings() {
    let objects = Arc::new(
        NativeObjects::new(
            Arc::new(MemoryStore::default()),
            RepositoryId::new([1; 16]),
            1024 * 1024,
        )
        .unwrap(),
    );
    let first = put_operation(&objects, &operation(roots::ROOT_OPERATION, "first")).await;
    let descendant = put_operation(&objects, &operation(first, "descendant")).await;
    let sibling = put_operation(&objects, &operation(roots::ROOT_OPERATION, "sibling")).await;
    let reducer = NativeOperationHeadReducer::new(objects);
    assert_eq!(
        reducer
            .maximal_heads(BTreeSet::from([
                roots::ROOT_OPERATION,
                first,
                descendant,
                sibling,
            ]))
            .await
            .unwrap(),
        BTreeSet::from([descendant, sibling])
    );
}

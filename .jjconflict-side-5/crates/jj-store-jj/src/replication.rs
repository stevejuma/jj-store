//! Pinned JJ operation ancestry for imported publication heads.
use crate::{objects::NativeObjects, roots};
use jj_lib::{dag_walk_async, object_id::ObjectId as _, op_store as native};
use jj_store_model::{ObjectKey, OperationId};
use jj_store_wal::{self as wal, publication::OperationHeadReducer};
use std::{collections::BTreeSet, sync::Arc};

pub struct NativeOperationHeadReducer {
    objects: Arc<NativeObjects>,
}

impl NativeOperationHeadReducer {
    pub fn new(objects: Arc<NativeObjects>) -> Self {
        Self { objects }
    }

    async fn parents(
        objects: Arc<NativeObjects>,
        id: OperationId,
    ) -> wal::Result<Vec<OperationId>> {
        if id == roots::ROOT_OPERATION {
            return Ok(Vec::new());
        }
        let operation: native::Operation = objects
            .get(ObjectKey::Operation(id))
            .await
            .map_err(|_| wal::Error::Corrupt("imported operation ancestry"))?;
        operation
            .parents
            .iter()
            .map(|parent| {
                OperationId::from_slice(parent.as_bytes())
                    .map_err(|_| wal::Error::Corrupt("imported operation parent"))
            })
            .collect()
    }
}

#[async_trait::async_trait]
impl OperationHeadReducer for NativeOperationHeadReducer {
    async fn verify_git(
        &self,
        staged: &wal::publication::GitStoreSnapshot,
        previous: Option<&wal::publication::GitStoreSnapshot>,
        accepted_code: Option<&BTreeSet<OperationId>>,
    ) -> wal::Result<()> {
        if accepted_code.is_some()
            && let Some(previous) = previous
        {
            staged.verify_event_admission(previous)?;
        }
        for (key, record) in &staged.values {
            if previous.is_some_and(|old| old.values.contains_key(key)) {
                continue;
            }
            if let wal::publication::GitRecordV1::PushIntent(intent) = record
                && let Some(accepted_code) = accepted_code
            {
                crate::git_bridge::verify_push_source(self.objects.clone(), accepted_code, intent)
                    .await
                    .map_err(|_| wal::Error::Corrupt("imported Git push source"))?;
            }
        }
        crate::git_bridge::verify_delta(self.objects.clone(), staged, previous)
            .await
            .map_err(|_| wal::Error::Corrupt("imported Git representation"))
    }
    async fn maximal_heads(
        &self,
        candidates: BTreeSet<OperationId>,
    ) -> wal::Result<BTreeSet<OperationId>> {
        let objects = self.objects.clone();
        let heads = dag_walk_async::heads(
            candidates,
            |id| *id,
            async move |&id| Self::parents(objects.clone(), id).await,
        )
        .await?;
        Ok(heads.into_iter().collect())
    }
}

use super::*;
use jj_store_wal::publication::{
    AgentCatalogKey, AgentCatalogKind, AgentCheckpointId, AgentObjectId, AgentObjectRef,
    AgentObjectV1,
};
use std::collections::BTreeSet;

const MAX_VERSIONS: usize = 4096;

impl AgentSessionPager {
    pub async fn begin_checkpoints(
        &self,
        session: Arc<NativeSession>,
        limit: usize,
    ) -> Result<AgentCheckpointPage> {
        valid_limit(limit)?;
        let _permit = self.cursors.acquire()?;
        within_timeout(
            self.cursors.timeout(),
            self.begin_checkpoint_page(session, limit),
        )
        .await
    }

    async fn begin_checkpoint_page(
        &self,
        session: Arc<NativeSession>,
        limit: usize,
    ) -> Result<AgentCheckpointPage> {
        let pin = self.pin(&session).await?;
        let snapshot = pin.snapshot.clone();
        let page = pin
            .store
            .list_checkpoints(None, limit)
            .await
            .map_err(map_wal)?;
        let checkpoints = resolve_checkpoints(&pin.store, page.selections).await?;
        authorize_epoch(&session, snapshot.epoch).await?;
        let next_cursor = page
            .next_after
            .map(|after| {
                self.cursors
                    .retain(pin, limit, Position::Checkpoints { after })
            })
            .transpose()?;
        Ok(AgentCheckpointPage {
            version: 1,
            snapshot,
            checkpoints,
            next_cursor,
        })
    }

    pub async fn resume_checkpoints(
        &self,
        session: Arc<NativeSession>,
        cursor: &str,
        limit: usize,
    ) -> Result<AgentCheckpointPage> {
        valid_limit(limit)?;
        let _permit = self.cursors.acquire()?;
        within_timeout(
            self.cursors.timeout(),
            self.resume_checkpoint_page(session, cursor, limit),
        )
        .await
    }

    async fn resume_checkpoint_page(
        &self,
        session: Arc<NativeSession>,
        cursor: &str,
        limit: usize,
    ) -> Result<AgentCheckpointPage> {
        authorize(&session).await?;
        let (claims, entry) = self.cursors.lookup(session.repository, cursor, limit)?;
        let Position::Checkpoints { after } = claims.position else {
            return Err(Error::InvalidCursor);
        };
        authorize_epoch(&session, entry.pin.snapshot.epoch).await?;
        let page = entry
            .pin
            .store
            .list_checkpoints(Some(after), limit)
            .await
            .map_err(map_wal)?;
        let checkpoints = resolve_checkpoints(&entry.pin.store, page.selections).await?;
        authorize_epoch(&session, entry.pin.snapshot.epoch).await?;
        let next_cursor = self.cursors.next(
            &claims,
            &entry,
            page.next_after.map(|after| Position::Checkpoints { after }),
        )?;
        Ok(AgentCheckpointPage {
            version: 1,
            snapshot: entry.pin.snapshot,
            checkpoints,
            next_cursor,
        })
    }

    pub async fn checkpoint(
        &self,
        session: Arc<NativeSession>,
        checkpoint_id: AgentCheckpointId,
    ) -> Result<AgentCheckpointDetail> {
        self.checkpoint_selected(session, checkpoint_id, None).await
    }

    pub async fn checkpoint_version(
        &self,
        session: Arc<NativeSession>,
        checkpoint_id: AgentCheckpointId,
        version_id: AgentObjectId,
    ) -> Result<AgentCheckpointDetail> {
        self.checkpoint_selected(session, checkpoint_id, Some(version_id))
            .await
    }

    async fn checkpoint_selected(
        &self,
        session: Arc<NativeSession>,
        checkpoint_id: AgentCheckpointId,
        version_id: Option<AgentObjectId>,
    ) -> Result<AgentCheckpointDetail> {
        let _permit = self.cursors.acquire()?;
        within_timeout(
            self.cursors.timeout(),
            self.checkpoint_selected_inner(session, checkpoint_id, version_id),
        )
        .await
    }

    async fn checkpoint_selected_inner(
        &self,
        session: Arc<NativeSession>,
        checkpoint_id: AgentCheckpointId,
        version_id: Option<AgentObjectId>,
    ) -> Result<AgentCheckpointDetail> {
        let pin = self.pin(&session).await?;
        let key = AgentCatalogKey::derive("checkpoint", checkpoint_id.as_bytes())
            .map_err(|_| Error::Corrupt)?;
        let mut reference = pin
            .store
            .lookup(AgentCatalogKind::Checkpoints, key)
            .await
            .map_err(map_wal)?
            .ok_or(Error::NotFound)?;
        let mut seen = BTreeSet::new();
        for _ in 0..MAX_VERSIONS {
            if !seen.insert(reference) {
                return Err(Error::Corrupt);
            }
            let AgentObjectV1::CheckpointVersion(checkpoint) =
                pin.store.read_object(reference).await.map_err(map_wal)?
            else {
                return Err(Error::Corrupt);
            };
            if checkpoint.checkpoint_id != checkpoint_id {
                return Err(Error::Corrupt);
            }
            if version_id.is_none_or(|id| reference.id == id) {
                authorize_epoch(&session, pin.snapshot.epoch).await?;
                return Ok(checkpoint_models::detail(
                    pin.snapshot,
                    reference,
                    checkpoint,
                ));
            }
            let Some(previous) = checkpoint.supersedes else {
                return Err(Error::NotFound);
            };
            reference = previous;
        }
        Err(Error::Limit)
    }
}

async fn resolve_checkpoints(
    store: &wal::publication::AgentStoreSnapshot,
    selections: Vec<(AgentCatalogKey, AgentObjectRef)>,
) -> Result<Vec<AgentCheckpointSummary>> {
    let mut rows = Vec::with_capacity(selections.len());
    for (key, reference) in selections {
        rows.push(resolve_checkpoint(store, key, reference).await?);
    }
    Ok(rows)
}

use super::*;
use jj_store_wal::publication::{AgentCatalogKey, AgentCatalogKind, AgentObjectV1, AgentSessionId};

impl AgentSessionPager {
    pub async fn detail(
        &self,
        session: Arc<NativeSession>,
        session_id: AgentSessionId,
    ) -> Result<AgentSessionDetail> {
        let _permit = self.cursors.acquire()?;
        within_timeout(
            self.cursors.timeout(),
            self.detail_inner(session, session_id),
        )
        .await
    }

    async fn detail_inner(
        &self,
        session: Arc<NativeSession>,
        session_id: AgentSessionId,
    ) -> Result<AgentSessionDetail> {
        let pin = self.pin(&session).await?;
        let key = AgentCatalogKey::derive("session", session_id.as_bytes())
            .map_err(|_| Error::Corrupt)?;
        let reference = pin
            .store
            .lookup(AgentCatalogKind::Sessions, key)
            .await
            .map_err(map_wal)?
            .ok_or(Error::NotFound)?;
        let AgentObjectV1::SessionSnapshot(snapshot) =
            pin.store.read_object(reference).await.map_err(map_wal)?
        else {
            return Err(Error::Corrupt);
        };
        if snapshot.session_id != session_id
            || snapshot.catalog_key().map_err(|_| Error::Corrupt)? != key
        {
            return Err(Error::Corrupt);
        }
        let mut transcripts = Vec::new();
        for transcript in &snapshot.transcripts {
            let AgentObjectV1::TranscriptManifest(manifest) =
                pin.store.read_object(*transcript).await.map_err(map_wal)?
            else {
                return Err(Error::Corrupt);
            };
            if manifest.session_id != session_id {
                return Err(Error::Corrupt);
            }
            transcripts.extend(
                manifest
                    .streams
                    .iter()
                    .map(|stream| models::transcript_descriptor(transcript.id, stream)),
            );
        }
        authorize_epoch(&session, pin.snapshot.epoch).await?;
        Ok(models::detail(
            pin.snapshot,
            reference,
            snapshot,
            transcripts,
        ))
    }
}

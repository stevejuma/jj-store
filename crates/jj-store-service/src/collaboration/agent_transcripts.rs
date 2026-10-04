use super::*;
use jj_store_wal::publication::{
    AgentCatalogKind, AgentObjectId, AgentObjectKind, AgentObjectRef, AgentObjectV1,
    AgentSessionId, SourceInstanceId,
};

const MAX_TRANSCRIPT_BYTES: usize = 8 * 1024 * 1024;

impl AgentSessionPager {
    pub async fn begin_transcript(
        &self,
        session: Arc<NativeSession>,
        request: AgentTranscriptRequest,
    ) -> Result<AgentTranscriptPage> {
        request.validate()?;
        let _permit = self.cursors.acquire()?;
        within_timeout(
            self.cursors.timeout(),
            self.begin_transcript_inner(session, request),
        )
        .await
    }

    async fn begin_transcript_inner(
        &self,
        session: Arc<NativeSession>,
        request: AgentTranscriptRequest,
    ) -> Result<AgentTranscriptPage> {
        let pin = self.pin(&session).await?;
        let (manifest, selected) = selected_transcript(&pin, &request).await?;
        let range = pin
            .store
            .transcript_range(
                manifest,
                request.source_instance_id,
                request.source_generation,
                None,
                request.limit,
                request.max_bytes,
            )
            .await
            .map_err(map_wal)?;
        let chunks = super::present_chunks(&pin.store, &selected, range.chunks).await?;
        authorize_epoch(&session, pin.snapshot.epoch).await?;
        let snapshot = pin.snapshot.clone();
        let next_cursor = range
            .next_after_sequence
            .map(|after_sequence| {
                self.cursors
                    .retain(pin, request.limit, request.position(after_sequence))
            })
            .transpose()?;
        transcript_page(snapshot, request, chunks, next_cursor)
    }

    pub async fn resume_transcript(
        &self,
        session: Arc<NativeSession>,
        cursor: &str,
        request: AgentTranscriptRequest,
    ) -> Result<AgentTranscriptPage> {
        request.validate()?;
        let _permit = self.cursors.acquire()?;
        within_timeout(
            self.cursors.timeout(),
            self.resume_transcript_inner(session, cursor, request),
        )
        .await
    }

    async fn resume_transcript_inner(
        &self,
        session: Arc<NativeSession>,
        cursor: &str,
        request: AgentTranscriptRequest,
    ) -> Result<AgentTranscriptPage> {
        authorize(&session).await?;
        let (claims, entry) = self
            .cursors
            .lookup(session.repository, cursor, request.limit)?;
        let Position::Transcript {
            session_id,
            transcript_id,
            source_instance_id,
            source_generation,
            max_bytes,
            after_sequence,
        } = claims.position
        else {
            return Err(Error::InvalidCursor);
        };
        if (
            session_id,
            transcript_id,
            source_instance_id,
            source_generation,
            max_bytes,
        ) != request.identity()
        {
            return Err(Error::InvalidCursor);
        }
        authorize_epoch(&session, entry.pin.snapshot.epoch).await?;
        let (manifest, selected) = selected_transcript(&entry.pin, &request).await?;
        let range = entry
            .pin
            .store
            .transcript_range(
                manifest,
                source_instance_id,
                source_generation,
                Some(after_sequence),
                request.limit,
                max_bytes,
            )
            .await
            .map_err(map_wal)?;
        let chunks = super::present_chunks(&entry.pin.store, &selected, range.chunks).await?;
        authorize_epoch(&session, entry.pin.snapshot.epoch).await?;
        let next_cursor = self.cursors.next(
            &claims,
            &entry,
            range
                .next_after_sequence
                .map(|sequence| request.position(sequence)),
        )?;
        transcript_page(entry.pin.snapshot, request, chunks, next_cursor)
    }
}

#[derive(Clone, Copy)]
pub struct AgentTranscriptRequest {
    pub session_id: AgentSessionId,
    pub transcript_id: AgentObjectId,
    pub source_instance_id: SourceInstanceId,
    pub source_generation: u64,
    pub limit: usize,
    pub max_bytes: usize,
}

impl AgentTranscriptRequest {
    fn validate(self) -> Result<()> {
        valid_limit(self.limit)?;
        if !(1..=MAX_TRANSCRIPT_BYTES).contains(&self.max_bytes) {
            return Err(Error::Limit);
        }
        Ok(())
    }

    fn identity(self) -> (AgentSessionId, AgentObjectId, SourceInstanceId, u64, usize) {
        (
            self.session_id,
            self.transcript_id,
            self.source_instance_id,
            self.source_generation,
            self.max_bytes,
        )
    }

    fn position(self, after_sequence: u64) -> Position {
        Position::Transcript {
            session_id: self.session_id,
            transcript_id: self.transcript_id,
            source_instance_id: self.source_instance_id,
            source_generation: self.source_generation,
            max_bytes: self.max_bytes,
            after_sequence,
        }
    }
}

async fn selected_transcript(
    pin: &AgentPin,
    request: &AgentTranscriptRequest,
) -> Result<(
    AgentObjectRef,
    jj_store_wal::publication::AgentSessionSnapshotV1,
)> {
    let key = AgentCatalogKey::derive("session", request.session_id.as_bytes())
        .map_err(|_| Error::Corrupt)?;
    let session = pin
        .store
        .lookup(AgentCatalogKind::Sessions, key)
        .await
        .map_err(map_wal)?
        .ok_or(Error::NotFound)?;
    let AgentObjectV1::SessionSnapshot(session) =
        pin.store.read_object(session).await.map_err(map_wal)?
    else {
        return Err(Error::Corrupt);
    };
    let transcript = session
        .transcripts
        .iter()
        .copied()
        .find(|value| value.id == request.transcript_id)
        .ok_or(Error::NotFound)?;
    if transcript.kind != AgentObjectKind::TranscriptManifest {
        return Err(Error::Corrupt);
    }
    let AgentObjectV1::TranscriptManifest(manifest) =
        pin.store.read_object(transcript).await.map_err(map_wal)?
    else {
        return Err(Error::Corrupt);
    };
    if session.session_id != request.session_id
        || session.catalog_key().map_err(|_| Error::Corrupt)? != key
        || manifest.session_id != request.session_id
    {
        return Err(Error::Corrupt);
    }
    if !manifest.streams.iter().any(|stream| {
        stream.source_instance_id == request.source_instance_id
            && stream.source_generation == request.source_generation
    }) {
        return Err(Error::NotFound);
    }
    Ok((transcript, session))
}

fn transcript_page(
    snapshot: AgentReadSnapshot,
    request: AgentTranscriptRequest,
    chunks: Vec<AgentTranscriptChunk>,
    next_cursor: Option<String>,
) -> Result<AgentTranscriptPage> {
    Ok(AgentTranscriptPage {
        version: 1,
        snapshot,
        session_id: request.session_id,
        transcript_id: request.transcript_id,
        source_instance_id: request.source_instance_id,
        source_generation: request.source_generation,
        chunks,
        next_cursor,
    })
}

use super::*;
impl LocalRepository {
    pub async fn browser_agent_transcript(
        &self,
        commit: MetadataCommit,
        request: LocalBrowserTranscriptRequest,
        after: Option<u64>,
    ) -> LocalResult<LocalBrowserTranscriptPage> {
        let store = self.browser_agent_store(commit).await?;
        let (_, session) = selected_session(&store, request.session_id).await?;
        let reference = session
            .transcripts
            .iter()
            .find(|r| r.id == request.transcript_id)
            .copied()
            .ok_or(LocalError::NotFound)?;
        let AgentObjectV1::TranscriptManifest(manifest) = store.object(reference).await? else {
            return Err(LocalError::Corrupt);
        };
        if manifest.session_id != request.session_id {
            return Err(LocalError::Corrupt);
        }
        let range = store
            .reads()
            .transcript_range(
                reference,
                request.source_instance_id,
                request.source_generation,
                after,
                request.limit,
                request.max_bytes,
            )
            .await?;
        let chunks = presentation::present_chunks(&store, &session, range.chunks)
            .await
            .map_err(map)?;
        Ok(LocalBrowserTranscriptPage {
            version: 1,
            snapshot: store.snapshot,
            session_id: request.session_id,
            transcript_id: request.transcript_id,
            source_instance_id: request.source_instance_id,
            source_generation: request.source_generation,
            chunks,
            next_after_sequence: range.next_after_sequence,
        })
    }
}

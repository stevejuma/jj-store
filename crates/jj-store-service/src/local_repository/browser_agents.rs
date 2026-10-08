//! Presentation of already recorded, independently companion-pinned agent data.
mod associations;
mod models;
mod transcripts;
use super::{LocalAgentStore, LocalError, LocalRepository, LocalResult, MetadataCommit};
use crate::collaboration as presentation;
use jj_store_model as m;
use jj_store_wal::publication::*;
pub use models::*;

impl LocalRepository {
    pub(crate) async fn browser_agent_store(
        &self,
        commit: MetadataCommit,
    ) -> LocalResult<LocalAgentStore> {
        let selected = self.selected_metadata(commit)?;
        self.agent_store_at(Some(&selected), selected.repository())
            .await
    }
    pub async fn browser_agent_sessions(
        &self,
        commit: MetadataCommit,
        limit: usize,
        after: Option<AgentCatalogKey>,
    ) -> LocalResult<LocalBrowserAgentPage> {
        if !(1..=256).contains(&limit) {
            return Err(LocalError::Invalid);
        }
        let store = self.browser_agent_store(commit).await?;
        let page = store.reads().list_sessions(after, limit).await?;
        let mut sessions = Vec::new();
        for (key, reference) in page.selections {
            let AgentObjectV1::SessionSnapshot(session) = store.object(reference).await? else {
                return Err(LocalError::Corrupt);
            };
            if session.catalog_key().map_err(|_| LocalError::Corrupt)? != key {
                return Err(LocalError::Corrupt);
            }
            sessions.push(presentation::session_summary(
                reference,
                &session,
                presentation::session_timing(&store, &session)
                    .await
                    .map_err(map)?,
            ));
        }
        Ok(LocalBrowserAgentPage {
            version: 1,
            snapshot: store.snapshot,
            sessions,
            next_after: page.next_after,
        })
    }
    pub async fn browser_agent_session(
        &self,
        commit: MetadataCommit,
        id: AgentSessionId,
    ) -> LocalResult<LocalBrowserAgentDetail> {
        let store = self.browser_agent_store(commit).await?;
        let (reference, session) = selected_session(&store, id).await?;
        let mut transcripts = Vec::new();
        for reference in &session.transcripts {
            let AgentObjectV1::TranscriptManifest(manifest) = store.object(*reference).await?
            else {
                return Err(LocalError::Corrupt);
            };
            if manifest.session_id != id {
                return Err(LocalError::Corrupt);
            }
            transcripts.extend(
                manifest
                    .streams
                    .iter()
                    .map(|s| presentation::transcript_descriptor(reference.id, s)),
            );
        }
        let subject = LocalAgentSubject::Session { session_id: id };
        let associations = associations::list(&store, &subject, 64, None).await?;
        let (activity, activity_truncated) = presentation::load_activity(&store, &session, &[])
            .await
            .map_err(map)?;
        let summary = presentation::session_summary(
            reference,
            &session,
            presentation::session_timing(&store, &session)
                .await
                .map_err(map)?,
        );
        let selected = self.selected_metadata(commit)?;
        let threads = self.thread_store_at(Some(&selected)).await?;
        let feedback =
            crate::agent_feedback::recorded(&store, &threads.state, &threads.graph, &activity)
                .await;
        let (conversation_feedback, conversation_unavailable) = match feedback {
            Ok(value) => (Some(value), None),
            Err(_) => (
                None,
                Some(
                    "Comments and reply links are unavailable at this selection. Refresh to retry."
                        .into(),
                ),
            ),
        };
        let detail = LocalBrowserAgentDetail {
            conversation_feedback,
            conversation_unavailable,
            version: 1,
            snapshot: store.snapshot.clone(),
            session: summary,
            evidence: session.evidence.iter().map(|r| r.id).collect(),
            event_batches: session.event_batches.iter().map(|r| r.id).collect(),
            transcripts,
            tools: session
                .tools
                .iter()
                .map(|(id, r)| presentation::AgentToolLink {
                    tool_call_id: *id,
                    snapshot_id: r.id,
                })
                .collect(),
            activity,
            activity_truncated,
            code: associations.associations,
            code_truncated: associations.next_after.is_some(),
        };
        if serde_json::to_vec(&detail)
            .map_err(|_| LocalError::Corrupt)?
            .len()
            > 12 * 1024 * 1024
        {
            return Err(LocalError::Limit);
        }
        Ok(detail)
    }
}
async fn selected_session(
    store: &LocalAgentStore,
    id: AgentSessionId,
) -> LocalResult<(AgentObjectRef, AgentSessionSnapshotV1)> {
    let key = AgentCatalogKey::derive("session", id.as_bytes()).map_err(|_| LocalError::Invalid)?;
    let reference = store
        .lookup(AgentCatalogKind::Sessions, key)
        .await?
        .ok_or(LocalError::NotFound)?;
    let AgentObjectV1::SessionSnapshot(session) = store.object(reference).await? else {
        return Err(LocalError::Corrupt);
    };
    if session.session_id != id || session.catalog_key().map_err(|_| LocalError::Corrupt)? != key {
        return Err(LocalError::Corrupt);
    }
    Ok((reference, session))
}
fn map(error: presentation::Error) -> LocalError {
    use presentation::Error as E;
    match error {
        E::NotFound => LocalError::NotFound,
        E::Corrupt => LocalError::Corrupt,
        E::Limit => LocalError::Limit,
        E::Busy => LocalError::Busy,
        E::InvalidCursor => LocalError::Invalid,
        E::CursorReset | E::Denied => LocalError::Stale,
        E::Unavailable => LocalError::Unavailable,
    }
}

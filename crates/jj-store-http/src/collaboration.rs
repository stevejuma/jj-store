use crate::*;
use jj_store_service::collaboration::{
    AgentRelationPage, AgentRelationRequest, AgentSessionDetail, AgentSessionPage,
    AgentTranscriptPage, AgentTranscriptRequest,
};

pub(super) async fn sessions(
    state: &State,
    session: Arc<NativeSession>,
    cursor: Option<String>,
    limit: usize,
) -> Result<Response> {
    let page = if let Some(cursor) = cursor {
        state.agent_sessions.resume(session, &cursor, limit).await?
    } else {
        state.agent_sessions.begin(session, limit).await?
    };
    state.json(&page)
}

pub(super) async fn detail(
    state: &State,
    session: Arc<NativeSession>,
    session_id: jj_store_service::collaboration::AgentSessionIdentity,
) -> Result<Response> {
    state.json(&state.agent_sessions.detail(session, session_id).await?)
}

pub(super) async fn transcript(
    state: &State,
    session: Arc<NativeSession>,
    cursor: Option<String>,
    request: AgentTranscriptRequest,
) -> Result<Response> {
    let page = if let Some(cursor) = cursor {
        state
            .agent_sessions
            .resume_transcript(session, &cursor, request)
            .await?
    } else {
        state
            .agent_sessions
            .begin_transcript(session, request)
            .await?
    };
    state.json(&page)
}

pub(super) async fn relations(
    state: &State,
    session: Arc<NativeSession>,
    cursor: Option<String>,
    request: AgentRelationRequest,
) -> Result<Response> {
    let page = if let Some(cursor) = cursor {
        state
            .agent_sessions
            .resume_relations(session, &cursor, request)
            .await?
    } else {
        state
            .agent_sessions
            .begin_relations(session, request)
            .await?
    };
    state.json(&page)
}

pub(super) type SessionPage = AgentSessionPage;
pub(super) type SessionDetail = AgentSessionDetail;
pub(super) type TranscriptPage = AgentTranscriptPage;
pub(super) type RelationPage = AgentRelationPage;

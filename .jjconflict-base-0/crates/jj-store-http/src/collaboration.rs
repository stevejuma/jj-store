use crate::*;
use jj_store_service::collaboration::{
    AgentCheckpointDetail, AgentCheckpointIdentity, AgentCheckpointPage,
    AgentCheckpointVersionIdentity, AgentRelationPage, AgentRelationRequest, AgentSessionDetail,
    AgentSessionPage, AgentTranscriptPage, AgentTranscriptRequest, ReviewThreadDetail,
    ReviewThreadIdentity, ReviewThreadPage,
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

pub(super) async fn checkpoints(
    state: &State,
    session: Arc<NativeSession>,
    cursor: Option<String>,
    limit: usize,
) -> Result<Response> {
    let page = if let Some(cursor) = cursor {
        state
            .agent_sessions
            .resume_checkpoints(session, &cursor, limit)
            .await?
    } else {
        state
            .agent_sessions
            .begin_checkpoints(session, limit)
            .await?
    };
    state.json(&page)
}

pub(super) async fn checkpoint(
    state: &State,
    session: Arc<NativeSession>,
    checkpoint_id: AgentCheckpointIdentity,
) -> Result<Response> {
    state.json(
        &state
            .agent_sessions
            .checkpoint(session, checkpoint_id)
            .await?,
    )
}

pub(super) async fn checkpoint_version(
    state: &State,
    session: Arc<NativeSession>,
    checkpoint_id: AgentCheckpointIdentity,
    version_id: AgentCheckpointVersionIdentity,
) -> Result<Response> {
    state.json(
        &state
            .agent_sessions
            .checkpoint_version(session, checkpoint_id, version_id)
            .await?,
    )
}

pub(super) async fn review_threads(
    state: &State,
    session: Arc<NativeSession>,
    cursor: Option<String>,
    limit: usize,
) -> Result<Response> {
    let page = if let Some(cursor) = cursor {
        state.review_threads.resume(session, &cursor, limit).await?
    } else {
        state.review_threads.begin(session, limit).await?
    };
    state.json(&page)
}

pub(super) async fn review_thread(
    state: &State,
    session: Arc<NativeSession>,
    thread_id: ReviewThreadIdentity,
) -> Result<Response> {
    state.json(&state.review_threads.detail(session, thread_id).await?)
}

pub(super) type SessionPage = AgentSessionPage;
pub(super) type SessionDetail = AgentSessionDetail;
pub(super) type TranscriptPage = AgentTranscriptPage;
pub(super) type RelationPage = AgentRelationPage;
pub(super) type CheckpointPage = AgentCheckpointPage;
pub(super) type CheckpointDetail = AgentCheckpointDetail;
pub(super) type ReviewPage = ReviewThreadPage;
pub(super) type ReviewActivitySummary = jj_store_service::collaboration::ReviewActivitySummary;
pub(super) type ReviewDetail = ReviewThreadDetail;

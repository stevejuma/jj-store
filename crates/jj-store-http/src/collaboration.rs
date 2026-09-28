use crate::*;
use jj_store_service::collaboration::{AgentSessionDetail, AgentSessionPage};

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

pub(super) type SessionPage = AgentSessionPage;
pub(super) type SessionDetail = AgentSessionDetail;

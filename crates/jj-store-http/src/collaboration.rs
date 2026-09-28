use crate::*;
use jj_store_service::collaboration::AgentSessionPage;

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

pub(super) type SessionPage = AgentSessionPage;

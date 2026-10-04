//! Enrich accepted tool snapshots using only the selected session's retained evidence.
use super::{agent::agent_error, *};
use jj_agent::{AgentCatalogKey, AgentCatalogKind, ToolInvocationSnapshotV1};

pub(super) async fn merge_selected_tool(
    base: &dyn AgentSnapshotRead,
    next: &mut ToolInvocationSnapshotV1,
) -> Result<bool> {
    let key =
        AgentCatalogKey::derive("session", next.session_id.as_bytes()).map_err(agent_error)?;
    let Some(reference) = base.lookup(AgentCatalogKind::Sessions, key).await? else {
        return Ok(false);
    };
    let AgentObjectV1::SessionSnapshot(session) = base.read_object(reference).await? else {
        return Err(Error::Corrupt("agent tool session selection"));
    };
    let Some(reference) = session.tools.get(&next.tool_call_id) else {
        return Ok(false);
    };
    let AgentObjectV1::ToolSnapshot(mut previous) = base.read_object(*reference).await? else {
        return Err(Error::Corrupt("agent tool selection"));
    };
    let conflict = previous.enrich(next).map_err(agent_error)?;
    *next = previous;
    Ok(conflict)
}

//! Resolve sanitized result observations against exact selected aliases and tools.
use super::{agent::agent_error, *};
use jj_agent::{
    AgentAliasV1, AgentCatalogKey, AgentCatalogKind, AgentEntityRefV1, CaptureState,
    NormalizationBatchV1, NormalizedToolRecordV1, PermissionState, ToolResultObservation,
};

pub(super) async fn resolve_tool_results(
    base: &dyn AgentSnapshotRead,
    normalized: &mut NormalizationBatchV1,
    results: &[ToolResultObservation],
    adopted: &[AgentAliasV1],
) -> Result<()> {
    for result in results {
        let known = match result.same_batch_call(normalized) {
            Some(value) => Some(value.clone()),
            None => selected_call(base, result, adopted).await?,
        };
        normalized
            .resolve_tool_result(result, known.as_ref())
            .map_err(agent_error)?;
    }
    normalized.sort_source_records();
    Ok(())
}

async fn selected_call(
    base: &dyn AgentSnapshotRead,
    result: &ToolResultObservation,
    adopted: &[AgentAliasV1],
) -> Result<Option<NormalizedToolRecordV1>> {
    if !(adopted.contains(&result.tool_alias)
        || selected_alias_matches(base, &result.tool_alias).await?)
        || !(adopted.contains(&result.session_alias)
            || selected_alias_matches(base, &result.session_alias).await?)
    {
        return Ok(None);
    }
    let AgentEntityRefV1::Tool {
        session_id,
        tool_call_id,
    } = result.tool_alias.target
    else {
        return Err(Error::Corrupt("agent result alias target"));
    };
    let key = AgentCatalogKey::derive("session", session_id.as_bytes()).map_err(agent_error)?;
    let Some(reference) = base.lookup(AgentCatalogKind::Sessions, key).await? else {
        return Ok(None);
    };
    let AgentObjectV1::SessionSnapshot(session) = base.read_object(reference).await? else {
        return Err(Error::Corrupt("agent result session selection"));
    };
    if session.session_id != session_id {
        return Err(Error::Corrupt("agent result session identity"));
    }
    let Some(reference) = session.tools.get(&tool_call_id) else {
        return Ok(None);
    };
    let AgentObjectV1::ToolSnapshot(tool) = base.read_object(*reference).await? else {
        return Err(Error::Corrupt("agent result tool selection"));
    };
    if tool.session_id != session_id || tool.tool_call_id != tool_call_id {
        return Err(Error::Corrupt("agent result tool identity"));
    }
    Ok(Some(NormalizedToolRecordV1 {
        source_sequence: result.source_sequence,
        tool_call_id,
        session_id,
        turn_id: tool.turn_id,
        native_name: tool.native_name,
        kind: tool.kind,
        state: tool.state,
        capture_state: CaptureState::Observed,
        permission_state: PermissionState::NotObserved,
        input: None,
        output: None,
        conflicting_terminal_states: Vec::new(),
    }))
}

async fn selected_alias_matches(
    base: &dyn AgentSnapshotRead,
    expected: &AgentAliasV1,
) -> Result<bool> {
    let key = expected.catalog_key().map_err(agent_error)?;
    let Some(reference) = base.lookup(AgentCatalogKind::Aliases, key).await? else {
        return Ok(false);
    };
    let AgentObjectV1::Alias(alias) = base.read_object(reference).await? else {
        return Err(Error::Corrupt("agent result alias selection"));
    };
    Ok(alias == *expected)
}

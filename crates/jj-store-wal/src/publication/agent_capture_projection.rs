use super::agent::agent_error;
use super::agent_capture::SessionSelection;
use super::*;
use jj_agent::{
    AgentCatalogEditV1, AgentCatalogKind, AgentObjectV1, AgentProjectionCatalogV1,
    NormalizerProfileV1,
};

impl AgentProjector<'_> {
    #[allow(clippy::too_many_arguments)]
    pub(super) async fn stage_projection(
        &self,
        base: &dyn AgentSnapshotRead,
        profile: &NormalizerProfileV1,
        activated_at_millis: i64,
        sessions: &[SessionSelection],
        event_batch: AgentObjectRef,
        previous_source: Option<AgentObjectRef>,
        source: AgentObjectRef,
    ) -> Result<AgentCatalogEditV1> {
        let mut projection = AgentProjectionCatalogV1 {
            schema_version: 1,
            normalizer_version: profile.identity(),
            activated_at_millis,
            sessions: Vec::new(),
            event_batches: Vec::new(),
            source_states: Vec::new(),
        };
        let key = projection.catalog_key().map_err(agent_error)?;
        let expected = base.lookup(AgentCatalogKind::Projections, key).await?;
        if let Some(reference) = expected {
            projection = match base.read_object(reference).await? {
                AgentObjectV1::ProjectionCatalog(value) => value,
                _ => return Err(Error::Corrupt("agent projection selection")),
            };
        }
        for selection in sessions {
            if let Some(previous) = selection.expected {
                projection.sessions.retain(|value| *value != previous);
            }
            push_unique(&mut projection.sessions, selection.replacement);
        }
        push_unique(&mut projection.event_batches, event_batch);
        if let Some(previous) = previous_source {
            projection.source_states.retain(|value| *value != previous);
        }
        push_unique(&mut projection.source_states, source);
        let replacement = self
            .stage_agent_object(&AgentObjectV1::ProjectionCatalog(projection))
            .await?;
        Ok(AgentCatalogEditV1 {
            catalog: AgentCatalogKind::Projections,
            key,
            expected,
            replacement: Some(replacement),
        })
    }
}

fn push_unique(values: &mut Vec<AgentObjectRef>, value: AgentObjectRef) {
    if !values.contains(&value) {
        values.push(value);
    }
}

//! Shared exact session or diagnostic-only Thread capture selection.
use super::{agent::agent_error, *};
use jj_agent::{AgentCatalogEditV1, AgentProjectionCatalogV1, CaptureSourceStateV1};
use jj_collab::{ThreadBindingV1, ThreadCommandV1};

enum Selection<'a> {
    Session(&'a AgentCatalogEditV1),
    Gap {
        source: &'a AgentCatalogEditV1,
        projection: &'a AgentCatalogEditV1,
    },
}
fn selection(delta: &AgentDeltaV1) -> Result<Selection<'_>> {
    delta.validate().map_err(agent_error)?;
    let sessions: Vec<_> = delta
        .edits
        .iter()
        .filter(|e| e.catalog == AgentCatalogKind::Sessions)
        .collect();
    match sessions.as_slice() {
        [one]
            if one
                .replacement
                .is_some_and(|r| r.kind == AgentObjectKind::SessionSnapshot) =>
        {
            Ok(Selection::Session(one))
        }
        [] if delta.edits.len() == 2 => {
            let source = delta.edits.iter().find(|e| {
                e.catalog == AgentCatalogKind::Sources
                    && e.replacement
                        .is_some_and(|r| r.kind == AgentObjectKind::SourceState)
            });
            let projection = delta.edits.iter().find(|e| {
                e.catalog == AgentCatalogKind::Projections
                    && e.replacement
                        .is_some_and(|r| r.kind == AgentObjectKind::ProjectionCatalog)
            });
            match (source, projection) {
                (Some(source), Some(projection)) => Ok(Selection::Gap { source, projection }),
                _ => Err(Error::Corrupt("bound capture gap shape")),
            }
        }
        _ => Err(Error::Corrupt(
            "bound capture requires one session or diagnostic gap",
        )),
    }
}
impl AgentProjector<'_> {
    pub fn validate_capture_binding_delta(delta: &AgentDeltaV1) -> Result<()> {
        selection(delta).map(|_| ())
    }
    pub async fn capture_binding_command(
        &self,
        delta: &AgentDeltaV1,
        binding: &ThreadBindingV1,
        actor: PrincipalId,
        request: RequestId,
    ) -> Result<ThreadCommandV1> {
        let command = match selection(delta)? {
            Selection::Session(edit) => {
                let snapshot = edit
                    .replacement
                    .ok_or(Error::Corrupt("capture session replacement"))?;
                let AgentObjectV1::SessionSnapshot(session) =
                    self.objects.read_agent_object(snapshot).await?
                else {
                    return Err(Error::Corrupt("capture session kind"));
                };
                if edit.key
                    != AgentCatalogKey::derive("session", session.session_id.as_bytes())
                        .map_err(agent_error)?
                {
                    return Err(Error::Corrupt("capture session identity"));
                }
                binding.capture_link(session.session_id, snapshot, actor, request)
            }
            Selection::Gap { source, projection } => {
                let source_ref = source
                    .replacement
                    .ok_or(Error::Corrupt("capture gap source"))?;
                let projection_ref = projection
                    .replacement
                    .ok_or(Error::Corrupt("capture gap projection"))?;
                let AgentObjectV1::SourceState(state) =
                    self.objects.read_agent_object(source_ref).await?
                else {
                    return Err(Error::Corrupt("capture gap source"));
                };
                let AgentObjectV1::ProjectionCatalog(view) =
                    self.objects.read_agent_object(projection_ref).await?
                else {
                    return Err(Error::Corrupt("capture gap projection"));
                };
                if state.catalog_key().map_err(agent_error)? != source.key
                    || view.catalog_key().map_err(agent_error)? != projection.key
                    || view.normalizer_version != state.parser_version
                    || !view.source_states.contains(&source_ref)
                {
                    return Err(Error::Corrupt("capture gap selected identity"));
                }
                let old_view = match projection.expected {
                    Some(reference) => match self.objects.read_agent_object(reference).await? {
                        AgentObjectV1::ProjectionCatalog(old) => old,
                        _ => return Err(Error::Corrupt("capture gap previous projection")),
                    },
                    None => AgentProjectionCatalogV1 {
                        schema_version: 1,
                        normalizer_version: view.normalizer_version.clone(),
                        activated_at_millis: view.activated_at_millis,
                        sessions: Vec::new(),
                        event_batches: Vec::new(),
                        source_states: Vec::new(),
                    },
                };
                let mut expected_sources = old_view.source_states;
                expected_sources.retain(|r| Some(*r) != source.expected);
                expected_sources.push(source_ref);
                if view.normalizer_version != old_view.normalizer_version
                    || view.sessions != old_view.sessions
                    || view.source_states != expected_sources
                    || view.event_batches.len() != old_view.event_batches.len() + 1
                    || !view.event_batches.starts_with(&old_view.event_batches)
                {
                    return Err(Error::Corrupt("capture gap projection continuity"));
                }
                let prior_receipts = match source.expected {
                    Some(reference) => match self.objects.read_agent_object(reference).await? {
                        AgentObjectV1::SourceState(old)
                            if old.integration == state.integration
                                && old.source_instance_id == state.source_instance_id
                                && old.generation == state.generation
                                && state.coverage.gaps >= old.coverage.gaps.saturating_add(1) =>
                        {
                            old.receipts
                        }
                        _ => return Err(Error::Corrupt("capture gap previous source")),
                    },
                    None => Vec::new(),
                };
                if state.receipts.len() != prior_receipts.len() + 1
                    || !state.receipts.starts_with(&prior_receipts)
                {
                    return Err(Error::Corrupt("capture gap receipt continuity"));
                }
                let events_ref = *view
                    .event_batches
                    .last()
                    .ok_or(Error::Corrupt("capture gap event selection"))?;
                let events = self.objects.read_agent_object(events_ref).await?;
                let evidence = match &events {
                    AgentObjectV1::EventBatch(events) => {
                        self.objects.read_agent_object(events.evidence).await?
                    }
                    _ => return Err(Error::Corrupt("capture gap events")),
                };
                validate_gap_evidence(&state, &events, &evidence)?;
                binding.capture_gap(source_ref, events_ref, actor, request)
            }
        };
        command.map_err(|_| Error::Corrupt("capture binding intent"))
    }
}
pub(super) fn validate_gap_evidence(
    source: &CaptureSourceStateV1,
    events: &AgentObjectV1,
    evidence: &AgentObjectV1,
) -> Result<()> {
    let (AgentObjectV1::EventBatch(events), AgentObjectV1::ObservationBatch(batch)) =
        (events, evidence)
    else {
        return Err(Error::Corrupt("capture gap evidence kinds"));
    };
    let receipt = source
        .receipts
        .last()
        .ok_or(Error::Corrupt("capture gap receipt"))?;
    if !events.events.is_empty()
        || source.native_session_key.is_some()
        || source.coverage.gaps == 0
        || events.normalizer_version != source.parser_version
        || source.integration != batch.integration
        || source.source_instance_id != batch.source_instance_id
        || receipt.batch_id != batch.batch_id
        || batch.observations.first().map(|o| o.source_sequence) != Some(receipt.first_sequence)
        || batch.observations.last().map(|o| o.source_sequence) != Some(receipt.last_sequence)
        || source.position
            != (jj_agent::SourcePositionV1::NativeSequence {
                sequence: receipt.last_sequence,
            })
    {
        return Err(Error::Corrupt("capture gap receipt evidence"));
    }
    Ok(())
}
impl AgentReader<'_> {
    /// New manual gap links require a current, authorized pinned source selection.
    pub async fn validate_capture_gap(
        &self,
        source_ref: AgentObjectRef,
        event_ref: AgentObjectRef,
    ) -> Result<()> {
        let AgentObjectV1::SourceState(source) = self.read_object(source_ref).await? else {
            return Err(Error::Corrupt("capture gap source"));
        };
        if self
            .lookup(
                AgentCatalogKind::Sources,
                source.catalog_key().map_err(agent_error)?,
            )
            .await?
            != Some(source_ref)
        {
            return Err(Error::Stale);
        }
        let key = AgentProjectionCatalogV1 {
            schema_version: 1,
            normalizer_version: source.parser_version.clone(),
            activated_at_millis: 0,
            sessions: Vec::new(),
            event_batches: Vec::new(),
            source_states: Vec::new(),
        }
        .catalog_key()
        .map_err(agent_error)?;
        let projection_ref = self
            .lookup(AgentCatalogKind::Projections, key)
            .await?
            .ok_or(Error::Stale)?;
        let AgentObjectV1::ProjectionCatalog(projection) = self.read_object(projection_ref).await?
        else {
            return Err(Error::Corrupt("capture gap projection"));
        };
        if !projection.source_states.contains(&source_ref)
            || !projection.event_batches.contains(&event_ref)
        {
            return Err(Error::Stale);
        }
        let events = self.read_object(event_ref).await?;
        let evidence = match &events {
            AgentObjectV1::EventBatch(events) => self.read_object(events.evidence).await?,
            _ => return Err(Error::Corrupt("capture gap events")),
        };
        validate_gap_evidence(&source, &events, &evidence)
    }
}

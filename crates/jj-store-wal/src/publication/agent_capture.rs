use super::agent::agent_error;
use super::agent_capture_records::{CaptureObjects, stage_capture_objects};
use super::*;
use jj_agent::{
    AgentAliasV1, AgentCatalogEditV1, AgentCatalogKey, AgentCatalogKind, AgentObjectV1,
    AgentSessionId, AgentSessionSnapshotV1, CaptureSourceReceiptV1, CaptureSourceStateV1,
    CompletenessV1, CoverageV1, NativeEntityKind, NormalizationBatchV1, NormalizedActivityRecordV1,
    NormalizerProfileV1, SanitizedObservationBatchV1, SessionState, SourcePositionV1,
    normalize_batch,
};

impl Coordinator {
    pub async fn stage_agent_capture(
        &self,
        base: &AgentStoreSnapshot,
        profile: &NormalizerProfileV1,
        batch: &SanitizedObservationBatchV1,
        activated_at_millis: i64,
    ) -> Result<AgentDeltaV1> {
        let normalized = normalize_batch(profile, batch).map_err(agent_error)?;
        let objects = stage_capture_objects(self, profile, batch, &normalized).await?;
        let (mut edits, sessions) = self
            .stage_sessions(base, batch, &normalized, &objects)
            .await?;
        edits.extend(self.stage_aliases(base, &normalized.aliases).await?);
        let (source_edit, source_ref) = self
            .stage_source_state(base, profile, batch, &normalized)
            .await?;
        let previous_source = source_edit.expected;
        edits.push(source_edit);
        edits.push(
            self.stage_projection(
                base,
                profile,
                activated_at_millis,
                &sessions,
                objects.event_batch,
                previous_source,
                source_ref,
            )
            .await?,
        );
        let delta = AgentDeltaV1 {
            schema_version: 1,
            edits,
        };
        delta.validate().map_err(agent_error)?;
        Ok(delta)
    }

    async fn stage_sessions(
        &self,
        base: &AgentStoreSnapshot,
        batch: &SanitizedObservationBatchV1,
        normalized: &NormalizationBatchV1,
        objects: &CaptureObjects,
    ) -> Result<(Vec<AgentCatalogEditV1>, Vec<SessionSelection>)> {
        let mut ids = BTreeSet::new();
        for alias in &normalized.aliases {
            if let jj_agent::AgentEntityRefV1::Session { session_id } = alias.target {
                ids.insert(session_id);
            }
        }
        let gaps = capture_gaps(batch, normalized);
        let mut edits = Vec::new();
        let mut selected = Vec::new();
        for session_id in ids {
            let key =
                AgentCatalogKey::derive("session", session_id.as_bytes()).map_err(agent_error)?;
            let expected = base.lookup(AgentCatalogKind::Sessions, key).await?;
            let mut session = match expected {
                Some(reference) => match base.read_object(reference).await? {
                    AgentObjectV1::SessionSnapshot(value) => value,
                    _ => return Err(Error::Corrupt("agent session selection")),
                },
                None => empty_session(session_id, batch.integration),
            };
            session.state = selected_state(normalized, session_id).unwrap_or(session.state);
            push_unique(&mut session.evidence, objects.evidence);
            push_unique(&mut session.event_batches, objects.event_batch);
            session.tools.extend(
                objects
                    .tools
                    .get(&session_id)
                    .into_iter()
                    .flat_map(|values| values.iter().map(|(key, value)| (*key, *value))),
            );
            session.coverage.completeness = CompletenessV1::Partial;
            session.coverage.gaps = session.coverage.gaps.saturating_add(gaps);
            let replacement = self
                .stage_agent_object(&AgentObjectV1::SessionSnapshot(session))
                .await?;
            edits.push(edit(AgentCatalogKind::Sessions, key, expected, replacement));
            selected.push(SessionSelection {
                expected,
                replacement,
            });
        }
        Ok((edits, selected))
    }

    async fn stage_aliases(
        &self,
        base: &AgentStoreSnapshot,
        aliases: &[AgentAliasV1],
    ) -> Result<Vec<AgentCatalogEditV1>> {
        let mut edits = Vec::new();
        for alias in aliases {
            let key = alias.catalog_key().map_err(agent_error)?;
            let expected = base.lookup(AgentCatalogKind::Aliases, key).await?;
            let replacement = self
                .stage_agent_object(&AgentObjectV1::Alias(alias.clone()))
                .await?;
            if expected != Some(replacement) {
                edits.push(edit(AgentCatalogKind::Aliases, key, expected, replacement));
            }
        }
        Ok(edits)
    }

    async fn stage_source_state(
        &self,
        base: &AgentStoreSnapshot,
        profile: &NormalizerProfileV1,
        batch: &SanitizedObservationBatchV1,
        normalized: &NormalizationBatchV1,
    ) -> Result<(AgentCatalogEditV1, AgentObjectRef)> {
        let sequences = batch
            .observations
            .first()
            .zip(batch.observations.last())
            .ok_or(Error::Corrupt("empty capture batch"))?;
        let mut state = CaptureSourceStateV1 {
            schema_version: 1,
            integration: batch.integration,
            source_instance_id: batch.source_instance_id,
            generation: 1,
            parser_version: profile.identity(),
            native_session_key: unique_native_session(&normalized.aliases),
            position: SourcePositionV1::NativeSequence {
                sequence: sequences.1.source_sequence,
            },
            receipts: Vec::new(),
            coverage: CoverageV1 {
                completeness: CompletenessV1::Partial,
                gaps: capture_gaps(batch, normalized),
                redacted_fields: 0,
                truncated: false,
            },
        };
        let key = state.catalog_key().map_err(agent_error)?;
        let expected = base.lookup(AgentCatalogKind::Sources, key).await?;
        if let Some(reference) = expected {
            state = match base.read_object(reference).await? {
                AgentObjectV1::SourceState(value) => value,
                _ => return Err(Error::Corrupt("agent source selection")),
            };
            if state
                .receipts
                .iter()
                .any(|value| value.batch_id == batch.batch_id)
                || state
                    .receipts
                    .last()
                    .is_some_and(|value| value.last_sequence >= sequences.0.source_sequence)
            {
                return Err(Error::Stale);
            }
            state.position = SourcePositionV1::NativeSequence {
                sequence: sequences.1.source_sequence,
            };
            state.parser_version = profile.identity();
            state.native_session_key = unique_native_session(&normalized.aliases);
            state.coverage.gaps = state
                .coverage
                .gaps
                .saturating_add(capture_gaps(batch, normalized));
        }
        state.receipts.push(CaptureSourceReceiptV1 {
            batch_id: batch.batch_id,
            first_sequence: sequences.0.source_sequence,
            last_sequence: sequences.1.source_sequence,
        });
        let replacement = self
            .stage_agent_object(&AgentObjectV1::SourceState(state))
            .await?;
        Ok((
            edit(AgentCatalogKind::Sources, key, expected, replacement),
            replacement,
        ))
    }
}

pub(super) struct SessionSelection {
    pub expected: Option<AgentObjectRef>,
    pub replacement: AgentObjectRef,
}

fn empty_session(
    session_id: AgentSessionId,
    integration: jj_agent::IntegrationId,
) -> AgentSessionSnapshotV1 {
    AgentSessionSnapshotV1 {
        schema_version: 1,
        session_id,
        integration,
        state: SessionState::Discovered,
        parent_session_id: None,
        evidence: Vec::new(),
        event_batches: Vec::new(),
        transcripts: Vec::new(),
        tools: BTreeMap::new(),
        coverage: CoverageV1 {
            completeness: CompletenessV1::Partial,
            gaps: 0,
            redacted_fields: 0,
            truncated: false,
        },
    }
}

fn selected_state(
    normalized: &NormalizationBatchV1,
    session_id: AgentSessionId,
) -> Option<SessionState> {
    normalized
        .records
        .iter()
        .rev()
        .find_map(|record| match record {
            NormalizedActivityRecordV1::SessionState(value) if value.session_id == session_id => {
                Some(value.state)
            }
            _ => None,
        })
}

fn capture_gaps(batch: &SanitizedObservationBatchV1, normalized: &NormalizationBatchV1) -> u32 {
    let sequence_gaps = batch
        .observations
        .windows(2)
        .map(|values| {
            values[1]
                .source_sequence
                .saturating_sub(values[0].source_sequence)
                .saturating_sub(1)
        })
        .sum::<u64>();
    sequence_gaps
        .saturating_add(normalized.diagnostics.len() as u64)
        .min(u32::MAX as u64) as u32
}

fn unique_native_session(aliases: &[AgentAliasV1]) -> Option<String> {
    let values: BTreeSet<_> = aliases
        .iter()
        .filter(|value| value.native_kind == NativeEntityKind::Session)
        .map(|value| value.native_id.clone())
        .collect();
    (values.len() == 1)
        .then(|| values.into_iter().next())
        .flatten()
}

fn edit(
    catalog: AgentCatalogKind,
    key: AgentCatalogKey,
    expected: Option<AgentObjectRef>,
    replacement: AgentObjectRef,
) -> AgentCatalogEditV1 {
    AgentCatalogEditV1 {
        catalog,
        key,
        expected,
        replacement: Some(replacement),
    }
}

fn push_unique(values: &mut Vec<AgentObjectRef>, value: AgentObjectRef) {
    if !values.contains(&value) {
        values.push(value);
    }
}

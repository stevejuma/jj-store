//! Typed AgentStore reducers for imported accepted facts.
use super::*;
use jj_agent::{
    CaptureSourceReceiptV1, CaptureSourceStateV1, CompletenessV1, CoverageV1, SessionState,
    ToolInvocationSnapshotV1, ToolState,
};

impl Coordinator {
    pub(super) async fn reduce_agent_catalog_value(
        &self,
        current_store: &AgentStoreSnapshot,
        incoming_store: &AgentStoreSnapshot,
        catalog: AgentCatalogKind,
        key: AgentCatalogKey,
        current: AgentObjectRef,
        incoming: AgentObjectRef,
    ) -> Result<AgentObjectRef> {
        if current == incoming {
            return Ok(current);
        }
        if current.kind != AgentObjectKind::CatalogConflict
            && incoming.kind != AgentObjectKind::CatalogConflict
        {
            let merged = match catalog {
                AgentCatalogKind::Sessions => self.merge_agent_sessions(current, incoming).await?,
                AgentCatalogKind::Checkpoints | AgentCatalogKind::Associations => {
                    self.select_agent_causal_descendant(catalog, current, incoming)
                        .await?
                }
                AgentCatalogKind::Relations => {
                    Some(self.merge_agent_relation_pages(current, incoming).await?)
                }
                AgentCatalogKind::Sources => {
                    self.select_agent_source_descendant(current, incoming)
                        .await?
                }
                AgentCatalogKind::Projections => {
                    self.merge_agent_projections(current, incoming).await?
                }
                AgentCatalogKind::Aliases => None,
            };
            if let Some(merged) = merged {
                return Ok(merged);
            }
        }
        let mut alternatives = self
            .agent_conflict_alternatives(current_store, catalog, key, current)
            .await?;
        alternatives.extend(
            self.agent_conflict_alternatives(incoming_store, catalog, key, incoming)
                .await?,
        );
        if alternatives.len() == 1 {
            return alternatives
                .first()
                .copied()
                .ok_or(Error::Corrupt("empty agent conflict"));
        }
        self.stage_agent_object(&AgentObjectV1::CatalogConflict(AgentCatalogConflictV1 {
            schema_version: 1,
            catalog,
            key,
            alternatives: alternatives.into_iter().collect(),
        }))
        .await
    }

    async fn merge_agent_sessions(
        &self,
        current: AgentObjectRef,
        incoming: AgentObjectRef,
    ) -> Result<Option<AgentObjectRef>> {
        let AgentObjectV1::SessionSnapshot(mut current) = self.read_agent_object(current).await?
        else {
            return Err(Error::Corrupt("agent session selection kind"));
        };
        let AgentObjectV1::SessionSnapshot(incoming) = self.read_agent_object(incoming).await?
        else {
            return Err(Error::Corrupt("agent session selection kind"));
        };
        if current.session_id != incoming.session_id
            || current.integration != incoming.integration
            || current.parent_session_id != incoming.parent_session_id
        {
            return Err(Error::Corrupt("agent session identity"));
        }
        let Some(state) = merge_session_state(current.state, incoming.state) else {
            return Ok(None);
        };
        current.state = state;
        union_agent_refs(&mut current.evidence, incoming.evidence, 4096)?;
        union_agent_refs(&mut current.event_batches, incoming.event_batches, 4096)?;
        union_agent_refs(&mut current.transcripts, incoming.transcripts, 64)?;
        for (tool_id, incoming_tool) in incoming.tools {
            match current.tools.get(&tool_id).copied() {
                None => {
                    current.tools.insert(tool_id, incoming_tool);
                }
                Some(selected) if selected == incoming_tool => {}
                Some(selected) => {
                    let Some(merged) = self.merge_agent_tools(selected, incoming_tool).await?
                    else {
                        return Ok(None);
                    };
                    current.tools.insert(tool_id, merged);
                }
            }
        }
        current.coverage = merge_coverage(current.coverage, incoming.coverage);
        self.stage_agent_object(&AgentObjectV1::SessionSnapshot(current))
            .await
            .map(Some)
    }

    async fn merge_agent_tools(
        &self,
        current: AgentObjectRef,
        incoming: AgentObjectRef,
    ) -> Result<Option<AgentObjectRef>> {
        let AgentObjectV1::ToolSnapshot(mut current) = self.read_agent_object(current).await?
        else {
            return Err(Error::Corrupt("agent tool selection kind"));
        };
        let AgentObjectV1::ToolSnapshot(incoming) = self.read_agent_object(incoming).await? else {
            return Err(Error::Corrupt("agent tool selection kind"));
        };
        if tool_identity(&current) != tool_identity(&incoming) {
            return Err(Error::Corrupt("agent tool identity"));
        }
        let Some(state) = merge_tool_state(current.state, incoming.state) else {
            return Ok(None);
        };
        let Some(input) = merge_optional_ref(current.input, incoming.input) else {
            return Ok(None);
        };
        let Some(output) = merge_optional_ref(current.output, incoming.output) else {
            return Ok(None);
        };
        current.state = state;
        current.input = input;
        current.output = output;
        current.attempts.extend(incoming.attempts);
        union_agent_refs(&mut current.evidence, incoming.evidence, 1024)?;
        self.stage_agent_object(&AgentObjectV1::ToolSnapshot(current))
            .await
            .map(Some)
    }

    async fn select_agent_causal_descendant(
        &self,
        catalog: AgentCatalogKind,
        current: AgentObjectRef,
        incoming: AgentObjectRef,
    ) -> Result<Option<AgentObjectRef>> {
        if self
            .agent_version_is_ancestor(catalog, current, incoming)
            .await?
        {
            Ok(Some(incoming))
        } else if self
            .agent_version_is_ancestor(catalog, incoming, current)
            .await?
        {
            Ok(Some(current))
        } else {
            Ok(None)
        }
    }

    async fn agent_version_is_ancestor(
        &self,
        catalog: AgentCatalogKind,
        ancestor: AgentObjectRef,
        descendant: AgentObjectRef,
    ) -> Result<bool> {
        let mut next = Some(descendant);
        let mut seen = BTreeSet::new();
        while let Some(reference) = next {
            if reference == ancestor {
                return Ok(true);
            }
            if !seen.insert(reference) || seen.len() > 4096 {
                return Err(Error::Corrupt("agent version cycle"));
            }
            next = match (catalog, self.read_agent_object(reference).await?) {
                (AgentCatalogKind::Checkpoints, AgentObjectV1::CheckpointVersion(value)) => {
                    value.supersedes
                }
                (AgentCatalogKind::Associations, AgentObjectV1::Association(value)) => {
                    value.supersedes
                }
                _ => return Err(Error::Corrupt("agent version selection kind")),
            };
        }
        Ok(false)
    }

    async fn select_agent_source_descendant(
        &self,
        current: AgentObjectRef,
        incoming: AgentObjectRef,
    ) -> Result<Option<AgentObjectRef>> {
        let AgentObjectV1::SourceState(current_value) = self.read_agent_object(current).await?
        else {
            return Err(Error::Corrupt("agent source selection kind"));
        };
        let AgentObjectV1::SourceState(incoming_value) = self.read_agent_object(incoming).await?
        else {
            return Err(Error::Corrupt("agent source selection kind"));
        };
        if source_identity(&current_value) != source_identity(&incoming_value) {
            return Err(Error::Corrupt("agent source identity"));
        }
        if source_receipts_prefix(&current_value.receipts, &incoming_value.receipts) {
            Ok(Some(incoming))
        } else if source_receipts_prefix(&incoming_value.receipts, &current_value.receipts) {
            Ok(Some(current))
        } else {
            Ok(None)
        }
    }

    async fn merge_agent_projections(
        &self,
        current: AgentObjectRef,
        incoming: AgentObjectRef,
    ) -> Result<Option<AgentObjectRef>> {
        let AgentObjectV1::ProjectionCatalog(mut current) = self.read_agent_object(current).await?
        else {
            return Err(Error::Corrupt("agent projection selection kind"));
        };
        let AgentObjectV1::ProjectionCatalog(incoming) = self.read_agent_object(incoming).await?
        else {
            return Err(Error::Corrupt("agent projection selection kind"));
        };
        if current.normalizer_version != incoming.normalizer_version {
            return Err(Error::Corrupt("agent projection identity"));
        }
        current.activated_at_millis = current
            .activated_at_millis
            .min(incoming.activated_at_millis);
        union_agent_refs(&mut current.sessions, incoming.sessions, 4096)?;
        union_agent_refs(&mut current.event_batches, incoming.event_batches, 4096)?;
        union_agent_refs(&mut current.source_states, incoming.source_states, 4096)?;
        self.stage_agent_object(&AgentObjectV1::ProjectionCatalog(current))
            .await
            .map(Some)
    }

    async fn merge_agent_relation_pages(
        &self,
        current: AgentObjectRef,
        incoming: AgentObjectRef,
    ) -> Result<AgentObjectRef> {
        let (subject, mut selections) = self.read_agent_relation_chain(current).await?;
        let (incoming_subject, incoming_selections) =
            self.read_agent_relation_chain(incoming).await?;
        if subject != incoming_subject {
            return Err(Error::Corrupt("agent relation subject"));
        }
        selections.extend(incoming_selections);
        if selections.len() > 4096 {
            return Err(Error::Limit("agent relation selections"));
        }
        let selections = selections.into_iter().collect::<Vec<_>>();
        let mut next = None;
        for chunk in selections.rchunks(256) {
            next = Some(
                self.stage_agent_object(&AgentObjectV1::RelationPage(AgentRelationPageV1 {
                    schema_version: 1,
                    subject: subject.clone(),
                    selections: chunk.to_vec(),
                    next,
                }))
                .await?,
            );
        }
        next.ok_or(Error::Corrupt("empty agent relation merge"))
    }

    async fn read_agent_relation_chain(
        &self,
        mut reference: AgentObjectRef,
    ) -> Result<(AgentRelationSubjectV1, BTreeSet<AgentObjectRef>)> {
        let mut subject = None;
        let mut selections = BTreeSet::new();
        for _ in 0..4096 {
            let AgentObjectV1::RelationPage(page) = self.read_agent_object(reference).await? else {
                return Err(Error::Corrupt("agent relation page kind"));
            };
            if subject.as_ref().is_some_and(|value| value != &page.subject) {
                return Err(Error::Corrupt("agent relation page subject"));
            }
            subject.get_or_insert_with(|| page.subject.clone());
            selections.extend(page.selections);
            let Some(next) = page.next else {
                return Ok((
                    subject.ok_or(Error::Corrupt("agent relation subject"))?,
                    selections,
                ));
            };
            reference = next;
        }
        Err(Error::Limit("agent relation pages"))
    }
}

fn union_agent_refs(
    selected: &mut Vec<AgentObjectRef>,
    incoming: Vec<AgentObjectRef>,
    limit: usize,
) -> Result<()> {
    let union = selected
        .iter()
        .copied()
        .chain(incoming)
        .collect::<BTreeSet<_>>();
    if union.len() > limit {
        return Err(Error::Limit("agent fact union"));
    }
    *selected = union.into_iter().collect();
    Ok(())
}

fn merge_optional_ref(
    current: Option<AgentObjectRef>,
    incoming: Option<AgentObjectRef>,
) -> Option<Option<AgentObjectRef>> {
    match (current, incoming) {
        (Some(current), Some(incoming)) if current != incoming => None,
        (Some(value), _) | (_, Some(value)) => Some(Some(value)),
        (None, None) => Some(None),
    }
}

fn merge_session_state(current: SessionState, incoming: SessionState) -> Option<SessionState> {
    match (current, incoming) {
        (left, right) if left == right => Some(left),
        (SessionState::Discovered | SessionState::Unknown, value)
        | (value, SessionState::Discovered | SessionState::Unknown) => Some(value),
        _ => None,
    }
}

fn terminal_tool_state(value: ToolState) -> bool {
    matches!(
        value,
        ToolState::Succeeded
            | ToolState::Failed
            | ToolState::Cancelled
            | ToolState::Interrupted
            | ToolState::Skipped
    )
}

fn merge_tool_state(current: ToolState, incoming: ToolState) -> Option<ToolState> {
    match (current, incoming) {
        (left, right) if left == right => Some(left),
        (left, right) if terminal_tool_state(left) && !terminal_tool_state(right) => Some(left),
        (left, right) if !terminal_tool_state(left) && terminal_tool_state(right) => Some(right),
        (ToolState::Unknown, value) | (value, ToolState::Unknown) => Some(value),
        (ToolState::Pending, ToolState::Running) | (ToolState::Running, ToolState::Pending) => {
            Some(ToolState::Running)
        }
        _ => None,
    }
}

fn tool_identity(
    value: &ToolInvocationSnapshotV1,
) -> (
    jj_agent::ToolCallId,
    AgentSessionId,
    Option<jj_agent::TurnId>,
    &str,
    jj_agent::ToolKind,
) {
    (
        value.tool_call_id,
        value.session_id,
        value.turn_id,
        &value.native_name,
        value.kind,
    )
}

fn merge_coverage(current: CoverageV1, incoming: CoverageV1) -> CoverageV1 {
    CoverageV1 {
        completeness: if current.completeness == CompletenessV1::Complete
            && incoming.completeness == CompletenessV1::Complete
        {
            CompletenessV1::Complete
        } else if current.completeness == CompletenessV1::Unknown
            || incoming.completeness == CompletenessV1::Unknown
        {
            CompletenessV1::Unknown
        } else {
            CompletenessV1::Partial
        },
        gaps: current.gaps.max(incoming.gaps),
        redacted_fields: current.redacted_fields.max(incoming.redacted_fields),
        truncated: current.truncated || incoming.truncated,
    }
}

fn source_identity(
    value: &CaptureSourceStateV1,
) -> (IntegrationId, SourceInstanceId, u64, &str, Option<&str>) {
    (
        value.integration,
        value.source_instance_id,
        value.generation,
        &value.parser_version,
        value.native_session_key.as_deref(),
    )
}

fn source_receipts_prefix(
    prefix: &[CaptureSourceReceiptV1],
    values: &[CaptureSourceReceiptV1],
) -> bool {
    prefix.len() <= values.len() && prefix.iter().zip(values).all(|(left, right)| left == right)
}

use crate::{
    AgentAliasV1, AgentEntityRefV1, CaptureState, NormalizationBatchV1,
    NormalizationDiagnosticKind, NormalizedActivityRecordV1, NormalizedToolRecordV1,
    PermissionState, Result, SanitizedPayloadV1, ToolState,
};

/// A sanitized response with no authority to create a session or invocation.
/// This intermediate type has no persistence encoding.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct ToolResultObservation {
    pub source_sequence: u64,
    pub session_alias: AgentAliasV1,
    pub tool_alias: AgentAliasV1,
    pub output: SanitizedPayloadV1,
}

impl ToolResultObservation {
    pub fn same_batch_call<'a>(
        &self,
        batch: &'a NormalizationBatchV1,
    ) -> Option<&'a NormalizedToolRecordV1> {
        if !batch.aliases.contains(&self.tool_alias) || !batch.aliases.contains(&self.session_alias)
        {
            return None;
        }
        batch.records.iter().find_map(|record| match record {
            NormalizedActivityRecordV1::Tool(tool)
                if self.tool_alias.target
                    == AgentEntityRefV1::Tool {
                        session_id: tool.session_id,
                        tool_call_id: tool.tool_call_id,
                    } =>
            {
                Some(tool)
            }
            _ => None,
        })
    }
}

impl NormalizationBatchV1 {
    /// `known` must come from a matching call or the provider's selected alias/snapshot.
    pub fn resolve_tool_result(
        &mut self,
        result: &ToolResultObservation,
        known: Option<&NormalizedToolRecordV1>,
    ) -> Result<()> {
        let Some(known) = known else {
            self.diagnostic(
                NormalizationDiagnosticKind::UnsupportedEvent,
                "codex-transcript-unmatched-tool-result",
                Some(result.source_sequence),
            );
            return Ok(());
        };
        result.session_alias.validate()?;
        result.tool_alias.validate()?;
        if result.session_alias.target
            != (AgentEntityRefV1::Session {
                session_id: known.session_id,
            })
            || result.tool_alias.target
                != (AgentEntityRefV1::Tool {
                    session_id: known.session_id,
                    tool_call_id: known.tool_call_id,
                })
            || result.session_alias.integration != self.profile.integration
            || result.tool_alias.integration != self.profile.integration
            || result.session_alias.source_instance_id != result.tool_alias.source_instance_id
        {
            return Err(crate::Error::Invalid("tool result resolution identity"));
        }
        let mut next = known.clone();
        next.source_sequence = result.source_sequence;
        next.state = ToolState::Unknown;
        next.capture_state = CaptureState::Observed;
        next.permission_state = PermissionState::NotObserved;
        next.input = None;
        next.output = Some(result.output.clone());
        next.conflicting_terminal_states.clear();
        self.alias(result.session_alias.clone());
        self.alias(result.tool_alias.clone());
        self.records.push(NormalizedActivityRecordV1::Tool(next));
        Ok(())
    }
}

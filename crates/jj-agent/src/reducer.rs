use crate::{
    CaptureState, NormalizationBatchV1, NormalizationDiagnosticKind, NormalizationDiagnosticV1,
    NormalizedActivityRecordV1, NormalizedToolRecordV1, PermissionState, ReducedToolsV1,
    ToolCallId, ToolState,
};
use std::collections::BTreeMap;

impl NormalizationBatchV1 {
    pub fn reduce_tools(&self) -> ReducedToolsV1 {
        let mut tools = BTreeMap::<ToolCallId, NormalizedToolRecordV1>::new();
        let mut diagnostics = Vec::new();
        for record in &self.records {
            let NormalizedActivityRecordV1::Tool(next) = record else {
                continue;
            };
            match tools.get_mut(&next.tool_call_id) {
                None => {
                    tools.insert(next.tool_call_id, next.clone());
                }
                Some(current) => merge_tool(current, next, &mut diagnostics),
            }
        }
        ReducedToolsV1 {
            tools: tools.into_values().collect(),
            diagnostics,
        }
    }
}

fn merge_tool(
    current: &mut NormalizedToolRecordV1,
    next: &NormalizedToolRecordV1,
    diagnostics: &mut Vec<NormalizationDiagnosticV1>,
) {
    current.input = current.input.clone().or_else(|| next.input.clone());
    current.output = current.output.clone().or_else(|| next.output.clone());
    current.capture_state = if current.capture_state == CaptureState::Observed
        || next.capture_state == CaptureState::Observed
    {
        CaptureState::Observed
    } else {
        CaptureState::Partial
    };
    if current.permission_state == PermissionState::NotObserved {
        current.permission_state = next.permission_state;
    }
    let current_terminal = terminal(current.state);
    let next_terminal = terminal(next.state);
    if !current_terminal || next_terminal {
        if current_terminal && next_terminal && current.state != next.state {
            for state in [current.state, next.state] {
                if !current.conflicting_terminal_states.contains(&state) {
                    current.conflicting_terminal_states.push(state);
                }
            }
            diagnostics.push(NormalizationDiagnosticV1 {
                kind: NormalizationDiagnosticKind::ConflictingTerminalState,
                code: "tool-terminal-conflict".into(),
                source_sequence: Some(next.source_sequence),
            });
        } else if !current_terminal {
            current.state = next.state;
        }
    }
}

fn terminal(state: ToolState) -> bool {
    matches!(
        state,
        ToolState::Succeeded
            | ToolState::Failed
            | ToolState::Cancelled
            | ToolState::Interrupted
            | ToolState::Skipped
    )
}

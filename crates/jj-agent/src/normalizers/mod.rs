mod codex;
mod codex_ids;
mod codex_tool_result;
mod codex_transcript;
mod cursor;
mod cursor_tool;
mod opencode;
mod opencode_snapshot;
mod opencode_tools;

use crate::{
    NormalizationBatchV1, NormalizerProfileV1, Result, SanitizedObservationBatchV1,
    ToolResultObservation,
};

pub(crate) fn normalize(
    profile: &NormalizerProfileV1,
    batch: &SanitizedObservationBatchV1,
    output: &mut NormalizationBatchV1,
    tool_results: &mut Vec<ToolResultObservation>,
) -> Result<()> {
    match profile.integration {
        crate::IntegrationId::Codex => codex::normalize(profile, batch, output, tool_results),
        crate::IntegrationId::Cursor => cursor::normalize(profile, batch, output),
        crate::IntegrationId::OpenCode => opencode::normalize(profile, batch, output),
        crate::IntegrationId::OwnedController => unreachable!("unsupported profile returned early"),
    }
}

pub(super) fn value(observation: &crate::SanitizedObservationV1) -> Option<serde_json::Value> {
    (observation.payload.media_type() == crate::SanitizedMediaType::Json)
        .then(|| serde_json::from_slice(observation.payload.bytes()).ok())
        .flatten()
}

pub(super) fn string<'a>(value: &'a serde_json::Value, path: &[&str]) -> Option<&'a str> {
    let mut value = value;
    for key in path {
        value = value.get(*key)?;
    }
    value.as_str()
}

pub(super) fn tool_kind(name: &str) -> crate::ToolKind {
    match name.to_ascii_lowercase().as_str() {
        "read" | "cat" => crate::ToolKind::Read,
        "write" | "edit" | "apply_patch" => crate::ToolKind::Edit,
        "delete" | "remove" => crate::ToolKind::Delete,
        "move" | "rename" => crate::ToolKind::Move,
        "search" | "grep" | "glob" => crate::ToolKind::Search,
        "bash" | "shell" | "exec" | "execute" => crate::ToolKind::Execute,
        "fetch" | "webfetch" => crate::ToolKind::Fetch,
        "plan" => crate::ToolKind::Plan,
        "delegate" | "task" => crate::ToolKind::Delegate,
        _ => crate::ToolKind::Other,
    }
}

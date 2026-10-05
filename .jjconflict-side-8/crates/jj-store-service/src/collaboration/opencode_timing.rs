//! Read-only timing projection from qualified immutable native API evidence.
use jj_store_wal::publication::{
    IntegrationId, NormalizerProfileV1, SanitizedMediaType, SanitizedObservationBatchV1,
    SanitizedObservationV1, SanitizedRepresentation, SourceKind, SourceTrust, ToolCallId,
    normalized_tool_native_id,
};
use serde_json::Value;

pub(super) fn record(
    evidence: &SanitizedObservationBatchV1,
    normalizer: &str,
    observation: &SanitizedObservationV1,
) -> Option<Value> {
    let profile = NormalizerProfileV1::opencode_1_18_30();
    if evidence.integration != IntegrationId::OpenCode
        || evidence.representation != SanitizedRepresentation::SanitizedNative
        || normalizer != profile.identity()
        || observation.source_kind != SourceKind::NativeApi
        || observation.source_trust != SourceTrust::ApprovedHarnessApi
        || observation.payload.media_type() != SanitizedMediaType::Json
    {
        return None;
    }
    let value: Value = serde_json::from_slice(observation.payload.bytes()).ok()?;
    if value["capture_format"] != "opencode-snapshot/v1"
        || value["product_version"] != profile.product_version
        || value["record"]["type"] != "message"
        || value["record"]["info"]["sessionID"] != value["session_id"]
    {
        return None;
    }
    Some(value)
}
fn time(value: &Value) -> Option<i64> {
    value.as_i64().filter(|time| *time >= 0)
}
pub(super) fn creation(
    evidence: &SanitizedObservationBatchV1,
    normalizer: &str,
    observation: &SanitizedObservationV1,
) -> Option<i64> {
    time(&record(evidence, normalizer, observation)?["record"]["info"]["time"]["created"])
}
pub(super) fn completion(
    evidence: &SanitizedObservationBatchV1,
    normalizer: &str,
    observation: &SanitizedObservationV1,
) -> Option<i64> {
    let value = record(evidence, normalizer, observation)?;
    let start = time(&value["record"]["info"]["time"]["created"])?;
    time(&value["record"]["info"]["time"]["completed"]).filter(|end| *end >= start)
}
pub(super) fn tool_span(
    evidence: &SanitizedObservationBatchV1,
    normalizer: &str,
    observation: &SanitizedObservationV1,
    tool: ToolCallId,
) -> Option<(i64, i64)> {
    let value = record(evidence, normalizer, observation)?;
    // Reuse the owning normalizer's native alias mapping rather than inventing IDs.
    let mut single = evidence.clone();
    single.observations = vec![observation.clone()];
    let native_id =
        normalized_tool_native_id(&NormalizerProfileV1::opencode_1_18_30(), &single, tool)
            .ok()??;
    let parts = value["record"]["parts"].as_array()?;
    let mut matching = parts.iter().filter(|part| {
        part["type"] == "tool"
            && part["sessionID"] == value["session_id"]
            && part["messageID"] == value["record"]["info"]["id"]
            && part["callID"].as_str().is_some_and(|call| {
                format!(
                    "{}/{call}",
                    value["session_id"].as_str().unwrap_or_default()
                ) == native_id
            })
    });
    let part = matching.next()?;
    if matching.next().is_some()
        || !matches!(
            part["state"]["status"].as_str(),
            Some("completed" | "error")
        )
    {
        return None;
    }
    let start = time(&part["state"]["time"]["start"])?;
    let end = time(&part["state"]["time"]["end"])?;
    (end >= start).then_some((start, end))
}

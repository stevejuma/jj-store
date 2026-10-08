//! Exact sanitized application context captured before dispatch, independent of the UI.
use super::*;
pub(super) fn capture(document: &prompt_context::Document, original: &str, context: &str) -> Value {
    json!({"version":1,"message":original,"blocks":document.blocks,"sectionRevisions":document.revisions,"delivery":"prepared","wireText":original,"wireContext":context})
}

pub(super) fn outcome(state: &mut State, request: RequestId, status: &str) -> Result<()> {
    let Some(intent) = state.intents.get(&request) else {
        return Ok(());
    };
    let Some(id) = intent.outcome.session_id.clone() else {
        return Ok(());
    };
    let Some(prompt) = intent.prompt.as_ref() else {
        return Ok(());
    };
    if prompt.state == delivery::DeliveryState::Delivered {
        return Ok(());
    }
    let delivery = match status {
        "queued" => "queued",
        "failed" | "cancelled" => "failed",
        "outcome_unknown" | "timed_out" => "unknown",
        _ => return Ok(()),
    };
    let saved = state.sessions.get(&id).ok_or(Error::Invalid)?;
    let Some(mut frame) = saved
        .events
        .iter()
        .rev()
        .find(|e| {
            matches!(
                e.payload["type"].as_str(),
                Some("prompt" | "context_delivery")
            ) && e.payload["requestId"] == request.to_string()
        })
        .map(|e| e.payload.clone())
    else {
        return Ok(());
    };
    if frame["sentContext"]["version"] != 1 || frame["sentContext"]["delivery"] == delivery {
        return Ok(());
    }
    frame["type"] = json!("context_delivery");
    frame["sentContext"]["delivery"] = json!(delivery);
    delivery::append(state, &id, frame)
}

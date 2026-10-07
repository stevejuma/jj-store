//! Exact sanitized application context captured before dispatch, independent of the UI.
use super::*;
pub(super) fn capture(saved: &SavedSession, original: &str, feedback: &str, first: bool) -> Value {
    let mut blocks = Vec::new();
    if first {
        if !saved.instructions.is_empty() {
            blocks.push(json!({"kind":"instructions","title":"Agent instructions","text":saved.instructions}));
        }
        if let Some(context) = &saved.context {
            blocks.push(json!({"kind":"origin","title":"Page and repository context","text":context,"data":serde_json::from_str::<Value>(context).ok()}));
        }
        let retained = commands::retained_context(saved);
        if !retained.is_empty() {
            blocks.push(json!({"kind":"continuation","title":"Retained conversation context","text":retained}));
        }
    }
    if !feedback.is_empty() {
        blocks.push(json!({"kind":"discussion","title":"Discussion replies","text":feedback}));
    }
    json!({"version":1,"message":original,"blocks":blocks,"delivery":"prepared"})
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

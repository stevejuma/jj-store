//! Private reply reservations and exact prompt preparation; no repository writes.
use super::*;
use context::{ContextBatch, ReplyRevision};
use serde::{Deserialize, Serialize};
use std::collections::BTreeSet;

#[derive(Clone, Debug, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub(super) struct PromptDelivery {
    pub original: String,
    pub batch: ContextBatch,
    pub state: DeliveryState,
    #[serde(default)]
    pub generation: u64,
    #[serde(default)]
    pub carries_initial_context: bool,
}
#[derive(Clone, Copy, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub(super) enum DeliveryState {
    Reserved,
    Delivered,
    Released,
}
pub(super) fn needs_initial_context(state: &State, id: &str) -> Result<bool> {
    let saved = state.sessions.get(id).ok_or(Error::Invalid)?;
    if saved.initial_context_delivered {
        return Ok(false);
    }
    // Legacy prompts have no structured delivery record. Preserve their existing
    // instruction behavior without inferring any human-reply delivery from text.
    if saved.context_generation == 0
        && saved.events.iter().any(|e| {
            e.payload["type"] == "prompt"
                && e.payload["requestId"]
                    .as_str()
                    .and_then(|v| v.parse::<RequestId>().ok())
                    .and_then(|r| state.intents.get(&r))
                    .is_none_or(|i| i.prompt.is_none())
        })
    {
        return Ok(false);
    }
    Ok(!state
        .intents
        .values()
        .filter(|i| i.outcome.session_id.as_deref() == Some(id))
        .any(|i| {
            i.prompt.as_ref().is_some_and(|p| {
                p.generation == saved.context_generation
                    && p.carries_initial_context
                    && p.state == DeliveryState::Reserved
            })
        }))
}
pub(super) fn excluded(state: &State, id: &str) -> BTreeSet<ReplyRevision> {
    let generation = state.sessions.get(id).map(|s| s.context_generation);
    let mut revisions = state
        .sessions
        .get(id)
        .map(|s| s.delivered_replies.clone())
        .unwrap_or_default();
    for intent in state
        .intents
        .values()
        .filter(|i| i.outcome.session_id.as_deref() == Some(id))
    {
        if let Some(prompt) = &intent.prompt
            && prompt.state == DeliveryState::Reserved
            && Some(prompt.generation) == generation
        {
            revisions.extend(prompt.batch.revisions.iter().cloned());
        }
    }
    revisions
}
#[allow(clippy::too_many_arguments)]
pub(super) fn prepare(
    state: &mut State,
    id: &str,
    request: RequestId,
    digest: String,
    original: String,
    batch: ContextBatch,
    frame: Value,
    carries_initial_context: bool,
) -> Result<()> {
    if state.intents.len() >= 4096 || state.intents.contains_key(&request) {
        return Err(Error::Limit);
    }
    let generation = state
        .sessions
        .get(id)
        .ok_or(Error::Invalid)?
        .context_generation;
    state.intents.insert(
        request,
        Intent {
            digest,
            outcome: Outcome {
                version: 1,
                request_id: request,
                session_id: Some(id.into()),
                state: "dispatching".into(),
                error: None,
            },
            prompt: Some(PromptDelivery {
                original,
                batch,
                state: DeliveryState::Reserved,
                generation,
                carries_initial_context,
            }),
        },
    );
    append(state, id, frame)
}
pub(super) fn append(state: &mut State, id: &str, mut payload: Value) -> Result<()> {
    payload["controllerSessionId"] = json!(id);
    let saved = state.sessions.get_mut(id).ok_or(Error::Invalid)?;
    if saved.events.len() >= 8192 {
        return Err(Error::Limit);
    }
    saved.view.last_sequence += 1;
    let observed_at_millis = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .ok()
        .and_then(|elapsed| u64::try_from(elapsed.as_millis()).ok());
    if let Some(now) = observed_at_millis {
        saved.view.last_activity_at_millis =
            Some(saved.view.last_activity_at_millis.unwrap_or(0).max(now));
    }
    saved.events.push(Event {
        sequence: saved.view.last_sequence,
        observed_at_millis,
        payload,
        committed: false,
    });
    Ok(())
}
pub(super) fn observe(state: &mut State, id: &str, event: &Value) -> Result<()> {
    let Some(request) = event["requestId"]
        .as_str()
        .and_then(|v| v.parse::<RequestId>().ok())
    else {
        return Ok(());
    };
    let Some(intent) = state.intents.get_mut(&request) else {
        return Ok(());
    };
    if intent.outcome.session_id.as_deref() != Some(id) {
        return Err(Error::Invalid);
    }
    let Some(prompt) = &mut intent.prompt else {
        return Ok(());
    };
    if event["type"] == "started" {
        // This transition is persisted in the same update as the start event.
        prompt.state = DeliveryState::Delivered;
        let saved = state.sessions.get_mut(id).ok_or(Error::Invalid)?;
        if prompt.generation == saved.context_generation {
            if prompt.carries_initial_context {
                saved.initial_context_delivered = true;
            }
            saved
                .delivered_replies
                .extend(prompt.batch.revisions.iter().cloned());
        }
        intent.outcome.state = "started".into();
        if let Some(mut frame) = saved
            .events
            .iter()
            .find(|e| {
                e.payload["type"] == "prompt" && e.payload["requestId"] == request.to_string()
            })
            .map(|e| e.payload.clone())
        {
            if frame["sentContext"]["version"] == 1 {
                frame["type"] = json!("context_delivery");
                frame["sentContext"]["delivery"] = json!("sent");
                append(state, id, frame)?;
            }
        }
    } else if prompt.state == DeliveryState::Reserved
        && ((event["type"] == "record" && event["record"]["status"] == "cancelled")
            || (event["type"] == "error"
                && matches!(
                    event["code"].as_str(),
                    Some(
                        "execution_mode_required"
                            | "queue_full"
                            | "request_limit"
                            | "dispatch_not_sent"
                    )
                )))
    {
        prompt.state = DeliveryState::Released;
    }
    Ok(())
}

pub(super) fn continuation(state: &mut State, id: &str) -> Result<()> {
    let saved = state.sessions.get(id).ok_or(Error::Invalid)?;
    let mut lines = Vec::new();
    let mut bytes = "Saved context from prior session (new adapter session):\n".len();
    let mut included = BTreeSet::new();
    // Whole newest records only: never truncate a structured feedback batch.
    for event in saved
        .events
        .iter()
        .rev()
        .filter(|e| matches!(e.payload["type"].as_str(), Some("prompt" | "record")))
    {
        let line = event.payload.to_string();
        if bytes + line.len() + 1 > 32768 {
            break;
        }
        bytes += line.len() + 1;
        if event.payload["type"] == "prompt"
            && let Some(request) = event.payload["requestId"]
                .as_str()
                .and_then(|v| v.parse::<RequestId>().ok())
            && let Some(prompt) = state.intents.get(&request).and_then(|i| i.prompt.as_ref())
        {
            included.extend(prompt.batch.revisions.iter().cloned());
        }
        lines.push(line);
    }
    lines.reverse();
    let saved = state.sessions.get_mut(id).ok_or(Error::Invalid)?;
    saved.context = Some(format!(
        "Saved context from prior session (new adapter session):\n{}",
        lines.join("\n")
    ));
    saved.view.adapter_session_id = None;
    saved.context_generation = saved
        .context_generation
        .checked_add(1)
        .ok_or(Error::Limit)?;
    saved.context_replies = included;
    saved.delivered_replies.clear();
    saved.initial_context_delivered = false;
    Ok(())
}

#[cfg(test)]
mod tests;

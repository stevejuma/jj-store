//! Prepared context is retained independently of authored prompt bytes.
use super::*;

pub(super) fn prepared(saved: &SavedSession, feedback: &str, first: bool) -> Result<String> {
    let history = commands::retained_context(saved);
    let mut sections = Vec::new();
    if first {
        sections.push(section("instructions", &saved.instructions));
        sections.push(section(
            "repository_context",
            saved.context.as_deref().unwrap_or(""),
        ));
        sections.push(section("conversation_history", &history));
    }
    sections.push(section("discussion_context", feedback));
    let body = sections
        .into_iter()
        .filter(|s| !s.is_empty())
        .collect::<Vec<_>>()
        .join("\n");
    let context = if body.is_empty() {
        String::new()
    } else {
        format!("<agent_context>\n{body}\n</agent_context>")
    };
    if context.len() > 512 * 1024 {
        return Err(Error::FeedbackLimit);
    }
    Ok(context)
}

// Fenced data prevents captured XML/Markdown from closing the context section.
// The fence is longer than every run in the body; no authored bytes are lost.
pub(super) fn section(name: &str, value: &str) -> String {
    if value.is_empty() {
        return String::new();
    }
    let longest = value.split(|c| c != '`').map(str::len).max().unwrap_or(0);
    let fence = "`".repeat(longest.max(2) + 1);
    let (language, body) = match serde_json::from_str::<Value>(value) {
        Ok(value) => (
            "json",
            serde_json::to_string_pretty(&value).unwrap_or_default(),
        ),
        Err(_) => ("text", value.to_owned()),
    };
    format!("<{name}>\n{fence}{language}\n{body}\n{fence}\n</{name}>")
}

/// Older frames retained a prepared prefix in text. Project only an exact
/// intent-owned suffix; never guess where authored text starts or rewrite evidence.
pub(super) fn presentation(event: &mut Event, state: &State) {
    if event.payload["type"] != "prompt" || event.payload.get("context").is_some() {
        return;
    }
    let Some(request) = event.payload["requestId"]
        .as_str()
        .and_then(|s| s.parse::<RequestId>().ok())
    else {
        return;
    };
    let Some(delivery) = state.intents.get(&request).and_then(|i| i.prompt.as_ref()) else {
        return;
    };
    let Some(captured) = event.payload["text"].as_str() else {
        return;
    };
    let Some(prefix) = captured.strip_suffix(&delivery.original) else {
        return;
    };
    let prefix = prefix.to_owned();
    event.payload["authoredText"] = json!(delivery.original);
    if !prefix.trim().is_empty() {
        event.payload["context"] = json!(format!(
            "<agent_context>\n{}\n</agent_context>",
            section("captured_context", &prefix)
        ));
    }
}

#[cfg(test)]
mod tests;

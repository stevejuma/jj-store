//! Bounded authored history, without injected context or observer frames.
use super::*;
pub(in crate::agent_execution) fn retained(state: &State, id: &str) -> String {
    let Some(saved) = state.sessions.get(id) else {
        return String::new();
    };
    let mut messages = Vec::new();
    let mut bytes = 0;
    let mut omitted = false;
    const NOTICE: &str = "_Earlier conversation was omitted to retain whole authored messages within the context limit._";
    for event in saved.events.iter().rev() {
        let (role, text) = match event.payload["type"].as_str() {
            Some("prompt") => {
                let original = event.payload["requestId"]
                    .as_str()
                    .and_then(|r| r.parse::<RequestId>().ok())
                    .and_then(|r| state.intents.get(&r))
                    .and_then(|i| i.prompt.as_ref())
                    .map(|p| p.original.as_str());
                let text = original
                    .or_else(|| event.payload["userText"].as_str())
                    .or_else(|| event.payload["authoredText"].as_str())
                    .or_else(|| {
                        event
                            .payload
                            .get("context")
                            .and_then(|_| event.payload["text"].as_str())
                    });
                let Some(text) = text else {
                    omitted = true; // Never guess legacy prefix boundaries.
                    continue;
                };
                ("User", text)
            }
            Some("record") => {
                let Some(text) = event.payload["assistantText"].as_str() else {
                    continue;
                };
                ("Assistant", text)
            }
            _ => continue,
        };
        if text.is_empty() {
            continue;
        }
        let rendered = format!("### {role}\n\n{text}");
        if bytes + rendered.len() + 2 + NOTICE.len() + 2 > 16384 || messages.len() == 20 {
            omitted = true;
            break;
        }
        bytes += rendered.len() + 2;
        messages.push(rendered);
    }
    messages.reverse();
    if omitted {
        messages.insert(0, NOTICE.into());
    }
    messages.join("\n\n")
}

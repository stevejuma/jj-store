//! Only visible authored messages and concise tool headers enter discovery input.
use serde_json::Value;
use std::collections::BTreeMap;
pub(super) const LIMIT: usize = 128 * 1024;
pub(super) fn append(text: &mut String, value: &str, partial: &mut bool) {
    let available = LIMIT.saturating_sub(text.len());
    let mut end = value.len().min(available);
    while !value.is_char_boundary(end) {
        end -= 1;
    }
    text.push_str(&value[..end]);
    *partial |= end < value.len();
}
pub(super) fn project(detail: &Value) -> (String, BTreeMap<String, u64>, bool) {
    let mut text = String::new();
    let mut counts = BTreeMap::new();
    let coverage = &detail["session"]["coverage"];
    let mut partial = detail["activity_truncated"] == true
        || coverage["truncated"] == true
        || coverage["completeness"] != "complete"
        || coverage["gaps"].as_u64().is_some_and(|n| n > 0);
    for activity in detail["activity"].as_array().into_iter().flatten() {
        match activity["type"].as_str() {
            Some("message") => {
                let role = activity["role"].as_str().unwrap_or("unknown");
                *counts.entry(role.to_owned()).or_insert(0) += 1;
                if matches!(role, "user" | "assistant") {
                    visible(&activity["content"], role, &mut text, &mut partial);
                }
            }
            Some("tool") => {
                *counts.entry("tools".into()).or_insert(0) += 1;
                let name = activity["tool"]["native_name"].as_str().unwrap_or("Tool");
                append(&mut text, &format!("\n[tool] {name}\n"), &mut partial);
            }
            Some("capture_gap") => partial = true,
            _ => {}
        }
    }
    (text, counts, partial)
}
fn visible(content: &Value, role: &str, text: &mut String, partial: &mut bool) {
    let Some(body) = content["content"].as_str() else {
        *partial = true;
        return;
    };
    match content["media_type"].as_str() {
        Some("text") => append(text, &format!("\n{role}: {body}\n"), partial),
        Some("json") => {
            let Ok(value) = serde_json::from_str::<Value>(body) else {
                *partial = true;
                return;
            };
            if role == "user" && value["schema"] == "jj-store.sent-context.v1" {
                if let Some(authored) = value["text"].as_str() {
                    append(text, &format!("\nuser: {authored}\n"), partial);
                } else {
                    *partial = true;
                }
                // The retained inspector context is not authored prompt text.
                return;
            }
            // OpenCode stores rich message parts. Only declared visible text is input;
            // sanitized JSON can still contain reasoning, tool bodies and attachments.
            let Value::Array(parts) = value else {
                *partial = true;
                return;
            };
            for part in parts {
                if part["type"] == "text" && part["ignored"] != true && part["synthetic"] != true {
                    if let Some(body) = part["text"].as_str() {
                        append(text, &format!("\n{role}: {body}\n"), partial);
                    } else {
                        *partial = true;
                    }
                } else if !matches!(part["type"].as_str(), Some("reasoning" | "tool")) {
                    *partial = true;
                }
            }
        }
        _ => *partial = true,
    }
}
#[cfg(test)]
mod tests {
    use super::*;
    use serde_json::json;
    #[test]
    fn visible_input_excludes_reasoning_system_and_tool_bodies_and_retains_gaps() {
        let detail = json!({"session":{"coverage":{"completeness":"partial","gaps":1}},"activity":[
            {"type":"message","role":"user","content":{"media_type":"text","content":"hello 🦀"}},
            {"type":"message","role":"assistant","content":{"media_type":"text","content":"answer"}},
            {"type":"message","role":"system","content":{"content":"SECRET_SYSTEM"}},
            {"type":"message","role":"reasoning","content":{"content":"SECRET_REASONING"}},
            {"type":"tool","tool":{"native_name":"Execute","output":"SECRET_TOOL_BODY"}}
        ]});
        let (text, counts, partial) = project(&detail);
        assert!(
            text.contains("hello 🦀") && text.contains("answer") && text.contains("[tool] Execute")
        );
        assert!(!text.contains("SECRET_"));
        assert!(partial);
        assert_eq!(counts["tools"], 1);
    }
    #[test]
    fn byte_budget_does_not_split_unicode_or_grow_across_sources() {
        let mut text = "x".repeat(LIMIT - 2);
        let mut partial = false;
        append(&mut text, "🦀", &mut partial);
        append(&mut text, "🦀 tail", &mut partial);
        assert_eq!(text.len(), LIMIT - 2);
        assert!(partial);
    }
    #[test]
    fn rich_parts_include_only_visible_text_and_unknown_json_is_not_guessed() {
        let parts = json!([
            {"type":"text","text":"Visible 🦀"},
            {"type":"reasoning","text":"SECRET_THOUGHT"},
            {"type":"tool","state":{"output":"SECRET_OUTPUT"}},
            {"type":"file","url":"SECRET_IMAGE"},
            {"type":"text","text":"SECRET_SYNTHETIC","synthetic":true},
            {"type":"text","text":"SECRET_IGNORED","ignored":true}
        ]);
        let detail = json!({"session":{"coverage":{"completeness":"complete"}},"activity":[
            {"type":"message","role":"assistant","content":{"media_type":"json","content":parts.to_string()}},
            {"type":"message","role":"user","content":{"media_type":"json","content":"{\"text\":\"SECRET_UNKNOWN\"}"}}
        ]});
        let (text, _, partial) = project(&detail);
        assert!(text.contains("Visible 🦀"));
        assert!(!text.contains("SECRET_"));
        assert!(partial);
    }
    #[test]
    fn owned_sent_context_keeps_authored_text_without_bootstrap_or_context_bodies() {
        let body = json!({"schema":"jj-store.sent-context.v1","text":"Review this request 🦀", "context":{
            "version":1,"bootstrap":"SECRET_INSTRUCTIONS","tool":"SECRET_TOOL_BODY"
        }});
        let detail = json!({"session":{"coverage":{"completeness":"complete"}},"activity":[
            {"type":"message","role":"user","content":{"media_type":"json","content":body.to_string()}}
        ]});
        let (text, _, partial) = project(&detail);
        assert!(text.contains("user: Review this request 🦀"));
        assert!(!text.contains("SECRET_") && !text.contains("sent-context"));
        assert!(!partial);
    }
}

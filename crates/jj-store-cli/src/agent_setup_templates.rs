use jj_agent::IntegrationId;
use jj_store_jj::operation_stores::AuthorityResult;
use serde_json::{Map, Value, json};
use std::path::Path;

pub(crate) fn render(
    integration: IntegrationId,
    original: Option<&[u8]>,
    executable: &Path,
) -> AuthorityResult<Vec<u8>> {
    let executable = executable
        .to_str()
        .ok_or("agent hook executable path is not UTF-8")?;
    match integration {
        IntegrationId::Codex => merge_codex(original, &hook_command(executable, "codex")?),
        IntegrationId::Cursor => merge_cursor(original, &hook_command(executable, "cursor")?),
        IntegrationId::OpenCode => render_opencode(original, executable),
        IntegrationId::OwnedController => Err("owned controller has no harness setup".into()),
    }
}

fn hook_command(executable: &str, integration: &str) -> AuthorityResult<String> {
    let executable = shlex::try_quote(executable)
        .map_err(|_| "agent hook executable path contains an unsupported byte")?;
    Ok(format!("{executable} agent hook {integration}"))
}

fn parse_object(original: Option<&[u8]>) -> AuthorityResult<Map<String, Value>> {
    let value = match original {
        Some(bytes) if !bytes.is_empty() => serde_json::from_slice(bytes)?,
        _ => json!({}),
    };
    value
        .as_object()
        .cloned()
        .ok_or_else(|| "agent hook configuration must be a JSON object".into())
}

fn hooks(root: &mut Map<String, Value>) -> AuthorityResult<&mut Map<String, Value>> {
    root.entry("hooks").or_insert_with(|| json!({}));
    root.get_mut("hooks")
        .and_then(Value::as_object_mut)
        .ok_or_else(|| "agent hooks must be a JSON object".into())
}

fn event<'a>(hooks: &'a mut Map<String, Value>, name: &str) -> AuthorityResult<&'a mut Vec<Value>> {
    hooks.entry(name).or_insert_with(|| json!([]));
    hooks
        .get_mut(name)
        .and_then(Value::as_array_mut)
        .ok_or_else(|| "agent hook event must be a JSON array".into())
}

fn has_command(value: &Value, command: &str) -> bool {
    value
        .get("command")
        .and_then(Value::as_str)
        .is_some_and(|value| value == command)
        || value
            .get("hooks")
            .and_then(Value::as_array)
            .is_some_and(|values| values.iter().any(|value| has_command(value, command)))
}

fn merge_codex(original: Option<&[u8]>, command: &str) -> AuthorityResult<Vec<u8>> {
    let mut root = parse_object(original)?;
    let hooks = hooks(&mut root)?;
    for name in [
        "SessionStart",
        "UserPromptSubmit",
        "PreToolUse",
        "PostToolUse",
        "SessionEnd",
    ] {
        let handlers = event(hooks, name)?;
        if !handlers.iter().any(|value| has_command(value, command)) {
            handlers.push(json!({
                "hooks": [{
                    "type": "command",
                    "command": command,
                    "timeout": 5,
                    "async": false
                }]
            }));
        }
    }
    Ok(serde_json::to_vec_pretty(&Value::Object(root))?)
}

fn merge_cursor(original: Option<&[u8]>, command: &str) -> AuthorityResult<Vec<u8>> {
    let mut root = parse_object(original)?;
    match root.get("version") {
        None => {
            root.insert("version".into(), json!(1));
        }
        Some(Value::Number(value)) if value.as_u64() == Some(1) => {}
        Some(_) => return Err("unsupported Cursor hooks version".into()),
    }
    let hooks = hooks(&mut root)?;
    for name in [
        "sessionStart",
        "beforeSubmitPrompt",
        "preToolUse",
        "postToolUse",
        "postToolUseFailure",
        "afterFileEdit",
        "afterAgentResponse",
        "sessionEnd",
    ] {
        let handlers = event(hooks, name)?;
        if !handlers.iter().any(|value| has_command(value, command)) {
            handlers.push(json!({"command": command}));
        }
    }
    Ok(serde_json::to_vec_pretty(&Value::Object(root))?)
}

fn render_opencode(original: Option<&[u8]>, executable: &str) -> AuthorityResult<Vec<u8>> {
    let executable = serde_json::to_string(executable)?;
    let template = format!(
        r#"import type {{ Plugin }} from "@opencode-ai/plugin"

const executable = {executable}
const forward = async (hook: string, input: unknown, output?: unknown) => {{
  const child = Bun.spawn([executable, "agent", "hook", "opencode"], {{
    stdin: "pipe",
    stdout: "ignore",
    stderr: "ignore",
  }})
  child.stdin.write(JSON.stringify({{ hook, input, output }}))
  child.stdin.end()
  await child.exited
}}

export const JjStoreAgentPlugin: Plugin = async () => ({{
  event: (input) => forward("event", input),
  "chat.message": (input, output) => forward("chat.message", input, output),
  "tool.execute.before": (input, output) => forward("tool.execute.before", input, output),
  "tool.execute.after": (input, output) => forward("tool.execute.after", input, output),
}})
"#
    )
    .into_bytes();
    if original.is_some_and(|value| value != template) {
        return Err("OpenCode jj-store plugin target already exists and is not owned".into());
    }
    Ok(template)
}

//! One sanitized context document supplies both the wire and retained inspector.
use super::*;
use serde::{Deserialize, Serialize};
mod cli;
mod history;
mod markdown;
mod origin;
mod tracking;
pub(super) use history::retained;

#[derive(Clone, Serialize, Deserialize)]
pub(super) struct Section {
    pub kind: String,
    pub title: String,
    pub text: String,
}
impl Section {
    fn new(kind: &str, title: &str, text: String) -> Self {
        Self {
            kind: kind.into(),
            title: title.into(),
            text,
        }
    }
}
#[derive(Default, Serialize, Deserialize)]
pub(super) struct Document {
    pub blocks: Vec<Section>,
    pub revisions: BTreeMap<String, String>,
}
impl Document {
    fn render(&self) -> Result<String> {
        let body = self
            .blocks
            .iter()
            .map(|b| section(&b.kind, &b.text))
            .collect::<Vec<_>>()
            .join("\n\n");
        let rendered = if body.is_empty() {
            String::new()
        } else {
            format!("<agent_context>\n\n{body}\n\n</agent_context>")
        };
        if rendered.len() > 512 * 1024 {
            return Err(Error::FeedbackLimit);
        }
        Ok(rendered)
    }
}

impl Supervisor {
    pub(super) async fn prompt_frame(
        &self,
        saved: &SavedSession,
        command: &Command,
        batch: &mut context::ContextBatch,
        first: bool,
    ) -> Result<Value> {
        let Action::Prompt {
            text,
            max_duration_ms,
        } = &command.action
        else {
            return Err(Error::Invalid);
        };
        self.initialize_section_tracking(&saved.view.id).await?;
        let sanitizer = runtime::sanitizer(self, &saved.workspace.path).await?;
        *batch = serde_json::from_value(runtime::safe(
            &sanitizer,
            serde_json::to_value(&batch).map_err(|_| Error::Invalid)?,
        )?)
        .map_err(|_| Error::Invalid)?;
        if batch.text.len() > 32768 {
            return Err(Error::FeedbackLimit);
        }
        let mut document = Document::default();
        if first {
            document.blocks.push(Section::new(
                "instructions",
                "Agent instructions",
                saved.instructions.clone(),
            ));
            document.blocks.push(Section::new(
                "cli",
                "Bundled JJ CLI",
                cli::guidance(saved, &self.0.resources.jj_executable),
            ));
            let mut workspace = format!(
                "Working directory: {}\n\nMetadata home: {}",
                saved.workspace.path.display(),
                saved.view.repository
            );
            if saved.view.conversation_id.is_none() {
                workspace.push_str(&format!(
                    "\n\nSelected operation: {}\n\nBase commit: {}",
                    saved.view.operation, saved.view.base_commit
                ));
            }
            workspace.push_str("\n\nCode edits belong in the listed code directories. Coordination metadata uses its own checkout.");
            document
                .blocks
                .push(Section::new("workspace", "Workspace", workspace));
            document.blocks.push(Section::new(
                "conversation_history",
                "Retained conversation",
                saved.continuation.clone(),
            ));
        }
        let known = self.read(|s| Ok(delivery::section_revisions(s, &saved.view.id)))?;
        for block in origin::sections(saved) {
            let block: Section = serde_json::from_value(runtime::safe(
                &sanitizer,
                serde_json::to_value(block).map_err(|_| Error::Invalid)?,
            )?)
            .map_err(|_| Error::Invalid)?;
            let revision = journal::digest(&block)?;
            if first || known.get(&block.kind) != Some(&revision) {
                document.revisions.insert(block.kind.clone(), revision);
                document.blocks.push(block);
            }
        }
        let discussion = origin::discussion(batch);
        if discussion.len() > 32768 {
            return Err(Error::FeedbackLimit);
        }
        document.blocks.push(Section::new(
            "discussion_context",
            "Discussion replies",
            discussion,
        ));
        document.blocks.retain(|b| !b.text.is_empty());
        let document: Document = serde_json::from_value(runtime::safe(
            &sanitizer,
            serde_json::to_value(document).map_err(|_| Error::Invalid)?,
        )?)
        .map_err(|_| Error::Invalid)?;
        let original = runtime::safe(&sanitizer, json!({"text":text}))?["text"]
            .as_str()
            .ok_or(Error::Invalid)?
            .to_owned();
        let context = document.render()?;
        // The structured source is sanitized before Markdown framing. Evidence and
        // dispatch receive precisely the same final text and context bytes.
        let captured = sent_context::capture(&document, &original, &context);
        Ok(
            json!({"version":2,"type":"prompt","requestId":command.request_id.to_string(),"text":original,"context":context,"maxDurationMs":max_duration_ms,"userText":original,"sentContext":captured}),
        )
    }
}

pub(super) fn section(name: &str, value: &str) -> String {
    if value.is_empty() {
        return String::new();
    }
    format!("<{name}>\n\n{}\n\n</{name}>", markdown::body(value))
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
            legacy_section("captured_context", &prefix)
        ));
    }
}

#[cfg(test)]
mod tests;

// Immutable legacy captures keep their original bytes inside an inert fence.
fn legacy_section(name: &str, value: &str) -> String {
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

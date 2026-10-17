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
            ..
        } = &command.action
        else {
            return Err(Error::Invalid);
        };
        self.initialize_section_tracking(&saved.view.id).await?;
        let sanitizer = runtime::sanitizer(self, &saved.workspace.path).await?;
        sanitize_batch(batch, |text| {
            runtime::safe(&sanitizer, json!({"text": text}))?["text"]
                .as_str()
                .map(str::to_owned)
                .ok_or(Error::Invalid)
        })?;
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
        for block in &mut document.blocks {
            block.text = runtime::safe(&sanitizer, json!({"text": &block.text}))?["text"]
                .as_str()
                .ok_or(Error::Invalid)?
                .to_owned();
        }
        // Trusted protocol tokens must never be rewritten by text redaction.
        // Only typed references enter this section; all captured strings above
        // are sanitized separately before Markdown framing.
        if !batch.replies.is_empty() || !batch.comments.is_empty() {
            document.blocks.push(Section::new(
                "comment_reply_protocol",
                "Comment reply protocol v1",
                reply_instructions(batch)?,
            ));
        }
        let original = runtime::safe(&sanitizer, json!({"text":text}))?["text"]
            .as_str()
            .ok_or(Error::Invalid)?
            .to_owned();
        let context = document.render()?;
        // The structured source is sanitized before Markdown framing. Evidence and
        // dispatch receive precisely the same final text and context bytes.
        let mut captured = sent_context::capture(&document, &original, &context);
        captured["commentReplyProtocol"] =
            serde_json::to_value(reply_protocol(batch)?).map_err(|_| Error::Invalid)?;
        if let Some(submission) = &batch.submission {
            captured["commentSubmission"] =
                serde_json::to_value(submission).map_err(|_| Error::Invalid)?;
        }
        Ok(
            json!({"version":3,"type":"prompt","requestId":command.request_id.to_string(),"text":original,"context":context,"maxDurationMs":max_duration_ms,"userText":original,"sentContext":captured}),
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

fn reply_protocol(batch: &context::ContextBatch) -> Result<jj_collab::CommentReplyProtocolV1> {
    batch.protocol()
}
fn sanitize_batch(
    batch: &mut context::ContextBatch,
    mut sanitize: impl FnMut(&str) -> Result<String>,
) -> Result<()> {
    batch.text = sanitize(&batch.text)?;
    for comment in &mut batch.comments {
        comment.body = sanitize(&comment.body)?;
        comment.surrounding = sanitize(&comment.surrounding)?;
        if let Some(source) = &mut comment.source {
            source.quote = sanitize(&source.quote)?;
        }
        if let Some(file) = &mut comment.file {
            file.quote = sanitize(&file.quote)?;
        }
    }
    for reply in &mut batch.replies {
        reply.body = sanitize(&reply.body)?;
        reply.thread_title = sanitize(&reply.thread_title)?;
        if let Some(source) = &mut reply.source {
            source.quote = sanitize(&source.quote)?;
        }
    }
    Ok(())
}
fn reply_instructions(batch: &context::ContextBatch) -> Result<String> {
    let protocol = reply_protocol(batch)?;
    let mut text = String::from(
        "When answering a supplied comment, emit one comment-reply block for its discussion. Copy the supplied discussion and comment identifiers exactly. List only the comments actually answered. Several comments at one anchor may share an answer. Answer different discussions in separate blocks anywhere among your normal prose and tools in the same turn. Do not manufacture identifiers, match quotes, resolve reviews, or run commands from tags. Example syntax (replace D1 and C1 C2 with the supplied identifiers):\n\n```xml\n<comment-reply discussion=\"D1\" comments=\"C1 C2\">\nAnswer in Markdown.\n</comment-reply>\n```\n\nSupplied references:\n",
    );
    for reference in &protocol.references {
        text.push_str(&format!(
            "\n- Discussion: {}; comment: {}; target: {}",
            reference.discussion,
            reference.comment,
            serde_json::to_string(&reference.target).map_err(|_| Error::Invalid)?
        ));
    }
    Ok(text)
}

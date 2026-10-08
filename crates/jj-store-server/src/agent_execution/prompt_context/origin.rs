//! Readable projection of captured application data; never an authority input.
use super::*;

pub(super) fn sections(saved: &SavedSession) -> Vec<Section> {
    let mut sections = Vec::new();
    if let Some(context) = &saved.context {
        match serde_json::from_str::<Value>(context) {
            Ok(value) if value.get("origin").is_some() => {
                sections.push(Section::new(
                    "origin",
                    "Page context",
                    readable(&value["origin"], 2),
                ));
            }
            Ok(value) => sections.push(Section::new("origin", "Page context", readable(&value, 2))),
            Err(_) => sections.push(Section::new("origin", "Page context", context.clone())),
        }
    }
    for binding in &saved.view.repository_bindings {
        sections.push(Section::new(&format!("repository_{}", binding.repository), "Code repository", format!(
            "Repository: {}\n\nDirectory: `repositories/{}`\n\nSelected operation: {}\n\nBase commit: {}",
            binding.repository, binding.repository, binding.selected_operation, binding.base_commit)));
    }
    sections
}

pub(super) fn readable(value: &Value, depth: usize) -> String {
    match value {
        Value::Null => String::new(),
        Value::String(value) => value.clone(),
        Value::Array(values) => values
            .iter()
            .map(|v| readable(v, depth))
            .filter(|v| !v.is_empty())
            .collect::<Vec<_>>()
            .join("\n\n"),
        Value::Object(values) => values
            .iter()
            .filter(|(_, v)| !v.is_null())
            .map(|(key, value)| {
                let title = key.replace('_', " ");
                // Origin.context is a historical JSON string, not a second prompt.
                let parsed = (key == "context")
                    .then(|| {
                        value
                            .as_str()
                            .and_then(|s| serde_json::from_str::<Value>(s).ok())
                    })
                    .flatten();
                let value = parsed.as_ref().unwrap_or(value);
                let text = readable(value, (depth + 1).min(6));
                if value.is_object()
                    || value.is_array()
                    || text.contains('\n')
                    || matches!(key.as_str(), "body" | "content" | "context")
                {
                    format!("{} {title}\n\n{text}", "#".repeat(depth.min(6)))
                } else {
                    format!("- {title}: {text}")
                }
            })
            .collect::<Vec<_>>()
            .join("\n\n"),
        _ => value.to_string(),
    }
}

pub(super) fn discussion(batch: &context::ContextBatch) -> String {
    let mut sections = Vec::new();
    if !batch.comments.is_empty() {
        sections.push(batch
            .comments
            .iter()
            .map(|comment| {
                format!(
                    "## Discussion {}\n\nComment: {}\n\n{}\n\nSource: {}\n\nSurrounding captured context:\n\n{}",
                    comment.reference.discussion,
                    comment.reference.comment,
                    comment.body,
                    comment
                        .source
                        .as_ref()
                        .map(|s| s.quote.as_str())
                        .unwrap_or("Whole section"),
                    comment.surrounding
                )
            })
            .collect::<Vec<_>>()
            .join("\n\n"));
    }
    if batch.replies.is_empty() {
        if sections.is_empty() {
            return batch.text.clone();
        }
        return sections.join("\n\n");
    }
    sections.push(batch.replies.iter().map(|reply| {
        let mut text = format!("## {}\n\nRepository: {}\n\nThread: {}\n\nMessage: {}\n\nRevision: {}\n\nActor: {}\n\nThread revision: {}",
            reply.thread_title, reply.reply.repository, reply.reply.thread, reply.reply.message,
            reply.reply.revision.id, reply.actor, reply.thread_revision.id);
        if let Some(parent) = reply.reply_to { text.push_str(&format!("\n\nReply to revision: {}", parent.id)); }
        if let Some(target) = &reply.original_message {
            text.push_str(&format!("\n\n### Original target\n\n{}", readable(&serde_json::to_value(target).unwrap_or_default(), 4)));
        }
        if let Some(source) = &reply.source {
            text.push_str(&format!("\n\n### Exact source\n\n{}\n\n{}", serde_json::to_string(source).unwrap_or_default(), source.quote));
        }
        text.push_str(&format!("\n\n### Reply\n\n{}", reply.body));
        text
    }).collect::<Vec<_>>().join("\n\n"));
    sections.join("\n\n")
}

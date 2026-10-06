//! Bounded exact message feedback shared by native and local selections.
use crate::agent_execution::{Error, Feedback, FeedbackMessage, FeedbackReply, Result};
use jj_collab::*;
use jj_store_wal::publication::*;
use jj_store_wal::publication::{AgentSnapshotRead, CollabTargetReaderV1};
#[cfg(test)]
mod tests;

pub(crate) async fn project(
    agents: &dyn AgentSnapshotRead,
    state: &ThreadStateV1,
    graph: &CollabGraphV1,
    rows: &[(String, AgentSessionId, MessageId)],
) -> Result<Feedback> {
    if rows.len() > 256 || state.messages.len() > 4096 || state.threads.len() > 4096 {
        return Err(Error::Limit);
    }
    let reader = CollabTargetReaderV1::new(agents.repository(), None, Some(agents), None)
        .map_err(|_| Error::Unavailable)?;
    let mut targets = std::collections::BTreeMap::new();
    let mut reads = 0;
    for session_id in rows
        .iter()
        .map(|r| r.1)
        .collect::<std::collections::BTreeSet<_>>()
    {
        let key = AgentCatalogKey::derive("session", session_id.as_bytes())
            .map_err(|_| Error::Invalid)?;
        let Some(reference) = agents
            .lookup(AgentCatalogKind::Sessions, key)
            .await
            .map_err(|_| Error::Unavailable)?
        else {
            continue;
        };
        let AgentObjectV1::SessionSnapshot(session) = agents
            .read_object(reference)
            .await
            .map_err(|_| Error::Unavailable)?
        else {
            continue;
        };
        if session.session_id != session_id {
            return Err(Error::Invalid);
        }
        for batch in &session.event_batches {
            reads += 1;
            if reads > 4096 {
                return Err(Error::Limit);
            }
            let AgentObjectV1::EventBatch(events) = agents
                .read_object(*batch)
                .await
                .map_err(|_| Error::Unavailable)?
            else {
                return Err(Error::Invalid);
            };
            for event in events.events {
                if let AgentEventV1::MessageCompleted {
                    event_id,
                    session_id: sid,
                    message_id,
                    content,
                    ..
                } = event
                    && sid == session_id
                {
                    targets.insert(
                        (sid, message_id),
                        ReactionTargetV1::AgentMessage {
                            session_id: sid,
                            message_id,
                            event_id,
                            batch: *batch,
                            content,
                        },
                    );
                }
            }
        }
    }
    let mut messages = Vec::new();
    for (row_id, session_id, message_id) in rows {
        let Some(target) = targets.get(&(*session_id, *message_id)).cloned() else {
            continue;
        };
        resolve_reaction_target(&reader, &target)
            .await
            .map_err(|_| Error::Unavailable)?;
        let exact = target.exact_key().map_err(|_| Error::Invalid)?;
        let mut threads = Vec::new();
        for (id, reference) in &state.threads {
            let CollabObjectV1::ThreadVersion(thread) =
                graph.get(*reference).map_err(|_| Error::Invalid)?
            else {
                return Err(Error::Invalid);
            };
            if thread.status != jj_collab::ThreadStatusV1::Tombstoned
                && thread
                    .agent_message
                    .as_ref()
                    .is_some_and(|t| t.exact_key().ok() == Some(exact))
            {
                threads.push(*id);
            }
        }
        let mut replies = Vec::new();
        for reference in state.messages.values() {
            let CollabObjectV1::HumanMessageRevision(message) =
                graph.get(*reference).map_err(|_| Error::Invalid)?
            else {
                return Err(Error::Invalid);
            };
            if threads.contains(&message.thread_id) && message.body.is_some() {
                replies.push(FeedbackReply {
                    thread_id: message.thread_id,
                    revision: *reference,
                    message: message.clone(),
                });
            }
        }
        replies.sort_by_key(|r| (r.message.accepted_at_millis, r.message.message_id));
        messages.push(FeedbackMessage {
            row_id: row_id.clone(),
            target,
            threads,
            replies,
        });
    }
    let feedback = Feedback {
        messages,
        write_unavailable: None,
    };
    if serde_json::to_vec(&feedback)
        .map_err(|_| Error::Invalid)?
        .len()
        > 4 * 1024 * 1024
    {
        return Err(Error::Limit);
    }
    Ok(feedback)
}

pub fn context(feedback: &Feedback) -> Result<String> {
    let mut lines = Vec::new();
    let mut count = 0;
    let mut bytes = 0;
    for message in &feedback.messages {
        for reply in &message.replies {
            let Some(body) = &reply.message.body else {
                continue;
            };
            // JSON framing preserves the boundary of authored feedback and new input.
            let line = serde_json::json!({"original_message":message.target,
                "reply_revision":reply.revision,"actor":reply.message.actor,"body":body})
            .to_string();
            count += 1;
            bytes += line.len() + 1;
            if count > 128 || bytes > 32768 {
                return Err(Error::Limit);
            }
            lines.push(line);
        }
    }
    Ok(if lines.is_empty() {
        String::new()
    } else {
        format!(
            "Committed human replies to earlier messages (follow-up context):\n{}\nEnd of committed replies.\n",
            lines.join("\n")
        )
    })
}

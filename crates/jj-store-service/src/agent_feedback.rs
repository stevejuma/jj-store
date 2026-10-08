//! Bounded exact message feedback shared by native and local selections.
use crate::agent_execution::FeedbackSubject;
use crate::agent_execution::{Error, Feedback, FeedbackMessage, FeedbackReply, Result};
use jj_collab::*;
use jj_store_wal::publication::*;
use jj_store_wal::publication::{AgentSnapshotRead, CollabTargetReaderV1};
mod source_parts;
#[cfg(test)]
mod tests;

pub(crate) async fn project(
    agents: &dyn AgentSnapshotRead,
    state: &ThreadStateV1,
    graph: &CollabGraphV1,
    rows: &[(
        String,
        AgentSessionId,
        crate::agent_execution::FeedbackSubject,
    )],
) -> Result<Feedback> {
    if rows.len() > 256 || state.messages.len() > 4096 || state.threads.len() > 4096 {
        return Err(Error::Limit);
    }
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
        for (tool_id, snapshot) in &session.tools {
            if !rows
                .iter()
                .any(|r| r.1 == session_id && r.2 == FeedbackSubject::Tool(*tool_id))
            {
                continue;
            }
            reads += 1;
            if reads > 4096 {
                return Err(Error::Limit);
            }
            let AgentObjectV1::ToolSnapshot(tool) = agents
                .read_object(*snapshot)
                .await
                .map_err(|_| Error::Unavailable)?
            else {
                return Err(Error::Invalid);
            };
            if tool.session_id != session_id || tool.tool_call_id != *tool_id {
                return Err(Error::Invalid);
            }
            let target = if let Some(content) = tool.output {
                ReactionTargetV1::ToolOutput {
                    session_id,
                    tool_id: *tool_id,
                    snapshot: *snapshot,
                    content,
                }
            } else {
                ReactionTargetV1::ToolInvocation {
                    session_id,
                    tool_id: *tool_id,
                    snapshot: *snapshot,
                }
            };
            targets.insert((session_id, FeedbackSubject::Tool(*tool_id)), target);
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
                        (sid, FeedbackSubject::Message(message_id)),
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
    let rows = rows
        .iter()
        .filter_map(|(id, session, subject)| {
            targets
                .get(&(*session, *subject))
                .map(|target| (id.clone(), target.clone()))
        })
        .collect::<Vec<_>>();
    project_targets(agents, state, graph, &rows).await
}

pub(crate) async fn recorded(
    agents: &dyn AgentSnapshotRead,
    state: &ThreadStateV1,
    graph: &CollabGraphV1,
    activity: &[crate::collaboration::AgentActivity],
) -> Result<Feedback> {
    use crate::collaboration::AgentActivity;
    let rows = activity
        .iter()
        .filter_map(|entry| match entry {
            AgentActivity::Message {
                message_id,
                reaction_target: Some(target),
                ..
            } => Some((message_id.to_string(), target.clone())),
            AgentActivity::Tool { tool, .. } => tool
                .reaction_target
                .clone()
                .map(|target| (tool.tool_call_id.to_string(), target)),
            _ => None,
        })
        .collect::<Vec<_>>();
    project_targets(agents, state, graph, &rows).await
}
async fn project_targets(
    agents: &dyn AgentSnapshotRead,
    state: &ThreadStateV1,
    graph: &CollabGraphV1,
    rows: &[(String, ReactionTargetV1)],
) -> Result<Feedback> {
    if rows.len() > 4096 || state.messages.len() > 4096 || state.threads.len() > 4096 {
        return Err(Error::Limit);
    }
    let reader = CollabTargetReaderV1::new(agents.repository(), None, Some(agents), None)
        .map_err(|_| Error::Unavailable)?;
    let mut answer_links = Vec::new();
    let mut submitted =
        std::collections::BTreeMap::<CollabObjectRef, Vec<jj_store_model::RequestId>>::new();
    for reference in state.links.values() {
        let CollabObjectV1::ThreadLink(link) = graph.get(*reference).map_err(|_| Error::Invalid)?
        else {
            return Err(Error::Invalid);
        };
        if link.active {
            if let ThreadLinkTargetV1::CommentBatch {
                comments,
                request_id,
                ..
            } = &link.target
            {
                for comment in comments {
                    if let ReactionTargetV1::HumanMessage { revision, .. } = comment {
                        submitted.entry(*revision).or_default().push(*request_id);
                    }
                }
            }
            if let ThreadLinkTargetV1::CommentReply { reply } = &link.target {
                answer_links.push(reply.clone());
            }
        }
    }
    let comment_revisions: std::collections::BTreeSet<_> = state
        .messages
        .values()
        .chain(submitted.keys())
        .copied()
        .collect();
    if comment_revisions.len() > 4096 {
        return Err(Error::Limit);
    }
    let mut messages = Vec::new();
    for (row_id, target) in rows {
        let target = target.clone();
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
        for reference in &comment_revisions {
            let CollabObjectV1::HumanMessageRevision(message) =
                graph.get(*reference).map_err(|_| Error::Invalid)?
            else {
                return Err(Error::Invalid);
            };
            let current = state.messages.get(&message.message_id);
            // Historical sent revisions remain readable only while their current
            // message is visible. A tombstone must not resurrect prior bodies.
            if !current.is_some_and(|r| matches!(graph.get(*r), Ok(CollabObjectV1::HumanMessageRevision(m)) if m.body.is_some())) { continue; }
            if threads.contains(&message.thread_id) && message.body.is_some() {
                replies.push(FeedbackReply {
                    historical: current != Some(reference),
                    batches: submitted.get(reference).cloned().unwrap_or_default(),
                    thread_id: message.thread_id,
                    revision: *reference,
                    message: message.clone(),
                    thread_revision: state.threads.get(&message.thread_id).copied(),
                    source: state.threads.get(&message.thread_id).and_then(|r|graph.get(*r).ok()).and_then(|o| if let CollabObjectV1::ThreadVersion(thread)=o {thread.comment_source.clone()} else {None}),
                    answers: answer_links.iter().filter(|reply| reply.comments.iter().any(|target| matches!(target, ReactionTargetV1::HumanMessage { revision, .. } if revision == reference))).cloned().collect(),
                });
            }
        }
        replies.sort_by_key(|r| (r.message.accepted_at_millis, r.message.message_id));
        messages.push(FeedbackMessage {
            row_id: row_id.clone(),
            text_parts: source_parts::parts(agents, &target).await?,
            answers: answer_links
                .iter()
                .filter(|reply| reply.answer.target.exact_key().ok() == Some(exact))
                .cloned()
                .collect(),
            target,
            threads,
            replies,
        });
    }
    let feedback = Feedback {
        messages,
        profile: None,
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
            if reply.historical {
                continue;
            }
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

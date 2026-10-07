//! Service-owned prompt preparation; not an HTTP contract or canonical store format.
use super::{Error, Result};
use jj_collab::*;
use jj_store_model::RepositoryId;
use jj_store_wal::publication::*;
use serde::{Deserialize, Serialize};
use std::collections::{BTreeMap, BTreeSet};

#[derive(Clone, Debug, PartialEq, Eq, PartialOrd, Ord, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct ReplyRevision {
    pub repository: RepositoryId,
    pub thread: ThreadId,
    pub message: HumanMessageId,
    pub revision: CollabObjectRef,
}
#[derive(Clone, Debug, Default, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct ContextBatch {
    pub revisions: BTreeSet<ReplyRevision>,
    pub text: String,
}

pub(crate) async fn collect(
    agents: &dyn AgentSnapshotRead,
    state: &ThreadStateV1,
    graph: &CollabGraphV1,
    rows: &[(
        String,
        AgentSessionId,
        crate::agent_execution::FeedbackSubject,
    )],
    sessions: &BTreeSet<AgentSessionId>,
    excluded: &BTreeSet<ReplyRevision>,
) -> Result<ContextBatch> {
    // Both routes share this exact AgentStore/ThreadStore selection.
    let feedback = crate::agent_feedback::project(agents, state, graph, rows).await?;
    let mut threads = BTreeMap::new();
    for message in feedback.messages {
        for thread in message.threads {
            threads.insert(thread, Some(message.target.clone()));
        }
    }
    for session in sessions {
        let Some(reference) = state.assignments.get(session) else {
            continue;
        };
        let CollabObjectV1::ThreadLink(link) = graph.get(*reference).map_err(|_| Error::Invalid)?
        else {
            return Err(Error::Invalid);
        };
        if !link.active || state.links.get(&link.id) != Some(reference) {
            continue;
        }
        let ThreadLinkTargetV1::Session {
            session_id,
            primary: true,
            ..
        } = link.target
        else {
            continue;
        };
        if session_id != *session {
            return Err(Error::Invalid);
        }
        let key =
            AgentCatalogKey::derive("session", session.as_bytes()).map_err(|_| Error::Invalid)?;
        if agents
            .lookup(AgentCatalogKind::Sessions, key)
            .await
            .map_err(|_| Error::Unavailable)?
            .is_none()
        {
            continue;
        }
        threads.entry(link.thread_id).or_insert(None);
    }
    let mut batch = ContextBatch::default();
    let mut lines = Vec::new();
    let mut bytes = 0;
    for reference in state.messages.values() {
        let CollabObjectV1::HumanMessageRevision(message) =
            graph.get(*reference).map_err(|_| Error::Invalid)?
        else {
            return Err(Error::Invalid);
        };
        let Some(original_message) = threads.get(&message.thread_id) else {
            continue;
        };
        let Some(body) = &message.body else { continue };
        let Some(thread_revision) = state.threads.get(&message.thread_id) else {
            continue;
        };
        let CollabObjectV1::ThreadVersion(thread) =
            graph.get(*thread_revision).map_err(|_| Error::Invalid)?
        else {
            return Err(Error::Invalid);
        };
        if thread.status == jj_collab::ThreadStatusV1::Tombstoned {
            continue;
        }
        let identity = ReplyRevision {
            repository: agents.repository(),
            thread: message.thread_id,
            message: message.message_id,
            revision: *reference,
        };
        if excluded.contains(&identity) || !batch.revisions.insert(identity.clone()) {
            continue;
        }
        let line = serde_json::json!({"reply": identity, "actor":message.actor, "body":body,
            "reply_to":message.reply_to, "original_message":original_message,
            "original_thread":{"id":thread.thread_id,"revision":thread_revision,"title":thread.title}}).to_string();
        bytes += line.len() + 1;
        if batch.revisions.len() > 128 || bytes > 32768 {
            return Err(Error::Limit);
        }
        lines.push(line);
    }
    if !lines.is_empty() {
        batch.text = format!(
            "Committed human discussion (follow-up context):\n{}\nEnd of committed discussion.\n",
            lines.join("\n")
        );
        if batch.text.len() > 32768 {
            return Err(Error::Limit);
        }
    }
    Ok(batch)
}

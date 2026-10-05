//! Current-message metadata for bounded native Thread directory rows.
use crate::collab_mutations::{Error, Result};
use crate::collab_reads::ThreadDirectoryMetadata;
use jj_collab::*;
use std::collections::{BTreeMap, BTreeSet};
pub(crate) fn project(
    state: &ThreadStateV1,
    graph: &CollabGraphV1,
) -> Result<BTreeMap<ThreadId, ThreadDirectoryMetadata>> {
    let mut rows = BTreeMap::new();
    let mut participants: BTreeMap<ThreadId, BTreeSet<jj_store_model::PrincipalId>> =
        BTreeMap::new();
    let mut latest_message = BTreeMap::new();
    for (id, reference) in &state.threads {
        let CollabObjectV1::ThreadVersion(t) = graph.get(*reference).map_err(|_| Error::Corrupt)?
        else {
            return Err(Error::Corrupt);
        };
        rows.insert(
            *id,
            ThreadDirectoryMetadata {
                message_count: 0,
                reply_count: 0,
                excerpt: String::new(),
                participants: vec![],
                last_activity_at_millis: t.accepted_at_millis,
            },
        );
    }
    for reference in state.messages.values() {
        let CollabObjectV1::HumanMessageRevision(m) =
            graph.get(*reference).map_err(|_| Error::Corrupt)?
        else {
            return Err(Error::Corrupt);
        };
        let Some(body) = &m.body else { continue };
        let row = rows.get_mut(&m.thread_id).ok_or(Error::Corrupt)?;
        row.message_count += 1;
        if m.reply_to.is_some() {
            row.reply_count += 1;
        }
        participants.entry(m.thread_id).or_default().insert(m.actor);
        row.last_activity_at_millis = row.last_activity_at_millis.max(m.accepted_at_millis);
        if latest_message
            .get(&m.thread_id)
            .is_none_or(|at| m.accepted_at_millis >= *at)
        {
            latest_message.insert(m.thread_id, m.accepted_at_millis);
            row.excerpt = body.chars().take(240).collect();
        }
    }
    for (id, actors) in participants {
        rows.get_mut(&id).ok_or(Error::Corrupt)?.participants = actors.into_iter().collect();
    }
    Ok(rows)
}

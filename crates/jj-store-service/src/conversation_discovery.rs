//! Rebuildable directory enrichment from selected ThreadStore metadata.
use jj_collab::*;
use jj_store_wal::publication::AgentSessionId;
use std::collections::BTreeMap;
pub(crate) fn project(
    state: &ThreadStateV1,
    graph: &CollabGraphV1,
) -> crate::collab_mutations::Result<BTreeMap<AgentSessionId, (Option<String>, String)>> {
    let mut by_owner = BTreeMap::<ThreadId, Vec<AgentSessionId>>::new();
    for (id, reference) in &state.assignments {
        let CollabObjectV1::ThreadLink(link) = graph
            .get(*reference)
            .map_err(|_| crate::collab_mutations::Error::Corrupt)?
        else {
            return Err(crate::collab_mutations::Error::Corrupt);
        };
        if link.active {
            by_owner.entry(link.thread_id).or_default().push(*id);
        }
    }
    let mut rows = BTreeMap::new();
    for reference in state.metadata.values() {
        let CollabObjectV1::ConversationMetadata(metadata) = graph
            .get(*reference)
            .map_err(|_| crate::collab_mutations::Error::Corrupt)?
        else {
            return Err(crate::collab_mutations::Error::Corrupt);
        };
        let owner = state
            .threads
            .get(&metadata.thread_id)
            .ok_or(crate::collab_mutations::Error::Corrupt)?;
        let CollabObjectV1::ThreadVersion(owner) = graph
            .get(*owner)
            .map_err(|_| crate::collab_mutations::Error::Corrupt)?
        else {
            return Err(crate::collab_mutations::Error::Corrupt);
        };
        if owner.status == ThreadStatusV1::Tombstoned {
            continue;
        }
        let mut ids = by_owner
            .get(&metadata.thread_id)
            .cloned()
            .unwrap_or_default();
        if let ConversationMetadataTargetV1::Recorded { session } = metadata.target {
            ids.push(session);
        }
        ids.extend(metadata.value.source_refs().iter().map(|s| s.session));
        let title = metadata.value.title().map(str::to_owned);
        let text = metadata.value.search_text();
        for id in ids {
            rows.insert(id, (title.clone(), text.clone()));
        }
    }
    Ok(rows)
}

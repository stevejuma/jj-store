//! Append-only acknowledged relationships; a sequence pins a query's snapshot.
use super::*;
use std::collections::BTreeMap;

pub(crate) struct Entry {
    pub parents: BTreeSet<m::CopyId>,
    pub sequence: u64,
}
#[derive(Default)]
pub(crate) struct StagedCopies {
    pub entries: BTreeMap<m::CopyId, Entry>,
    pub children: BTreeMap<m::CopyId, BTreeMap<u64, m::CopyId>>,
    edges: usize,
    pub sequence: u64,
}
impl StagedCopies {
    pub fn admit(
        &self,
        id: m::CopyId,
        parents: &BTreeSet<m::CopyId>,
        limits: BackendLimits,
    ) -> BackendResult<()> {
        if let Some(entry) = self.entries.get(&id) {
            return if &entry.parents == parents {
                Ok(())
            } else {
                Err(other("staged copy identity changed"))
            };
        }
        let nodes = self
            .entries
            .len()
            .checked_add(1)
            .ok_or_else(|| other("staged copy limit"))?;
        let edges = self
            .edges
            .checked_add(parents.len())
            .ok_or_else(|| other("staged copy limit"))?;
        let bytes = nodes
            .checked_mul(1024)
            .and_then(|n| {
                edges
                    .checked_mul(512)
                    .and_then(|edges| n.checked_add(edges))
            })
            .and_then(|n| n.checked_add(256))
            .ok_or_else(|| other("staged copy limit"))?;
        if nodes > limits.max_nodes || edges > limits.max_work || bytes > limits.max_bytes {
            return Err(other("staged copy resource limit"));
        }
        Ok(())
    }
    pub fn insert(
        &mut self,
        id: m::CopyId,
        parents: BTreeSet<m::CopyId>,
        limits: BackendLimits,
    ) -> BackendResult<()> {
        self.admit(id, &parents, limits)?;
        if self.entries.contains_key(&id) {
            return Ok(());
        }
        let sequence = self
            .sequence
            .checked_add(1)
            .ok_or_else(|| other("staged copy sequence"))?;
        for parent in &parents {
            self.children
                .entry(*parent)
                .or_default()
                .insert(sequence, id);
        }
        self.edges += parents.len();
        self.entries.insert(id, Entry { parents, sequence });
        self.sequence = sequence;
        Ok(())
    }
}

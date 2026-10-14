//! Compact read-only access to the owned, checksum-verified JJ format-6 segment.
//! Layout follows jj-lib 0.45.1 default_index/readonly.rs (Apache-2.0).
use super::*;
use crate::query::memory::Lease;
use std::collections::{BinaryHeap, btree_map::Entry};

pub(super) struct Graph {
    bytes: Vec<u8>,
    commits: u32,
    changes: u32,
    parents: u32,
    _memory: Lease,
}
impl Graph {
    pub(super) fn load(
        bytes: Vec<u8>,
        memory: Lease,
        maximum: usize,
        mut work: impl FnMut(usize) -> Result<()>,
    ) -> Result<Self> {
        let word = |offset: usize| -> Result<u32> {
            Ok(u32::from_le_bytes(
                bytes
                    .get(offset..offset + 4)
                    .ok_or(Error::Corrupt)?
                    .try_into()
                    .unwrap(),
            ))
        };
        if word(0)? != 6 || word(4)? != 0 {
            return Err(Error::Corrupt);
        }
        let commits = word(8)?;
        let changes = word(12)?;
        let parents = word(16)?;
        let change_overflow = word(20)?;
        let expected = 24
            + u64::from(commits) * 40
            + u64::from(changes) * 20
            + (u64::from(parents) + u64::from(change_overflow)) * 4;
        if commits == 0
            || commits as usize > maximum
            || changes > commits
            || expected != bytes.len() as u64
        {
            return Err(Error::Corrupt);
        }
        let graph = Self {
            bytes,
            commits,
            changes,
            parents,
            _memory: memory,
        };
        let mut prior = None;
        for pos in 0..commits {
            work(1)?;
            let row = graph.row(pos)?;
            let change = graph.word(row + 12);
            if change >= changes {
                return Err(Error::Corrupt);
            }
            let mut generation = 0;
            graph.for_each_parent(pos, |parent| {
                work(1)?;
                if parent >= pos {
                    return Err(Error::Corrupt);
                }
                generation = generation.max(
                    graph
                        .word(graph.row(parent)?)
                        .checked_add(1)
                        .ok_or(Error::Corrupt)?,
                );
                Ok(())
            })?;
            if graph.word(row) != generation {
                return Err(Error::Corrupt);
            }
            let lookup = graph.lookup_position(pos)?;
            let id = graph.id(lookup)?;
            if prior.is_some_and(|prior| prior >= id) {
                return Err(Error::Corrupt);
            }
            prior = Some(id);
        }
        Ok(graph)
    }
    fn word(&self, offset: usize) -> u32 {
        u32::from_le_bytes(self.bytes[offset..offset + 4].try_into().unwrap())
    }
    fn row(&self, pos: u32) -> Result<usize> {
        if pos >= self.commits {
            return Err(Error::Corrupt);
        }
        Ok(24 + pos as usize * 36)
    }
    fn lookup_position(&self, sorted: u32) -> Result<u32> {
        if sorted >= self.commits {
            return Err(Error::Corrupt);
        }
        let pos = self.word(24 + self.commits as usize * 36 + sorted as usize * 4);
        self.row(pos)?;
        Ok(pos)
    }
    fn id(&self, pos: u32) -> Result<m::GitSha1Id> {
        let row = self.row(pos)?;
        m::GitSha1Id::from_slice(&self.bytes[row + 16..row + 36]).map_err(|_| Error::Corrupt)
    }
    fn position(&self, id: m::GitSha1Id, budget: &Budget) -> Result<u32> {
        budget.work(self.commits.max(1).ilog2() as usize + 2)?;
        let mut low = 0;
        let mut high = self.commits;
        while low < high {
            let mid = low + (high - low) / 2;
            let pos = self.lookup_position(mid)?;
            match self.id(pos)?.cmp(&id) {
                std::cmp::Ordering::Less => low = mid + 1,
                std::cmp::Ordering::Greater => high = mid,
                std::cmp::Ordering::Equal => return Ok(pos),
            }
        }
        Err(Error::NotCommitted)
    }
    pub(super) fn change(&self, id: m::GitSha1Id, budget: &Budget) -> Result<m::ChangeId> {
        let pos = self.position(id, budget)?;
        let change = self.word(self.row(pos)? + 12);
        if change >= self.changes {
            return Err(Error::Corrupt);
        }
        let offset = 24 + self.commits as usize * 40 + change as usize * 16;
        m::ChangeId::from_slice(&self.bytes[offset..offset + 16]).map_err(|_| Error::Corrupt)
    }
    fn for_each_parent(&self, pos: u32, mut visit: impl FnMut(u32) -> Result<()>) -> Result<()> {
        let row = self.row(pos)?;
        let first = self.word(row + 4);
        let second = self.word(row + 8);
        if first == u32::MAX && second == u32::MAX {
            // Overflow offset zero also encodes its first word as MAX; the
            // second word distinguishes that from an empty parent list.
        } else if first & 0x8000_0000 != 0 {
            let start = !first;
            let count = !second;
            if !(3..=256).contains(&count)
                || start.checked_add(count).is_none_or(|n| n > self.parents)
            {
                return Err(Error::Corrupt);
            }
            let offset = 24 + self.commits as usize * 40 + self.changes as usize * 20;
            for parent in start..start + count {
                visit(self.word(offset + parent as usize * 4))?;
            }
        } else {
            visit(first)?;
            if second != u32::MAX {
                visit(second)?;
            }
        }
        Ok(())
    }
    pub(super) fn difference(
        &self,
        before: &BTreeSet<m::GitSha1Id>,
        after: &BTreeSet<m::GitSha1Id>,
        budget: &Budget,
        maximum: usize,
    ) -> Result<operations::SetDelta<m::GitSha1Id>> {
        if before == after {
            for id in before {
                self.position(*id, budget)?;
            }
            return Ok(operations::SetDelta {
                added: vec![],
                removed: vec![],
            });
        }
        let mut walk = Frontier::new(maximum);
        for (heads, side) in [(before, 1), (after, 2)] {
            for id in heads {
                walk.push(self.position(*id, budget)?, side, budget)?;
            }
        }
        let mut added = vec![];
        let mut removed = vec![];
        // Parent positions precede children. Once no one-sided node remains,
        // every remaining ancestor is shared and need not be walked at all.
        while walk.differing > 0 {
            budget.work(1)?;
            let pos = walk.queue.pop().ok_or(Error::Corrupt)?;
            let side = walk.sides.remove(&pos).ok_or(Error::Corrupt)?;
            if side != 3 {
                walk.differing -= 1;
            }
            if side == 1 {
                removed.push(self.id(pos)?);
            }
            if side == 2 {
                added.push(self.id(pos)?);
            }
            self.for_each_parent(pos, |parent| {
                if parent >= pos {
                    return Err(Error::Corrupt);
                }
                walk.push(parent, side, budget)
            })?;
        }
        budget.work((added.len() + removed.len()).saturating_mul(32))?;
        added.sort_unstable();
        removed.sort_unstable();
        Ok(operations::SetDelta { added, removed })
    }
}

#[cfg(test)]
#[path = "comparison_tests.rs"]
mod tests;
struct Frontier {
    queue: BinaryHeap<u32>,
    sides: BTreeMap<u32, u8>,
    differing: usize,
    admitted: usize,
    capacity: usize,
    maximum: usize,
    memory: Vec<Lease>,
}
impl Frontier {
    fn new(maximum: usize) -> Self {
        Self {
            queue: BinaryHeap::new(),
            sides: BTreeMap::new(),
            differing: 0,
            admitted: 0,
            capacity: 0,
            maximum,
            memory: vec![],
        }
    }
    fn push(&mut self, pos: u32, side: u8, budget: &Budget) -> Result<()> {
        budget.work(self.sides.len().max(1).ilog2() as usize + 3)?;
        if !self.sides.contains_key(&pos) {
            if self.admitted >= self.maximum {
                return Err(Error::Limit);
            }
            if self.admitted == self.capacity {
                let count = (self.maximum - self.capacity).min(256);
                // Queue/map nodes, retained result IDs and allocation rounding.
                self.memory.push(Lease::reserve(count as u64 * 256 + 1024)?);
                self.capacity += count;
            }
            self.admitted += 1;
        }
        match self.sides.entry(pos) {
            Entry::Vacant(entry) => {
                entry.insert(side);
                self.queue.push(pos);
                if side != 3 {
                    self.differing += 1;
                }
            }
            Entry::Occupied(mut entry) => {
                if *entry.get() != 3 && *entry.get() | side == 3 {
                    self.differing -= 1;
                }
                *entry.get_mut() |= side;
            }
        }
        Ok(())
    }
}

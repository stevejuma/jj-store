//! Flat retained native identities; lexical seeks allocate only selected keys.
use super::*;
use crate::query::content::Direction;
use std::cmp::Ordering;

pub(super) fn id(id: &b::CommitId) -> Result<m::CommitId> {
    m::CommitId::from_slice(id.as_bytes()).map_err(|_| Error::Corrupt)
}

pub(super) fn commit_weight(commits: u32) -> u64 {
    // Flat targets (80), membership IDs (64), group offsets (4), plus 44
    // bytes/row for upstream prefix bitsets and bounded traversal scratch.
    // Four additional bytes retain lexical ranks over exact visible JJ order.
    // Heads/reverse labels and fixed overhead are reserved independently.
    u64::from(commits) * 196
}

#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub(in crate::query) struct VisibleTarget {
    change: m::ChangeId,
    pub commit: m::CommitId,
}
pub(in crate::query) struct References {
    rows: Vec<VisibleTarget>,
    starts: Vec<u32>,
}
#[derive(Clone, Copy)]
pub(in crate::query) struct Group<'a> {
    pub change: m::ChangeId,
    pub targets: &'a [VisibleTarget],
}
impl Group<'_> {
    pub fn key(&self) -> String {
        format!("change:{}", self.change)
    }
}
impl References {
    pub fn with_capacity(commits: usize) -> Self {
        Self {
            rows: Vec::with_capacity(commits),
            starts: Vec::with_capacity(commits),
        }
    }
    pub fn push(&mut self, change: m::ChangeId, commit: m::CommitId) -> Result<()> {
        if self.rows.len() == self.rows.capacity() {
            return Err(Error::Corrupt);
        }
        self.rows.push(VisibleTarget { change, commit });
        Ok(())
    }
    pub fn finish(&mut self, budget: &Budget) -> Result<()> {
        sort(&mut self.rows, budget)?;
        for (offset, row) in self.rows.iter().enumerate() {
            budget.work(1)?;
            if offset == 0 || row.change != self.rows[offset - 1].change {
                self.starts
                    .push(u32::try_from(offset).map_err(|_| Error::Limit)?);
            }
        }
        Ok(())
    }
    pub fn len(&self) -> usize {
        self.starts.len()
    }
    fn group(&self, offset: usize) -> Group<'_> {
        let start = self.starts[offset] as usize;
        let end = self
            .starts
            .get(offset + 1)
            .map_or(self.rows.len(), |n| *n as usize);
        Group {
            change: self.rows[start].change,
            targets: &self.rows[start..end],
        }
    }
    pub fn first(&self) -> Option<Group<'_>> {
        (!self.starts.is_empty()).then(|| self.group(0))
    }
    pub fn last(&self) -> Option<Group<'_>> {
        self.starts.len().checked_sub(1).map(|n| self.group(n))
    }
    pub fn get(&self, key: &str) -> Option<Group<'_>> {
        let offset = self
            .starts
            .partition_point(|n| compare_key(self.rows[*n as usize].change, key) == Ordering::Less);
        let group = (offset < self.len()).then(|| self.group(offset))?;
        (compare_key(group.change, key) == Ordering::Equal).then_some(group)
    }
    pub fn window(
        &self,
        anchor: Option<&str>,
        direction: Direction,
    ) -> impl DoubleEndedIterator<Item = Group<'_>> {
        let (start, end) = match direction {
            Direction::Forward => (
                anchor.map_or(0, |key| {
                    self.starts.partition_point(|n| {
                        compare_key(self.rows[*n as usize].change, key) != Ordering::Greater
                    })
                }),
                self.len(),
            ),
            Direction::Backward => (
                0,
                anchor.map_or(self.len(), |key| {
                    self.starts.partition_point(|n| {
                        compare_key(self.rows[*n as usize].change, key) == Ordering::Less
                    })
                }),
            ),
        };
        (start..end).map(|offset| self.group(offset))
    }
}
fn compare_key(change: m::ChangeId, key: &str) -> Ordering {
    // Arbitrary anchors retain byte-for-byte String ordering, including prefixes,
    // non-hex text and bookmark/change boundaries. No heap key per comparison.
    let mut bytes = [0; 39];
    bytes[..7].copy_from_slice(b"change:");
    const HEX: &[u8; 16] = b"0123456789abcdef";
    for (n, byte) in change.as_bytes().iter().enumerate() {
        bytes[7 + n * 2] = HEX[(byte >> 4) as usize];
        bytes[8 + n * 2] = HEX[(byte & 15) as usize];
    }
    bytes.as_slice().cmp(key.as_bytes())
}

/// In-place heapsort has no repository-sized merge buffer. Admission and
/// cancellation occur at most every 1024 comparisons, including construction.
pub(super) fn sort<T: Ord>(values: &mut [T], budget: &Budget) -> Result<()> {
    struct Charge<'a> {
        budget: &'a Budget,
        pending: usize,
    }
    impl Charge<'_> {
        fn comparison(&mut self) -> Result<()> {
            self.pending += 1;
            if self.pending == 1024 {
                self.budget.work(self.pending)?;
                self.pending = 0;
            }
            Ok(())
        }
        fn sift<T: Ord>(&mut self, values: &mut [T], mut root: usize) -> Result<()> {
            while root < values.len() / 2 {
                let mut child = root * 2 + 1;
                if child + 1 < values.len() {
                    self.comparison()?;
                    if values[child] < values[child + 1] {
                        child += 1;
                    }
                }
                self.comparison()?;
                if values[root] >= values[child] {
                    break;
                }
                values.swap(root, child);
                root = child;
            }
            Ok(())
        }
    }
    budget.check()?;
    let mut charge = Charge { budget, pending: 0 };
    for start in (0..values.len() / 2).rev() {
        charge.sift(values, start)?;
    }
    for end in (1..values.len()).rev() {
        values.swap(0, end);
        charge.sift(&mut values[..end], 0)?;
    }
    budget.work(charge.pending)
}

#[cfg(test)]
mod tests;

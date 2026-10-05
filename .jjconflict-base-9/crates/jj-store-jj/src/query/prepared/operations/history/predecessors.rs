//! Exact compact predecessor metadata occurrences, never operation bodies.
use super::*;

pub(super) fn write(
    disk: &Tables,
    source: &impl Source,
    output: &mut impl FnMut(&[u8]) -> Result<()>,
) -> Result<()> {
    let mut row = disk.first(OP_GRAPH)?;
    while let Some((id, _)) = row {
        let known = required(disk, EVO_KNOWN, &id)?;
        if known.len() != 5 || known[0] > 1 {
            return Err(Error::Corrupt);
        }
        output(&known)?;
        row = disk.after(OP_GRAPH, &id)?;
    }
    let commit_len = source.commit_len();
    if commit_len != 20 && commit_len != 64 {
        return Err(Error::Corrupt);
    }
    let mut count = 0u64;
    let mut row = disk.first(EVO_LOOKUP)?;
    while let Some((id, value)) = row {
        source.work(1)?;
        if id.len() != commit_len + 4 || !value.is_empty() {
            return Err(Error::Corrupt);
        }
        count = count.checked_add(1).ok_or(Error::Limit)?;
        row = disk.after(EVO_LOOKUP, &id)?;
    }
    output(&(commit_len as u32).to_be_bytes())?;
    output(&count.to_be_bytes())?;
    let mut row = disk.first(EVO_LOOKUP)?;
    while let Some((id, _)) = row {
        output(&id)?;
        row = disk.after(EVO_LOOKUP, &id)?;
    }
    Ok(())
}

pub(super) struct Lookup {
    known: Vec<bool>,
    // Fixed-width commit identity plus four-byte operation rank. No per-ID heap.
    entries: Vec<u8>,
    commit_len: usize,
    legacy: Option<usize>,
}
impl Lookup {
    pub(super) fn known(&self, position: usize) -> bool {
        self.known[position]
    }
    pub(super) fn load(
        file: &mut impl std::io::Read,
        digest: &mut Sha256,
        remaining: &mut u64,
        nodes: &[Node],
        work: &mut impl FnMut(usize) -> Result<()>,
    ) -> Result<Self> {
        // The containing graph admits the complete file before any decoding.
        *remaining = remaining
            .checked_sub(nodes.len() as u64 * 5 + 12)
            .ok_or(Error::Corrupt)?;
        if nodes.len() > u32::MAX as usize {
            return Err(Error::Limit);
        }
        let mut known = Vec::with_capacity(nodes.len());
        let mut positions = vec![usize::MAX; nodes.len()];
        for at in 0..nodes.len() {
            work(1)?;
            let mut value = [0; 5];
            file.read_exact(&mut value).map_err(|_| Error::Corrupt)?;
            digest.update(value);
            let position = number(&value[1..])? as usize;
            if value[0] > 1 || position >= nodes.len() || positions[position] != usize::MAX {
                return Err(Error::Corrupt);
            }
            known.push(value[0] == 1);
            positions[position] = at;
        }
        let mut header = [0; 12];
        file.read_exact(&mut header).map_err(|_| Error::Corrupt)?;
        digest.update(header);
        let commit_len = number(&header[..4])? as usize;
        let count = u64::from_be_bytes(header[4..].try_into().unwrap());
        if (commit_len != 20 && commit_len != 64)
            || count.checked_mul(commit_len as u64 + 4) != Some(*remaining)
        {
            return Err(Error::Corrupt);
        }
        let size = usize::try_from(*remaining).map_err(|_| Error::Limit)?;
        let mut entries: Vec<u8> = Vec::with_capacity(size);
        let mut previous = [0; 68];
        let width = commit_len + 4;
        for n in 0..count {
            work(1)?;
            let mut bytes = [0; 68];
            let bytes = &mut bytes[..width];
            file.read_exact(bytes).map_err(|_| Error::Corrupt)?;
            digest.update(&*bytes);
            if n > 0 && &previous[..width] >= &*bytes {
                return Err(Error::Corrupt);
            }
            previous[..width].copy_from_slice(bytes);
            let position = number(&bytes[commit_len..])? as usize;
            let node = *positions.get(position).ok_or(Error::Corrupt)?;
            if !known[node] {
                return Err(Error::Corrupt);
            }
            bytes[commit_len..].copy_from_slice(&(node as u32).to_be_bytes());
            entries.extend_from_slice(bytes);
        }
        *remaining = 0;
        Ok(Self {
            known,
            entries,
            commit_len,
            legacy: None,
        })
    }
    pub(super) fn bind(
        &mut self,
        nodes: &[Node],
        ordered: &[usize],
        work: &mut impl FnMut(usize) -> Result<()>,
    ) -> Result<()> {
        for (rank, &position) in ordered.iter().enumerate() {
            work(1)?;
            if !self.known[position] {
                self.legacy = Some(rank);
                break;
            }
        }
        let width = self.commit_len + 4;
        for entry in self.entries.chunks_exact_mut(width) {
            work(1)?;
            let position = number(&entry[self.commit_len..])? as usize;
            let rank = u32::try_from(nodes[position].rank).map_err(|_| Error::Limit)?;
            entry[self.commit_len..].copy_from_slice(&rank.to_be_bytes());
        }
        let mut at = 0;
        while at < self.entries.len() {
            work(1)?;
            let mut end = at + width;
            while end < self.entries.len()
                && self.entries[end..end + self.commit_len]
                    == self.entries[at..at + self.commit_len]
            {
                work(1)?;
                end += width;
            }
            // Commit groups arrive lexically sorted. Swap only their ranks;
            // admitted in-place heapsort allocates no additional array.
            let rows = &mut self.entries[at..end];
            let count = rows.len() / width;
            for root in (0..count / 2).rev() {
                sift(rows, width, root, count, work)?;
            }
            for end in (1..count).rev() {
                swap(rows, width, 0, end);
                sift(rows, width, 0, end, work)?;
            }
            at = end;
        }
        Ok(())
    }
    pub(super) fn next(
        &self,
        pending: &[backend::CommitId],
        start: usize,
        work: &mut impl FnMut(usize) -> Result<()>,
    ) -> Result<Option<usize>> {
        let mut next = self.legacy.filter(|rank| *rank >= start);
        let width = self.commit_len + 4;
        let count = self.entries.len() / width;
        for commit in pending {
            work(count.max(1).ilog2() as usize + 2)?;
            if commit.as_bytes().len() != self.commit_len {
                return Err(Error::Corrupt);
            }
            let mut left = 0;
            let mut right = count;
            while left < right {
                let mid = left + (right - left) / 2;
                let entry = &self.entries[mid * width..(mid + 1) * width];
                if &entry[..self.commit_len] < commit.as_bytes()
                    || (&entry[..self.commit_len] == commit.as_bytes()
                        && rank(&self.entries, width, mid) < start)
                {
                    left = mid + 1;
                } else {
                    right = mid;
                }
            }
            if left < count
                && &self.entries[left * width..left * width + self.commit_len] == commit.as_bytes()
            {
                let candidate = rank(&self.entries, width, left);
                next = Some(next.map_or(candidate, |rank| rank.min(candidate)));
            }
        }
        Ok(next)
    }
}
fn rank(rows: &[u8], width: usize, at: usize) -> usize {
    u32::from_be_bytes(
        rows[(at + 1) * width - 4..(at + 1) * width]
            .try_into()
            .unwrap(),
    ) as usize
}
fn swap(rows: &mut [u8], width: usize, left: usize, right: usize) {
    for n in 0..4 {
        rows.swap((left + 1) * width - 4 + n, (right + 1) * width - 4 + n);
    }
}
fn sift(
    rows: &mut [u8],
    width: usize,
    mut root: usize,
    end: usize,
    work: &mut impl FnMut(usize) -> Result<()>,
) -> Result<()> {
    while root < end / 2 {
        let mut child = root * 2 + 1;
        work(2)?;
        if child + 1 < end && rank(rows, width, child) < rank(rows, width, child + 1) {
            child += 1;
        }
        if rank(rows, width, root) >= rank(rows, width, child) {
            break;
        }
        swap(rows, width, root, child);
        root = child;
    }
    Ok(())
}

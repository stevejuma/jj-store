//! Streaming writer for pinned JJ commit-index format 6; no native persistence.
use super::*;
use blake2::{Blake2b512, Digest as _};
use std::io::{BufWriter, Write as _};

struct Output<'a, S: Source> {
    file: BufWriter<std::fs::File>,
    digest: Blake2b512,
    space: Arc<Space>,
    budget: &'a S,
}
impl<S: Source> Output<'_, S> {
    fn bytes(&mut self, bytes: &[u8]) -> Result<()> {
        self.budget.work(1)?;
        // Reserve buffered bytes too; no write can overshoot the stage allowance.
        self.space.reserve(bytes.len() as u64).map_err(io_error)?;
        self.file.write_all(bytes).map_err(io_error)?;
        self.digest.update(bytes);
        Ok(())
    }
    fn u32(&mut self, value: u32) -> Result<()> {
        self.bytes(&value.to_le_bytes())
    }
}

#[allow(clippy::too_many_arguments)]
pub(super) fn write(
    path: &Path,
    operation: &jj_lib::op_store::OperationId,
    disk: &mut Tables,
    total: u32,
    commit_len: usize,
    change_len: usize,
    budget: &impl Source,
    space: Arc<Space>,
) -> Result<()> {
    let mut position = 0u32;
    let mut parent_overflow = 0u32;
    while let Some((order, id)) = disk.last(ORDER)? {
        budget.work(1)?;
        disk.remove(ORDER, &order)?;
        let node = Node::decode(&required(disk, NODES, &id)?, commit_len, change_len)?;
        let mut generation = 0u32;
        let mut parents = Vec::with_capacity(node.parents.len());
        for parent in node.parents {
            budget.work(1)?;
            let info = required(disk, POSITIONS, parent.as_bytes())?;
            if info.len() != 8 {
                return Err(Error::Corrupt);
            }
            parents.push(number(&info[..4])?);
            generation = generation.max(number(&info[4..])?.checked_add(1).ok_or(Error::Limit)?);
        }
        let (first, second) = match parents.as_slice() {
            [] => (u32::MAX, u32::MAX),
            [a] => (*a, u32::MAX),
            [a, b] => (*a, *b),
            parents => {
                let start = parent_overflow;
                for &parent in parents {
                    disk.put(
                        PARENT_OVERFLOW,
                        &parent_overflow.to_be_bytes(),
                        &parent.to_be_bytes(),
                    )?;
                    parent_overflow = parent_overflow
                        .checked_add(1)
                        .filter(|n| *n < 0x8000_0000)
                        .ok_or(Error::Limit)?;
                }
                (!start, !(parents.len() as u32))
            }
        };
        let mut graph = Vec::with_capacity(12 + change_len + commit_len);
        for value in [generation, first, second] {
            graph.extend(value.to_le_bytes());
        }
        graph.extend_from_slice(&node.change);
        graph.extend_from_slice(&id);
        disk.put(GRAPH, &position.to_be_bytes(), &graph)?;
        let mut lookup = position.to_be_bytes().to_vec();
        lookup.extend(generation.to_be_bytes());
        disk.put(POSITIONS, &id, &lookup)?;
        let mut change = node.change;
        change.extend(position.to_be_bytes());
        disk.put(CHANGES, &change, &[])?;
        position += 1;
    }
    if position != total {
        return Err(Error::Corrupt);
    }
    let mut changes = 0u32;
    let mut change_overflow = 0u32;
    let mut row = disk.first(CHANGES)?;
    while let Some((key, _)) = row {
        budget.work(1)?;
        if key.len() != change_len + 4 {
            return Err(Error::Corrupt);
        }
        let change = &key[..change_len];
        row = disk.after(CHANGES, &key)?;
        let pointer = if row
            .as_ref()
            .is_some_and(|(next, _)| next.starts_with(change))
        {
            let start = change_overflow;
            disk.put(
                CHANGE_OVERFLOW,
                &change_overflow.to_be_bytes(),
                &key[change_len..],
            )?;
            change_overflow += 1;
            while let Some((next, _)) = row.as_ref().filter(|(next, _)| next.starts_with(change)) {
                budget.work(1)?;
                disk.put(
                    CHANGE_OVERFLOW,
                    &change_overflow.to_be_bytes(),
                    &next[change_len..],
                )?;
                change_overflow = change_overflow
                    .checked_add(1)
                    .filter(|n| *n < 0x8000_0000)
                    .ok_or(Error::Limit)?;
                row = disk.after(CHANGES, next)?;
            }
            !start
        } else {
            number(&key[change_len..])?
        };
        let mut info = changes.to_be_bytes().to_vec();
        info.extend(pointer.to_be_bytes());
        disk.put(CHANGE_LOOKUP, change, &info)?;
        changes += 1;
    }

    let segments = path.join("segments");
    let links = path.join("op_links");
    std::fs::create_dir_all(&segments).map_err(io_error)?;
    std::fs::create_dir_all(&links).map_err(io_error)?;
    let temporary = tempfile::NamedTempFile::new_in(&segments).map_err(io_error)?;
    let mut out = Output {
        file: BufWriter::with_capacity(64 * 1024, temporary.reopen().map_err(io_error)?),
        digest: Blake2b512::new(),
        space: space.clone(),
        budget,
    };
    for value in [6, 0, total, changes, parent_overflow, change_overflow] {
        out.u32(value)?;
    }
    let mut row = disk.first(GRAPH)?;
    while let Some((key, graph)) = row {
        out.bytes(&graph[..12])?;
        let change = &graph[12..12 + change_len];
        let lookup = required(disk, CHANGE_LOOKUP, change)?;
        out.u32(number(&lookup[..4])?)?;
        out.bytes(&graph[12 + change_len..])?;
        row = disk.after(GRAPH, &key)?;
    }
    let mut row = disk.first(POSITIONS)?;
    while let Some((key, info)) = row {
        out.u32(number(&info[..4])?)?;
        row = disk.after(POSITIONS, &key)?;
    }
    let mut row = disk.first(CHANGE_LOOKUP)?;
    while let Some((key, _)) = row {
        out.bytes(&key)?;
        row = disk.after(CHANGE_LOOKUP, &key)?;
    }
    let mut row = disk.first(CHANGE_LOOKUP)?;
    while let Some((key, info)) = row {
        out.u32(number(&info[4..])?)?;
        row = disk.after(CHANGE_LOOKUP, &key)?;
    }
    for table in [PARENT_OVERFLOW, CHANGE_OVERFLOW] {
        let mut row = disk.first(table)?;
        while let Some((key, info)) = row {
            out.u32(number(&info)?)?;
            row = disk.after(table, &key)?;
        }
    }
    out.file.flush().map_err(io_error)?;
    out.file.get_ref().sync_all().map_err(io_error)?;
    let digest = out.digest.finalize();
    temporary
        .persist(segments.join(format!("{digest:x}")))
        .map_err(|e| io_error(e.error))?;
    budget.check()?;
    // SegmentControl field 1, bytes64; changed-path fields absent, as upstream.
    space.reserve(66).map_err(io_error)?;
    let mut link = tempfile::NamedTempFile::new_in(&links).map_err(io_error)?;
    link.write_all(&[0x0a, 64]).map_err(io_error)?;
    link.write_all(&digest).map_err(io_error)?;
    link.as_file().sync_all().map_err(io_error)?;
    budget.check()?;
    link.persist(links.join(operation.hex()))
        .map_err(|e| io_error(e.error))?;
    Ok(())
}

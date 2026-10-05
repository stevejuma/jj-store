//! Compact exact operation topology. No bodies, stores or source capability.
mod window;
use super::*;
use futures::{Stream, TryStreamExt as _};
pub(crate) use window::read;
const MAGIC: &[u8; 8] = b"JSTOPG1\0";
const FRAMING: u64 = 80 + 32;
const RECORD: u64 = 80;

pub(in crate::query::prepared) fn capture(
    disk: &mut Tables,
    op: &Operation,
    source: &impl Source,
) -> Result<()> {
    source.work(op.parent_ids().len() + 1)?;
    if op.id().as_bytes().len() != 64 || op.parent_ids().len() > source.max_parents() {
        return Err(Error::Limit);
    }
    let size = 16 + op.parent_ids().len() * 64;
    let _memory = Lease::reserve(size as u64 * 2 + 256)?;
    let mut bytes = Vec::with_capacity(size);
    let end = op.metadata().time.end;
    bytes.extend_from_slice(&end.timestamp.0.to_be_bytes());
    bytes.extend_from_slice(&end.tz_offset.to_be_bytes());
    bytes.extend_from_slice(&(op.parent_ids().len() as u32).to_be_bytes());
    for parent in op.parent_ids() {
        if parent.as_bytes().len() != 64 {
            return Err(Error::Corrupt);
        }
        bytes.extend_from_slice(parent.as_bytes());
    }
    disk.put(OP_GRAPH, op.id().as_bytes(), &bytes)
}

pub(super) fn write(
    path: &Path,
    selected: &jj_lib::op_store::OperationId,
    disk: &Tables,
    source: &impl Source,
    space: Arc<Space>,
) -> Result<()> {
    let mut count = 0u64;
    let mut row = disk.first(OP_GRAPH)?;
    while let Some((id, _)) = row {
        source.work(1)?;
        count = count.checked_add(1).ok_or(Error::Limit)?;
        row = disk.after(OP_GRAPH, &id)?;
    }
    let mut file = BufWriter::new(
        std::fs::File::options()
            .write(true)
            .create_new(true)
            .open(path.join(HISTORY_NAME))
            .map_err(io_error)?,
    );
    let mut digest = Sha256::new();
    let mut output = |bytes: &[u8]| -> Result<()> {
        source.work(1)?;
        space.reserve(bytes.len() as u64).map_err(io_error)?;
        file.write_all(bytes).map_err(io_error)?;
        digest.update(bytes);
        Ok(())
    };
    output(MAGIC)?;
    output(selected.as_bytes())?;
    output(&count.to_be_bytes())?;
    let mut row = disk.first(OP_GRAPH)?;
    while let Some((id, bytes)) = row {
        output(&id)?;
        output(&bytes)?;
        row = disk.after(OP_GRAPH, &id)?;
    }
    space.reserve(32).map_err(io_error)?;
    file.write_all(&digest.finalize()).map_err(io_error)?;
    file.flush().map_err(io_error)?;
    file.get_ref().sync_all().map_err(io_error)
}

struct Node {
    key: Order,
    parents: Vec<OperationId>,
}
/// Same comparison as op_walk::OperationByEndTime, including timezone and ID.
#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
struct Order {
    millis: i64,
    offset: i32,
    id: OperationId,
}
pub(crate) struct Graph {
    nodes: Vec<Node>,
    weight: u64,
    _memory: Lease,
}
impl Graph {
    fn check_operation(&self, operation: &Operation) -> Result<()> {
        let id = OperationId::from_slice(operation.id().as_bytes()).map_err(|_| Error::Corrupt)?;
        let position = self
            .nodes
            .binary_search_by_key(&id, |node| node.key.id)
            .map_err(|_| Error::NotCommitted)?;
        let node = &self.nodes[position];
        let end = operation.metadata().time.end;
        if end.timestamp.0 != node.key.millis
            || end.tz_offset != node.key.offset
            || operation.parent_ids().len() != node.parents.len()
            || operation
                .parent_ids()
                .iter()
                .zip(&node.parents)
                .any(|(actual, expected)| actual.as_bytes() != expected.as_bytes())
        {
            return Err(Error::Corrupt);
        }
        Ok(())
    }
    pub(crate) fn weight(&self) -> u64 {
        self.weight
    }
    pub(super) fn load(
        path: &Path,
        selected: OperationId,
        ids: &[super::Record],
        work: &mut impl FnMut(usize) -> Result<()>,
        read_bytes: &mut impl FnMut(usize) -> Result<()>,
    ) -> Result<Option<Self>> {
        let path = path.join(HISTORY_NAME);
        let metadata = match std::fs::symlink_metadata(&path) {
            Ok(metadata) => metadata,
            Err(error) if error.kind() == std::io::ErrorKind::NotFound => return Ok(None),
            Err(_) => return Err(Error::Incomplete),
        };
        if !metadata.is_file()
            || metadata.file_type().is_symlink()
            || metadata.len() < FRAMING + RECORD
        {
            return Err(Error::Corrupt);
        }
        read_bytes(usize::try_from(metadata.len()).map_err(|_| Error::Limit)?)?;
        // Includes vectors, parent IDs, decoder buffers and transient cycle proof.
        let weight = metadata
            .len()
            .checked_mul(4)
            .and_then(|bytes| bytes.checked_add(ids.len() as u64 * 256 + 64 * 1024))
            .ok_or(Error::Limit)?;
        let memory = Lease::reserve(weight)?;
        let mut file = BufReader::new(std::fs::File::open(&path).map_err(|_| Error::Incomplete)?);
        let mut digest = Sha256::new();
        let mut header = [0; 80];
        file.read_exact(&mut header).map_err(|_| Error::Corrupt)?;
        if &header[..8] != MAGIC
            || &header[8..72] != selected.as_bytes()
            || u64::from_be_bytes(header[72..].try_into().unwrap()) != ids.len() as u64
        {
            return Err(Error::Corrupt);
        }
        digest.update(header);
        let mut nodes = Vec::with_capacity(ids.len());
        let mut remaining = metadata.len() - FRAMING;
        for expected in ids {
            work(1)?;
            remaining = remaining.checked_sub(RECORD).ok_or(Error::Corrupt)?;
            let mut bytes = [0; 80];
            file.read_exact(&mut bytes).map_err(|_| Error::Corrupt)?;
            digest.update(bytes);
            if &bytes[..64] != expected.id.as_bytes() {
                return Err(Error::Corrupt);
            }
            let parents = u32::from_be_bytes(bytes[76..].try_into().unwrap()) as usize;
            if parents > 1024 {
                return Err(Error::Corrupt);
            }
            remaining = remaining
                .checked_sub(parents as u64 * 64)
                .ok_or(Error::Corrupt)?;
            let mut parent_ids = Vec::with_capacity(parents);
            for _ in 0..parents {
                work(ids.len().max(1).ilog2() as usize + 2)?;
                let mut bytes = [0; 64];
                file.read_exact(&mut bytes).map_err(|_| Error::Corrupt)?;
                digest.update(bytes);
                let parent = OperationId::new(bytes);
                if ids
                    .binary_search_by_key(&parent, |record| record.id)
                    .is_err()
                {
                    return Err(Error::Corrupt);
                }
                parent_ids.push(parent);
            }
            nodes.push(Node {
                key: Order {
                    id: expected.id,
                    millis: i64::from_be_bytes(bytes[64..72].try_into().unwrap()),
                    offset: i32::from_be_bytes(bytes[72..76].try_into().unwrap()),
                },
                parents: parent_ids,
            });
        }
        let mut checksum = [0; 32];
        file.read_exact(&mut checksum).map_err(|_| Error::Corrupt)?;
        if remaining != 0
            || checksum != <[u8; 32]>::from(digest.finalize())
            || file.read(&mut [0; 1]).map_err(|_| Error::Corrupt)? != 0
        {
            return Err(Error::Corrupt);
        }
        let graph = Self {
            nodes,
            weight,
            _memory: memory,
        };
        graph.validate(work)?;
        Ok(Some(graph))
    }
    fn position(
        &self,
        id: OperationId,
        work: &mut impl FnMut(usize) -> Result<()>,
    ) -> Result<usize> {
        work(self.nodes.len().max(1).ilog2() as usize + 2)?;
        self.nodes
            .binary_search_by_key(&id, |node| node.key.id)
            .map_err(|_| Error::NotCommitted)
    }
    fn validate(&self, work: &mut impl FnMut(usize) -> Result<()>) -> Result<()> {
        let mut children = vec![0u32; self.nodes.len()];
        for node in &self.nodes {
            work(1)?;
            for parent in &node.parents {
                let position = self.position(*parent, work)?;
                children[position] = children[position].checked_add(1).ok_or(Error::Limit)?;
            }
        }
        let mut pending: Vec<_> = children
            .iter()
            .enumerate()
            .filter_map(|(position, count)| (*count == 0).then_some(position))
            .collect();
        let mut count = 0;
        while let Some(position) = pending.pop() {
            work(1)?;
            count += 1;
            for parent in &self.nodes[position].parents {
                let position = self.position(*parent, work)?;
                children[position] -= 1;
                if children[position] == 0 {
                    pending.push(position);
                }
            }
        }
        if count != self.nodes.len() {
            return Err(Error::Corrupt);
        }
        Ok(())
    }
    pub(crate) fn walk<'a>(
        self: Arc<Self>,
        start: OperationId,
        mut work: impl FnMut(usize) -> Result<()> + 'a,
    ) -> Result<impl Stream<Item = Result<OperationId>> + 'a> {
        let head = self.nodes[self.position(start, &mut work)?].key;
        // JJ may retain a whole branch chunk before yielding a node. Admit a
        // conservative complete compact frontier before entering that decoder.
        let memory = Lease::reserve(self.weight.checked_mul(4).ok_or(Error::Limit)?)?;
        Ok(jj_lib::dag_walk_async::topo_order_reverse_lazy(
            [Ok(head)],
            |node| node.id,
            async move |node: &Order| {
                let mut neighbors = || -> Result<Vec<Result<Order>>> {
                    let position = self.position(node.id, &mut work)?;
                    work(self.nodes[position].parents.len() + 1)?;
                    self.nodes[position]
                        .parents
                        .iter()
                        .map(|id| Ok(Ok(self.nodes[self.position(*id, &mut work)?].key)))
                        .collect()
                };
                match neighbors() {
                    Ok(nodes) => nodes,
                    Err(error) => vec![Err(error)],
                }
            },
            |_| Error::Corrupt,
        )
        .map_ok(move |node| {
            let _memory = &memory;
            node.id
        }))
    }
}

#[cfg(test)]
mod tests;

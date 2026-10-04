//! Bounded disk-backed construction of the pinned JJ 0.45.1 commit index.
//!
//! Scratch tables retain IDs and compact graph fields, never commit bodies.
//! Ordering and format follow jj-lib's Apache-2.0 default_index builder (format
//! 6), with Kahn's queue ordered by committer timestamp then commit ID.
mod disk;
mod segment;
use super::{Error, Result, access::Budget};
use disk::{Disk, Space, Table, io_error};
use jj_lib::{backend, object_id::ObjectId as _, operation::Operation, store::Store};
use redb::TableDefinition;
use std::{path::Path, sync::Arc};

const OPS: Table = TableDefinition::new("pending_ops");
const SEEN_OPS: Table = TableDefinition::new("seen_ops");
const PENDING: Table = TableDefinition::new("pending_commits");
const NODES: Table = TableDefinition::new("nodes");
const CHILDREN: Table = TableDefinition::new("children");
const QUEUE: Table = TableDefinition::new("queue");
const ORDER: Table = TableDefinition::new("order");
const POSITIONS: Table = TableDefinition::new("positions");
const GRAPH: Table = TableDefinition::new("graph");
const CHANGES: Table = TableDefinition::new("changes");
const CHANGE_LOOKUP: Table = TableDefinition::new("change_lookup");
const PARENT_OVERFLOW: Table = TableDefinition::new("parent_overflow");
const CHANGE_OVERFLOW: Table = TableDefinition::new("change_overflow");

fn number(bytes: &[u8]) -> Result<u32> {
    bytes
        .try_into()
        .map(u32::from_be_bytes)
        .map_err(|_| Error::Corrupt)
}
fn required(disk: &Disk, table: Table, key: &[u8]) -> Result<Vec<u8>> {
    disk.get(table, key)?.ok_or(Error::Incomplete)
}
fn count(disk: &Disk, table: Table, key: &[u8]) -> Result<u32> {
    disk.get(table, key)?.map_or(Ok(0), |n| number(&n))
}

struct Node {
    timestamp: i64,
    change: Vec<u8>,
    parents: Vec<backend::CommitId>,
}
impl Node {
    fn encode(&self) -> Vec<u8> {
        let mut bytes = self.timestamp.to_be_bytes().to_vec();
        bytes.extend_from_slice(&self.change);
        for parent in &self.parents {
            bytes.extend_from_slice(parent.as_bytes());
        }
        bytes
    }
    fn decode(bytes: &[u8], commit_len: usize, change_len: usize) -> Result<Self> {
        let header = 8 + change_len;
        if bytes.len() < header || !(bytes.len() - header).is_multiple_of(commit_len) {
            return Err(Error::Corrupt);
        }
        Ok(Self {
            timestamp: i64::from_be_bytes(bytes[..8].try_into().unwrap()),
            change: bytes[8..header].to_vec(),
            parents: bytes[header..]
                .chunks_exact(commit_len)
                .map(backend::CommitId::from_bytes)
                .collect(),
        })
    }
    fn sort_key(&self, id: &[u8]) -> Vec<u8> {
        let mut key = ((self.timestamp as u64) ^ (1 << 63)).to_be_bytes().to_vec();
        key.extend_from_slice(id);
        key
    }
}

/// The containing cache owns cleanup and reserves `max_bytes` before entry.
/// No operation link is installed until a complete segment has been synced.
pub(super) async fn build(
    path: &Path,
    operation: &Operation,
    store: &Arc<Store>,
    budget: &Budget,
    max_bytes: u64,
) -> Result<()> {
    let source = NativeSource {
        operation,
        store,
        budget,
    };
    build_from(path, operation.id(), &source, max_bytes).await
}

/// Only the traversal calls these readers, with IDs reached from the admitted
/// operation. Implementations retain profile-specific byte/identity admission.
pub(crate) struct CommitFields {
    pub parents: Vec<backend::CommitId>,
    pub change: backend::ChangeId,
    pub timestamp: i64,
}

pub(crate) trait Source {
    fn check(&self) -> Result<()>;
    fn work(&self, amount: usize) -> Result<()>;
    fn max_commits(&self) -> usize;
    fn max_parents(&self) -> usize;
    fn commit_len(&self) -> usize;
    fn change_len(&self) -> usize;
    async fn operation(&self, id: &jj_lib::op_store::OperationId) -> Result<Operation>;
    async fn view(&self, operation: &Operation) -> Result<jj_lib::view::View>;
    async fn commit(&self, id: &backend::CommitId) -> Result<CommitFields>;
}
struct NativeSource<'a> {
    operation: &'a Operation,
    store: &'a Arc<Store>,
    budget: &'a Budget,
}
impl Source for NativeSource<'_> {
    fn check(&self) -> Result<()> {
        self.budget.check()
    }
    fn work(&self, amount: usize) -> Result<()> {
        self.budget.work(amount)
    }
    fn max_commits(&self) -> usize {
        self.budget.limits.max_index_commits
    }
    fn max_parents(&self) -> usize {
        self.budget.limits.metadata.max_parent_edges
    }
    fn commit_len(&self) -> usize {
        self.store.commit_id_length()
    }
    fn change_len(&self) -> usize {
        self.store.change_id_length()
    }
    async fn operation(&self, id: &jj_lib::op_store::OperationId) -> Result<Operation> {
        let data = self
            .budget
            .until(self.operation.op_store().read_operation(id))
            .await?
            .map_err(|_| Error::Incomplete)?;
        Ok(Operation::new(
            self.operation.op_store().clone(),
            id.clone(),
            data,
        ))
    }
    async fn view(&self, operation: &Operation) -> Result<jj_lib::view::View> {
        self.budget
            .until(operation.view())
            .await?
            .map_err(|_| Error::Incomplete)
    }
    async fn commit(&self, id: &backend::CommitId) -> Result<CommitFields> {
        // Bypass the bounded commit-body cache; native capability checks remain.
        let commit = self
            .budget
            .until(self.store.backend().read_commit(id))
            .await?
            .map_err(|_| self.budget.failure().unwrap_or(Error::Incomplete))?;
        Ok(CommitFields {
            parents: commit.parents,
            change: commit.change_id,
            timestamp: commit.committer.timestamp.timestamp.0,
        })
    }
}

pub(crate) async fn build_from(
    path: &Path,
    operation: &jj_lib::op_store::OperationId,
    budget: &impl Source,
    max_bytes: u64,
) -> Result<()> {
    budget.check()?;
    let scratch = tempfile::Builder::new()
        .prefix(".prepare-")
        .tempdir_in(path)
        .map_err(io_error)?;
    let space = Space::new(max_bytes);
    let mut disk = Disk::new(&scratch.path().join("graph.redb"), space.clone())?;
    disk.put(OPS, operation.as_bytes(), &[])?;
    // Unlike upstream's operation walk, no frontier or retained view set grows
    // in memory with history. Order is irrelevant to the set of historical roots.
    while let Some((id, _)) = disk.first(OPS)? {
        budget.work(1)?;
        disk.remove(OPS, &id)?;
        if disk.get(SEEN_OPS, &id)?.is_some() {
            continue;
        }
        let id = jj_lib::op_store::OperationId::from_bytes(&id);
        let op = budget.operation(&id).await?;
        for parent in op.parent_ids() {
            disk.put(OPS, parent.as_bytes(), &[])?;
        }
        let view = budget.view(&op).await?;
        for commit in op
            .all_referenced_commit_ids()
            .chain(view.all_referenced_commit_ids())
        {
            budget.work(1)?;
            disk.put(PENDING, commit.as_bytes(), &[])?;
        }
        disk.put(SEEN_OPS, id.as_bytes(), &[])?;
    }
    let mut total = 0u32;
    let commit_len = budget.commit_len();
    let change_len = budget.change_len();
    while let Some((id, _)) = disk.first(PENDING)? {
        budget.work(1)?;
        disk.remove(PENDING, &id)?;
        if disk.get(NODES, &id)?.is_some() {
            continue;
        }
        if total as usize >= budget.max_commits() || total >= 0x7fff_ffff {
            return Err(Error::IndexBudget);
        }
        let data = budget.commit(&backend::CommitId::from_bytes(&id)).await?;
        if data.parents.len() > budget.max_parents() {
            return Err(Error::Limit);
        }
        let node = Node {
            timestamp: data.timestamp,
            change: data.change.as_bytes().to_vec(),
            parents: data.parents,
        };
        for parent in &node.parents {
            budget.work(1)?;
            disk.put(PENDING, parent.as_bytes(), &[])?;
            let n = count(&disk, CHILDREN, parent.as_bytes())?
                .checked_add(1)
                .ok_or(Error::Limit)?;
            disk.put(CHILDREN, parent.as_bytes(), &n.to_be_bytes())?;
        }
        disk.put(NODES, &id, &node.encode())?;
        total += 1;
    }
    // Match upstream reverse-topological ordering, including skewed timestamps.
    let mut row = disk.first(NODES)?;
    while let Some((id, bytes)) = row {
        budget.work(1)?;
        if count(&disk, CHILDREN, &id)? == 0 {
            let node = Node::decode(&bytes, commit_len, change_len)?;
            disk.put(QUEUE, &node.sort_key(&id), &id)?;
        }
        row = disk.after(NODES, &id)?;
    }
    let mut emitted = 0u32;
    while let Some((key, id)) = disk.last(QUEUE)? {
        budget.work(1)?;
        disk.remove(QUEUE, &key)?;
        let node = Node::decode(&required(&disk, NODES, &id)?, commit_len, change_len)?;
        disk.put(ORDER, &emitted.to_be_bytes(), &id)?;
        emitted += 1;
        for parent in node.parents {
            budget.work(1)?;
            let remaining = count(&disk, CHILDREN, parent.as_bytes())?
                .checked_sub(1)
                .ok_or(Error::Corrupt)?;
            disk.put(CHILDREN, parent.as_bytes(), &remaining.to_be_bytes())?;
            if remaining == 0 {
                let node = Node::decode(
                    &required(&disk, NODES, parent.as_bytes())?,
                    commit_len,
                    change_len,
                )?;
                disk.put(QUEUE, &node.sort_key(parent.as_bytes()), parent.as_bytes())?;
            }
        }
    }
    if emitted != total {
        return Err(Error::Corrupt);
    }
    segment::write(
        path, operation, &mut disk, total, commit_len, change_len, budget, space,
    )?;
    drop(disk);
    drop(scratch);
    Ok(())
}

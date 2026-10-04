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
    budget.check()?;
    let scratch = tempfile::Builder::new()
        .prefix(".prepare-")
        .tempdir_in(path)
        .map_err(io_error)?;
    let space = Space::new(max_bytes);
    let mut disk = Disk::new(&scratch.path().join("graph.redb"), space.clone())?;
    disk.put(OPS, operation.id().as_bytes(), &[])?;
    // Unlike upstream's operation walk, no frontier or retained view set grows
    // in memory with history. Order is irrelevant to the set of historical roots.
    while let Some((id, _)) = disk.first(OPS)? {
        budget.work(1)?;
        disk.remove(OPS, &id)?;
        if disk.get(SEEN_OPS, &id)?.is_some() {
            continue;
        }
        let id = jj_lib::op_store::OperationId::from_bytes(&id);
        let data = budget
            .until(operation.op_store().read_operation(&id))
            .await?
            .map_err(|_| Error::Incomplete)?;
        let op = Operation::new(operation.op_store().clone(), id.clone(), data);
        for parent in op.parent_ids() {
            disk.put(OPS, parent.as_bytes(), &[])?;
        }
        let view = budget
            .until(op.view())
            .await?
            .map_err(|_| Error::Incomplete)?;
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
    let commit_len = store.commit_id_length();
    let change_len = store.change_id_length();
    while let Some((id, _)) = disk.first(PENDING)? {
        budget.work(1)?;
        disk.remove(PENDING, &id)?;
        if disk.get(NODES, &id)?.is_some() {
            continue;
        }
        if total as usize >= budget.limits.max_index_commits || total >= 0x7fff_ffff {
            return Err(budget.fail(Error::IndexBudget));
        }
        // Bypass Store's commit-body cache. The guarded backend still checks
        // identity, current authorization, membership, bytes and cancellation.
        let data = budget
            .until(
                store
                    .backend()
                    .read_commit(&backend::CommitId::from_bytes(&id)),
            )
            .await?
            .map_err(|_| budget.failure().unwrap_or(Error::Incomplete))?;
        if data.parents.len() > budget.limits.metadata.max_parent_edges {
            return Err(budget.fail(Error::Limit));
        }
        let node = Node {
            timestamp: data.committer.timestamp.timestamp.0,
            change: data.change_id.as_bytes().to_vec(),
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

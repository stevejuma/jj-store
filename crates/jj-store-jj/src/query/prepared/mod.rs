//! Bounded disk-backed construction of the pinned JJ 0.45.1 commit index.
//!
//! Scratch tables retain IDs and compact graph fields, never commit bodies.
//! Ordering and format follow jj-lib's Apache-2.0 default_index builder (format
//! 6), with Kahn's queue ordered by committer timestamp then commit ID.
mod disk;
pub(crate) mod filters;
pub(crate) mod operations;
mod segment;
mod tables;
pub(crate) mod visible;
#[cfg(feature = "native")]
use super::access::Budget;
use super::{Error, Result};
use disk::{Disk, Space, Table, io_error};
#[cfg(feature = "native")]
use jj_lib::store::Store;
use jj_lib::{backend, object_id::ObjectId as _, operation::Operation};
use redb::TableDefinition;
use std::{path::Path, sync::Arc};
use tables::Tables;

const OPS: Table = TableDefinition::new("pending_ops");
const OP_CHILDREN: Table = TableDefinition::new("local_operation_children");
const OP_GRAPH: Table = TableDefinition::new("operation_graph");
const EVO_ANCHORS: Table = TableDefinition::new("evolution_anchor_occurrences");
const EVO_LOOKUP: Table = TableDefinition::new("evolution_operation_occurrences");
const EVO_KNOWN: Table = TableDefinition::new("evolution_known_operations");
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
const FILTER_CHANGES: Table = TableDefinition::new("filter_changes");
const FILTER_BOOKMARKS: Table = TableDefinition::new("filter_bookmarks");

fn number(bytes: &[u8]) -> Result<u32> {
    bytes
        .try_into()
        .map(u32::from_be_bytes)
        .map_err(|_| Error::Corrupt)
}
fn required(disk: &Tables, table: Table, key: &[u8]) -> Result<Vec<u8>> {
    disk.get(table, key)?.ok_or(Error::Incomplete)
}
fn count(disk: &Tables, table: Table, key: &[u8]) -> Result<u32> {
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

#[derive(Default)]
#[cfg(feature = "native")]
pub(super) struct Features {
    pub filtering: bool,
    pub operations: bool,
}

/// The containing cache owns cleanup and reserves `max_bytes` before entry.
/// No operation link is installed until a complete segment has been synced.
#[cfg(feature = "native")]
pub(super) async fn build(
    path: &Path,
    operation: &Operation,
    store: &Arc<Store>,
    budget: &Budget,
    max_bytes: u64,
    inline: bool,
    features: Features,
) -> Result<()> {
    let source = NativeSource {
        operation,
        store,
        budget,
        inline,
        filtering: features.filtering,
        lookup: features.operations,
        view_identity: std::cell::Cell::new(None),
    };
    build_with_tables(
        path,
        operation.id(),
        &source,
        max_bytes,
        inline || max_bytes < 32 * 1024 * 1024,
    )
    .await
}

/// Only the traversal calls these readers, with IDs reached from the admitted
/// operation. Implementations retain profile-specific byte/identity admission.
pub(crate) struct CommitFields {
    pub parents: Vec<backend::CommitId>,
    pub change: backend::ChangeId,
    pub timestamp: i64,
    pub description: Option<Description>,
}
pub(crate) struct Description {
    pub text: String,
    _memory: super::memory::Lease,
}
impl Description {
    #[cfg(feature = "native")]
    fn from_string(text: String) -> Result<Self> {
        let memory = super::memory::Lease::reserve(4096 + text.len() as u64 * 2)?;
        Ok(Self {
            text,
            _memory: memory,
        })
    }
    pub fn from_bytes(bytes: &[u8]) -> Result<Self> {
        // Lossy UTF-8 can expand each input byte to a three-byte replacement.
        let memory = super::memory::Lease::reserve(4096 + bytes.len() as u64 * 6)?;
        Ok(Self {
            text: String::from_utf8_lossy(bytes).into_owned(),
            _memory: memory,
        })
    }
}

pub(crate) trait Source {
    fn check(&self) -> Result<()>;
    fn work(&self, amount: usize) -> Result<()>;
    fn max_commits(&self) -> usize;
    fn index_limit_error(&self) -> Error {
        Error::IndexBudget
    }
    fn max_parents(&self) -> usize;
    fn commit_len(&self) -> usize;
    fn change_len(&self) -> usize;
    fn operation_lookup(&self) -> bool {
        false
    }
    fn reference_filter(&self) -> bool {
        false
    }
    fn operation_digest(&self, _: &Operation) -> Result<[u8; 32]> {
        Err(Error::Invalid)
    }
    fn view_digest(&self, _: &jj_lib::view::View) -> Result<[u8; 32]> {
        Err(Error::Invalid)
    }
    async fn operation(&self, id: &jj_lib::op_store::OperationId) -> Result<Operation>;
    async fn view(&self, operation: &Operation) -> Result<jj_lib::view::View>;
    async fn commit(&self, id: &backend::CommitId) -> Result<CommitFields>;
}
#[cfg(feature = "native")]
struct NativeSource<'a> {
    operation: &'a Operation,
    store: &'a Arc<Store>,
    budget: &'a Budget,
    inline: bool,
    filtering: bool,
    lookup: bool,
    view_identity: std::cell::Cell<Option<[u8; 32]>>,
}
#[cfg(feature = "native")]
impl Source for NativeSource<'_> {
    fn check(&self) -> Result<()> {
        self.budget.check()
    }
    fn work(&self, amount: usize) -> Result<()> {
        self.budget.work(amount)
    }
    fn max_commits(&self) -> usize {
        if self.inline {
            self.budget
                .limits
                .max_index_commits
                .min(self.budget.limits.max_inline_rebuild_commits)
        } else {
            self.budget.limits.max_index_commits
        }
    }
    fn index_limit_error(&self) -> Error {
        if self.inline
            && self.budget.limits.max_inline_rebuild_commits < self.budget.limits.max_index_commits
        {
            Error::IndexRebuildRequired
        } else {
            Error::IndexBudget
        }
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
    fn operation_lookup(&self) -> bool {
        self.lookup
    }
    fn operation_digest(&self, operation: &Operation) -> Result<[u8; 32]> {
        use sha2::{Digest as _, Sha256};
        self.budget.work(1)?;
        Ok(Sha256::digest(operation.id().as_bytes()).into())
    }
    fn view_digest(&self, _: &jj_lib::view::View) -> Result<[u8; 32]> {
        self.budget.work(1)?;
        // Captured by the immediately preceding verified view read. Avoid
        // serializing/hashing the complete view again for a disposable hint.
        self.view_identity.get().ok_or(Error::Corrupt)
    }
    fn reference_filter(&self) -> bool {
        self.filtering
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
        use sha2::{Digest as _, Sha256};
        let view = self
            .budget
            .until(operation.view())
            .await?
            .map_err(|_| Error::Incomplete)?;
        if self.lookup {
            self.view_identity.set(Some(
                Sha256::digest(operation.store_operation().view_id.as_bytes()).into(),
            ));
        }
        Ok(view)
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
            description: if self.filtering {
                Some(Description::from_string(commit.description)?)
            } else {
                None
            },
        })
    }
}

pub(crate) async fn build_from(
    path: &Path,
    operation: &jj_lib::op_store::OperationId,
    budget: &impl Source,
    max_bytes: u64,
) -> Result<()> {
    build_with_tables(path, operation, budget, max_bytes, false).await
}
async fn build_with_tables(
    path: &Path,
    operation: &jj_lib::op_store::OperationId,
    budget: &impl Source,
    max_bytes: u64,
    small: bool,
) -> Result<()> {
    budget.check()?;
    let scratch = tempfile::Builder::new()
        .prefix(".prepare-")
        .tempdir_in(path)
        .map_err(io_error)?;
    let space = Space::new(max_bytes);
    let mut disk = Tables::new(&scratch.path().join("graph.redb"), space.clone(), small)?;
    let mut filter = if budget.reference_filter() {
        Some(filters::Builder::new(path, space.clone())?)
    } else {
        None
    };
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
        if budget.operation_lookup() {
            operations::history::capture(&mut disk, &op, budget)?;
        }
        for parent in op.parent_ids() {
            disk.put(OPS, parent.as_bytes(), &[])?;
            if budget.operation_lookup() {
                budget.work(1)?;
                if parent.as_bytes().len() != 64 || id.as_bytes().len() != 64 {
                    return Err(Error::Corrupt);
                }
                let mut edge = [0; 128];
                edge[..64].copy_from_slice(parent.as_bytes());
                edge[64..].copy_from_slice(id.as_bytes());
                disk.put(OP_CHILDREN, &edge, &[])?;
            }
        }
        let view = budget.view(&op).await?;
        if filter.is_some() && op.id() == operation {
            for (name, target) in &view.store_view().local_bookmarks {
                budget.work(1)?;
                let mut ids = Vec::new();
                for id in target.iter().flatten() {
                    budget.work(1)?;
                    ids.extend_from_slice(id.as_bytes());
                }
                disk.put(
                    FILTER_BOOKMARKS,
                    format!("bookmark:{}", name.as_str()).as_bytes(),
                    &ids,
                )?;
            }
        }
        for commit in op
            .all_referenced_commit_ids()
            .chain(view.all_referenced_commit_ids())
        {
            budget.work(1)?;
            disk.put(PENDING, commit.as_bytes(), &[])?;
        }
        if budget.operation_lookup() {
            let mut fingerprints = budget.operation_digest(&op)?.to_vec();
            fingerprints.extend(budget.view_digest(&view)?);
            disk.put(SEEN_OPS, id.as_bytes(), &fingerprints)?;
        } else {
            disk.put(SEEN_OPS, id.as_bytes(), &[])?;
        }
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
            return Err(budget.index_limit_error());
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
        if let Some(filter) = &mut filter {
            filter.commit(
                &id,
                &data.description.as_ref().ok_or(Error::Incomplete)?.text,
                budget,
            )?;
            let mut key = node.change.clone();
            key.extend_from_slice(&id);
            disk.put(FILTER_CHANGES, &key, &[])?;
        }
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
    if budget.operation_lookup() {
        operations::write(path, operation, &disk, budget, space.clone())?;
    }
    segment::write(
        path,
        operation,
        &mut disk,
        total,
        commit_len,
        change_len,
        budget,
        space.clone(),
    )?;
    if let Some(mut filter) = filter {
        let mut row = disk.first(FILTER_BOOKMARKS)?;
        while let Some((key, ids)) = row {
            budget.work(1)?;
            if !ids.len().is_multiple_of(commit_len) {
                return Err(Error::Corrupt);
            }
            let rank = filter.reference(
                std::str::from_utf8(&key).map_err(|_| Error::Corrupt)?,
                budget,
            )?;
            for id in ids.chunks_exact(commit_len) {
                filter.target(rank, id, budget)?;
            }
            row = disk.after(FILTER_BOOKMARKS, &key)?;
        }
        let mut row = disk.first(FILTER_CHANGES)?;
        while let Some((key, _)) = row {
            budget.work(1)?;
            if key.len() != change_len + commit_len {
                return Err(Error::Corrupt);
            }
            let change = &key[..change_len];
            let key_name = format!("change:{}", backend::ChangeId::from_bytes(change).hex());
            let rank = filter.reference(&key_name, budget)?;
            filter.target(rank, &key[change_len..], budget)?;
            row = disk.after(FILTER_CHANGES, &key)?;
            while let Some((key, _)) = row.as_ref().filter(|(key, _)| key.starts_with(change)) {
                budget.work(1)?;
                if key.len() != change_len + commit_len {
                    return Err(Error::Corrupt);
                }
                filter.target(rank, &key[change_len..], budget)?;
                row = disk.after(FILTER_CHANGES, key)?;
            }
        }
        filter.finish(path, operation, budget, space)?;
    }
    drop(disk);
    drop(scratch);
    Ok(())
}

//! Bounded closure verification with an incrementally reusable verified base.
use jj_store_model::{EnvelopeDigest, ObjectKey, RepositoryId};
use serde::{Deserialize, Serialize};
use std::sync::atomic::{AtomicU64, Ordering};
use std::{
    collections::{BTreeMap, BTreeSet},
    time::Duration,
};

/// Process-local capability for one verification source. It is deliberately not
/// serializable: restart recovery must verify the selected persistent catalog.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct SourceScope {
    repository: RepositoryId,
    instance: u64,
}
impl SourceScope {
    pub fn repository(self) -> RepositoryId {
        self.repository
    }
    pub fn new(repository: RepositoryId) -> Self {
        static NEXT: AtomicU64 = AtomicU64::new(0);
        let instance = NEXT
            .fetch_update(Ordering::Relaxed, Ordering::Relaxed, |v| v.checked_add(1))
            .expect("verification source namespace exhausted");
        Self {
            repository,
            instance,
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(tag = "type", rename_all = "snake_case", deny_unknown_fields)]
pub enum Representation {
    Virtual,
    Stored {
        envelope_digest: EnvelopeDigest,
        bytes: u64,
    },
}
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct VerifiedNode {
    pub key: ObjectKey,
    pub representation: Representation,
    pub dependencies: BTreeSet<ObjectKey>,
}
#[derive(Debug, thiserror::Error)]
pub enum InventoryError {
    #[error("dependency read or integrity failure: {0}")]
    Source(String),
    #[error("dependency source returned a different object identity")]
    Key,
    #[error("verified inventory belongs to another source or repository")]
    Scope,
    #[error("dependency graph contains a cycle")]
    Cycle,
    #[error("dependency inventory exceeded {0} budget")]
    Limit(&'static str),
}
pub type Result<T> = std::result::Result<T, InventoryError>;
#[async_trait::async_trait]
pub trait DependencySource: Send + Sync {
    fn scope(&self) -> SourceScope;
    /// Verify content, then enumerate *all* native dependencies. Implementations
    /// must not treat an existing key/HEAD alone as proof of integrity.
    async fn load_verified(&self, key: ObjectKey, max_bytes: u64) -> Result<VerifiedNode>;
}
#[derive(Clone, Copy, Debug)]
pub struct Budget {
    pub max_nodes: usize,
    pub max_edges: usize,
    pub max_new_bytes: u64,
    pub timeout: Duration,
}
impl Default for Budget {
    fn default() -> Self {
        Self {
            max_nodes: 100_000,
            max_edges: 1_000_000,
            max_new_bytes: 1024 * 1024 * 1024,
            timeout: Duration::from_secs(30),
        }
    }
}
/// All entries and their transitive dependencies were verified through the
/// source. Private construction prevents callers from certifying arbitrary IDs.
/// A publication coordinator must bind this to a selected committed inventory.
#[derive(Clone, Debug, Default)]
pub struct VerifiedInventory {
    entries: BTreeMap<ObjectKey, VerifiedNode>,
    edges: usize,
    scope: Option<SourceScope>,
}
impl VerifiedInventory {
    pub fn entries(&self) -> &BTreeMap<ObjectKey, VerifiedNode> {
        &self.entries
    }
    pub fn edge_count(&self) -> usize {
        self.edges
    }
}
#[derive(Debug)]
pub struct InventoryUpdate {
    pub complete: VerifiedInventory,
    pub added: BTreeSet<ObjectKey>,
    pub loaded_bytes: u64,
}
/// Reuse verified immutable base entries without refetching them. Always retain
/// the full base, including historical objects no longer reachable from heads.
pub async fn extend(
    source: &dyn DependencySource,
    base: &VerifiedInventory,
    roots: impl IntoIterator<Item = ObjectKey>,
    budget: Budget,
) -> Result<InventoryUpdate> {
    if base.scope.is_some_and(|scope| scope != source.scope()) {
        return Err(InventoryError::Scope);
    }
    if base.entries.len() > budget.max_nodes {
        return Err(InventoryError::Limit("nodes"));
    }
    if base.edges > budget.max_edges {
        return Err(InventoryError::Limit("edges"));
    }
    let deadline = tokio::time::Instant::now()
        .checked_add(budget.timeout)
        .ok_or(InventoryError::Limit("time"))?;
    let mut complete = base.clone();
    complete.scope = Some(source.scope());
    let mut added = BTreeSet::new();
    let mut loaded_bytes = 0u64;
    let mut visiting = BTreeSet::new();
    let mut done = BTreeSet::new();
    // Explicit enter/exit stack avoids recursive futures and bounds stack depth.
    let mut stack = Vec::new();
    for root in roots {
        if tokio::time::Instant::now() >= deadline {
            return Err(InventoryError::Limit("time"));
        }
        if stack.len() >= budget.max_edges {
            return Err(InventoryError::Limit("edges"));
        }
        stack.push((root, false));
    }
    while let Some((key, exit)) = stack.pop() {
        if tokio::time::Instant::now() >= deadline {
            return Err(InventoryError::Limit("time"));
        }
        if exit {
            visiting.remove(&key);
            done.insert(key);
            continue;
        }
        if base.entries.contains_key(&key) || done.contains(&key) {
            continue;
        }
        if !visiting.insert(key) {
            return Err(InventoryError::Cycle);
        }
        if complete.entries.len() >= budget.max_nodes {
            return Err(InventoryError::Limit("nodes"));
        }
        let node = tokio::time::timeout_at(
            deadline,
            source.load_verified(key, budget.max_new_bytes - loaded_bytes),
        )
        .await
        .map_err(|_| InventoryError::Limit("time"))??;
        if node.key != key {
            return Err(InventoryError::Key);
        }
        if let Representation::Stored { bytes, .. } = node.representation {
            loaded_bytes = loaded_bytes
                .checked_add(bytes)
                .ok_or(InventoryError::Limit("bytes"))?;
            if loaded_bytes > budget.max_new_bytes {
                return Err(InventoryError::Limit("bytes"));
            }
        }
        complete.edges = complete
            .edges
            .checked_add(node.dependencies.len())
            .ok_or(InventoryError::Limit("edges"))?;
        if complete.edges > budget.max_edges {
            return Err(InventoryError::Limit("edges"));
        }
        stack.push((key, true));
        stack.extend(node.dependencies.iter().rev().map(|id| (*id, false)));
        added.insert(key);
        complete.entries.insert(key, node);
    }
    Ok(InventoryUpdate {
        complete,
        added,
        loaded_bytes,
    })
}

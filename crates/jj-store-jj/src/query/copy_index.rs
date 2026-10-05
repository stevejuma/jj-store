//! Disposable descriptor relations. Source pins and allocation leases stay owned
//! by the provider; native body loading and publication never enter this index.
use super::{Error, Result};
use jj_store_model::CopyId;
use std::collections::{BTreeMap, BTreeSet};
use tokio::time::Instant;
use tokio_util::sync::CancellationToken;

const HEADER: usize = 1024 * 1024;
// Conservatively cover both adjacency maps and sealing's degree/ready scratch.
const NODE: usize = 1024;
const EDGE: usize = 512;
// Cover visited/result and pending sets during the two traversal phases.
const LOOKUP_NODE: usize = 512;

#[derive(Clone, Copy, Debug)]
pub struct Limits {
    pub max_nodes: usize,
    pub max_edges: usize,
    pub max_work: usize,
    pub max_bytes: usize,
}
impl Default for Limits {
    fn default() -> Self {
        Self {
            max_nodes: 100_000,
            max_edges: 1_000_000,
            max_work: 10_000_000,
            max_bytes: 64 * 1024 * 1024,
        }
    }
}
#[derive(Clone, Copy, Debug)]
pub struct LookupLimits {
    pub max_nodes: usize,
    pub max_work: usize,
    pub max_bytes: usize,
}
impl Default for LookupLimits {
    fn default() -> Self {
        Self {
            max_nodes: 100_000,
            max_work: 1_000_000,
            max_bytes: 64 * 1024 * 1024,
        }
    }
}
#[derive(Debug)]
pub struct Index {
    parents: BTreeMap<CopyId, BTreeSet<CopyId>>,
    children: BTreeMap<CopyId, BTreeSet<CopyId>>,
    edges: usize,
}
pub struct Builder {
    index: Index,
    limits: Limits,
    work: usize,
}
fn allocation(nodes: usize, edges: usize) -> Option<usize> {
    nodes
        .checked_mul(NODE)?
        .checked_add(edges.checked_mul(EDGE)?)?
        .checked_add(HEADER)
}
impl Builder {
    pub fn new(limits: Limits) -> Result<Self> {
        if limits.max_nodes == 0 || limits.max_work == 0 || limits.max_bytes < HEADER {
            return Err(Error::IndexBudget);
        }
        Ok(Self {
            index: Index {
                parents: BTreeMap::new(),
                children: BTreeMap::new(),
                edges: 0,
            },
            limits,
            work: 0,
        })
    }
    /// Takes an already bounded descriptor relation; admits growth before any
    /// reverse-map insertion. Exact repeated descriptors are idempotent.
    pub fn insert(&mut self, id: CopyId, parents: BTreeSet<CopyId>) -> Result<()> {
        let work = self
            .work
            .checked_add(parents.len())
            .and_then(|n| n.checked_add(1))
            .filter(|n| *n <= self.limits.max_work)
            .ok_or(Error::IndexBudget)?;
        if let Some(previous) = self.index.parents.get(&id) {
            if previous != &parents {
                return Err(Error::Corrupt);
            }
            self.work = work;
            return Ok(());
        }
        let nodes = self
            .index
            .parents
            .len()
            .checked_add(1)
            .ok_or(Error::IndexBudget)?;
        let edges = self
            .index
            .edges
            .checked_add(parents.len())
            .ok_or(Error::IndexBudget)?;
        if nodes > self.limits.max_nodes
            || edges > self.limits.max_edges
            || allocation(nodes, edges).is_none_or(|bytes| bytes > self.limits.max_bytes)
        {
            return Err(Error::IndexBudget);
        }
        for parent in &parents {
            self.index.children.entry(*parent).or_default().insert(id);
        }
        self.index.parents.insert(id, parents);
        self.index.edges = edges;
        self.work = work;
        Ok(())
    }
    pub fn allocated_bytes(&self) -> usize {
        self.index.allocated_bytes()
    }
    /// Only complete, acyclic relations become queryable. The provider retains
    /// its separately admitted global allocation charge through this transfer.
    pub async fn finish(self, deadline: Instant, cancel: &CancellationToken) -> Result<Index> {
        let mut step = Step::new(self.limits.max_work, self.work, deadline, cancel);
        step.check()?;
        for parents in self.index.parents.values() {
            step.spend().await?;
            for parent in parents {
                step.spend().await?;
                if !self.index.parents.contains_key(parent) {
                    return Err(Error::Incomplete);
                }
            }
        }
        let mut children: BTreeMap<_, _> = self
            .index
            .parents
            .keys()
            .map(|id| (*id, self.index.children.get(id).map_or(0, BTreeSet::len)))
            .collect();
        let mut ready: BTreeSet<_> = children
            .iter()
            .filter_map(|(id, count)| (*count == 0).then_some(*id))
            .collect();
        let mut visited = 0;
        while let Some(id) = ready.pop_first() {
            step.spend().await?;
            visited += 1;
            for parent in &self.index.parents[&id] {
                step.spend().await?;
                let count = children.get_mut(parent).expect("complete parent relation");
                *count -= 1;
                if *count == 0 {
                    ready.insert(*parent);
                }
            }
        }
        step.check()?;
        if visited != self.index.parents.len() {
            return Err(Error::Corrupt);
        }
        Ok(self.index)
    }
}
impl Index {
    pub fn node_count(&self) -> usize {
        self.parents.len()
    }
    pub fn edge_count(&self) -> usize {
        self.edges
    }
    /// Conservative construction peak, including sealing scratch.
    pub fn allocated_bytes(&self) -> usize {
        allocation(self.node_count(), self.edges).expect("admitted index allocation")
    }
    pub fn parents(&self, id: CopyId) -> Option<&BTreeSet<CopyId>> {
        self.parents.get(&id)
    }
    /// Exact upstream family: ancestors, then all their descendants. An
    /// unrelated parent of a descendant is not an ancestor of the request.
    pub async fn family(
        &self,
        seeds: &BTreeSet<CopyId>,
        limits: LookupLimits,
        deadline: Instant,
        cancel: &CancellationToken,
    ) -> Result<BTreeSet<CopyId>> {
        let mut step = Step::new(limits.max_work, 0, deadline, cancel);
        step.check()?;
        let admit = |count: usize| {
            if limits.max_nodes == 0
                || limits.max_work == 0
                || count > limits.max_nodes
                || count
                    .checked_mul(LOOKUP_NODE)
                    .is_none_or(|bytes| bytes > limits.max_bytes)
            {
                Err(Error::Limit)
            } else {
                Ok(())
            }
        };
        admit(seeds.len())?;
        for id in seeds {
            step.spend().await?;
            if !self.parents.contains_key(id) {
                return Err(Error::NotCommitted);
            }
        }
        let mut family = seeds.clone();
        let mut pending = seeds.clone();
        while let Some(id) = pending.pop_first() {
            step.spend().await?;
            for parent in &self.parents[&id] {
                step.spend().await?;
                if !family.contains(parent) {
                    admit(family.len() + 1)?;
                    family.insert(*parent);
                    pending.insert(*parent);
                }
            }
        }
        // Every ancestor is a required output, so both phases share the same
        // admission. No unrelated graph scan or adjacency clone is needed.
        pending = family.clone();
        while let Some(id) = pending.pop_first() {
            step.spend().await?;
            if let Some(children) = self.children.get(&id) {
                for child in children {
                    step.spend().await?;
                    if !family.contains(child) {
                        admit(family.len() + 1)?;
                        family.insert(*child);
                        pending.insert(*child);
                    }
                }
            }
        }
        step.check()?;
        Ok(family)
    }
}
struct Step<'a> {
    limit: usize,
    work: usize,
    since_yield: usize,
    deadline: Instant,
    cancel: &'a CancellationToken,
}
impl<'a> Step<'a> {
    fn new(limit: usize, work: usize, deadline: Instant, cancel: &'a CancellationToken) -> Self {
        Self {
            limit,
            work,
            since_yield: 0,
            deadline,
            cancel,
        }
    }
    fn check(&self) -> Result<()> {
        if self.cancel.is_cancelled() {
            Err(Error::Canceled)
        } else if Instant::now() >= self.deadline || self.work > self.limit {
            Err(Error::Limit)
        } else {
            Ok(())
        }
    }
    async fn spend(&mut self) -> Result<()> {
        self.check()?;
        self.work = self.work.checked_add(1).ok_or(Error::Limit)?;
        self.since_yield += 1;
        if self.since_yield >= 128 {
            self.since_yield = 0;
            tokio::task::yield_now().await;
        }
        self.check()
    }
}

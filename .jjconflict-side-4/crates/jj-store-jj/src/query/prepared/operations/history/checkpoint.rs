//! Data-only historical traversal checkpoints, with no stored request callback.
use super::*;
use crate::query::memory::Retained;
use futures::FutureExt as _;
use std::collections::VecDeque;

struct Items {
    nodes: Vec<Order>,
    _memory: Lease,
}
impl Items {
    fn retain(nodes: Vec<Order>) -> Result<Arc<Self>> {
        // The incoming allocation is already covered by traversal scratch.
        let memory =
            Lease::reserve(nodes.capacity() as u64 * std::mem::size_of::<Order>() as u64 + 4096)?;
        Ok(Arc::new(Self {
            nodes,
            _memory: memory,
        }))
    }
}
struct Seen {
    bits: Vec<u64>,
    _memory: Lease,
}
impl Seen {
    fn empty(count: usize, work: &mut impl FnMut(usize) -> Result<()>) -> Result<Arc<Self>> {
        let words = count.div_ceil(64);
        work(words + 1)?;
        let memory = Lease::reserve(words as u64 * 16 + 4096)?;
        Ok(Arc::new(Self {
            bits: vec![0; words],
            _memory: memory,
        }))
    }
    fn insert(
        this: &mut Arc<Self>,
        position: usize,
        work: &mut impl FnMut(usize) -> Result<()>,
    ) -> Result<()> {
        work(1)?;
        if Arc::get_mut(this).is_none() {
            work(this.bits.len())?;
            let memory = Lease::reserve(this.bits.len() as u64 * 16 + 4096)?;
            *this = Arc::new(Self {
                bits: this.bits.clone(),
                _memory: memory,
            });
        }
        let value = &mut Arc::get_mut(this).ok_or(Error::Unavailable)?.bits[position / 64];
        let mask = 1 << (position % 64);
        if *value & mask != 0 {
            return Err(Error::Corrupt);
        }
        *value |= mask;
        Ok(())
    }
}
#[derive(Clone)]
struct State {
    frontier: Arc<Items>,
    pending: Arc<Items>,
    at: usize,
    seen: Arc<Seen>,
    _memory: Arc<Lease>,
}
impl State {
    fn next(
        &mut self,
        graph: &Graph,
        work: &mut impl FnMut(usize) -> Result<()>,
    ) -> Result<Option<OperationId>> {
        if self.at == self.pending.nodes.len() {
            if self.frontier.nodes.is_empty() {
                return Ok(None);
            }
            // Upstream lookahead can visit a complete branch chunk. Admission
            // covers its arrays/maps before calling the pinned implementation.
            let _scratch = Lease::reserve(graph.weight.checked_mul(4).ok_or(Error::Limit)?)?;
            work(self.frontier.nodes.len() + 1)?;
            let mut frontier = self.frontier.nodes.clone();
            let nodes = jj_lib::dag_walk_async::topo_order_reverse_chunked(
                &mut frontier,
                |node| node.id,
                async |node: &Order| graph.neighbors(node, work),
                |_| Error::Corrupt,
            )
            .now_or_never()
            .ok_or(Error::Unavailable)??;
            self.frontier = Items::retain(frontier)?;
            self.pending = Items::retain(nodes.into_vec())?;
            self.at = 0;
        }
        let id = self.pending.nodes.get(self.at).ok_or(Error::Corrupt)?.id;
        let position = graph.position(id, work)?;
        Seen::insert(&mut self.seen, position, work)?;
        self.at += 1;
        Ok(Some(id))
    }
}
struct Entry {
    start: OperationId,
    after: OperationId,
    state: Arc<Retained<State>>,
}
#[derive(Default)]
pub(super) struct Cache(VecDeque<Entry>);
#[cfg(test)]
impl Cache {
    pub(super) fn len(&self) -> usize {
        self.0.len()
    }
    pub(super) fn clear(&mut self) {
        self.0.clear();
    }
}
enum Mode {
    Selected(usize),
    Historical(State),
}
pub(super) struct Cursor<W> {
    graph: Arc<Graph>,
    start: OperationId,
    first: Option<OperationId>,
    last: Option<OperationId>,
    mode: Mode,
    work: W,
}
impl Graph {
    pub(super) fn cursor<W: FnMut(usize) -> Result<()>>(
        self: Arc<Self>,
        start: OperationId,
        after: Option<OperationId>,
        mut work: W,
    ) -> Result<Cursor<W>> {
        let mut first = None;
        let mode = if start == self.selected {
            let rank = if let Some(id) = after {
                let at = self.position(id, &mut work).map_err(|error| {
                    if error == Error::NotCommitted {
                        Error::Invalid
                    } else {
                        error
                    }
                })?;
                self.nodes[at].rank
            } else {
                0
            };
            Mode::Selected(rank)
        } else {
            let cached = {
                let mut cache = self.checkpoints.lock().expect("operation checkpoints");
                work(cache.0.len() + 1)?;
                after.and_then(|after| {
                    let at = cache
                        .0
                        .iter()
                        .position(|entry| entry.start == start && entry.after == after)?;
                    let entry = cache.0.remove(at)?;
                    let value = entry.state.get();
                    if value.is_some() {
                        cache.0.push_back(entry);
                    }
                    value
                })
            };
            let state = if let Some(state) = cached {
                first = after;
                (*state).clone()
            } else {
                let head = self.nodes[self.position(start, &mut work)?].key;
                let memory = Arc::new(Lease::reserve(4096)?);
                let frontier = Items::retain(vec![head])?;
                State {
                    frontier,
                    pending: Items::retain(vec![])?,
                    at: 0,
                    seen: Seen::empty(self.nodes.len(), &mut work)?,
                    _memory: memory,
                }
            };
            Mode::Historical(state)
        };
        Ok(Cursor {
            graph: self,
            start,
            first,
            last: None,
            mode,
            work,
        })
    }
}
impl<W: FnMut(usize) -> Result<()>> Cursor<W> {
    pub(super) fn next(&mut self) -> Result<Option<OperationId>> {
        let id = if let Some(id) = self.first.take() {
            (self.work)(1)?;
            Some(id)
        } else {
            match &mut self.mode {
                Mode::Selected(rank) => {
                    (self.work)(1)?;
                    let id = self
                        .graph
                        .ordered
                        .get(*rank)
                        .map(|at| self.graph.nodes[*at].key.id);
                    *rank += usize::from(id.is_some());
                    id
                }
                Mode::Historical(state) => state.next(&self.graph, &mut self.work)?,
            }
        };
        self.last = id;
        Ok(id)
    }
    pub(super) fn save(&mut self, after: OperationId) -> Result<()> {
        let Mode::Historical(state) = &self.mode else {
            return Ok(());
        };
        if self.last != Some(after) {
            return Err(Error::Corrupt);
        }
        // Cache only data at the final row, before any probe can advance it.
        let memory = Arc::new(Lease::reserve(4096)?);
        let mut snapshot = state.clone();
        snapshot._memory = memory;
        let state = Retained::new(Arc::new(snapshot));
        let mut cache = self
            .graph
            .checkpoints
            .lock()
            .expect("operation checkpoints");
        (self.work)(cache.0.len() + 1)?;
        cache
            .0
            .retain(|entry| !(entry.start == self.start && entry.after == after));
        if cache.0.len() == 32 {
            cache.0.pop_front();
        }
        cache.0.push_back(Entry {
            start: self.start,
            after,
            state,
        });
        Ok(())
    }
}

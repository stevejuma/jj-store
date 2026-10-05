//! Exact upstream topology and retained selection ranks; no source capability.
use super::*;
use futures::FutureExt as _;
#[cfg(test)]
use futures::StreamExt as _;

impl Graph {
    pub(super) fn neighbors(
        &self,
        node: &Order,
        work: &mut impl FnMut(usize) -> Result<()>,
    ) -> Vec<Result<Order>> {
        let mut read = || -> Result<Vec<Result<Order>>> {
            let position = self.position(node.id, work)?;
            work(self.nodes[position].parents.len() + 1)?;
            self.nodes[position]
                .parents
                .iter()
                .map(|id| Ok(Ok(self.nodes[self.position(*id, work)?].key)))
                .collect()
        };
        read().unwrap_or_else(|error| vec![Err(error)])
    }

    pub(super) fn derive_selected(
        &mut self,
        work: &mut impl FnMut(usize) -> Result<()>,
    ) -> Result<()> {
        // The resident lease includes both order and inverse rank arrays. JJ can
        // retain a complete branch chunk before yielding: charge its scratch too.
        let _scratch = Lease::reserve(self.weight.checked_mul(4).ok_or(Error::Limit)?)?;
        let head = self.nodes[self.position(self.selected, work)?].key;
        let ordered = {
            let mut stream = std::pin::pin!(jj_lib::dag_walk_async::topo_order_reverse_lazy(
                [Ok(head)],
                |node| node.id,
                async |node: &Order| {
                    // Also precharge the rank-array lookup for yielded nodes.
                    match work(self.nodes.len().max(1).ilog2() as usize + 2) {
                        Ok(()) => self.neighbors(node, work),
                        Err(error) => vec![Err(error)],
                    }
                },
                |_| Error::Corrupt,
            ));
            let mut positions = Vec::with_capacity(self.nodes.len());
            // This walker reads only immutable compact nodes and has no I/O or
            // scheduler wait. Fail closed if the pinned algorithm ever suspends.
            while let Some(node) = stream
                .try_next()
                .now_or_never()
                .ok_or(Error::Unavailable)??
            {
                let position = self
                    .nodes
                    .binary_search_by_key(&node.id, |node| node.key.id)
                    .map_err(|_| Error::Corrupt)?;
                positions.push(position);
            }
            positions
        };
        if ordered.len() != self.nodes.len() {
            return Err(Error::Corrupt);
        }
        for (rank, &position) in ordered.iter().enumerate() {
            work(1)?;
            if self.nodes[position].rank != usize::MAX {
                return Err(Error::Corrupt);
            }
            self.nodes[position].rank = rank;
        }
        self.ordered = ordered;
        Ok(())
    }

    #[cfg(test)]
    pub(crate) fn window<'a>(
        self: Arc<Self>,
        start: OperationId,
        after: Option<OperationId>,
        mut work: impl FnMut(usize) -> Result<()> + 'a,
    ) -> Result<impl Stream<Item = Result<OperationId>> + 'a> {
        if start != self.selected {
            return Ok(self.walk(start, work)?.right_stream());
        }
        let first = if let Some(id) = after {
            let position = self.position(id, &mut work).map_err(|error| {
                if error == Error::NotCommitted {
                    Error::Invalid
                } else {
                    error
                }
            })?;
            self.nodes[position].rank
        } else {
            0
        };
        Ok(futures::stream::iter(first..self.ordered.len())
            .map(move |rank| {
                work(1)?;
                Ok(self.nodes[self.ordered[rank]].key.id)
            })
            .left_stream())
    }

    #[cfg(test)]
    pub(crate) fn walk<'a>(
        self: Arc<Self>,
        start: OperationId,
        mut work: impl FnMut(usize) -> Result<()> + 'a,
    ) -> Result<impl Stream<Item = Result<OperationId>> + 'a> {
        let head = self.nodes[self.position(start, &mut work)?].key;
        let memory = Lease::reserve(self.weight.checked_mul(4).ok_or(Error::Limit)?)?;
        Ok(jj_lib::dag_walk_async::topo_order_reverse_lazy(
            [Ok(head)],
            |node| node.id,
            async move |node: &Order| self.neighbors(node, &mut work),
            |_| Error::Corrupt,
        )
        .map_ok(move |node| {
            let _memory = &memory;
            node.id
        }))
    }
}

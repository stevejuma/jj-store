use super::memory::{Budget, Lease, query_capacity};
use crate::work::{
    Result, WorkBrowseQuery,
    projection::{Projection, Selection},
};
use jj_store_wal::publication::WorkStateV1;
use std::{
    collections::VecDeque,
    sync::{Arc, Mutex},
};

pub(in crate::local_repository) struct Selected {
    pub keys: Selection,
    _lease: Lease,
}
pub(in crate::local_repository) struct Data {
    pub state: WorkStateV1,
    pub projection: Projection,
    pub retention: usize,
    queries: Mutex<VecDeque<(WorkBrowseQuery, Arc<Selected>)>>,
    budget: Arc<Budget>,
    _lease: Lease,
}
impl Data {
    pub(super) fn new(
        state: WorkStateV1,
        lease: Lease,
        retention: usize,
        budget: Arc<Budget>,
    ) -> Result<Self> {
        let projection = Projection::new(&state)?;
        Ok(Self {
            state,
            projection,
            retention,
            queries: Mutex::new(VecDeque::new()),
            budget,
            _lease: lease,
        })
    }
    pub fn select(&self, query: &WorkBrowseQuery) -> Result<Arc<Selected>> {
        crate::work::projection::validate_shape(query)?;
        let mut identity = query.clone();
        identity.cursor = None;
        let mut queries = self.queries.lock().unwrap();
        if let Some(index) = queries.iter().position(|(q, _)| q == &identity) {
            let entry = queries.remove(index).unwrap();
            let selected = entry.1.clone();
            queries.push_back(entry);
            return Ok(selected);
        }
        if queries.len() == 8 {
            queries.pop_front();
        }
        let lease = match self.budget.reserve(query_capacity(&self.state)?) {
            Ok(lease) => lease,
            Err(crate::work::Error::Busy) => {
                queries.clear();
                self.budget.reserve(query_capacity(&self.state)?)?
            }
            Err(error) => return Err(error),
        };
        let selected = Arc::new(Selected {
            keys: self.projection.select(&self.state, &identity)?,
            _lease: lease,
        });
        queries.push_back((identity, selected.clone()));
        Ok(selected)
    }
}

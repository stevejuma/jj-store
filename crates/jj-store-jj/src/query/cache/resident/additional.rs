//! Data-only historical residents; private construction files are not retained.
use super::*;

#[derive(Clone)]
pub(in crate::query) struct Data {
    pub index: projection::Prepared,
    pub memory: Arc<super::super::super::memory::Indexes>,
}
impl Data {
    pub fn reader(&self, budget: Arc<Budget>) -> Result<Box<dyn jj_lib::index::ReadonlyIndex>> {
        budget.check()?;
        if self.index.num_commits() as usize > budget.limits.max_index_commits {
            return Err(budget.fail(Error::IndexBudget));
        }
        Ok(self.index.reader_with_memory(budget, self.memory.clone()))
    }
}
struct Additional {
    repository: m::RepositoryId,
    epoch: m::EpochId,
    operation: m::OperationId,
    data: Data,
    weight: u64,
    retained: Arc<AtomicU64>,
}
impl Drop for Additional {
    fn drop(&mut self) {
        self.retained.fetch_sub(self.weight, Ordering::AcqRel);
    }
}
#[derive(Default)]
pub(super) struct AdditionalResidents {
    entries: Mutex<Vec<Arc<super::super::super::memory::Retained<Additional>>>>,
    retained: Arc<AtomicU64>,
}
impl AdditionalResidents {
    pub fn get(&self, access: &dyn ReadAccess, operation: m::OperationId) -> Option<Data> {
        let mut entries = self.entries.lock().unwrap();
        entries.retain(|entry| entry.present());
        let (position, saved) = entries.iter().enumerate().find_map(|(position, entry)| {
            entry
                .get_if(|e| {
                    e.repository == access.repository()
                        && e.epoch == access.epoch()
                        && e.operation == operation
                })
                .map(|e| (position, e))
        })?;
        let entry = entries.remove(position);
        entries.push(entry);
        Some(saved.data.clone())
    }
    pub fn remember(
        &self,
        access: &dyn ReadAccess,
        operation: m::OperationId,
        data: Data,
        policy: Policy,
    ) {
        let weight = data
            .memory
            .directory_bytes()
            .saturating_add(data.index.weight())
            .saturating_add(4096);
        let ceiling = policy
            .max_total_bytes
            .min(super::super::super::memory::LIMIT);
        if weight > ceiling {
            return;
        }
        let mut entries = self.entries.lock().unwrap();
        entries.retain(|entry| {
            entry.present()
                && entry
                    .get_if(|e| {
                        e.repository == access.repository()
                            && e.epoch == access.epoch()
                            && e.operation == operation
                    })
                    .is_none()
        });
        while !entries.is_empty()
            && (entries.len() >= policy.max_entries
                || self.retained.load(Ordering::Acquire) + weight > ceiling)
        {
            entries.remove(0);
        }
        if self.retained.load(Ordering::Acquire) + weight > ceiling {
            return;
        }
        self.retained.fetch_add(weight, Ordering::AcqRel);
        entries.push(super::super::super::memory::Retained::new(Arc::new(
            Additional {
                repository: access.repository(),
                epoch: access.epoch(),
                operation,
                data,
                weight,
                retained: self.retained.clone(),
            },
        )));
    }
}

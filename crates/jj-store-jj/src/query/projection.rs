//! Immutable selected visibility/annotations, with fresh budgets on each reader.
use super::*;
use jj_lib::{
    backend as b,
    default_index::DefaultReadonlyIndex,
    index::{
        ChangeIdIndex, Index as _, IndexError, IndexResult, MutableIndex, ReadonlyIndex,
        ResolvedChangeState, ResolvedChangeTargets,
    },
    object_id::{HexPrefix, ObjectId as _, PrefixResolution},
    op_store::RefTarget,
    revset::ResolvedExpression,
    store::Store,
    view::View,
};
use std::collections::BTreeMap;
use std::path::Path;
mod decorations;
mod filter;
mod mutable;
pub(in crate::query) mod references;
use references::{References, commit_weight, id};
mod changes;
use changes::Changes;

#[derive(Clone)]
pub(super) struct Prepared {
    raw: DefaultReadonlyIndex,
    pub operations: Option<Arc<prepared::operations::Lookup>>,
    decorations: Arc<Decorations>,
    filter: Option<Arc<filter::Filter>>,
}
pub(super) struct Location<'a> {
    pub generation: &'a Path,
    pub path: &'a Path,
    pub selected: m::OperationId,
}
impl Prepared {
    pub async fn build(
        raw: DefaultReadonlyIndex,
        view: &View,
        store: &Arc<Store>,
        access: &dyn ReadAccess,
        budget: &Budget,
        location: Location<'_>,
    ) -> Result<Self> {
        let Location {
            generation,
            path,
            selected,
        } = location;
        let decorations = Decorations::build(&raw, view, store, access, budget).await?;
        let filter = filter::Filter::load(
            path,
            generation,
            selected,
            raw.num_commits() as usize + view.store_view().local_bookmarks.len(),
            budget,
        )?;
        let operations = if path
            .join(prepared::operations::NAME)
            .try_exists()
            .map_err(|_| Error::Unavailable)?
        {
            Some(Arc::new(prepared::operations::Lookup::load(
                path,
                selected,
                budget.limits.max_index_commits as u64,
                |amount| budget.work(amount),
                |amount| budget.read_bytes(amount),
            )?))
        } else {
            None
        };
        Ok(Self {
            raw,
            operations,
            decorations: Arc::new(decorations),
            filter,
        })
    }
    pub fn num_commits(&self) -> u32 {
        self.raw.num_commits()
    }
    pub fn weight(&self) -> u64 {
        self.operations.as_ref().map_or(0, |lookup| lookup.weight())
            + self.decorations.weight
            + if self.filter.is_some() {
                16 * 1024 * 1024 + 65536
            } else {
                0
            }
    }
    pub fn has_reference_filter(&self) -> bool {
        self.filter.is_some()
    }
    pub fn reader_with_memory(
        &self,
        budget: Arc<Budget>,
        memory: Arc<memory::Indexes>,
    ) -> Box<dyn ReadonlyIndex> {
        Box::new(Reader {
            prepared: self.clone(),
            budget,
            _memory: memory,
        })
    }
}
struct Decorations {
    heads: BTreeSet<b::CommitId>,
    visible: prepared::visible::Visible<m::CommitId>,
    changes: Box<dyn ChangeIdIndex>,
    references: References,
    bookmarks: BTreeMap<m::CommitId, Vec<Bookmark>>,
    workspaces: BTreeMap<m::CommitId, Vec<String>>,
    weight: u64,
    _memory: memory::Lease,
}
#[cfg(test)]
mod tests;

pub(super) struct Reader {
    pub prepared: Prepared,
    budget: Arc<Budget>,
    _memory: Arc<memory::Indexes>,
}
impl Reader {
    pub fn commit_flags(&self, commit: m::CommitId, change: m::ChangeId) -> Result<(bool, bool)> {
        let references = &self.prepared.decorations.references;
        self.budget
            .work(references.len().max(1).ilog2() as usize + 2)?;
        let Some(group) = references.get(&format!("change:{change}")) else {
            return Ok((false, false));
        };
        self.budget
            .work(group.targets.len().max(1).ilog2() as usize + 2)?;
        Ok((
            group
                .targets
                .binary_search_by_key(&commit, |row| row.commit)
                .is_ok(),
            group.targets.len() > 1,
        ))
    }
    pub fn log_window(
        &self,
        after: Option<m::CommitId>,
        limit: usize,
    ) -> Result<(&[m::CommitId], Option<m::CommitId>)> {
        self.prepared
            .decorations
            .visible
            .window(after, limit, |amount| self.budget.work(amount))
    }
    pub fn reference_filter(&self) -> Result<&crate::query::prepared::filters::Reader> {
        self.prepared
            .filter
            .as_ref()
            .map(|filter| &filter.reader)
            .ok_or(Error::IndexRebuildRequired)
    }
    pub fn annotations(&self, id: m::CommitId) -> Result<(Vec<Bookmark>, Vec<String>)> {
        self.prepared.decorations.annotations(id, &self.budget)
    }
    pub fn reference_changes(&self) -> &References {
        &self.prepared.decorations.references
    }
    pub fn workspace_names(&self, id: m::CommitId) -> Result<Vec<String>> {
        let names = self
            .prepared
            .decorations
            .workspaces
            .get(&id)
            .map(Vec::as_slice)
            .unwrap_or_default();
        self.budget.work(1 + names.len())?;
        Ok(names.to_vec())
    }
}
impl ReadonlyIndex for Reader {
    fn as_index(&self) -> &dyn jj_lib::index::Index {
        &self.prepared.raw
    }
    fn change_id_index(
        &self,
        heads: &mut dyn Iterator<Item = &b::CommitId>,
    ) -> IndexResult<Box<dyn ChangeIdIndex>> {
        Ok(Box::new(Changes {
            decorations: self.prepared.decorations.clone(),
            budget: self.budget.clone(),
            matches: heads.cloned().collect::<BTreeSet<_>>() == self.prepared.decorations.heads,
        }))
    }
    fn start_modification(&self) -> Box<dyn MutableIndex> {
        Box::new(mutable::Merged::new(self))
    }
}

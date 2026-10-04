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
pub(in crate::query) mod references;
use references::{References, commit_weight, id};
mod changes;
use changes::Changes;

#[derive(Clone)]
pub(super) struct Prepared {
    raw: DefaultReadonlyIndex,
    decorations: Arc<Decorations>,
    filter: Option<Arc<filter::Filter>>,
}
impl Prepared {
    pub async fn build(
        raw: DefaultReadonlyIndex,
        view: &View,
        store: &Arc<Store>,
        access: &dyn ReadAccess,
        budget: &Budget,
        generation: &Path,
        path: &Path,
        selected: m::OperationId,
    ) -> Result<Self> {
        let decorations = Decorations::build(&raw, view, store, access, budget).await?;
        let filter = filter::Filter::load(
            path,
            generation,
            selected,
            raw.num_commits() as usize + view.store_view().local_bookmarks.len(),
            budget,
        )?;
        Ok(Self {
            raw,
            decorations: Arc::new(decorations),
            filter,
        })
    }
    pub fn num_commits(&self) -> u32 {
        self.raw.num_commits()
    }
    pub fn weight(&self) -> u64 {
        self.decorations.weight
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
    visible: Vec<m::CommitId>,
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
    ) -> Box<dyn ChangeIdIndex> {
        Box::new(Changes {
            decorations: self.prepared.decorations.clone(),
            budget: self.budget.clone(),
            matches: heads.cloned().collect::<BTreeSet<_>>() == self.prepared.decorations.heads,
        })
    }
    fn start_modification(&self) -> Box<dyn MutableIndex> {
        ReadonlyIndex::start_modification(&self.prepared.raw)
    }
}

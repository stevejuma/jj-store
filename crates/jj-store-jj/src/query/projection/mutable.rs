//! Adapt owned readonly readers to the pinned default-index merge boundary.
use super::*;
use jj_lib::{commit::Commit, default_index::DefaultMutableIndex, index::Index};

pub(super) struct Merged {
    raw: Box<DefaultMutableIndex>,
    // Merging can retain mapped segments after the original reader is dropped.
    // Keep their admission alive without extending the primary resident's life.
    memory: Vec<Arc<memory::Indexes>>,
}

impl Merged {
    pub(super) fn new(reader: &Reader) -> Self {
        Self {
            raw: ReadonlyIndex::start_modification(&reader.prepared.raw)
                .downcast::<DefaultMutableIndex>()
                .expect("prepared default index must create a default mutable index"),
            memory: vec![reader._memory.clone()],
        }
    }
}

#[async_trait::async_trait]
impl MutableIndex for Merged {
    fn as_index(&self) -> &dyn Index {
        self.raw.as_index()
    }

    fn change_id_index(
        &self,
        heads: &mut dyn Iterator<Item = &b::CommitId>,
    ) -> IndexResult<Box<dyn ChangeIdIndex + '_>> {
        MutableIndex::change_id_index(self.raw.as_ref(), heads)
    }

    async fn add_commit(&mut self, commit: &Commit) -> IndexResult<()> {
        MutableIndex::add_commit(self.raw.as_mut(), commit).await
    }

    fn merge_in(&mut self, other: &dyn ReadonlyIndex) -> IndexResult<()> {
        let raw = if let Some(reader) = other.downcast_ref::<Reader>() {
            if !self
                .memory
                .iter()
                .any(|owner| Arc::ptr_eq(owner, &reader._memory))
            {
                self.memory.push(reader._memory.clone());
            }
            &reader.prepared.raw
        } else if let Some(raw) = other.downcast_ref::<DefaultReadonlyIndex>() {
            raw
        } else {
            return Err(IndexError::Other(
                "incompatible historical index backend".into(),
            ));
        };
        MutableIndex::merge_in(self.raw.as_mut(), raw)
    }
}

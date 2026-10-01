//! Root-pinned ReviewStore reads and persistent typed catalog edits.
use super::*;
use jj_review::{ReviewCatalogKey, ReviewCatalogKind, ReviewIndexPageV1};

#[derive(Clone)]
pub struct ReviewStoreSnapshot {
    coordinator: Coordinator,
    repository: RepositoryId,
    reference: RecordRef,
    root: ReviewRootV1,
    objects: BTreeSet<ReviewObjectRef>,
}
impl ReviewStoreSnapshot {
    pub fn repository_id(&self) -> RepositoryId {
        self.repository
    }
    pub fn object_count(&self) -> usize {
        self.objects.len()
    }
    pub fn reference(&self) -> RecordRef {
        self.reference
    }
    pub fn root(&self) -> &ReviewRootV1 {
        &self.root
    }
    pub async fn lookup(
        &self,
        catalog: ReviewCatalogKind,
        key: ReviewCatalogKey,
    ) -> Result<Option<ReviewObjectRef>> {
        self.coordinator
            .lookup_review_catalog(catalog_root(&self.root, catalog), key)
            .await
    }
    pub async fn read_object(&self, reference: ReviewObjectRef) -> Result<ReviewObjectV1> {
        if !self.objects.contains(&reference) {
            return Err(Error::Stale);
        }
        self.coordinator.read_review_object(reference).await
    }

    pub(super) async fn catalog_entries(
        &self,
        catalog: ReviewCatalogKind,
        limit: usize,
    ) -> Result<BTreeMap<ReviewCatalogKey, ReviewObjectRef>> {
        let mut pending = catalog_root(&self.root, catalog)
            .into_iter()
            .collect::<Vec<_>>();
        let mut seen = BTreeSet::new();
        let mut entries = BTreeMap::new();
        while let Some(reference) = pending.pop() {
            if !seen.insert(reference) {
                continue;
            }
            if seen.len().saturating_add(entries.len()) > limit {
                return Err(Error::Limit("review baseline catalog"));
            }
            match self.coordinator.read_review_page(reference).await? {
                ReviewIndexPageV1::Leaf { entries: leaf, .. } => {
                    for (key, value) in leaf {
                        if entries.insert(key, value).is_some() {
                            return Err(Error::Corrupt("review baseline duplicate"));
                        }
                    }
                }
                ReviewIndexPageV1::Branch { children, .. } => {
                    pending.extend(children.into_values());
                }
            }
        }
        if seen.len().saturating_add(entries.len()) > limit {
            return Err(Error::Limit("review baseline catalog"));
        }
        Ok(entries)
    }
}

impl Coordinator {
    pub async fn review_store(&self, reference: RecordRef) -> Result<ReviewStoreSnapshot> {
        let (root, closure) = self.read_review_root_closure(reference).await?;
        self.verify_delta(None, closure.native).await?;
        Ok(ReviewStoreSnapshot {
            coordinator: self.clone(),
            repository: self.records.repository,
            reference,
            root,
            objects: closure.objects,
        })
    }

    pub async fn stage_review_delta(
        &self,
        base: &ReviewStoreSnapshot,
        delta: &ReviewDeltaV1,
    ) -> Result<RecordRef> {
        let root = ReviewCatalogEditor::new(self)
            .apply_delta(base, delta)
            .await?;
        self.stage_review_root(&root).await
    }
    async fn lookup_review_catalog(
        &self,
        page: Option<ReviewObjectRef>,
        key: ReviewCatalogKey,
    ) -> Result<Option<ReviewObjectRef>> {
        ReviewCatalogEditor::new(self)
            .lookup_review_catalog(page, key)
            .await
    }
    pub(super) async fn read_review_page(
        &self,
        reference: ReviewObjectRef,
    ) -> Result<ReviewIndexPageV1> {
        ReviewCatalogEditor::new(self)
            .read_review_page(reference)
            .await
    }
}

pub(super) fn catalog_root(
    root: &ReviewRootV1,
    catalog: ReviewCatalogKind,
) -> Option<ReviewObjectRef> {
    match catalog {
        ReviewCatalogKind::Threads => root.threads,
        ReviewCatalogKind::Comments => root.comments,
        ReviewCatalogKind::ThreadEvents => root.thread_events,
        ReviewCatalogKind::AnchorRelations => root.anchor_relations,
        ReviewCatalogKind::ArtifactRelations => root.artifact_relations,
        ReviewCatalogKind::Statuses => root.statuses,
    }
}
pub(super) fn set_catalog_root(
    root: &mut ReviewRootV1,
    catalog: ReviewCatalogKind,
    value: Option<ReviewObjectRef>,
) {
    match catalog {
        ReviewCatalogKind::Threads => root.threads = value,
        ReviewCatalogKind::Comments => root.comments = value,
        ReviewCatalogKind::ThreadEvents => root.thread_events = value,
        ReviewCatalogKind::AnchorRelations => root.anchor_relations = value,
        ReviewCatalogKind::ArtifactRelations => root.artifact_relations = value,
        ReviewCatalogKind::Statuses => root.statuses = value,
    }
}

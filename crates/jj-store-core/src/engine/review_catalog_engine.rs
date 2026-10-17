//! Shared persistent catalog editing without publication capability.
use super::{
    review::review_error,
    review_catalog::{catalog_root, set_catalog_root},
    *,
};
use futures::future::BoxFuture;
use jj_review::{ReviewCatalogKey, ReviewCatalogKind, ReviewIndexPageV1};
const LEAF_ENTRIES: usize = 32;
pub struct ReviewCatalogEditor<'a> {
    objects: &'a dyn ReviewObjectIo,
}
impl<'source> ReviewCatalogEditor<'source> {
    pub fn new(objects: &'source dyn ReviewObjectIo) -> Self {
        Self { objects }
    }
    async fn read_review_object(&self, reference: ReviewObjectRef) -> Result<ReviewObjectV1> {
        self.objects.read_review_object(reference).await
    }
    async fn stage_review_object(&self, object: &ReviewObjectV1) -> Result<ReviewObjectRef> {
        self.objects.stage_review_object(object).await
    }
    /// Builds an independent catalog from validated selections without publication.
    pub async fn create_conversion_catalog(
        &self,
        entries: BTreeMap<ReviewCatalogKey, ReviewObjectRef>,
    ) -> Result<Option<ReviewObjectRef>> {
        if entries.len() > 100_000 {
            return Err(Error::Limit("conversion review catalog"));
        }
        self.build_review_catalog(String::new(), entries).await
    }
    pub async fn apply_delta(
        &self,
        base: &dyn ReviewSnapshotRead,
        delta: &ReviewDeltaV1,
    ) -> Result<ReviewRootV1> {
        delta.validate().map_err(review_error)?;
        if base.repository() != self.objects.repository() {
            return Err(Error::Corrupt("review store repository"));
        }
        for edit in &delta.edits {
            if base.lookup(edit.catalog, edit.key).await? != edit.expected {
                return Err(Error::Stale);
            }
        }
        let mut root = base.root().clone();
        for catalog in [
            ReviewCatalogKind::Threads,
            ReviewCatalogKind::Comments,
            ReviewCatalogKind::ThreadEvents,
            ReviewCatalogKind::AnchorRelations,
            ReviewCatalogKind::ArtifactRelations,
            ReviewCatalogKind::Statuses,
        ] {
            let changes: BTreeMap<_, _> = delta
                .edits
                .iter()
                .filter(|edit| edit.catalog == catalog)
                .map(|edit| (edit.key, edit.replacement))
                .collect();
            if changes.is_empty() {
                continue;
            }
            let selected = self
                .rewrite_review_catalog(catalog_root(&root, catalog), String::new(), changes)
                .await?;
            set_catalog_root(&mut root, catalog, selected);
        }
        Ok(root)
    }

    pub async fn lookup_review_catalog(
        &self,
        mut page: Option<ReviewObjectRef>,
        key: ReviewCatalogKey,
    ) -> Result<Option<ReviewObjectRef>> {
        let key_text = key.to_string();
        for _ in 0..=64 {
            let Some(reference) = page else {
                return Ok(None);
            };
            match self.read_review_page(reference).await? {
                ReviewIndexPageV1::Leaf { prefix, entries } => {
                    if !key_text.starts_with(&prefix) {
                        return Err(Error::Corrupt("review catalog traversal"));
                    }
                    return Ok(entries.get(&key).copied());
                }
                ReviewIndexPageV1::Branch { prefix, children } => {
                    if !key_text.starts_with(&prefix) || prefix.len() >= key_text.len() {
                        return Err(Error::Corrupt("review catalog traversal"));
                    }
                    page = children
                        .get(&key_text[prefix.len()..=prefix.len()])
                        .copied();
                }
            }
        }
        Err(Error::Limit("review catalog depth"))
    }

    fn rewrite_review_catalog<'a>(
        &'a self,
        page: Option<ReviewObjectRef>,
        prefix: String,
        changes: BTreeMap<ReviewCatalogKey, Option<ReviewObjectRef>>,
    ) -> BoxFuture<'a, Result<Option<ReviewObjectRef>>> {
        Box::pin(async move {
            let Some(reference) = page else {
                let entries = changes
                    .into_iter()
                    .filter_map(|(key, value)| value.map(|value| (key, value)))
                    .collect();
                return self.build_review_catalog(prefix, entries).await;
            };
            match self.read_review_page(reference).await? {
                ReviewIndexPageV1::Leaf {
                    prefix: stored,
                    mut entries,
                } => {
                    if stored != prefix {
                        return Err(Error::Corrupt("review catalog prefix"));
                    }
                    for (key, value) in changes {
                        match value {
                            Some(value) => {
                                entries.insert(key, value);
                            }
                            None => {
                                entries.remove(&key);
                            }
                        }
                    }
                    self.build_review_catalog(prefix, entries).await
                }
                ReviewIndexPageV1::Branch {
                    prefix: stored,
                    mut children,
                } => {
                    if stored != prefix {
                        return Err(Error::Corrupt("review catalog prefix"));
                    }
                    let mut grouped: BTreeMap<String, BTreeMap<_, _>> = BTreeMap::new();
                    for (key, value) in changes {
                        let text = key.to_string();
                        if !text.starts_with(&prefix) || prefix.len() >= text.len() {
                            return Err(Error::Corrupt("review catalog edit prefix"));
                        }
                        grouped
                            .entry(text[prefix.len()..=prefix.len()].to_owned())
                            .or_default()
                            .insert(key, value);
                    }
                    for (nibble, changes) in grouped {
                        let child = children.get(&nibble).copied();
                        let next = self
                            .rewrite_review_catalog(child, format!("{prefix}{nibble}"), changes)
                            .await?;
                        match next {
                            Some(next) => {
                                children.insert(nibble, next);
                            }
                            None => {
                                children.remove(&nibble);
                            }
                        }
                    }
                    if children.is_empty() {
                        Ok(None)
                    } else {
                        self.stage_review_page(ReviewIndexPageV1::Branch { prefix, children })
                            .await
                            .map(Some)
                    }
                }
            }
        })
    }

    fn build_review_catalog<'a>(
        &'a self,
        prefix: String,
        entries: BTreeMap<ReviewCatalogKey, ReviewObjectRef>,
    ) -> BoxFuture<'a, Result<Option<ReviewObjectRef>>> {
        Box::pin(async move {
            if entries.is_empty() {
                return Ok(None);
            }
            if entries.len() <= LEAF_ENTRIES {
                return self
                    .stage_review_page(ReviewIndexPageV1::Leaf { prefix, entries })
                    .await
                    .map(Some);
            }
            if prefix.len() >= 64 {
                return Err(Error::Corrupt("review catalog key collision"));
            }
            let mut grouped: BTreeMap<String, BTreeMap<_, _>> = BTreeMap::new();
            for (key, value) in entries {
                let text = key.to_string();
                grouped
                    .entry(text[prefix.len()..=prefix.len()].to_owned())
                    .or_default()
                    .insert(key, value);
            }
            let mut children = BTreeMap::new();
            for (nibble, entries) in grouped {
                let child = self
                    .build_review_catalog(format!("{prefix}{nibble}"), entries)
                    .await?
                    .ok_or(Error::Corrupt("empty review catalog child"))?;
                children.insert(nibble, child);
            }
            self.stage_review_page(ReviewIndexPageV1::Branch { prefix, children })
                .await
                .map(Some)
        })
    }

    pub async fn read_review_page(&self, reference: ReviewObjectRef) -> Result<ReviewIndexPageV1> {
        match self.read_review_object(reference).await? {
            ReviewObjectV1::IndexPage(page) => Ok(page),
            _ => Err(Error::Corrupt("review catalog page kind")),
        }
    }
    async fn stage_review_page(&self, page: ReviewIndexPageV1) -> Result<ReviewObjectRef> {
        self.stage_review_object(&ReviewObjectV1::IndexPage(page))
            .await
    }
}

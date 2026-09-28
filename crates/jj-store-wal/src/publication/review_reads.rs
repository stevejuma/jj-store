//! Bounded root-pinned ReviewStore catalog reads.
use super::{review_catalog::catalog_root, *};
use jj_review::{ReviewCatalogKey, ReviewCatalogKind, ReviewIndexPageV1};

const MAX_LIST: usize = 256;
const MAX_CATALOG_PAGES: usize = 16_512;

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct ReviewCatalogPage {
    pub selections: Vec<(ReviewCatalogKey, ReviewObjectRef)>,
    pub next_after: Option<ReviewCatalogKey>,
}

impl ReviewStoreSnapshot {
    pub async fn list_threads(
        &self,
        after: Option<ReviewCatalogKey>,
        limit: usize,
    ) -> Result<ReviewCatalogPage> {
        self.list_catalog(ReviewCatalogKind::Threads, after, limit)
            .await
    }

    pub async fn list_catalog(
        &self,
        catalog: ReviewCatalogKind,
        after: Option<ReviewCatalogKey>,
        limit: usize,
    ) -> Result<ReviewCatalogPage> {
        if limit == 0 || limit > MAX_LIST {
            return Err(Error::Limit("review catalog page"));
        }
        let Some(root) = catalog_root(self.root(), catalog) else {
            return Ok(ReviewCatalogPage {
                selections: Vec::new(),
                next_after: None,
            });
        };
        let after_text = after.map(|value| value.to_string());
        let mut stack = vec![root];
        let mut pages = 0;
        let mut selections = Vec::with_capacity(limit + 1);
        while let Some(reference) = stack.pop() {
            pages += 1;
            if pages > MAX_CATALOG_PAGES {
                return Err(Error::Limit("review catalog enumeration"));
            }
            match self.read_object(reference).await? {
                ReviewObjectV1::IndexPage(ReviewIndexPageV1::Leaf { entries, .. }) => {
                    for (key, value) in entries {
                        if after_text
                            .as_ref()
                            .is_some_and(|after| key.to_string() <= *after)
                        {
                            continue;
                        }
                        selections.push((key, value));
                        if selections.len() > limit {
                            let next_after = selections.get(limit - 1).map(|value| value.0);
                            selections.truncate(limit);
                            return Ok(ReviewCatalogPage {
                                selections,
                                next_after,
                            });
                        }
                    }
                }
                ReviewObjectV1::IndexPage(ReviewIndexPageV1::Branch { children, .. }) => {
                    stack.extend(children.into_values().rev());
                }
                _ => return Err(Error::Corrupt("review catalog enumeration kind")),
            }
        }
        Ok(ReviewCatalogPage {
            selections,
            next_after: None,
        })
    }
}

//! Shared bounded reads over one audited immutable AgentStore root.
use super::*;
use super::{agent::agent_error, agent_catalog::catalog_root};
use jj_agent::{
    AgentCatalogKey, AgentCatalogKind, AgentIndexPageV1, AgentRelationSubjectV1, SourceInstanceId,
};
const MAX_LIST: usize = 256;
const MAX_CATALOG_PAGES: usize = 16_512;
const MAX_STREAM_PAGES: usize = 4096;
const MAX_TRANSCRIPT_BYTES: usize = 8 * 1024 * 1024;
pub struct AgentReader<'a> {
    snapshot: &'a dyn AgentSnapshotRead,
}
impl<'a> AgentReader<'a> {
    pub fn new(snapshot: &'a dyn AgentSnapshotRead) -> Self {
        Self { snapshot }
    }
    fn root(&self) -> &AgentRootV1 {
        self.snapshot.root()
    }
    pub(super) async fn read_object(&self, reference: AgentObjectRef) -> Result<AgentObjectV1> {
        self.snapshot.read_object(reference).await
    }
    pub(super) async fn lookup(
        &self,
        catalog: AgentCatalogKind,
        key: AgentCatalogKey,
    ) -> Result<Option<AgentObjectRef>> {
        self.snapshot.lookup(catalog, key).await
    }
}
impl AgentReader<'_> {
    pub async fn list_sessions(
        &self,
        after: Option<AgentCatalogKey>,
        limit: usize,
    ) -> Result<AgentCatalogPage> {
        self.list_catalog(AgentCatalogKind::Sessions, after, limit)
            .await
    }

    pub async fn list_checkpoints(
        &self,
        after: Option<AgentCatalogKey>,
        limit: usize,
    ) -> Result<AgentCatalogPage> {
        self.list_catalog(AgentCatalogKind::Checkpoints, after, limit)
            .await
    }

    pub async fn list_catalog(
        &self,
        catalog: AgentCatalogKind,
        after: Option<AgentCatalogKey>,
        limit: usize,
    ) -> Result<AgentCatalogPage> {
        valid_limit(limit)?;
        let Some(root) = catalog_root(self.root(), catalog) else {
            return Ok(AgentCatalogPage {
                selections: Vec::new(),
                next_after: None,
            });
        };
        let after_text = after.map(|value| value.to_string());
        let mut stack = vec![(root, String::new())];
        let mut pages = 0;
        let mut selections = Vec::with_capacity(limit + 1);
        while let Some((reference, expected_prefix)) = stack.pop() {
            pages += 1;
            if pages > MAX_CATALOG_PAGES {
                return Err(Error::Limit("agent catalog enumeration"));
            }
            let page = match self.read_object(reference).await? {
                AgentObjectV1::IndexPage(page) => page,
                _ => return Err(Error::Corrupt("agent catalog page kind")),
            };
            match page {
                AgentIndexPageV1::Leaf { prefix, entries } => {
                    if prefix != expected_prefix {
                        return Err(Error::Corrupt("agent catalog prefix"));
                    }
                    selections.extend(entries.into_iter().filter(|(key, _)| {
                        after_text
                            .as_ref()
                            .is_none_or(|after| key.to_string() > *after)
                    }));
                    if selections.len() > limit {
                        break;
                    }
                }
                AgentIndexPageV1::Branch { prefix, children } => {
                    if prefix != expected_prefix {
                        return Err(Error::Corrupt("agent catalog prefix"));
                    }
                    for (nibble, child) in children.into_iter().rev() {
                        let child_prefix = format!("{prefix}{nibble}");
                        if after_text
                            .as_ref()
                            .is_none_or(|after| subtree_has_later(&child_prefix, after))
                        {
                            stack.push((child, child_prefix));
                        }
                    }
                }
            }
        }
        selections.sort_by_key(|(key, _)| *key);
        let next_after = (selections.len() > limit).then(|| selections[limit - 1].0);
        selections.truncate(limit);
        Ok(AgentCatalogPage {
            selections,
            next_after,
        })
    }

    pub async fn relations(
        &self,
        subject: &AgentRelationSubjectV1,
        cursor: Option<AgentRelationCursor>,
        limit: usize,
    ) -> Result<AgentRelationRange> {
        valid_limit(limit)?;
        let first = match cursor {
            Some(cursor) => cursor,
            None => {
                let Some(page) = self
                    .lookup(
                        AgentCatalogKind::Relations,
                        subject.catalog_key().map_err(agent_error)?,
                    )
                    .await?
                else {
                    return Ok(AgentRelationRange {
                        selections: Vec::new(),
                        next: None,
                    });
                };
                AgentRelationCursor { page, offset: 0 }
            }
        };
        let mut current = Some(first);
        let mut seen = BTreeSet::new();
        let mut selections = Vec::with_capacity(limit);
        while let Some(cursor) = current {
            if !seen.insert(cursor.page) || seen.len() > MAX_STREAM_PAGES {
                return Err(Error::Limit("agent relation traversal"));
            }
            let page = match self.read_object(cursor.page).await? {
                AgentObjectV1::RelationPage(page) => page,
                _ => return Err(Error::Corrupt("agent relation page kind")),
            };
            if &page.subject != subject || usize::from(cursor.offset) > page.selections.len() {
                return Err(Error::Stale);
            }
            for (offset, selection) in page
                .selections
                .iter()
                .copied()
                .enumerate()
                .skip(usize::from(cursor.offset))
            {
                if selections.len() == limit {
                    return Ok(AgentRelationRange {
                        selections,
                        next: Some(AgentRelationCursor {
                            page: cursor.page,
                            offset: u16::try_from(offset)
                                .map_err(|_| Error::Limit("agent relation offset"))?,
                        }),
                    });
                }
                selections.push(selection);
            }
            current = page
                .next
                .map(|page| AgentRelationCursor { page, offset: 0 });
        }
        Ok(AgentRelationRange {
            selections,
            next: None,
        })
    }

    pub async fn transcript_range(
        &self,
        manifest: AgentObjectRef,
        source: SourceInstanceId,
        generation: u64,
        after_sequence: Option<u64>,
        limit: usize,
        max_bytes: usize,
    ) -> Result<TranscriptRange> {
        valid_limit(limit)?;
        if max_bytes == 0 || max_bytes > MAX_TRANSCRIPT_BYTES {
            return Err(Error::Limit("agent transcript bytes"));
        }
        let manifest = match self.read_object(manifest).await? {
            AgentObjectV1::TranscriptManifest(value) => value,
            _ => return Err(Error::Corrupt("agent transcript manifest kind")),
        };
        let stream = manifest
            .streams
            .iter()
            .find(|value| {
                value.source_instance_id == source && value.source_generation == generation
            })
            .ok_or(Error::Stale)?;
        super::agent_transcript_read::read_range(
            self.snapshot,
            stream,
            after_sequence,
            limit,
            max_bytes,
        )
        .await
    }
}

fn valid_limit(limit: usize) -> Result<()> {
    if limit == 0 || limit > MAX_LIST {
        return Err(Error::Limit("agent read page"));
    }
    Ok(())
}
fn subtree_has_later(prefix: &str, after: &str) -> bool {
    let mut upper = prefix.to_owned();
    upper.extend(std::iter::repeat_n('f', 64 - prefix.len()));
    upper.as_str() > after
}

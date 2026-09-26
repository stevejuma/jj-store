//! Persistent radix catalog. Only roots selected by a manifest are authoritative.
use crate::{
    Error, Result,
    records::{RecordKind, RecordRef, Records},
};
use futures::future::BoxFuture;
use jj_store_model::RecordDigest;
use serde::{Deserialize, Serialize};
use std::collections::BTreeMap;

const LEAF_CAPACITY: usize = 32;
#[derive(Clone, Serialize, Deserialize)]
#[serde(tag = "type", rename_all = "snake_case", deny_unknown_fields)]
enum Node {
    Leaf {
        prefix: String,
        entries: BTreeMap<RecordDigest, RecordRef>,
    },
    Branch {
        prefix: String,
        children: BTreeMap<String, RecordRef>,
    },
}
#[derive(Clone)]
pub struct Catalog {
    records: Records,
}
/// Shared verification for immutable historical roots against a selected superset.
/// Every distinct page is read once, even across many retained snapshots.
pub(crate) struct CatalogAudit<'a> {
    expected: &'a BTreeMap<RecordDigest, RecordRef>,
    seen: BTreeMap<RecordDigest, String>,
    max_pages: usize,
}
impl<'a> CatalogAudit<'a> {
    pub(crate) fn new(expected: &'a BTreeMap<RecordDigest, RecordRef>, max_pages: usize) -> Self {
        Self {
            expected,
            seen: BTreeMap::new(),
            max_pages,
        }
    }
    pub(crate) async fn verify(
        &mut self,
        catalog: &Catalog,
        root: Option<RecordRef>,
    ) -> Result<()> {
        let mut stack = root
            .into_iter()
            .map(|r| (r, String::new()))
            .collect::<Vec<_>>();
        while let Some((reference, prefix)) = stack.pop() {
            if reference.kind != RecordKind::Catalog || reference.attempt.is_some() {
                return Err(Error::Corrupt("historical catalog reference"));
            }
            if let Some(previous) = self.seen.get(&reference.digest) {
                if previous != &prefix {
                    return Err(Error::Corrupt("historical catalog prefix"));
                }
                continue;
            }
            if self.seen.len() >= self.max_pages {
                return Err(Error::Limit("retained catalog pages"));
            }
            match catalog.load(reference, &prefix).await? {
                Node::Leaf { entries, .. } => {
                    if entries.iter().any(|(k, v)| self.expected.get(k) != Some(v)) {
                        return Err(Error::Corrupt("historical catalog is not retained"));
                    }
                }
                Node::Branch { children, .. } => stack.extend(
                    children
                        .into_iter()
                        .map(|(n, r)| (r, format!("{prefix}{n}"))),
                ),
            }
            self.seen.insert(reference.digest, prefix);
        }
        Ok(())
    }
}
impl Catalog {
    pub fn new(records: Records) -> Self {
        Self { records }
    }
    async fn load(&self, reference: RecordRef, prefix: &str) -> Result<Node> {
        let node: Node = self.records.get(reference, RecordKind::Catalog).await?;
        match &node {
            Node::Leaf { prefix: p, entries } => {
                if p != prefix
                    || entries.len() > LEAF_CAPACITY
                    || entries.keys().any(|k| !k.to_string().starts_with(prefix))
                {
                    return Err(Error::Corrupt("catalog leaf"));
                }
            }
            Node::Branch {
                prefix: p,
                children,
            } => {
                if p != prefix
                    || prefix.len() >= 64
                    || children.is_empty()
                    || children.len() > 16
                    || children.keys().any(|n| {
                        n.len() != 1
                            || !n
                                .bytes()
                                .all(|b| b.is_ascii_digit() || (b'a'..=b'f').contains(&b))
                    })
                {
                    return Err(Error::Corrupt("catalog branch"));
                }
            }
        }
        Ok(node)
    }
    pub async fn get(
        &self,
        root: Option<RecordRef>,
        key: RecordDigest,
    ) -> Result<Option<RecordRef>> {
        let mut current = root;
        let text = key.to_string();
        let mut prefix = String::new();
        while let Some(reference) = current {
            match self.load(reference, &prefix).await? {
                Node::Leaf { entries, .. } => return Ok(entries.get(&key).copied()),
                Node::Branch { children, .. } => {
                    let nibble = &text[prefix.len()..prefix.len() + 1];
                    current = children.get(nibble).copied();
                    prefix.push_str(nibble);
                }
            }
        }
        Ok(None)
    }
    pub async fn update(
        &self,
        root: Option<RecordRef>,
        changes: BTreeMap<RecordDigest, RecordRef>,
    ) -> Result<Option<RecordRef>> {
        if changes.is_empty() {
            return Ok(root);
        }
        Ok(Some(self.update_at(root, changes, String::new()).await?))
    }
    fn update_at(
        &self,
        root: Option<RecordRef>,
        changes: BTreeMap<RecordDigest, RecordRef>,
        prefix: String,
    ) -> BoxFuture<'_, Result<RecordRef>> {
        Box::pin(async move {
            let node = if let Some(root) = root {
                self.load(root, &prefix).await?
            } else {
                Node::Leaf {
                    prefix: prefix.clone(),
                    entries: BTreeMap::new(),
                }
            };
            let (mut children, changes) = match node {
                Node::Leaf { mut entries, .. } => {
                    entries.extend(changes);
                    if entries.len() <= LEAF_CAPACITY {
                        return self
                            .records
                            .put(RecordKind::Catalog, None, &Node::Leaf { prefix, entries })
                            .await;
                    }
                    (BTreeMap::new(), entries)
                }
                Node::Branch { children, .. } => (children, changes),
            };
            if prefix.len() >= 64 {
                return Err(Error::Corrupt("catalog digest collision"));
            }
            let mut groups: BTreeMap<String, BTreeMap<RecordDigest, RecordRef>> = BTreeMap::new();
            for (key, value) in changes {
                let text = key.to_string();
                if !text.starts_with(&prefix) {
                    return Err(Error::Corrupt("catalog update prefix"));
                }
                groups
                    .entry(text[prefix.len()..prefix.len() + 1].to_owned())
                    .or_default()
                    .insert(key, value);
            }
            for (nibble, changes) in groups {
                let child = self
                    .update_at(
                        children.get(&nibble).copied(),
                        changes,
                        format!("{prefix}{nibble}"),
                    )
                    .await?;
                children.insert(nibble, child);
            }
            self.records
                .put(
                    RecordKind::Catalog,
                    None,
                    &Node::Branch { prefix, children },
                )
                .await
        })
    }
    /// Explicit bounded enumeration for bootstrap/restore. Never bucket LIST.
    pub async fn entries(
        &self,
        root: Option<RecordRef>,
        limit: usize,
    ) -> Result<BTreeMap<RecordDigest, RecordRef>> {
        let mut out = BTreeMap::new();
        let mut stack = Vec::new();
        if let Some(root) = root {
            stack.push((root, String::new()));
        }
        let mut pages = 0usize;
        while let Some((reference, prefix)) = stack.pop() {
            pages += 1;
            if pages > limit.saturating_mul(65).saturating_add(1) {
                return Err(Error::Limit("catalog pages"));
            }
            match self.load(reference, &prefix).await? {
                Node::Leaf { entries, .. } => {
                    if out.len().saturating_add(entries.len()) > limit {
                        return Err(Error::Limit("catalog entries"));
                    }
                    out.extend(entries);
                }
                Node::Branch { children, .. } => {
                    stack.extend(
                        children
                            .into_iter()
                            .rev()
                            .map(|(n, r)| (r, format!("{prefix}{n}"))),
                    );
                }
            }
        }
        Ok(out)
    }
}

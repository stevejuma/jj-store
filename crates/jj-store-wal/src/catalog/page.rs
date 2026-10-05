//! Exclusive keyset scans. The selected root remains the caller's authority.
use super::*;
use serde::de::DeserializeOwned;

pub struct Page {
    pub entries: BTreeMap<RecordDigest, RecordRef>,
    pub next: Option<RecordDigest>,
}
/// Cumulative admission for catalog paths and their descriptors. Each encoded
/// record is bounded before decoding, including the one-row lookahead.
pub struct ScanBudget {
    reads: usize,
    bytes: u64,
}
impl ScanBudget {
    pub fn new(reads: usize, bytes: u64) -> Self {
        Self { reads, bytes }
    }
    pub(crate) async fn get<T: Serialize + DeserializeOwned>(
        &mut self,
        records: &Records,
        reference: RecordRef,
        kind: RecordKind,
    ) -> Result<T> {
        if self.reads == 0 || self.bytes == 0 {
            return Err(Error::Limit("catalog scan"));
        }
        self.reads -= 1;
        let (value, bytes) = records.get_bounded(reference, kind, self.bytes).await?;
        self.bytes = self
            .bytes
            .checked_sub(bytes)
            .ok_or(Error::Limit("catalog bytes"))?;
        Ok(value)
    }
}
impl Catalog {
    /// Reads only the window's radix paths. The optional anchor need not exist;
    /// higher-layer signed continuations establish query and snapshot binding.
    pub async fn page(
        &self,
        root: Option<RecordRef>,
        after: Option<RecordDigest>,
        limit: usize,
        budget: &mut ScanBudget,
    ) -> Result<Page> {
        if !(1..=256).contains(&limit) {
            return Err(Error::Limit("catalog scan size"));
        }
        let after_text = after.map(|key| key.to_string());
        let mut stack = root
            .into_iter()
            .map(|r| (r, String::new()))
            .collect::<Vec<_>>();
        let mut entries = BTreeMap::new();
        while let Some((reference, prefix)) = stack.pop() {
            if after_text
                .as_ref()
                .is_some_and(|key| prefix.as_str() < &key[..prefix.len()])
            {
                continue;
            }
            let node = budget
                .get(&self.records, reference, RecordKind::Catalog)
                .await?;
            Self::validate(&node, &prefix)?;
            match node {
                Node::Leaf { entries: leaf, .. } => {
                    for (key, value) in leaf {
                        if after.is_some_and(|after| key <= after) {
                            continue;
                        }
                        if entries.len() == limit {
                            let next = entries.last_key_value().map(|(key, _)| *key);
                            return Ok(Page { entries, next });
                        }
                        entries.insert(key, value);
                    }
                }
                Node::Branch { children, .. } => stack.extend(
                    children
                        .into_iter()
                        .rev()
                        .map(|(n, r)| (r, format!("{prefix}{n}"))),
                ),
            }
        }
        Ok(Page {
            entries,
            next: None,
        })
    }
}

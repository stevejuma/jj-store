//! Bounded additions between immutable roots. No bucket listing or full maps.
use super::*;

enum Side {
    Stored(RecordRef),
    // A leaf partition produced by a shape transition, at most32 entries.
    Leaf(BTreeMap<RecordDigest, RecordRef>),
}

impl Side {
    async fn load(self, catalog: &Catalog, prefix: &str, budget: &mut ScanBudget) -> Result<Node> {
        let node = match self {
            Self::Stored(reference) => {
                budget
                    .get(&catalog.records, reference, RecordKind::Catalog)
                    .await?
            }
            Self::Leaf(entries) => Node::Leaf {
                prefix: prefix.to_owned(),
                entries,
            },
        };
        Catalog::validate(&node, prefix)?;
        Ok(node)
    }
}
fn side(reference: Option<RecordRef>) -> Result<Side> {
    match reference {
        Some(reference) if reference.kind != RecordKind::Catalog || reference.attempt.is_some() => {
            Err(Error::Corrupt("catalog comparison reference"))
        }
        Some(reference) => Ok(Side::Stored(reference)),
        None => Ok(Side::Leaf(BTreeMap::new())),
    }
}
fn children(node: Node, depth: usize) -> Result<BTreeMap<String, Side>> {
    match node {
        Node::Branch { children, .. } => children
            .into_iter()
            .map(|(nibble, reference)| Ok((nibble, side(Some(reference))?)))
            .collect(),
        Node::Leaf { entries, .. } => {
            if depth >= 64 {
                return Err(Error::Corrupt("catalog comparison depth"));
            }
            let mut groups = BTreeMap::<String, BTreeMap<RecordDigest, RecordRef>>::new();
            for (key, value) in entries {
                let text = key.to_string();
                groups
                    .entry(text[depth..depth + 1].to_owned())
                    .or_default()
                    .insert(key, value);
            }
            Ok(groups
                .into_iter()
                .map(|(nibble, entries)| (nibble, Side::Leaf(entries)))
                .collect())
        }
    }
}
impl Catalog {
    /// Exclusive addition window. Consuming all pages from None proves exact
    /// baseline retention; an anchored or partial page certifies only its walk.
    pub async fn additions_page(
        &self,
        before: Option<RecordRef>,
        after: Option<RecordRef>,
        anchor: Option<RecordDigest>,
        limit: usize,
        budget: &mut ScanBudget,
    ) -> Result<Page> {
        if !(1..=256).contains(&limit) {
            return Err(Error::Limit("catalog scan size"));
        }
        let anchor_text = anchor.map(|key| key.to_string());
        let mut stack = vec![(side(before)?, side(after)?, String::new())];
        let mut entries = BTreeMap::new();
        while let Some((before, after, prefix)) = stack.pop() {
            if anchor_text
                .as_ref()
                .is_some_and(|key| prefix.as_str() < &key[..prefix.len()])
                || matches!((&before, &after), (Side::Stored(a), Side::Stored(b)) if a == b)
            {
                continue;
            }
            let pair = async {
                Ok((
                    before.load(self, &prefix, budget).await?,
                    after.load(self, &prefix, budget).await?,
                ))
            }
            .await;
            let (before, after) = match pair {
                Ok(pair) => pair,
                Err(error) if ScanBudget::admission(&error) && !entries.is_empty() => {
                    let next = entries.last_key_value().map(|(key, _)| *key);
                    return Ok(Page { entries, next });
                }
                Err(error) => return Err(error),
            };
            match (before, after) {
                (Node::Leaf { entries: base, .. }, Node::Leaf { entries: new, .. }) => {
                    if base.iter().any(|(key, value)| new.get(key) != Some(value)) {
                        return Err(Error::Corrupt("catalog base not retained"));
                    }
                    for (key, value) in new {
                        if anchor.is_some_and(|anchor| key <= anchor) || base.contains_key(&key) {
                            continue;
                        }
                        if entries.len() == limit {
                            let next = entries.last_key_value().map(|(key, _)| *key);
                            return Ok(Page { entries, next });
                        }
                        entries.insert(key, value);
                    }
                }
                (before, after) => {
                    let mut pairs = children(before, prefix.len())?
                        .into_iter()
                        .map(|(nibble, base)| (nibble, (Some(base), None)))
                        .collect::<BTreeMap<_, _>>();
                    for (nibble, new) in children(after, prefix.len())? {
                        pairs.entry(nibble).or_default().1 = Some(new);
                    }
                    if stack.len().saturating_add(pairs.len()) > 1024 {
                        return Err(Error::Limit("catalog comparison frontier"));
                    }
                    for (nibble, (base, new)) in pairs.into_iter().rev() {
                        stack.push((
                            base.unwrap_or_else(|| Side::Leaf(BTreeMap::new())),
                            new.unwrap_or_else(|| Side::Leaf(BTreeMap::new())),
                            format!("{prefix}{nibble}"),
                        ));
                    }
                }
            }
        }
        Ok(Page {
            entries,
            next: None,
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use jj_store_model::RepositoryId;
    use std::sync::Arc;

    fn entry(n: u8) -> (RecordDigest, RecordRef) {
        let mut bytes = [0; 32];
        bytes[0] = n;
        let digest = RecordDigest::new(bytes);
        (
            digest,
            RecordRef {
                kind: RecordKind::Inventory,
                digest,
                attempt: None,
            },
        )
    }
    fn catalog() -> Catalog {
        Catalog::new(Records::new(
            Arc::new(jj_store_object::memory::MemoryStore::default()),
            RepositoryId::new([7; 16]),
        ))
    }
    fn budget() -> ScanBudget {
        ScanBudget::new(768, 8 * 1024 * 1024)
    }

    #[tokio::test]
    async fn additions_keep_shape_transitions_and_exclusive_missing_anchors_exact() {
        let catalog = catalog();
        let base = (0..32).map(entry).collect::<BTreeMap<_, _>>();
        let before = catalog.update(None, base.clone()).await.unwrap();
        let additions = (32..73).map(entry).collect::<BTreeMap<_, _>>();
        let after = catalog.update(before, additions.clone()).await.unwrap();
        let mut anchor = None;
        let mut actual = BTreeMap::new();
        loop {
            let page = catalog
                .additions_page(before, after, anchor, 3, &mut budget())
                .await
                .unwrap();
            assert!(!page.entries.is_empty() || page.next.is_none());
            actual.extend(page.entries);
            anchor = page.next;
            if anchor.is_none() {
                break;
            }
        }
        assert_eq!(actual, additions);
        for n in 0..=74 {
            let mut missing = entry(n).0.as_bytes().to_owned();
            missing[31] = 1;
            let anchor = RecordDigest::new(missing);
            let page = catalog
                .additions_page(before, after, Some(anchor), 256, &mut budget())
                .await
                .unwrap();
            assert_eq!(
                page.entries,
                additions
                    .iter()
                    .filter(|(key, _)| **key > anchor)
                    .map(|(key, value)| (*key, *value))
                    .collect()
            );
            assert!(page.next.is_none());
        }
        assert_eq!(catalog.entries(before, 32).await.unwrap(), base);
        let empty = catalog
            .additions_page(after, after, None, 256, &mut ScanBudget::new(0, 0))
            .await
            .unwrap();
        assert!(empty.entries.is_empty() && empty.next.is_none());
        let cold = catalog
            .additions_page(None, after, None, 256, &mut budget())
            .await
            .unwrap();
        assert_eq!(cold.entries, base.into_iter().chain(additions).collect());
    }

    #[tokio::test]
    async fn additions_reject_replacements_missing_branches_and_invalid_references() {
        let catalog = catalog();
        let before = catalog
            .update(None, (0..33).map(entry).collect())
            .await
            .unwrap();
        let missing = catalog
            .update(None, (1..33).map(entry).collect())
            .await
            .unwrap();
        assert!(matches!(
            catalog
                .additions_page(before, missing, None, 256, &mut budget())
                .await,
            Err(Error::Corrupt("catalog base not retained"))
        ));
        assert!(matches!(
            catalog
                .additions_page(before, None, None, 256, &mut budget())
                .await,
            Err(Error::Corrupt("catalog base not retained"))
        ));
        let replaced = catalog
            .update(before, BTreeMap::from([(entry(17).0, entry(99).1)]))
            .await
            .unwrap();
        assert!(matches!(
            catalog
                .additions_page(before, replaced, None, 256, &mut budget())
                .await,
            Err(Error::Corrupt("catalog base not retained"))
        ));
        assert!(matches!(
            catalog
                .additions_page(Some(entry(0).1), Some(entry(0).1), None, 256, &mut budget())
                .await,
            Err(Error::Corrupt("catalog comparison reference"))
        ));
        assert!(matches!(
            catalog
                .additions_page(before, before, None, 0, &mut budget())
                .await,
            Err(Error::Limit("catalog scan size"))
        ));
    }

    #[tokio::test]
    async fn additions_admit_reads_and_bytes_and_resume_after_nonempty_prefix() {
        let catalog = catalog();
        let before = catalog
            .update(None, BTreeMap::from([entry(0)]))
            .await
            .unwrap();
        let after = catalog
            .update(before, (1..34).map(entry).collect())
            .await
            .unwrap();
        assert!(matches!(
            catalog
                .additions_page(
                    before,
                    after,
                    None,
                    256,
                    &mut ScanBudget::new(1, 1024 * 1024)
                )
                .await,
            Err(Error::Limit(_))
        ));
        assert!(
            catalog
                .additions_page(before, after, None, 256, &mut ScanBudget::new(768, 1))
                .await
                .is_err()
        );
        // One baseline leaf, one new branch, then its first populated leaf.
        let first = catalog
            .additions_page(
                before,
                after,
                None,
                256,
                &mut ScanBudget::new(3, 1024 * 1024),
            )
            .await
            .unwrap();
        assert!(!first.entries.is_empty());
        assert_eq!(
            first.next,
            first.entries.last_key_value().map(|(key, _)| *key)
        );
        let rest = catalog
            .additions_page(before, after, first.next, 256, &mut budget())
            .await
            .unwrap();
        assert!(rest.next.is_none());
        assert_eq!(
            first
                .entries
                .into_iter()
                .chain(rest.entries)
                .collect::<BTreeMap<_, _>>(),
            (1..34).map(entry).collect()
        );
    }
}

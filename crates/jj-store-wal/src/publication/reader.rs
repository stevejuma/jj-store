//! Read-only manifest-selected log and inventory traversal for durable consumers.
use super::*;

#[cfg(test)]
#[path = "reader_tests.rs"]
mod tests;

/// A bounded catalog window, in digest order. It is not a complete inventory
/// audit; publication/recovery certified the selected root's closure.
pub struct InventoryPage {
    pub keys: Vec<ObjectKey>,
    pub next: Option<RecordDigest>,
}
pub(super) async fn inventory_page(
    store: Arc<dyn ObjectStore>,
    repository: RepositoryId,
    root: Option<RecordRef>,
    after: Option<RecordDigest>,
    limit: usize,
) -> Result<InventoryPage> {
    if root.is_none() {
        return Err(Error::Corrupt("missing committed inventory"));
    }
    let records = Records::new(store, repository);
    let catalog = Catalog::new(records.clone());
    // Reserve descriptor work independently so a catalog frontier cannot spend
    // the reads needed to hydrate its admitted rows. Valid16MiB descriptors fit
    // a fresh window. The combined encoded-byte ceiling remains32MiB.
    let mut paths = crate::catalog::ScanBudget::new(768, 8 * 1024 * 1024);
    let page = catalog.page(root, after, limit, &mut paths).await?;
    let used = 8 * 1024 * 1024 - paths.remaining_bytes();
    inventory_window(&records, page, used).await
}

pub(super) async fn inventory_additions_page(
    store: Arc<dyn ObjectStore>,
    repository: RepositoryId,
    before: Option<RecordRef>,
    after: Option<RecordRef>,
    anchor: Option<RecordDigest>,
    limit: usize,
) -> Result<InventoryPage> {
    if before.is_none() || after.is_none() {
        return Err(Error::Corrupt("missing committed inventory"));
    }
    let records = Records::new(store, repository);
    let catalog = Catalog::new(records.clone());
    let mut paths = crate::catalog::ScanBudget::new(768, 8 * 1024 * 1024);
    let page = catalog
        .additions_page(before, after, anchor, limit, &mut paths)
        .await?;
    let used = 8 * 1024 * 1024 - paths.remaining_bytes();
    inventory_window(&records, page, used).await
}

async fn inventory_window(
    records: &Records,
    page: crate::catalog::Page,
    used: u64,
) -> Result<InventoryPage> {
    let mut budget = crate::catalog::ScanBudget::new(256, 32 * 1024 * 1024 - used);
    let mut keys = Vec::with_capacity(page.entries.len());
    let mut last = None;
    for (key, reference) in page.entries {
        let node: VerifiedNode = match budget.get(records, reference, RecordKind::Inventory).await {
            Ok(node) => node,
            Err(error) if crate::catalog::ScanBudget::admission(&error) && !keys.is_empty() => {
                return Ok(InventoryPage { keys, next: last });
            }
            Err(error) => return Err(error),
        };
        if object_key(node.key) != key {
            return Err(Error::Corrupt("inventory catalog key"));
        }
        keys.push(node.key);
        last = Some(key);
    }
    Ok(InventoryPage {
        keys,
        next: page.next,
    })
}

/// Verify one descriptor against the selected immutable catalog root.
/// Publication/recovery establish complete closure. Readers separately verify
/// each dependency they follow: enumerating all dependencies here would turn a
/// single view read into one lookup per bookmark, before reference pagination.
pub(super) async fn inventory_member(
    store: Arc<dyn ObjectStore>,
    repository: RepositoryId,
    root: Option<RecordRef>,
    key: ObjectKey,
) -> Result<Option<VerifiedNode>> {
    if root.is_none() {
        return Err(Error::Corrupt("missing committed inventory"));
    }
    let records = Records::new(store, repository);
    let catalog = Catalog::new(records.clone());
    let Some(reference) = catalog.get(root, object_key(key)).await? else {
        return Ok(None);
    };
    let node: VerifiedNode = records.get(reference, RecordKind::Inventory).await?;
    if node.key != key {
        return Err(Error::Corrupt("inventory catalog key"));
    }
    Ok(Some(node))
}

/// Constructed only by selected history traversal, never by deserializing a cursor.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct SelectedRecord {
    reference: RecordRef,
    transaction: PublishedTransaction,
    principal: PrincipalId,
}
impl SelectedRecord {
    pub fn reference(&self) -> RecordRef {
        self.reference
    }
    pub fn transaction(&self) -> &PublishedTransaction {
        &self.transaction
    }
    /// Authenticated publication actor from the selected, verified receipt.
    pub fn principal(&self) -> PrincipalId {
        self.principal
    }
    pub(super) fn into_transaction(self) -> PublishedTransaction {
        self.transaction
    }
    pub async fn inventory(
        &self,
        store: Arc<dyn ObjectStore>,
        limit: usize,
    ) -> Result<Vec<VerifiedNode>> {
        inventory(
            store,
            self.transaction.repository,
            self.transaction.inventory,
            &self.transaction.after.heads,
            limit,
        )
        .await
    }
}
pub(super) async fn inventory(
    store: Arc<dyn ObjectStore>,
    repository: RepositoryId,
    root: Option<RecordRef>,
    heads: &BTreeSet<OperationId>,
    limit: usize,
) -> Result<Vec<VerifiedNode>> {
    if root.is_none() {
        return Err(Error::Corrupt("missing committed inventory"));
    }
    let records = Records::new(store, repository);
    let catalog = Catalog::new(records.clone());
    let entries = catalog.entries(root, limit).await?;
    let mut out = Vec::new();
    for (key, reference) in entries {
        let node: VerifiedNode = records.get(reference, RecordKind::Inventory).await?;
        if object_key(node.key) != key {
            return Err(Error::Corrupt("inventory catalog key"));
        }
        out.push(node);
    }
    let keys = out.iter().map(|n| n.key).collect::<BTreeSet<_>>();
    if heads
        .iter()
        .any(|id| !keys.contains(&ObjectKey::Operation(*id)))
        || out
            .iter()
            .any(|n| n.dependencies.iter().any(|k| !keys.contains(k)))
    {
        return Err(Error::Corrupt("incomplete committed inventory"));
    }
    Ok(out)
}
pub(super) async fn selected(
    snapshot: &ManifestSnapshot,
    store: Arc<dyn ObjectStore>,
    after: WalPosition,
    limit: usize,
) -> Result<Vec<SelectedRecord>> {
    let manifest = &snapshot.manifest;
    manifest.validate(manifest.repository)?;
    let records = Records::new(store, manifest.repository);
    let catalog = Catalog::new(records.clone());
    if limit > 1024 || after.0 > manifest.published.0 {
        return Err(Error::Limit("committed page"));
    }
    let count = (manifest.published.0 - after.0).min(limit as u64);
    let mut out = Vec::new();
    let mut previous = if after.0 == 0 {
        None
    } else {
        Some(
            catalog
                .get(manifest.history, sequence_key(after))
                .await?
                .ok_or(Error::Corrupt("historical position missing"))?,
        )
    };
    let (mut previous_snapshot, mut previous_domains) = if let Some(reference) = previous {
        let prior: PublishedTransaction = records.get(reference, RecordKind::Wal).await?;
        if prior.repository != manifest.repository
            || prior.after.epoch != manifest.epoch
            || prior.after.position != after
        {
            return Err(Error::Corrupt("previous log position"));
        }
        let domains = if manifest.format >= 2 && after.0 >= manifest.domain_history_start {
            Some(
                prior
                    .after_domains
                    .ok_or(Error::Corrupt("domain history missing"))?,
            )
        } else {
            if prior.before_domains.is_some() || prior.after_domains.is_some() {
                return Err(Error::Corrupt("unexpected legacy domain history"));
            }
            None
        };
        (prior.after, domains)
    } else {
        (
            Snapshot {
                epoch: manifest.epoch,
                position: WalPosition(0),
                code_state_version: 0,
                heads: BTreeSet::from([OperationId::new([0; 64])]),
            },
            None,
        )
    };
    for offset in 1..=count {
        let position = WalPosition(after.0 + offset);
        let reference = catalog
            .get(manifest.history, sequence_key(position))
            .await?
            .ok_or(Error::Corrupt("committed position missing"))?;
        let transaction: PublishedTransaction = records.get(reference, RecordKind::Wal).await?;
        if transaction.repository != manifest.repository
            || transaction.after.epoch != manifest.epoch
            || transaction.before.epoch != manifest.epoch
            || transaction.after.position != position
            || transaction.before != previous_snapshot
            || transaction.before.position.0.checked_add(1) != Some(position.0)
            || transaction.previous != previous
            || Some(transaction.attempt) != reference.attempt
        {
            return Err(Error::Corrupt("committed log chain"));
        }
        if manifest.format >= 2 && position.0 >= manifest.domain_history_start {
            let after_domains = transaction
                .after_domains
                .ok_or(Error::Corrupt("domain history missing"))?;
            match (previous_domains, transaction.before_domains) {
                (Some(prior), Some(before)) if prior == before => {}
                (None, None)
                    if transaction.kind == SemanticKind::CollaborationEnabled
                        && after_domains.agent.version == 0
                        && after_domains.review.version == 0 => {}
                (None, Some(before)) if before.agent.version == 0 && before.review.version == 0 => {
                }
                _ => return Err(Error::Corrupt("domain history chain")),
            }
            previous_domains = Some(after_domains);
        } else if transaction.before_domains.is_some() || transaction.after_domains.is_some() {
            return Err(Error::Corrupt("unexpected legacy domain history"));
        }
        let receipt: Receipt = records
            .get(transaction.receipt, RecordKind::Receipt)
            .await?;
        if receipt.accepted != transaction.after {
            return Err(Error::Corrupt("log receipt snapshot"));
        }
        previous_snapshot = transaction.after.clone();
        previous = Some(reference);
        out.push(SelectedRecord {
            reference,
            transaction,
            principal: receipt.principal,
        });
    }
    if after.0 + count == manifest.published.0
        && (previous != manifest.tail
            || previous_snapshot != manifest.snapshot()
            || (manifest.format >= 2
                && manifest.published.0 >= manifest.domain_history_start
                && previous_domains != manifest.domains()))
    {
        return Err(Error::Corrupt("manifest tail disagreement"));
    }
    Ok(out)
}

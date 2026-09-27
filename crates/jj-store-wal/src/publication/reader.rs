//! Read-only manifest-selected log and inventory traversal for durable consumers.
use super::*;

/// Constructed only by selected history traversal, never by deserializing a cursor.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct SelectedRecord {
    reference: RecordRef,
    transaction: PublishedTransaction,
}
impl SelectedRecord {
    pub fn reference(&self) -> RecordRef {
        self.reference
    }
    pub fn transaction(&self) -> &PublishedTransaction {
        &self.transaction
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
    let mut previous_snapshot = if let Some(reference) = previous {
        let prior: PublishedTransaction = records.get(reference, RecordKind::Wal).await?;
        if prior.repository != manifest.repository
            || prior.after.epoch != manifest.epoch
            || prior.after.position != after
        {
            return Err(Error::Corrupt("previous log position"));
        }
        prior.after
    } else {
        Snapshot {
            epoch: manifest.epoch,
            position: WalPosition(0),
            heads: BTreeSet::from([OperationId::new([0; 64])]),
        }
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
        });
    }
    if after.0 + count == manifest.published.0
        && (previous != manifest.tail || previous_snapshot != manifest.snapshot())
    {
        return Err(Error::Corrupt("manifest tail disagreement"));
    }
    Ok(out)
}

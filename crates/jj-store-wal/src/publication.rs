//! One manifest CAS selects immutable native inventory, history and receipts.
use crate::{
    Error, Result,
    catalog::{Catalog, CatalogAudit},
    inventory::{Budget, DependencySource, Representation, VerifiedNode},
    records::{self, RecordKind, RecordRef, Records},
};
use jj_store_model::{
    AttemptId, EpochId, IntentDigest, ObjectKey, OperationId, PrincipalId, PublicationId,
    RecordDigest, RepositoryId, RequestId, WalPosition,
};
use jj_store_object::{Condition, Key, ObjectStore, StagedUpload, VersionToken, WriteOutcome};
use serde::{Deserialize, Serialize};
use sha2::{Digest as _, Sha256};
use std::{
    collections::{BTreeMap, BTreeSet},
    sync::Arc,
    time::Duration,
};

const MAX_HEADS: usize = 256;
const MAX_MANIFEST: u64 = 128 * 1024;
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct Snapshot {
    pub epoch: EpochId,
    pub position: WalPosition,
    pub heads: BTreeSet<OperationId>,
}
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct Manifest {
    pub format: u32,
    pub repository: RepositoryId,
    pub epoch: EpochId,
    pub revision: u64,
    pub published: WalPosition,
    pub heads: BTreeSet<OperationId>,
    pub tail: Option<RecordRef>,
    pub inventory: Option<RecordRef>,
    pub receipts: Option<RecordRef>,
    pub history: Option<RecordRef>,
    pub checkpoint: Option<RecordRef>,
    pub initial_settings: RecordRef,
    pub settings: RecordRef,
}
impl Manifest {
    pub fn snapshot(&self) -> Snapshot {
        Snapshot {
            epoch: self.epoch,
            position: self.published,
            heads: self.heads.clone(),
        }
    }
    fn validate(&self, repository: RepositoryId) -> Result<()> {
        if self.format != 1
            || self.repository != repository
            || self.heads.is_empty()
            || self.heads.len() > MAX_HEADS
            || self.inventory.is_none()
            || self.initial_settings.kind != RecordKind::Settings
            || self.settings.kind != RecordKind::Settings
        {
            return Err(Error::Corrupt("manifest fields"));
        }
        if self.published.0 > self.revision
            || (self.published.0 == 0) != self.tail.is_none()
            || (self.published.0 == 0 && (self.history.is_some() || self.receipts.is_some()))
            || (self.published.0 > 0 && (self.history.is_none() || self.receipts.is_none()))
        {
            return Err(Error::Corrupt("manifest publication position"));
        }
        for r in [self.inventory, self.receipts, self.history]
            .into_iter()
            .flatten()
        {
            if r.kind != RecordKind::Catalog || r.attempt.is_some() {
                return Err(Error::Corrupt("manifest catalog reference"));
            }
        }
        if self
            .tail
            .is_some_and(|r| r.kind != RecordKind::Wal || r.attempt.is_none())
            || self
                .checkpoint
                .is_some_and(|r| r.kind != RecordKind::Checkpoint || r.attempt.is_some())
        {
            return Err(Error::Corrupt("manifest tail or checkpoint reference"));
        }
        Ok(())
    }
}
#[derive(Clone, Debug)]
pub struct ManifestSnapshot {
    manifest: Manifest,
    digest: RecordDigest,
    version: VersionToken,
}
impl ManifestSnapshot {
    /// Loads current authority. Historical read sessions must authenticate with
    /// this current policy even when their content snapshot is older.
    pub async fn load(store: Arc<dyn ObjectStore>, repository: RepositoryId) -> Result<Self> {
        let key = Key::new(format!("repos/{repository}/manifest.json"))?;
        let download = store.get(&key, None).await?;
        let version = download.metadata.version.clone();
        let bytes = download.read_bounded(MAX_MANIFEST).await?;
        let manifest: Manifest = records::decode(&bytes)?;
        manifest.validate(repository)?;
        Ok(Self {
            manifest,
            digest: records::digest(&bytes),
            version,
        })
    }
    pub fn manifest(&self) -> &Manifest {
        &self.manifest
    }
    /// Read the immutable settings selected by this checked snapshot.
    pub async fn settings(&self, store: Arc<dyn ObjectStore>) -> Result<Vec<u8>> {
        let records = Records::new(store, self.manifest.repository);
        let settings: Settings = records
            .get(self.manifest.settings, RecordKind::Settings)
            .await?;
        if settings.data.len() > 64 * 1024 {
            return Err(Error::Limit("settings"));
        }
        Ok(settings.data)
    }
    pub fn digest(&self) -> RecordDigest {
        self.digest
    }
}
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(tag = "type", rename_all = "snake_case", deny_unknown_fields)]
pub enum Transition {
    Native {
        remove: BTreeSet<OperationId>,
        add: OperationId,
    },
    Strict {
        expected: Snapshot,
        heads: BTreeSet<OperationId>,
    },
}
#[derive(Clone, Copy, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum SemanticKind {
    OperationPublished,
    HeadsNormalized,
    SettingsChanged,
}
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct Request {
    pub principal: PrincipalId,
    pub request_id: RequestId,
    pub publication_id: PublicationId,
    pub transition: Option<Transition>,
    pub settings: Option<Vec<u8>>,
    pub result: Vec<u8>,
    pub kind: SemanticKind,
}
impl Request {
    fn intent(&self) -> Result<IntentDigest> {
        Ok(IntentDigest::new(
            Sha256::digest(records::encode(self, 8 * 1024 * 1024)?).into(),
        ))
    }
    fn validate(&self) -> Result<()> {
        if self.result.len() > 1024 * 1024
            || self.settings.as_ref().is_some_and(|s| s.len() > 64 * 1024)
        {
            return Err(Error::Limit("request payload"));
        }
        if self.settings.is_some() != (self.kind == SemanticKind::SettingsChanged)
            || (self.transition.is_none() && self.settings.is_none())
        {
            return Err(Error::Corrupt("publication kind"));
        }
        match &self.transition {
            Some(Transition::Native { remove, .. }) if remove.len() > MAX_HEADS => {
                return Err(Error::Limit("heads"));
            }
            Some(Transition::Strict { expected, heads })
                if heads.is_empty()
                    || heads.len() > MAX_HEADS
                    || expected.heads.len() > MAX_HEADS =>
            {
                return Err(Error::Limit("heads"));
            }
            _ => {}
        }
        Ok(())
    }
}
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct Receipt {
    pub principal: PrincipalId,
    pub request_id: RequestId,
    pub publication_id: PublicationId,
    pub intent: IntentDigest,
    pub accepted: Snapshot,
    pub result: Vec<u8>,
}
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct PublishedTransaction {
    pub attempt: AttemptId,
    pub repository: RepositoryId,
    pub before: Snapshot,
    pub after: Snapshot,
    pub previous_manifest: RecordDigest,
    pub previous: Option<RecordRef>,
    pub receipt: RecordRef,
    pub inventory: Option<RecordRef>,
    pub settings: RecordRef,
    pub kind: SemanticKind,
    pub published_operations: BTreeSet<OperationId>,
}
#[derive(Clone, Debug, PartialEq, Eq)]
pub enum PublishOutcome {
    Committed(Receipt),
    OutcomeUnknown {
        request_id: RequestId,
        publication_id: PublicationId,
    },
}
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct Checkpoint {
    pub captured: Manifest,
    pub captured_digest: RecordDigest,
}
#[derive(Clone, Debug, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
struct Settings {
    data: Vec<u8>,
}
#[derive(Clone, Copy, Debug)]
pub struct Limits {
    pub inventory: Budget,
    pub cas_attempts: usize,
    pub timeout: Duration,
}
impl Default for Limits {
    fn default() -> Self {
        Self {
            inventory: Budget::default(),
            cas_attempts: 8,
            timeout: Duration::from_secs(60),
        }
    }
}
/// Every publication transport supplies the same authorization and semantic
/// guard. There is no production allow-all default. Authorization also applies
/// to receipt replay; semantic validation is skipped for an already committed
/// identical request so a late retry cannot rerun its original mutation.
#[async_trait::async_trait]
pub trait PublicationGuard: Send + Sync {
    async fn authorize(&self, base: &ManifestSnapshot, request: &Request) -> Result<()>;
    async fn validate_transition(
        &self,
        base: &ManifestSnapshot,
        request: &Request,
        after: &Snapshot,
    ) -> Result<()>;
}
#[derive(Clone)]
pub struct Coordinator {
    records: Records,
    catalog: Catalog,
    source: Arc<dyn DependencySource>,
    guard: Arc<dyn PublicationGuard>,
    limits: Limits,
}
impl Coordinator {
    pub fn new(
        store: Arc<dyn ObjectStore>,
        repository: RepositoryId,
        source: Arc<dyn DependencySource>,
        guard: Arc<dyn PublicationGuard>,
        limits: Limits,
    ) -> Result<Self> {
        if source.scope().repository() != repository {
            return Err(Error::Corrupt("dependency source repository"));
        }
        if limits.cas_attempts == 0 || limits.timeout.is_zero() {
            return Err(Error::Limit("publication limits"));
        }
        let records = Records::new(store, repository);
        let catalog = Catalog::new(records.clone());
        Ok(Self {
            records,
            catalog,
            source,
            guard,
            limits,
        })
    }
    pub fn manifest_key(&self) -> Key {
        Key::new(format!("repos/{}/manifest.json", self.records.repository))
            .expect("typed repository key")
    }
    pub async fn read(&self) -> Result<ManifestSnapshot> {
        ManifestSnapshot::load(self.records.store.clone(), self.records.repository).await
    }

    pub async fn initialize(&self, epoch: EpochId, settings: Vec<u8>) -> Result<ManifestSnapshot> {
        if settings.len() > 64 * 1024 {
            return Err(Error::Limit("settings"));
        }
        tokio::time::timeout(self.limits.timeout, async {
            let heads = BTreeSet::from([OperationId::new([0; 64])]);
            let delta = self
                .verify_delta(None, heads.iter().copied().map(ObjectKey::Operation))
                .await?;
            let inventory = self.persist_inventory(None, &delta).await?;
            let settings = self
                .records
                .put(RecordKind::Settings, None, &Settings { data: settings })
                .await?;
            let manifest = Manifest {
                format: 1,
                repository: self.records.repository,
                epoch,
                revision: 0,
                published: WalPosition(0),
                heads,
                tail: None,
                inventory,
                receipts: None,
                history: None,
                checkpoint: None,
                initial_settings: settings,
                settings,
            };
            let bytes = records::encode(&manifest, MAX_MANIFEST)?;
            let upload = StagedUpload::from_bytes(&bytes, MAX_MANIFEST).await?;
            let outcome = self
                .records
                .store
                .put(&self.manifest_key(), Condition::Absent, &upload)
                .await?;
            match self.read().await {
                Ok(snapshot) if snapshot.manifest.epoch == epoch => {
                    if snapshot.manifest.initial_settings != settings {
                        return Err(Error::IntentConflict);
                    }
                    Ok(snapshot)
                }
                Ok(_) => Err(Error::AlreadyExists),
                Err(_) if outcome == WriteOutcome::Unknown => Err(Error::OutcomeUnknown),
                Err(err) => Err(err),
            }
        })
        .await
        .map_err(|_| Error::OutcomeUnknown)?
    }
    /// Stable request identity is preserved on all uncertain outcomes. Lookup is
    /// performed before any head transformation, including after checkpointing.
    pub async fn publish(&self, request: &Request) -> Result<PublishOutcome> {
        request.validate()?;
        let unknown = || PublishOutcome::OutcomeUnknown {
            request_id: request.request_id,
            publication_id: request.publication_id,
        };
        match tokio::time::timeout(self.limits.timeout, self.publish_inner(request)).await {
            Ok(Ok(receipt)) => Ok(PublishOutcome::Committed(receipt)),
            Ok(Err(Error::OutcomeUnknown)) | Err(_) => Ok(unknown()),
            Ok(Err(err)) => Err(err),
        }
    }
    async fn publish_inner(&self, request: &Request) -> Result<Receipt> {
        let intent = request.intent()?;
        for _ in 0..self.limits.cas_attempts {
            let base = self.read().await?;
            self.guard.authorize(&base, request).await?;
            if let Some(receipt) = self.lookup(&base.manifest, request, intent).await? {
                return Ok(receipt);
            }
            let heads = match &request.transition {
                None => base.manifest.heads.clone(),
                Some(Transition::Native { remove, add }) => {
                    let mut heads = base
                        .manifest
                        .heads
                        .difference(remove)
                        .copied()
                        .collect::<BTreeSet<_>>();
                    heads.insert(*add);
                    heads
                }
                Some(Transition::Strict { expected, heads }) => {
                    if *expected != base.manifest.snapshot() {
                        return Err(Error::Stale);
                    }
                    heads.clone()
                }
            };
            if heads.is_empty() || heads.len() > MAX_HEADS {
                return Err(Error::Limit("operation heads"));
            }
            let guarded_after = Snapshot {
                epoch: base.manifest.epoch,
                position: WalPosition(
                    base.manifest
                        .published
                        .0
                        .checked_add(1)
                        .ok_or(Error::Limit("publication sequence"))?,
                ),
                heads: heads.clone(),
            };
            self.guard
                .validate_transition(&base, request, &guarded_after)
                .await?;
            let delta = self
                .verify_delta(
                    base.manifest.inventory,
                    heads.iter().copied().map(ObjectKey::Operation),
                )
                .await?;
            let inventory = self
                .persist_inventory(base.manifest.inventory, &delta)
                .await?;
            let settings = if let Some(data) = &request.settings {
                self.records
                    .put(RecordKind::Settings, None, &Settings { data: data.clone() })
                    .await?
            } else {
                base.manifest.settings
            };
            let position = WalPosition(
                base.manifest
                    .published
                    .0
                    .checked_add(1)
                    .ok_or(Error::Limit("publication sequence"))?,
            );
            let after = Snapshot {
                epoch: base.manifest.epoch,
                position,
                heads: heads.clone(),
            };
            let receipt = Receipt {
                principal: request.principal,
                request_id: request.request_id,
                publication_id: request.publication_id,
                intent,
                accepted: after.clone(),
                result: request.result.clone(),
            };
            let receipt_ref = self
                .records
                .put(RecordKind::Receipt, None, &receipt)
                .await?;
            let receipt_keys = self.receipt_keys(
                request.principal,
                request.request_id,
                request.publication_id,
            );
            let receipts = self
                .catalog
                .update(
                    base.manifest.receipts,
                    BTreeMap::from([(receipt_keys.0, receipt_ref), (receipt_keys.1, receipt_ref)]),
                )
                .await?;
            let attempt = records::new_attempt()?;
            let transaction = PublishedTransaction {
                attempt,
                repository: self.records.repository,
                before: base.manifest.snapshot(),
                after,
                previous_manifest: base.digest,
                previous: base.manifest.tail,
                receipt: receipt_ref,
                inventory,
                settings,
                kind: request.kind,
                published_operations: delta
                    .values()
                    .filter_map(|n| match (n.key, &n.representation) {
                        (ObjectKey::Operation(id), Representation::Stored { .. }) => Some(id),
                        _ => None,
                    })
                    .collect(),
            };
            let tail = self
                .records
                .put(RecordKind::Wal, Some(attempt), &transaction)
                .await?;
            let history = self
                .catalog
                .update(
                    base.manifest.history,
                    BTreeMap::from([(sequence_key(position), tail)]),
                )
                .await?;
            let manifest = Manifest {
                revision: base
                    .manifest
                    .revision
                    .checked_add(1)
                    .ok_or(Error::Limit("manifest revision"))?,
                published: position,
                heads,
                tail: Some(tail),
                inventory,
                receipts,
                history,
                settings,
                ..base.manifest
            };
            let bytes = records::encode(&manifest, MAX_MANIFEST)?;
            let upload = StagedUpload::from_bytes(&bytes, MAX_MANIFEST).await?;
            self.guard.authorize(&base, request).await?;
            match self
                .records
                .store
                .put(
                    &self.manifest_key(),
                    Condition::Matches(base.version),
                    &upload,
                )
                .await?
            {
                WriteOutcome::Applied(_) => return Ok(receipt),
                WriteOutcome::PreconditionFailed => continue,
                WriteOutcome::Unknown => {
                    // The candidate may already be behind a newer selected tail.
                    // Absence is not a proven rejection; never retry under a new ID.
                    let latest = self.read().await.map_err(|_| Error::OutcomeUnknown)?;
                    self.guard.authorize(&latest, request).await?;
                    return self
                        .lookup(&latest.manifest, request, intent)
                        .await?
                        .ok_or(Error::OutcomeUnknown);
                }
            }
        }
        Err(Error::Contended)
    }
    fn receipt_keys(
        &self,
        principal: PrincipalId,
        request: RequestId,
        publication: PublicationId,
    ) -> (RecordDigest, RecordDigest) {
        let mut req = Sha256::new_with_prefix(b"jj-store/request/v1");
        req.update(self.records.repository.as_bytes());
        req.update(principal.as_bytes());
        req.update(request.as_bytes());
        let mut pub_id = Sha256::new_with_prefix(b"jj-store/publication/v1");
        pub_id.update(self.records.repository.as_bytes());
        pub_id.update(publication.as_bytes());
        (
            RecordDigest::new(req.finalize().into()),
            RecordDigest::new(pub_id.finalize().into()),
        )
    }
    async fn lookup(
        &self,
        manifest: &Manifest,
        request: &Request,
        intent: IntentDigest,
    ) -> Result<Option<Receipt>> {
        let (r, p) = self.receipt_keys(
            request.principal,
            request.request_id,
            request.publication_id,
        );
        let a = self.catalog.get(manifest.receipts, r).await?;
        let b = self.catalog.get(manifest.receipts, p).await?;
        if a.is_some() && b.is_some() && a != b {
            return Err(Error::Corrupt("receipt index disagreement"));
        }
        let Some(reference) = a.or(b) else {
            return Ok(None);
        };
        let receipt: Receipt = self.records.get(reference, RecordKind::Receipt).await?;
        if receipt.intent != intent
            || receipt.principal != request.principal
            || receipt.request_id != request.request_id
            || receipt.publication_id != request.publication_id
        {
            return Err(Error::IntentConflict);
        }
        if receipt.accepted.epoch != manifest.epoch
            || receipt.accepted.position.0 > manifest.published.0
            || receipt.accepted.position.0 == 0
        {
            return Err(Error::Corrupt("receipt position"));
        }
        if a != b {
            return Err(Error::Corrupt("receipt companion index missing"));
        }
        Ok(Some(receipt))
    }
    /// None means no selected receipt was observed, never proof of rejection.
    pub async fn status(&self, request: &Request) -> Result<Option<Receipt>> {
        request.validate()?;
        let snapshot = self.read().await?;
        self.guard.authorize(&snapshot, request).await?;
        self.lookup(&snapshot.manifest, request, request.intent()?)
            .await
    }
    pub async fn settings(&self, snapshot: &ManifestSnapshot) -> Result<Vec<u8>> {
        let manifest = &snapshot.manifest;
        manifest.validate(self.records.repository)?;
        let settings: Settings = self
            .records
            .get(manifest.settings, RecordKind::Settings)
            .await?;
        Ok(settings.data)
    }
    async fn verify_delta(
        &self,
        base: Option<RecordRef>,
        roots: impl IntoIterator<Item = ObjectKey>,
    ) -> Result<BTreeMap<ObjectKey, VerifiedNode>> {
        let budget = self.limits.inventory;
        tokio::time::timeout(budget.timeout, async {
            let mut delta = BTreeMap::new();
            let mut done = BTreeSet::new();
            let mut visiting = BTreeSet::new();
            let mut stack = Vec::new();
            let mut bytes = 0u64;
            let mut edges = 0usize;
            for key in roots {
                if stack.len() >= budget.max_nodes {
                    return Err(Error::Limit("inventory roots"));
                }
                stack.push((key, false));
            }
            while let Some((key, exit)) = stack.pop() {
                if exit {
                    visiting.remove(&key);
                    done.insert(key);
                    continue;
                }
                if done.contains(&key) {
                    continue;
                }
                if visiting.contains(&key) {
                    return Err(crate::inventory::InventoryError::Cycle.into());
                }
                if done.len().saturating_add(visiting.len()) >= budget.max_nodes {
                    return Err(Error::Limit("inventory nodes"));
                }
                if let Some(reference) = self.catalog.get(base, object_key(key)).await? {
                    let node: VerifiedNode =
                        self.records.get(reference, RecordKind::Inventory).await?;
                    if node.key != key {
                        return Err(Error::Corrupt("inventory key"));
                    }
                    done.insert(key);
                    continue;
                }
                visiting.insert(key);
                let node = self
                    .source
                    .load_verified(key, budget.max_new_bytes - bytes)
                    .await?;
                if node.key != key {
                    return Err(crate::inventory::InventoryError::Key.into());
                }
                if let Representation::Stored { bytes: len, .. } = node.representation {
                    bytes = bytes
                        .checked_add(len)
                        .ok_or(Error::Limit("inventory bytes"))?;
                    if bytes > budget.max_new_bytes {
                        return Err(Error::Limit("inventory bytes"));
                    }
                }
                edges = edges
                    .checked_add(node.dependencies.len())
                    .ok_or(Error::Limit("inventory edges"))?;
                if edges > budget.max_edges {
                    return Err(Error::Limit("inventory edges"));
                }
                stack.push((key, true));
                stack.extend(node.dependencies.iter().rev().map(|key| (*key, false)));
                delta.insert(key, node);
            }
            Ok(delta)
        })
        .await
        .map_err(|_| Error::Limit("inventory time"))?
    }
    async fn persist_inventory(
        &self,
        base: Option<RecordRef>,
        delta: &BTreeMap<ObjectKey, VerifiedNode>,
    ) -> Result<Option<RecordRef>> {
        let mut changes = BTreeMap::new();
        for (key, node) in delta {
            let reference = self.records.put(RecordKind::Inventory, None, node).await?;
            changes.insert(object_key(*key), reference);
        }
        self.catalog.update(base, changes).await
    }
    /// Reads only exact manifest-selected historical positions, with a caller
    /// bound on work. Staged/orphaned candidates are never enumerated.
    pub async fn committed(
        &self,
        snapshot: &ManifestSnapshot,
        after: WalPosition,
        limit: usize,
    ) -> Result<Vec<PublishedTransaction>> {
        let manifest = &snapshot.manifest;
        manifest.validate(self.records.repository)?;
        if limit > 1024 || after.0 > manifest.published.0 {
            return Err(Error::Limit("committed page"));
        }
        let count = (manifest.published.0 - after.0).min(limit as u64);
        let mut out = Vec::new();
        let mut previous = if after.0 == 0 {
            None
        } else {
            Some(
                self.catalog
                    .get(manifest.history, sequence_key(after))
                    .await?
                    .ok_or(Error::Corrupt("historical position missing"))?,
            )
        };
        let mut previous_snapshot = if let Some(reference) = previous {
            let prior: PublishedTransaction = self.records.get(reference, RecordKind::Wal).await?;
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
            let reference = self
                .catalog
                .get(manifest.history, sequence_key(position))
                .await?
                .ok_or(Error::Corrupt("committed position missing"))?;
            let transaction: PublishedTransaction =
                self.records.get(reference, RecordKind::Wal).await?;
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
            let receipt: Receipt = self
                .records
                .get(transaction.receipt, RecordKind::Receipt)
                .await?;
            if receipt.accepted != transaction.after {
                return Err(Error::Corrupt("log receipt snapshot"));
            }
            previous_snapshot = transaction.after.clone();
            previous = Some(reference);
            out.push(transaction);
        }
        if after.0 + count == manifest.published.0
            && (previous != manifest.tail || previous_snapshot != manifest.snapshot())
        {
            return Err(Error::Corrupt("manifest tail disagreement"));
        }
        Ok(out)
    }
    pub async fn checkpoint(&self) -> Result<ManifestSnapshot> {
        tokio::time::timeout(self.limits.timeout, async {
            let captured = self.read().await?;
            let checkpoint = self
                .records
                .put(
                    RecordKind::Checkpoint,
                    None,
                    &Checkpoint {
                        captured: captured.manifest.clone(),
                        captured_digest: captured.digest,
                    },
                )
                .await?;
            for _ in 0..self.limits.cas_attempts {
                let base = self.read().await?;
                if base.manifest.epoch != captured.manifest.epoch {
                    return Err(Error::Stale);
                }
                // Do not replace a checkpoint already ahead of the captured one.
                if let Some(existing) = base.manifest.checkpoint {
                    let existing: Checkpoint =
                        self.records.get(existing, RecordKind::Checkpoint).await?;
                    if existing.captured.published >= captured.manifest.published {
                        return Ok(base);
                    }
                }
                let manifest = Manifest {
                    revision: base
                        .manifest
                        .revision
                        .checked_add(1)
                        .ok_or(Error::Limit("manifest revision"))?,
                    checkpoint: Some(checkpoint),
                    ..base.manifest
                };
                let bytes = records::encode(&manifest, MAX_MANIFEST)?;
                let upload = StagedUpload::from_bytes(&bytes, MAX_MANIFEST).await?;
                match self
                    .records
                    .store
                    .put(
                        &self.manifest_key(),
                        Condition::Matches(base.version),
                        &upload,
                    )
                    .await?
                {
                    WriteOutcome::Applied(version) => {
                        return Ok(ManifestSnapshot {
                            manifest,
                            digest: records::digest(&bytes),
                            version,
                        });
                    }
                    WriteOutcome::PreconditionFailed => continue,
                    WriteOutcome::Unknown => {
                        let latest = self.read().await.map_err(|_| Error::OutcomeUnknown)?;
                        if latest.manifest.checkpoint == Some(checkpoint) {
                            return Ok(latest);
                        }
                        return Err(Error::OutcomeUnknown);
                    }
                }
            }
            Err(Error::Contended)
        })
        .await
        .map_err(|_| Error::OutcomeUnknown)?
    }
    /// Cold integrity recovery reads only selected records. No process cache or
    /// local index is needed. Limits make incomplete verification an error.
    pub async fn recover(&self, max_entries: usize) -> Result<ManifestSnapshot> {
        tokio::time::timeout(self.limits.timeout, async {
            let snapshot = self.read().await?;
            let manifest = &snapshot.manifest;
            if manifest.published.0 > max_entries as u64 {
                return Err(Error::Limit("recovery history"));
            }
            let _initial: Settings = self
                .records
                .get(manifest.initial_settings, RecordKind::Settings)
                .await?;
            self.settings(&snapshot).await?;
            let inventory = self.inventory(&snapshot, max_entries).await?;
            let mut remaining = self.limits.inventory.max_new_bytes;
            for expected in inventory {
                let actual = self.source.load_verified(expected.key, remaining).await?;
                if actual != expected {
                    return Err(Error::Corrupt("committed native descriptor changed"));
                }
                if let Representation::Stored { bytes, .. } = actual.representation {
                    remaining = remaining
                        .checked_sub(bytes)
                        .ok_or(Error::Limit("recovery bytes"))?;
                }
            }
            let receipt_entries = self
                .catalog
                .entries(manifest.receipts, max_entries.saturating_mul(2))
                .await?;
            if receipt_entries.len() as u64 != manifest.published.0.saturating_mul(2) {
                return Err(Error::Corrupt("receipt catalog cardinality"));
            }
            let history_entries = self.catalog.entries(manifest.history, max_entries).await?;
            if history_entries.len() as u64 != manifest.published.0 {
                return Err(Error::Corrupt("history catalog cardinality"));
            }
            let inventory_entries = self
                .catalog
                .entries(manifest.inventory, max_entries)
                .await?;
            let max_pages = max_entries
                .saturating_mul(4)
                .saturating_add(1)
                .saturating_mul(65);
            let mut inventory_audit = CatalogAudit::new(&inventory_entries, max_pages);
            let mut receipt_audit = CatalogAudit::new(&receipt_entries, max_pages);
            let mut history_audit = CatalogAudit::new(&history_entries, max_pages);
            let mut settings_seen = BTreeSet::new();
            let mut after = WalPosition(0);
            while after.0 < manifest.published.0 {
                let page = self.committed(&snapshot, after, 1024).await?;
                for tx in page {
                    if tx.inventory.is_none() {
                        return Err(Error::Corrupt("historical inventory missing"));
                    }
                    inventory_audit.verify(&self.catalog, tx.inventory).await?;
                    for id in tx.after.heads.iter().chain(&tx.published_operations) {
                        if self
                            .catalog
                            .get(tx.inventory, object_key(ObjectKey::Operation(*id)))
                            .await?
                            .is_none()
                        {
                            return Err(Error::Corrupt(
                                "historical operation missing from inventory",
                            ));
                        }
                    }
                    if settings_seen.insert(tx.settings.digest) {
                        let _: Settings =
                            self.records.get(tx.settings, RecordKind::Settings).await?;
                    }
                    let receipt: Receipt =
                        self.records.get(tx.receipt, RecordKind::Receipt).await?;
                    let (r, p) = self.receipt_keys(
                        receipt.principal,
                        receipt.request_id,
                        receipt.publication_id,
                    );
                    if receipt_entries.get(&r) != Some(&tx.receipt)
                        || receipt_entries.get(&p) != Some(&tx.receipt)
                    {
                        return Err(Error::Corrupt("selected receipt missing from catalog"));
                    }
                    if tx.after.position == manifest.published
                        && (tx.inventory != manifest.inventory || tx.settings != manifest.settings)
                    {
                        return Err(Error::Corrupt("manifest folded state"));
                    }
                    after = tx.after.position;
                }
            }
            let mut checkpoint_ref = manifest.checkpoint;
            let mut checkpoint_position = manifest.published.0.saturating_add(1);
            let mut checkpoints = 0usize;
            while let Some(reference) = checkpoint_ref {
                checkpoints += 1;
                if checkpoints > max_entries.saturating_add(1) {
                    return Err(Error::Limit("retained checkpoints"));
                }
                let checkpoint: Checkpoint =
                    self.records.get(reference, RecordKind::Checkpoint).await?;
                checkpoint.captured.validate(manifest.repository)?;
                if checkpoint.captured.epoch != manifest.epoch
                    || checkpoint.captured.published > manifest.published
                    || checkpoint.captured.published.0 >= checkpoint_position
                    || records::digest(&records::encode(&checkpoint.captured, MAX_MANIFEST)?)
                        != checkpoint.captured_digest
                {
                    return Err(Error::Corrupt("checkpoint captured state"));
                }
                inventory_audit
                    .verify(&self.catalog, checkpoint.captured.inventory)
                    .await?;
                receipt_audit
                    .verify(&self.catalog, checkpoint.captured.receipts)
                    .await?;
                history_audit
                    .verify(&self.catalog, checkpoint.captured.history)
                    .await?;
                let expected = if checkpoint.captured.published.0 == 0 {
                    None
                } else {
                    self.catalog
                        .get(
                            manifest.history,
                            sequence_key(checkpoint.captured.published),
                        )
                        .await?
                };
                if checkpoint.captured.tail != expected {
                    return Err(Error::Corrupt("checkpoint tail is not committed"));
                }
                if let Some(reference) = expected {
                    let tx: PublishedTransaction =
                        self.records.get(reference, RecordKind::Wal).await?;
                    if checkpoint.captured.snapshot() != tx.after
                        || checkpoint.captured.inventory != tx.inventory
                        || checkpoint.captured.settings != tx.settings
                    {
                        return Err(Error::Corrupt("checkpoint folded state"));
                    }
                }
                checkpoint_position = checkpoint.captured.published.0;
                checkpoint_ref = checkpoint.captured.checkpoint;
            }
            Ok(snapshot)
        })
        .await
        .map_err(|_| Error::Limit("recovery time"))?
    }
    /// Bounded bootstrap/restore inventory, including retained historical data.
    pub async fn inventory(
        &self,
        snapshot: &ManifestSnapshot,
        limit: usize,
    ) -> Result<Vec<VerifiedNode>> {
        let manifest = &snapshot.manifest;
        manifest.validate(self.records.repository)?;
        let entries = self.catalog.entries(manifest.inventory, limit).await?;
        let mut out = Vec::new();
        for (key, reference) in entries {
            let node: VerifiedNode = self.records.get(reference, RecordKind::Inventory).await?;
            if object_key(node.key) != key {
                return Err(Error::Corrupt("inventory catalog key"));
            }
            out.push(node);
        }
        let keys = out.iter().map(|n| n.key).collect::<BTreeSet<_>>();
        if manifest
            .heads
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
}
fn object_key(key: ObjectKey) -> RecordDigest {
    let mut hash = Sha256::new_with_prefix(b"jj-store/object/v1");
    hash.update([key.kind() as u8]);
    hash.update(key.as_bytes());
    RecordDigest::new(hash.finalize().into())
}
fn sequence_key(position: WalPosition) -> RecordDigest {
    let mut hash = Sha256::new_with_prefix(b"jj-store/position/v1");
    hash.update(position.0.to_be_bytes());
    RecordDigest::new(hash.finalize().into())
}

//! Manifest-bound origin feed, membership, and immutable closure planning.
use super::*;
use crate::{catalog::CatalogClosure, records::RecordKind};

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct OriginFeedItemV1 {
    pub local_position: WalPosition,
    pub envelope: OriginEnvelopeV1,
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct OriginFeedPageV1 {
    pub schema_version: u32,
    pub repository: RepositoryId,
    pub source_replica: ReplicaId,
    pub log_epoch: EpochId,
    pub after: WalPosition,
    pub through: WalPosition,
    pub upper_bound: WalPosition,
    pub origins: Vec<OriginFeedItemV1>,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct OriginOfferV1 {
    pub origin_key: OriginKeyV1,
    pub origin_digest: OriginDigest,
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(tag = "status", rename_all = "snake_case", deny_unknown_fields)]
pub enum OriginOfferResultV1 {
    Missing { offer: OriginOfferV1 },
    Known { receipt: OriginReceiptV1 },
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct OriginClosureV1 {
    pub envelope: OriginEnvelopeV1,
    pub native: Vec<VerifiedNode>,
    pub control: BTreeSet<RecordRef>,
}

impl Coordinator {
    pub async fn ensure_origin_replication(&self) -> Result<()> {
        if self.read().await?.manifest.work_root.is_some() {
            return Err(Error::Unsupported("WorkStore origin replication"));
        }
        Ok(())
    }

    /// Return the immutable first-origin baseline selected by this source
    /// snapshot. Repositories created directly in replication mode may have no
    /// seed because their complete feed begins at position zero.
    pub async fn bootstrap_checkpoint(
        &self,
        snapshot: &ManifestSnapshot,
    ) -> Result<Option<BootstrapCheckpointV1>> {
        self.ensure_origin_replication().await?;
        let Some(root_ref) = snapshot.manifest.replication_root else {
            return Ok(None);
        };
        let root = self.read_replication_root(root_ref).await?;
        if root.local_sequence == 0 {
            return Ok(None);
        }
        let key = OriginKeyV1 {
            stream: root.local_stream,
            sequence: 1,
        };
        let Some(receipt) = self.origin_receipt(snapshot, key).await? else {
            return Err(Error::Corrupt("bootstrap seed receipt"));
        };
        let publication = self.origin_publication(receipt).await?;
        if !publication.is_seed_baseline()
            || receipt.accepted.replica != root.local_stream.replica
            || receipt.accepted.log_epoch != snapshot.manifest.epoch
            || receipt.accepted.position > snapshot.manifest.published
        {
            return Ok(None);
        }
        Ok(Some(BootstrapCheckpointV1 {
            schema_version: 1,
            source_replica: root.local_stream.replica,
            log_epoch: snapshot.manifest.epoch,
            through: receipt.accepted.position,
            source_manifest: snapshot.digest(),
            seed: OriginOfferV1 {
                origin_key: key,
                origin_digest: receipt.origin_digest,
            },
            code_heads: publication.published_operations,
            agent_root: publication
                .agent_root
                .ok_or(Error::Corrupt("bootstrap agent root"))?,
            review_root: publication
                .review_root
                .ok_or(Error::Corrupt("bootstrap review root"))?,
        }))
    }

    pub async fn read_replication_record(&self, reference: RecordRef) -> Result<Vec<u8>> {
        if reference.attempt.is_some() || !reference.kind.replication_transferable() {
            return Err(Error::Corrupt("replication control record"));
        }
        self.records.read_bytes(reference).await
    }

    pub async fn write_replication_record(
        &self,
        reference: RecordRef,
        bytes: &[u8],
    ) -> Result<WriteOutcome> {
        self.records.put_replication_bytes(reference, bytes).await
    }

    /// A feed is pinned to the supplied manifest snapshot. `through` advances
    /// over local entries without origins so echo-only scans can make progress.
    pub async fn committed_origin_feed(
        &self,
        snapshot: &ManifestSnapshot,
        after: WalPosition,
        limit: usize,
    ) -> Result<OriginFeedPageV1> {
        self.ensure_origin_replication().await?;
        if limit == 0 || limit > 1024 || after > snapshot.manifest.published {
            return Err(Error::Limit("origin feed"));
        }
        let root_ref = snapshot
            .manifest
            .replication_root
            .ok_or(Error::Corrupt("replication root missing"))?;
        let root = self.read_replication_root(root_ref).await?;
        let transactions = self.committed(snapshot, after, limit).await?;
        let through = transactions
            .last()
            .map_or(after, |transaction| transaction.after.position);
        let mut origins = Vec::new();
        for transaction in transactions {
            for reference in transaction.origins {
                let receipt: OriginReceiptV1 = self
                    .records
                    .get(reference, RecordKind::OriginReceipt)
                    .await?;
                if receipt.accepted.replica != root.local_stream.replica
                    || receipt.accepted.log_epoch != snapshot.manifest.epoch
                    || receipt.accepted.position != transaction.after.position
                {
                    return Err(Error::Corrupt("origin feed delivery"));
                }
                let publication = self.origin_publication(receipt).await?;
                origins.push(OriginFeedItemV1 {
                    local_position: transaction.after.position,
                    envelope: OriginEnvelopeV1 {
                        origin_digest: receipt.origin_digest,
                        publication,
                    },
                });
            }
        }
        Ok(OriginFeedPageV1 {
            schema_version: 1,
            repository: self.records.repository,
            source_replica: root.local_stream.replica,
            log_epoch: snapshot.manifest.epoch,
            after,
            through,
            upper_bound: snapshot.manifest.published,
            origins,
        })
    }

    /// Read a page from the issuer and log epoch negotiated by the caller,
    /// never past the fixed bound captured during that negotiation.
    pub async fn committed_origin_feed_at(
        &self,
        snapshot: &ManifestSnapshot,
        source_replica: ReplicaId,
        log_epoch: EpochId,
        after: WalPosition,
        upper_bound: WalPosition,
        limit: usize,
    ) -> Result<OriginFeedPageV1> {
        self.ensure_origin_replication().await?;
        if limit == 0
            || limit > 1024
            || snapshot.manifest.epoch != log_epoch
            || after > upper_bound
            || upper_bound > snapshot.manifest.published
        {
            return Err(Error::Limit("bounded origin feed"));
        }
        let root_ref = snapshot
            .manifest
            .replication_root
            .ok_or(Error::Corrupt("replication root missing"))?;
        let root = self.read_replication_root(root_ref).await?;
        if root.local_stream.replica != source_replica {
            return Err(Error::Corrupt("origin feed issuer"));
        }
        let remaining = usize::try_from(upper_bound.0 - after.0)
            .unwrap_or(usize::MAX)
            .min(limit);
        let transactions = self.committed(snapshot, after, remaining).await?;
        let through = transactions
            .last()
            .map_or(after, |transaction| transaction.after.position);
        let mut origins = Vec::new();
        for transaction in transactions {
            for reference in transaction.origins {
                let receipt: OriginReceiptV1 = self
                    .records
                    .get(reference, RecordKind::OriginReceipt)
                    .await?;
                if receipt.accepted.replica != source_replica
                    || receipt.accepted.log_epoch != log_epoch
                    || receipt.accepted.position != transaction.after.position
                {
                    return Err(Error::Corrupt("origin feed delivery"));
                }
                let publication = self.origin_publication(receipt).await?;
                origins.push(OriginFeedItemV1 {
                    local_position: transaction.after.position,
                    envelope: OriginEnvelopeV1 {
                        origin_digest: receipt.origin_digest,
                        publication,
                    },
                });
            }
        }
        Ok(OriginFeedPageV1 {
            schema_version: 1,
            repository: self.records.repository,
            source_replica,
            log_epoch,
            after,
            through,
            upper_bound,
            origins,
        })
    }

    pub async fn offer_origins(
        &self,
        snapshot: &ManifestSnapshot,
        offers: &[OriginOfferV1],
    ) -> Result<Vec<OriginOfferResultV1>> {
        self.ensure_origin_replication().await?;
        if offers.len() > 1024 {
            return Err(Error::Limit("origin offers"));
        }
        let mut unique = BTreeSet::new();
        let mut results = Vec::with_capacity(offers.len());
        for offer in offers {
            if !unique.insert(offer.origin_key) {
                return Err(Error::Corrupt("duplicate origin offer"));
            }
            match self.origin_receipt(snapshot, offer.origin_key).await? {
                Some(receipt) if receipt.origin_digest == offer.origin_digest => {
                    results.push(OriginOfferResultV1::Known { receipt });
                }
                Some(_) => return Err(Error::Corrupt("origin key digest mismatch")),
                None => results.push(OriginOfferResultV1::Missing { offer: *offer }),
            }
        }
        Ok(results)
    }

    /// Resolve the immutable source closure for one selected origin. The
    /// caller can stream these exact identities through any authenticated
    /// transport; this method performs no remote I/O.
    pub async fn origin_closure(
        &self,
        snapshot: &ManifestSnapshot,
        key: OriginKeyV1,
        max_entries: usize,
    ) -> Result<OriginClosureV1> {
        self.ensure_origin_replication().await?;
        let receipt = self
            .origin_receipt(snapshot, key)
            .await?
            .ok_or(Error::Corrupt("origin not selected"))?;
        let publication = self.origin_publication(receipt).await?;
        let native = reader::inventory(
            self.records.store.clone(),
            self.records.repository,
            Some(publication.required_inventory_root),
            &publication.published_operations,
            max_entries,
        )
        .await?;
        let CatalogClosure { pages, entries } = self
            .catalog
            .closure(Some(publication.required_inventory_root), max_entries)
            .await?;
        let mut control = pages;
        control.extend(entries.into_values());
        if publication.changed_domains.contains(&OriginDomainV1::Agent) {
            let root = publication
                .agent_root
                .ok_or(Error::Corrupt("origin agent root missing"))?;
            control.insert(root);
            let (_, _, objects) = self.read_agent_root_closure(root).await?;
            control.extend(objects.into_iter().map(super::agent::agent_record));
        }
        if publication
            .changed_domains
            .contains(&OriginDomainV1::Review)
        {
            let root = publication
                .review_root
                .ok_or(Error::Corrupt("origin review root missing"))?;
            control.insert(root);
            let (_, closure) = self.read_review_root_closure(root).await?;
            control.extend(
                closure
                    .objects
                    .into_iter()
                    .map(super::review::review_record),
            );
            control.extend(
                closure
                    .agent_objects
                    .into_iter()
                    .map(super::agent::agent_record),
            );
        }
        Ok(OriginClosureV1 {
            envelope: OriginEnvelopeV1 {
                origin_digest: receipt.origin_digest,
                publication,
            },
            native,
            control,
        })
    }
}

//! Manifest-selected logical origins, distinct from local WAL delivery.
use super::*;

const ORIGIN_BYTES: u64 = 4 * 1024 * 1024;

#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct OriginStreamV1 {
    pub repository: RepositoryId,
    pub replica: ReplicaId,
    pub writer_epoch: WriterEpoch,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct OriginKeyV1 {
    pub stream: OriginStreamV1,
    pub sequence: u64,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct OriginPrefixV1 {
    pub stream: OriginStreamV1,
    pub sequence: u64,
    pub terminal_digest: OriginDigest,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum OriginDomainV1 {
    Repository,
    Code,
    Agent,
    Review,
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct OriginPublicationV1 {
    pub schema_version: u32,
    pub origin_key: OriginKeyV1,
    pub previous_origin_digest: Option<OriginDigest>,
    pub causal_frontier: Vec<OriginPrefixV1>,
    pub semantic_format_version: u32,
    pub principal: PrincipalId,
    pub request_id: RequestId,
    pub publication_id: PublicationId,
    pub intent: IntentDigest,
    pub published_operations: BTreeSet<OperationId>,
    pub agent_root: Option<RecordRef>,
    pub review_root: Option<RecordRef>,
    pub required_inventory_root: RecordRef,
    pub changed_domains: BTreeSet<OriginDomainV1>,
}

impl OriginPublicationV1 {
    fn validate(&self, repository: RepositoryId) -> Result<()> {
        if self.schema_version != 1
            || self.semantic_format_version != 1
            || self.origin_key.stream.repository != repository
            || self.origin_key.sequence == 0
            || self.published_operations.len() > MAX_HEADS
            || self.required_inventory_root.kind != RecordKind::Catalog
            || self.required_inventory_root.attempt.is_some()
            || self
                .agent_root
                .is_some_and(|r| r.kind != RecordKind::AgentRoot || r.attempt.is_some())
            || self
                .review_root
                .is_some_and(|r| r.kind != RecordKind::ReviewRoot || r.attempt.is_some())
            || self.changed_domains.is_empty()
        {
            return Err(Error::Corrupt("origin publication"));
        }
        let mut previous = None;
        for prefix in &self.causal_frontier {
            if prefix.stream.repository != repository
                || prefix.sequence == 0
                || previous.is_some_and(|stream| stream >= prefix.stream)
            {
                return Err(Error::Corrupt("origin causal frontier"));
            }
            previous = Some(prefix.stream);
        }
        Ok(())
    }

    pub fn digest(&self) -> Result<OriginDigest> {
        self.validate(self.origin_key.stream.repository)?;
        Ok(OriginDigest::new(
            Sha256::digest(records::encode(self, ORIGIN_BYTES)?).into(),
        ))
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct LocalDeliveryV1 {
    pub replica: ReplicaId,
    pub log_epoch: EpochId,
    pub position: WalPosition,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct OriginReceiptV1 {
    pub schema_version: u32,
    pub origin_key: OriginKeyV1,
    pub origin_digest: OriginDigest,
    pub envelope: RecordRef,
    pub accepted: LocalDeliveryV1,
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct ReplicationRootV1 {
    pub schema_version: u32,
    pub local_stream: OriginStreamV1,
    pub local_sequence: u64,
    pub local_terminal_digest: Option<OriginDigest>,
    pub accepted_origins: Option<RecordRef>,
    pub prefixes: Vec<OriginPrefixV1>,
}

impl ReplicationRootV1 {
    pub fn empty(repository: RepositoryId, replica: ReplicaId, writer_epoch: WriterEpoch) -> Self {
        Self {
            schema_version: 1,
            local_stream: OriginStreamV1 {
                repository,
                replica,
                writer_epoch,
            },
            local_sequence: 0,
            local_terminal_digest: None,
            accepted_origins: None,
            prefixes: Vec::new(),
        }
    }

    fn validate(&self, repository: RepositoryId) -> Result<()> {
        if self.schema_version != 1
            || self.local_stream.repository != repository
            || (self.local_sequence == 0) != self.local_terminal_digest.is_none()
            || self
                .accepted_origins
                .is_some_and(|r| r.kind != RecordKind::Catalog || r.attempt.is_some())
        {
            return Err(Error::Corrupt("replication root"));
        }
        let mut previous = None;
        let mut local = None;
        for prefix in &self.prefixes {
            if prefix.stream.repository != repository
                || prefix.sequence == 0
                || previous.is_some_and(|stream| stream >= prefix.stream)
            {
                return Err(Error::Corrupt("replication prefixes"));
            }
            if prefix.stream == self.local_stream {
                local = Some((prefix.sequence, prefix.terminal_digest));
            }
            previous = Some(prefix.stream);
        }
        if self.local_sequence == 0 {
            if local.is_some() {
                return Err(Error::Corrupt("empty local origin prefix"));
            }
        } else if local != Some((self.local_sequence, self.local_terminal_digest.unwrap())) {
            return Err(Error::Corrupt("local origin prefix"));
        }
        Ok(())
    }
}

pub(super) struct AuthoredOrigin {
    pub root: RecordRef,
    pub receipt: RecordRef,
}

impl Coordinator {
    pub async fn stage_replication_root(&self, root: &ReplicationRootV1) -> Result<RecordRef> {
        root.validate(self.records.repository)?;
        self.records
            .put(RecordKind::ReplicationRoot, None, root)
            .await
    }

    pub async fn read_replication_root(&self, reference: RecordRef) -> Result<ReplicationRootV1> {
        let root: ReplicationRootV1 = self
            .records
            .get(reference, RecordKind::ReplicationRoot)
            .await?;
        root.validate(self.records.repository)?;
        Ok(root)
    }

    #[allow(clippy::too_many_arguments)]
    pub(super) async fn author_origin(
        &self,
        base: &ManifestSnapshot,
        position: WalPosition,
        principal: PrincipalId,
        request_id: RequestId,
        publication_id: PublicationId,
        intent: IntentDigest,
        published_operations: BTreeSet<OperationId>,
        after_domains: Option<RepositoryDomains>,
        inventory: Option<RecordRef>,
        changed_domains: BTreeSet<OriginDomainV1>,
    ) -> Result<Option<AuthoredOrigin>> {
        let Some(root_ref) = base.manifest.replication_root else {
            return Ok(None);
        };
        let mut root = self.read_replication_root(root_ref).await?;
        let sequence = root
            .local_sequence
            .checked_add(1)
            .ok_or(Error::Limit("origin sequence"))?;
        let origin_key = OriginKeyV1 {
            stream: root.local_stream,
            sequence,
        };
        let publication = OriginPublicationV1 {
            schema_version: 1,
            origin_key,
            previous_origin_digest: root.local_terminal_digest,
            causal_frontier: root.prefixes.clone(),
            semantic_format_version: 1,
            principal,
            request_id,
            publication_id,
            intent,
            published_operations,
            agent_root: after_domains.map(|domains| domains.agent.root),
            review_root: after_domains.map(|domains| domains.review.root),
            required_inventory_root: inventory.ok_or(Error::Corrupt("origin inventory root"))?,
            changed_domains,
        };
        let origin_digest = publication.digest()?;
        let envelope = self
            .records
            .put(RecordKind::Origin, None, &publication)
            .await?;
        let receipt = OriginReceiptV1 {
            schema_version: 1,
            origin_key,
            origin_digest,
            envelope,
            accepted: LocalDeliveryV1 {
                replica: root.local_stream.replica,
                log_epoch: base.manifest.epoch,
                position,
            },
        };
        let receipt_ref = self
            .records
            .put(RecordKind::OriginReceipt, None, &receipt)
            .await?;
        root.accepted_origins = self
            .catalog
            .update(
                root.accepted_origins,
                BTreeMap::from([(origin_key_digest(origin_key), receipt_ref)]),
            )
            .await?;
        root.local_sequence = sequence;
        root.local_terminal_digest = Some(origin_digest);
        root.prefixes
            .retain(|prefix| prefix.stream != root.local_stream);
        root.prefixes.push(OriginPrefixV1 {
            stream: root.local_stream,
            sequence,
            terminal_digest: origin_digest,
        });
        root.prefixes.sort_by_key(|prefix| prefix.stream);
        Ok(Some(AuthoredOrigin {
            root: self.stage_replication_root(&root).await?,
            receipt: receipt_ref,
        }))
    }

    pub async fn origin_receipt(
        &self,
        snapshot: &ManifestSnapshot,
        key: OriginKeyV1,
    ) -> Result<Option<OriginReceiptV1>> {
        let Some(root_ref) = snapshot.manifest.replication_root else {
            return Ok(None);
        };
        let root = self.read_replication_root(root_ref).await?;
        let Some(reference) = self
            .catalog
            .get(root.accepted_origins, origin_key_digest(key))
            .await?
        else {
            return Ok(None);
        };
        let receipt: OriginReceiptV1 = self
            .records
            .get(reference, RecordKind::OriginReceipt)
            .await?;
        if receipt.schema_version != 1 || receipt.origin_key != key {
            return Err(Error::Corrupt("origin receipt"));
        }
        Ok(Some(receipt))
    }

    pub async fn origin_publication(
        &self,
        receipt: OriginReceiptV1,
    ) -> Result<OriginPublicationV1> {
        if receipt.schema_version != 1
            || receipt.envelope.kind != RecordKind::Origin
            || receipt.envelope.attempt.is_some()
        {
            return Err(Error::Corrupt("origin receipt"));
        }
        let origin: OriginPublicationV1 = self
            .records
            .get(receipt.envelope, RecordKind::Origin)
            .await?;
        if origin.origin_key != receipt.origin_key || origin.digest()? != receipt.origin_digest {
            return Err(Error::Corrupt("origin receipt digest"));
        }
        Ok(origin)
    }

    pub(super) async fn audit_replication(
        &self,
        manifest: &Manifest,
        max_entries: usize,
    ) -> Result<()> {
        let Some(root_ref) = manifest.replication_root else {
            return Ok(());
        };
        let root = self.read_replication_root(root_ref).await?;
        let entries = self
            .catalog
            .entries(root.accepted_origins, max_entries)
            .await?;
        let mut observed = BTreeMap::<OriginStreamV1, BTreeMap<u64, OriginDigest>>::new();
        for (key, reference) in entries {
            if reference.kind != RecordKind::OriginReceipt || reference.attempt.is_some() {
                return Err(Error::Corrupt("origin receipt reference"));
            }
            let receipt: OriginReceiptV1 = self
                .records
                .get(reference, RecordKind::OriginReceipt)
                .await?;
            if key != origin_key_digest(receipt.origin_key)
                || receipt.accepted.replica != root.local_stream.replica
                || receipt.accepted.log_epoch != manifest.epoch
                || receipt.accepted.position.0 == 0
                || receipt.accepted.position > manifest.published
            {
                return Err(Error::Corrupt("origin receipt delivery"));
            }
            let origin = self.origin_publication(receipt).await?;
            for prefix in &origin.causal_frontier {
                let dependency_key = OriginKeyV1 {
                    stream: prefix.stream,
                    sequence: prefix.sequence,
                };
                let dependency_ref = self
                    .catalog
                    .get(root.accepted_origins, origin_key_digest(dependency_key))
                    .await?
                    .ok_or(Error::Corrupt("origin causal dependency"))?;
                let dependency: OriginReceiptV1 = self
                    .records
                    .get(dependency_ref, RecordKind::OriginReceipt)
                    .await?;
                if dependency.origin_digest != prefix.terminal_digest {
                    return Err(Error::Corrupt("origin causal digest"));
                }
            }
            observed
                .entry(origin.origin_key.stream)
                .or_default()
                .insert(origin.origin_key.sequence, receipt.origin_digest);
        }
        for prefix in &root.prefixes {
            let stream = observed
                .get(&prefix.stream)
                .ok_or(Error::Corrupt("origin prefix stream"))?;
            if stream.get(&prefix.sequence) != Some(&prefix.terminal_digest)
                || stream.keys().next_back().copied() != Some(prefix.sequence)
            {
                return Err(Error::Corrupt("origin prefix terminal"));
            }
        }
        let local = observed.get(&root.local_stream);
        if root.local_sequence == 0 {
            if local.is_some_and(|entries| !entries.is_empty()) {
                return Err(Error::Corrupt("empty local origin stream"));
            }
        } else {
            let local = local.ok_or(Error::Corrupt("local origin stream"))?;
            if local.len() as u64 != root.local_sequence
                || local.keys().copied().ne(1..=root.local_sequence)
                || local.get(&root.local_sequence) != root.local_terminal_digest.as_ref()
            {
                return Err(Error::Corrupt("local origin continuity"));
            }
        }
        Ok(())
    }
}

pub(super) fn origin_key_digest(key: OriginKeyV1) -> RecordDigest {
    let mut hash = Sha256::new_with_prefix(b"jj-store/origin-key/v1\0");
    hash.update(key.stream.repository.as_bytes());
    hash.update(key.stream.replica.as_bytes());
    hash.update(key.stream.writer_epoch.as_bytes());
    hash.update(key.sequence.to_be_bytes());
    RecordDigest::new(hash.finalize().into())
}

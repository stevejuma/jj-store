//! One source-authored causal baseline for a replication-disabled format-2 repository.
use super::*;
use crate::records::RecordKind;

const MAX_SEED_RESULT: u64 = 1024 * 1024;

#[derive(Clone, Copy, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct SeedReplicationRequestV1 {
    pub schema_version: u32,
    pub principal: PrincipalId,
    pub request_id: RequestId,
    pub publication_id: PublicationId,
    pub replica: ReplicaId,
    pub writer_epoch: WriterEpoch,
}

impl SeedReplicationRequestV1 {
    fn validate(&self) -> Result<()> {
        if self.schema_version != 1 {
            return Err(Error::Corrupt("replication seed request"));
        }
        Ok(())
    }

    fn intent(&self, repository: RepositoryId) -> Result<IntentDigest> {
        let mut hash = Sha256::new_with_prefix(b"jj-store/replication-seed/v1\0");
        hash.update(records::encode(&(repository, self), 1024 * 1024)?);
        Ok(IntentDigest::new(hash.finalize().into()))
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct SeedReplicationResultV1 {
    pub schema_version: u32,
    pub receipt: OriginReceiptV1,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(tag = "status", rename_all = "snake_case", deny_unknown_fields)]
pub enum SeedReplicationOutcomeV1 {
    Seeded(SeedReplicationResultV1),
    OutcomeUnknown {
        request_id: RequestId,
        publication_id: PublicationId,
    },
}

impl Coordinator {
    /// Upgrade a format-2 repository that has no replication catalog. The
    /// baseline is one real first origin, not reconstructed historical events.
    pub async fn seed_replication(
        &self,
        request: &SeedReplicationRequestV1,
    ) -> Result<SeedReplicationOutcomeV1> {
        request.validate()?;
        let intent = request.intent(self.records.repository)?;
        let unknown = || SeedReplicationOutcomeV1::OutcomeUnknown {
            request_id: request.request_id,
            publication_id: request.publication_id,
        };
        match tokio::time::timeout(
            self.limits.timeout,
            self.seed_replication_inner(request, intent),
        )
        .await
        {
            Ok(Ok(result)) => Ok(SeedReplicationOutcomeV1::Seeded(result)),
            Ok(Err(Error::OutcomeUnknown)) | Err(_) => Ok(unknown()),
            Ok(Err(error)) => Err(error),
        }
    }

    async fn seed_replication_inner(
        &self,
        request: &SeedReplicationRequestV1,
        intent: IntentDigest,
    ) -> Result<SeedReplicationResultV1> {
        for _ in 0..self.limits.cas_attempts {
            let base = self.read().await?;
            if base.manifest.thread_root.is_some() || base.manifest.reaction_root.is_some() {
                return Err(Error::Unsupported(
                    "Thread/Reaction reducer 3 is not qualified",
                ));
            }

            self.guard
                .authorize_replication_seed(&base, request)
                .await?;
            if let Some(receipt) = self
                .lookup_identity(
                    &base.manifest,
                    request.principal,
                    request.request_id,
                    request.publication_id,
                    intent,
                )
                .await?
            {
                return self.seed_result(&base, request, &receipt.result).await;
            }
            if base.manifest.work_root.is_some() {
                return Err(Error::Unsupported("WorkStore origin replication"));
            }
            if !matches!(base.manifest.format, 2 | 4) || base.manifest.replication_root.is_some() {
                return Err(Error::AlreadyExists);
            }
            let domains = base
                .manifest
                .domains()
                .ok_or(Error::Corrupt("replication seed domains"))?;
            self.inventory(&base, self.limits.inventory.max_nodes)
                .await?;
            self.agent_store(domains.agent.root).await?;
            self.review_store(domains.review.root).await?;
            if let Some(git) = domains.git {
                let staged = self.git_store(git.root).await?;
                let accepted = self.accepted_code_operations(&base).await?;
                self.operation_heads
                    .verify_git(&staged, None, Some(&accepted))
                    .await?;
            }

            let position = WalPosition(
                base.manifest
                    .published
                    .0
                    .checked_add(1)
                    .ok_or(Error::Limit("publication sequence"))?,
            );
            let stream = OriginStreamV1 {
                repository: self.records.repository,
                replica: request.replica,
                writer_epoch: request.writer_epoch,
            };
            let origin_key = OriginKeyV1 {
                stream,
                sequence: 1,
            };
            let publication = OriginPublicationV1 {
                schema_version: 1,
                origin_key,
                previous_origin_digest: None,
                causal_frontier: Vec::new(),
                semantic_format_version: if domains.git.is_some() { 2 } else { 1 },
                principal: request.principal,
                request_id: request.request_id,
                publication_id: request.publication_id,
                intent,
                published_operations: base.manifest.heads.clone(),
                agent_root: Some(domains.agent.root),
                review_root: Some(domains.review.root),
                git_root: domains.git.map(|s| s.root),
                required_inventory_root: base
                    .manifest
                    .inventory
                    .ok_or(Error::Corrupt("replication seed inventory"))?,
                changed_domains: [
                    OriginDomainV1::Repository,
                    OriginDomainV1::Code,
                    OriginDomainV1::Agent,
                    OriginDomainV1::Review,
                ]
                .into_iter()
                .chain(domains.git.map(|_| OriginDomainV1::Git))
                .collect(),
                domain_facts: None,
            };
            publication.validate(self.records.repository)?;
            let origin_digest = publication.digest()?;
            let envelope = self
                .records
                .put(RecordKind::Origin, None, &publication)
                .await?;
            let origin_receipt = OriginReceiptV1 {
                schema_version: 1,
                origin_key,
                origin_digest,
                envelope,
                accepted: LocalDeliveryV1 {
                    replica: request.replica,
                    log_epoch: base.manifest.epoch,
                    position,
                },
            };
            let origin_receipt_ref = self
                .records
                .put(RecordKind::OriginReceipt, None, &origin_receipt)
                .await?;
            let accepted_origins = self
                .catalog
                .update(
                    None,
                    BTreeMap::from([(
                        super::replication::origin_key_digest(origin_key),
                        origin_receipt_ref,
                    )]),
                )
                .await?;
            let replication_root = self
                .stage_replication_root(&ReplicationRootV1 {
                    schema_version: 1,
                    local_stream: stream,
                    local_sequence: 1,
                    local_terminal_digest: Some(origin_digest),
                    accepted_origins,
                    prefixes: vec![OriginPrefixV1 {
                        stream,
                        sequence: 1,
                        terminal_digest: origin_digest,
                    }],
                })
                .await?;
            let result = SeedReplicationResultV1 {
                schema_version: 1,
                receipt: origin_receipt,
            };
            let result_bytes = records::encode(&result, MAX_SEED_RESULT)?;
            let after = Snapshot {
                epoch: base.manifest.epoch,
                position,
                code_state_version: base.manifest.code_state_version,
                heads: base.manifest.heads.clone(),
            };
            let receipt = Receipt {
                principal: request.principal,
                request_id: request.request_id,
                publication_id: request.publication_id,
                intent,
                accepted: after.clone(),
                result: result_bytes,
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
                inventory: base.manifest.inventory,
                settings: base.manifest.settings,
                kind: SemanticKind::ReplicationSeeded,
                published_operations: BTreeSet::new(),
                before_domains: Some(domains),
                after_domains: Some(domains),
                origins: vec![origin_receipt_ref],
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
                tail: Some(tail),
                receipts,
                history,
                replication_root: Some(replication_root),
                ..base.manifest.clone()
            };
            self.guard
                .authorize_replication_seed(&base, request)
                .await?;
            match self.write_manifest(&base, manifest).await {
                Ok(Some(_)) => return Ok(result),
                Ok(None) => continue,
                Err(Error::OutcomeUnknown) => {
                    let latest = self.read().await.map_err(|_| Error::OutcomeUnknown)?;
                    self.guard
                        .authorize_replication_seed(&latest, request)
                        .await?;
                    let receipt = self
                        .lookup_identity(
                            &latest.manifest,
                            request.principal,
                            request.request_id,
                            request.publication_id,
                            intent,
                        )
                        .await?
                        .ok_or(Error::OutcomeUnknown)?;
                    return self.seed_result(&latest, request, &receipt.result).await;
                }
                Err(error) => return Err(error),
            }
        }
        Err(Error::Contended)
    }

    async fn seed_result(
        &self,
        snapshot: &ManifestSnapshot,
        request: &SeedReplicationRequestV1,
        bytes: &[u8],
    ) -> Result<SeedReplicationResultV1> {
        let result: SeedReplicationResultV1 = records::decode(bytes)?;
        if result.schema_version != 1
            || result.receipt.origin_key.stream.replica != request.replica
            || result.receipt.origin_key.stream.writer_epoch != request.writer_epoch
            || self
                .origin_receipt(snapshot, result.receipt.origin_key)
                .await?
                != Some(result.receipt)
        {
            return Err(Error::Corrupt("replication seed result"));
        }
        Ok(result)
    }
}

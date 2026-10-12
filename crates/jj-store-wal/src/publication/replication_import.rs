//! Atomic imported-origin publication through the destination manifest CAS.
use super::*;
use crate::records::RecordKind;

const MAX_IMPORT_ORIGINS: usize = 128;
const MAX_IMPORT_RESULT: u64 = 1024 * 1024;

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct OriginEnvelopeV1 {
    pub origin_digest: OriginDigest,
    pub publication: OriginPublicationV1,
}

impl OriginEnvelopeV1 {
    fn validate(&self, repository: RepositoryId) -> Result<()> {
        self.publication.validate(repository)?;
        if self.publication.digest()? != self.origin_digest {
            return Err(Error::Corrupt("origin envelope digest"));
        }
        Ok(())
    }
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct ApplyOriginsRequestV1 {
    pub schema_version: u32,
    pub principal: PrincipalId,
    pub request_id: RequestId,
    pub publication_id: PublicationId,
    pub origins: Vec<OriginEnvelopeV1>,
}

impl ApplyOriginsRequestV1 {
    fn validate(&self, repository: RepositoryId) -> Result<()> {
        if self.schema_version != 1
            || self.origins.is_empty()
            || self.origins.len() > MAX_IMPORT_ORIGINS
        {
            return Err(Error::Limit("import origins"));
        }
        let mut keys = BTreeSet::new();
        for origin in &self.origins {
            origin.validate(repository)?;
            if !keys.insert(origin.publication.origin_key) {
                return Err(Error::Corrupt("duplicate imported origin"));
            }
        }
        Ok(())
    }

    fn intent(&self) -> Result<IntentDigest> {
        Ok(IntentDigest::new(
            Sha256::digest(records::encode(self, 8 * 1024 * 1024)?).into(),
        ))
    }
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct ApplyOriginsResultV1 {
    pub schema_version: u32,
    pub publication_position: Option<WalPosition>,
    pub receipts: Vec<OriginReceiptV1>,
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(tag = "status", rename_all = "snake_case", deny_unknown_fields)]
pub enum ApplyOriginsOutcomeV1 {
    Applied(ApplyOriginsResultV1),
    OutcomeUnknown {
        request_id: RequestId,
        publication_id: PublicationId,
    },
}

impl Coordinator {
    pub async fn apply_origins(
        &self,
        request: &ApplyOriginsRequestV1,
    ) -> Result<ApplyOriginsOutcomeV1> {
        request.validate(self.records.repository)?;
        let intent = request.intent()?;
        let unknown = || ApplyOriginsOutcomeV1::OutcomeUnknown {
            request_id: request.request_id,
            publication_id: request.publication_id,
        };
        match tokio::time::timeout(
            self.limits.timeout,
            self.apply_origins_inner(request, intent),
        )
        .await
        {
            Ok(Ok(result)) => Ok(ApplyOriginsOutcomeV1::Applied(result)),
            Ok(Err(Error::OutcomeUnknown)) | Err(_) => Ok(unknown()),
            Ok(Err(error)) => Err(error),
        }
    }

    async fn apply_origins_inner(
        &self,
        request: &ApplyOriginsRequestV1,
        intent: IntentDigest,
    ) -> Result<ApplyOriginsResultV1> {
        let _lease = self.guard.publication_lease().await?;
        let authorization_base = self.read().await?;
        self.guard
            .authorize_replication(&authorization_base, request)
            .await?;
        let mut inventories = BTreeMap::new();
        for envelope in &request.origins {
            inventories.insert(
                envelope.publication.origin_key,
                self.verify_import_closure(&envelope.publication).await?,
            );
        }
        for _ in 0..self.limits.cas_attempts {
            let base = self.read().await?;
            if base.manifest.thread_root.is_some() || base.manifest.reaction_root.is_some() {
                return Err(Error::Unsupported(
                    "Thread/Reaction reducer 3 is not qualified",
                ));
            }

            self.guard.authorize_replication(&base, request).await?;
            self.ensure_origin_replication().await?;
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
                return self.import_result(&base, request, &receipt.result).await;
            }
            let root_ref = base
                .manifest
                .replication_root
                .ok_or(Error::Corrupt("replication root missing"))?;
            let mut root = self.read_replication_root(root_ref).await?;
            let accepted = self.accepted_origin_digests(&root).await?;
            let mut known = BTreeMap::new();
            let mut unseen = BTreeMap::new();
            for envelope in &request.origins {
                let key = envelope.publication.origin_key;
                if let Some(digest) = accepted.get(&key) {
                    if *digest != envelope.origin_digest {
                        return Err(Error::Corrupt("origin key digest mismatch"));
                    }
                    let receipt = self
                        .origin_receipt(&base, key)
                        .await?
                        .ok_or(Error::Corrupt("accepted origin receipt missing"))?;
                    let selected = self.origin_publication(receipt).await?;
                    if selected != envelope.publication {
                        return Err(Error::Corrupt("accepted origin envelope mismatch"));
                    }
                    known.insert(key, receipt);
                } else {
                    unseen.insert(key, envelope.clone());
                }
            }
            if unseen.is_empty() {
                return self.ordered_import_result(request, None, &known);
            }
            let ordered = order_unseen(&root, &accepted, unseen)?;
            let position = WalPosition(
                base.manifest
                    .published
                    .0
                    .checked_add(1)
                    .ok_or(Error::Limit("publication sequence"))?,
            );
            let mut inventory_changes = BTreeMap::new();
            let mut heads = base.manifest.heads.clone();
            let mut published_operations = BTreeSet::new();
            let before_domains = base.manifest.domains();
            let mut after_domains = before_domains;
            let mut agent_changed = false;
            let mut review_changed = false;
            let mut git_changed = false;
            let mut accepted_code = if ordered.iter().any(|e| e.publication.git_root.is_some()) {
                self.accepted_code_operations(&base).await?
            } else {
                BTreeSet::new()
            };
            for envelope in &ordered {
                for (key, reference) in inventories
                    .get(&envelope.publication.origin_key)
                    .ok_or(Error::Corrupt("origin inventory missing"))?
                {
                    match self.catalog.get(base.manifest.inventory, *key).await? {
                        Some(existing) if existing != *reference => {
                            return Err(Error::Corrupt("import inventory disagreement"));
                        }
                        Some(_) => {}
                        None => {
                            inventory_changes.insert(*key, *reference);
                        }
                    }
                }
                if envelope
                    .publication
                    .changed_domains
                    .contains(&OriginDomainV1::Code)
                {
                    heads.extend(envelope.publication.published_operations.iter().copied());
                    accepted_code.extend(envelope.publication.published_operations.iter().copied());
                    published_operations
                        .extend(envelope.publication.published_operations.iter().copied());
                }
                if let Some(authored) = envelope.publication.git_root {
                    let domains = after_domains
                        .as_mut()
                        .ok_or(Error::Corrupt("Git domain unavailable"))?;
                    let previous = match domains.git {
                        Some(state) => self.git_store(state.root).await?,
                        None => self.empty_git_store().await?,
                    };
                    let source = self.git_store(authored).await?;
                    let values: Vec<_> = source.values.into_values().collect();
                    let reduced = self.stage_git_records(&previous, &values).await?;
                    self.operation_heads
                        .verify_git(
                            &reduced,
                            Some(&previous),
                            if envelope.publication.is_seed_baseline() {
                                None
                            } else {
                                Some(&accepted_code)
                            },
                        )
                        .await?;
                    let changed = domains.git.is_none() || reduced.reference != previous.reference;
                    git_changed |= changed;
                    if changed || domains.git.is_some() {
                        domains.git = Some(DomainState {
                            root: reduced.reference,
                            version: domains.git.map_or(0, |s| s.version),
                        });
                    }
                }
                if envelope
                    .publication
                    .changed_domains
                    .contains(&OriginDomainV1::Agent)
                {
                    let authored = envelope
                        .publication
                        .agent_root
                        .ok_or(Error::Corrupt("origin agent root missing"))?;
                    let domains = after_domains
                        .as_mut()
                        .ok_or(Error::Corrupt("agent domain unavailable"))?;
                    let baseline;
                    let delta = if envelope.publication.is_seed_baseline() {
                        baseline = self.baseline_agent_delta(authored).await?;
                        baseline.as_ref()
                    } else {
                        envelope
                            .publication
                            .domain_facts
                            .as_ref()
                            .and_then(|facts| facts.agent.as_ref())
                    };
                    let reduced = match delta {
                        Some(delta) => {
                            self.reduce_imported_agent_facts(domains.agent.root, authored, delta)
                                .await?
                        }
                        None if envelope.publication.is_seed_baseline() => domains.agent.root,
                        None => return Err(Error::Corrupt("origin agent facts missing")),
                    };
                    agent_changed |= reduced != domains.agent.root;
                    domains.agent.root = reduced;
                }
                if envelope
                    .publication
                    .changed_domains
                    .contains(&OriginDomainV1::Review)
                {
                    let authored = envelope
                        .publication
                        .review_root
                        .ok_or(Error::Corrupt("origin review root missing"))?;
                    let domains = after_domains
                        .as_mut()
                        .ok_or(Error::Corrupt("review domain unavailable"))?;
                    let baseline;
                    let delta = if envelope.publication.is_seed_baseline() {
                        baseline = self.baseline_review_delta(authored).await?;
                        baseline.as_ref()
                    } else {
                        envelope
                            .publication
                            .domain_facts
                            .as_ref()
                            .and_then(|facts| facts.review.as_ref())
                    };
                    let reduced = match delta {
                        Some(delta) => {
                            self.reduce_imported_review_facts(domains.review.root, authored, delta)
                                .await?
                        }
                        None if envelope.publication.is_seed_baseline() => domains.review.root,
                        None => return Err(Error::Corrupt("origin review facts missing")),
                    };
                    review_changed |= reduced != domains.review.root;
                    domains.review.root = reduced;
                }
            }
            if let Some(domains) = after_domains.as_mut() {
                if agent_changed {
                    domains.agent.version = domains
                        .agent
                        .version
                        .checked_add(1)
                        .ok_or(Error::Limit("agent state version"))?;
                }
                if review_changed {
                    domains.review.version = domains
                        .review
                        .version
                        .checked_add(1)
                        .ok_or(Error::Limit("review state version"))?;
                }
                if git_changed {
                    let git = domains
                        .git
                        .as_mut()
                        .ok_or(Error::Corrupt("Git domain reduction"))?;
                    git.version = git
                        .version
                        .checked_add(1)
                        .ok_or(Error::Limit("Git state version"))?;
                }
            }
            let candidates = heads;
            let heads = self
                .operation_heads
                .maximal_heads(candidates.clone())
                .await?;
            if heads.is_empty() || heads.len() > MAX_HEADS || !heads.is_subset(&candidates) {
                return Err(Error::Limit("operation heads"));
            }
            let inventory = self
                .catalog
                .update(base.manifest.inventory, inventory_changes)
                .await?;
            let code_changed = heads != base.manifest.heads;
            let code_state_version = if code_changed {
                base.manifest
                    .code_state_version
                    .checked_add(1)
                    .ok_or(Error::Limit("code state version"))?
            } else {
                base.manifest.code_state_version
            };
            let after = Snapshot {
                epoch: base.manifest.epoch,
                position,
                code_state_version,
                heads: heads.clone(),
            };
            let mut imported = BTreeMap::new();
            let mut origin_refs = Vec::with_capacity(ordered.len());
            for envelope in ordered {
                let envelope_ref = self
                    .records
                    .put(RecordKind::Origin, None, &envelope.publication)
                    .await?;
                let receipt = OriginReceiptV1 {
                    schema_version: 1,
                    origin_key: envelope.publication.origin_key,
                    origin_digest: envelope.origin_digest,
                    envelope: envelope_ref,
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
                        BTreeMap::from([(
                            super::replication::origin_key_digest(receipt.origin_key),
                            receipt_ref,
                        )]),
                    )
                    .await?;
                update_prefix(
                    &mut root.prefixes,
                    receipt.origin_key,
                    receipt.origin_digest,
                );
                imported.insert(receipt.origin_key, receipt);
                origin_refs.push(receipt_ref);
            }
            let replication_root = self.stage_replication_root(&root).await?;
            known.extend(imported);
            let result = self.ordered_import_result(request, Some(position), &known)?;
            let result_bytes = records::encode(&result, MAX_IMPORT_RESULT)?;
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
                inventory,
                settings: base.manifest.settings,
                kind: SemanticKind::ReplicationImported,
                published_operations,
                before_domains,
                after_domains,
                origins: origin_refs,
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
                format: if after_domains.and_then(|d| d.git).is_some() {
                    4
                } else {
                    base.manifest.format
                },
                git_root: after_domains.and_then(|d| d.git).map(|s| s.root),
                git_state_version: after_domains.and_then(|d| d.git).map_or(0, |s| s.version),
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
                code_state_version,
                agent_root: after_domains.map(|domains| domains.agent.root),
                agent_state_version: after_domains.map_or(0, |domains| domains.agent.version),
                review_root: after_domains.map(|domains| domains.review.root),
                review_state_version: after_domains.map_or(0, |domains| domains.review.version),
                replication_root: Some(replication_root),
                ..base.manifest.clone()
            };
            self.guard.authorize_replication(&base, request).await?;
            self.ensure_origin_replication().await?;
            match self.write_manifest(&base, manifest).await {
                Ok(Some(_)) => return Ok(result),
                Ok(None) => continue,
                Err(Error::OutcomeUnknown) => {
                    let latest = self.read().await.map_err(|_| Error::OutcomeUnknown)?;
                    self.guard.authorize_replication(&latest, request).await?;
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
                    return self.import_result(&latest, request, &receipt.result).await;
                }
                Err(error) => return Err(error),
            }
        }
        Err(Error::Contended)
    }

    async fn verify_import_closure(
        &self,
        publication: &OriginPublicationV1,
    ) -> Result<BTreeMap<RecordDigest, RecordRef>> {
        let limit = self.limits.inventory.max_nodes;
        let entries = self
            .catalog
            .entries(Some(publication.required_inventory_root), limit)
            .await?;
        let nodes = reader::inventory(
            self.records.store.clone(),
            self.records.repository,
            Some(publication.required_inventory_root),
            &publication.published_operations,
            limit,
        )
        .await?;
        let mut native = BTreeSet::new();
        for node in nodes {
            let actual = self
                .source
                .load_verified(node.key, self.limits.inventory.max_new_bytes)
                .await?;
            if actual != node {
                return Err(Error::Corrupt("imported native descriptor changed"));
            }
            native.insert(node.key);
        }
        if publication.changed_domains.contains(&OriginDomainV1::Agent) {
            let (_, required) = self
                .read_agent_root_dependencies(
                    publication
                        .agent_root
                        .ok_or(Error::Corrupt("origin agent root missing"))?,
                )
                .await?;
            if !required.is_subset(&native) {
                return Err(Error::Corrupt("imported agent native dependency"));
            }
        }
        if publication
            .changed_domains
            .contains(&OriginDomainV1::Review)
        {
            let (_, required) = self
                .read_review_root_dependencies(
                    publication
                        .review_root
                        .ok_or(Error::Corrupt("origin review root missing"))?,
                )
                .await?;
            if !required.is_subset(&native) {
                return Err(Error::Corrupt("imported review native dependency"));
            }
        }
        if let Some(root) = publication.git_root
            && !self.git_store(root).await?.native.is_subset(&native)
        {
            return Err(Error::Corrupt("imported Git native dependency"));
        }
        Ok(entries)
    }

    pub(super) async fn accepted_code_operations(
        &self,
        snapshot: &ManifestSnapshot,
    ) -> Result<BTreeSet<OperationId>> {
        let mut code = snapshot.manifest.heads.clone();
        let mut after = WalPosition(0);
        while after < snapshot.manifest.published {
            let entries = self.committed(snapshot, after, 1024).await?;
            if entries.is_empty() {
                return Err(Error::Corrupt("accepted code history gap"));
            }
            for entry in entries {
                code.extend(entry.published_operations);
                after = entry.after.position;
            }
            if code.len() > self.limits.inventory.max_nodes {
                return Err(Error::Limit("accepted code history"));
            }
        }
        if let Some(reference) = snapshot.manifest.replication_root {
            let root = self.read_replication_root(reference).await?;
            for reference in self
                .catalog
                .entries(root.accepted_origins, self.limits.inventory.max_nodes)
                .await?
                .into_values()
            {
                let receipt: OriginReceiptV1 = self
                    .records
                    .get(reference, RecordKind::OriginReceipt)
                    .await?;
                let origin = self.origin_publication(receipt).await?;
                code.extend(origin.published_operations);
                if code.len() > self.limits.inventory.max_nodes {
                    return Err(Error::Limit("accepted code origins"));
                }
            }
        }
        Ok(code)
    }

    async fn accepted_origin_digests(
        &self,
        root: &ReplicationRootV1,
    ) -> Result<BTreeMap<OriginKeyV1, OriginDigest>> {
        let entries = self
            .catalog
            .entries(root.accepted_origins, self.limits.inventory.max_nodes)
            .await?;
        let mut accepted = BTreeMap::new();
        for (key, reference) in entries {
            let receipt: OriginReceiptV1 = self
                .records
                .get(reference, RecordKind::OriginReceipt)
                .await?;
            if key != super::replication::origin_key_digest(receipt.origin_key)
                || accepted
                    .insert(receipt.origin_key, receipt.origin_digest)
                    .is_some()
            {
                return Err(Error::Corrupt("accepted origin catalog"));
            }
        }
        Ok(accepted)
    }

    fn ordered_import_result(
        &self,
        request: &ApplyOriginsRequestV1,
        position: Option<WalPosition>,
        receipts: &BTreeMap<OriginKeyV1, OriginReceiptV1>,
    ) -> Result<ApplyOriginsResultV1> {
        Ok(ApplyOriginsResultV1 {
            schema_version: 1,
            publication_position: position,
            receipts: request
                .origins
                .iter()
                .map(|origin| {
                    receipts
                        .get(&origin.publication.origin_key)
                        .copied()
                        .ok_or(Error::Corrupt("import result receipt missing"))
                })
                .collect::<Result<Vec<_>>>()?,
        })
    }

    async fn import_result(
        &self,
        snapshot: &ManifestSnapshot,
        request: &ApplyOriginsRequestV1,
        bytes: &[u8],
    ) -> Result<ApplyOriginsResultV1> {
        let result: ApplyOriginsResultV1 = records::decode(bytes)?;
        if result.schema_version != 1 || result.receipts.len() != request.origins.len() {
            return Err(Error::Corrupt("import result"));
        }
        for (expected, receipt) in request.origins.iter().zip(&result.receipts) {
            if receipt.origin_key != expected.publication.origin_key
                || receipt.origin_digest != expected.origin_digest
                || self.origin_receipt(snapshot, receipt.origin_key).await? != Some(*receipt)
            {
                return Err(Error::Corrupt("import result receipt"));
            }
        }
        Ok(result)
    }
}

fn order_unseen(
    root: &ReplicationRootV1,
    accepted: &BTreeMap<OriginKeyV1, OriginDigest>,
    mut pending: BTreeMap<OriginKeyV1, OriginEnvelopeV1>,
) -> Result<Vec<OriginEnvelopeV1>> {
    let mut prefixes = root
        .prefixes
        .iter()
        .map(|prefix| (prefix.stream, (prefix.sequence, prefix.terminal_digest)))
        .collect::<BTreeMap<_, _>>();
    let mut available = accepted.clone();
    let mut ordered = Vec::with_capacity(pending.len());
    while !pending.is_empty() {
        let ready = pending.iter().find_map(|(key, envelope)| {
            let expected = prefixes
                .get(&key.stream)
                .map_or(1, |(sequence, _)| sequence + 1);
            let previous = prefixes.get(&key.stream).map(|(_, digest)| *digest);
            let causal_ready = envelope.publication.causal_frontier.iter().all(|prefix| {
                available.get(&OriginKeyV1 {
                    stream: prefix.stream,
                    sequence: prefix.sequence,
                }) == Some(&prefix.terminal_digest)
            });
            (key.sequence == expected
                && envelope.publication.previous_origin_digest == previous
                && causal_ready)
                .then_some(*key)
        });
        let Some(key) = ready else {
            return Err(Error::Corrupt("origin causal gap"));
        };
        let envelope = pending.remove(&key).expect("selected pending origin");
        available.insert(key, envelope.origin_digest);
        prefixes.insert(key.stream, (key.sequence, envelope.origin_digest));
        ordered.push(envelope);
    }
    Ok(ordered)
}

fn update_prefix(prefixes: &mut Vec<OriginPrefixV1>, key: OriginKeyV1, digest: OriginDigest) {
    prefixes.retain(|prefix| prefix.stream != key.stream);
    prefixes.push(OriginPrefixV1 {
        stream: key.stream,
        sequence: key.sequence,
        terminal_digest: digest,
    });
    prefixes.sort_by_key(|prefix| prefix.stream);
}

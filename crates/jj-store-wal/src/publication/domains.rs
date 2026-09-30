//! Independent application-domain roots published through the repository CAS.
use super::*;

#[derive(Clone, Copy, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum Domain {
    Agent,
    Review,
    Work,
}
impl Domain {
    fn record_kind(self) -> RecordKind {
        match self {
            Self::Agent => RecordKind::AgentRoot,
            Self::Review => RecordKind::ReviewRoot,
            Self::Work => RecordKind::WorkRoot,
        }
    }
    fn semantic_kind(self) -> SemanticKind {
        match self {
            Self::Agent => SemanticKind::AgentStateChanged,
            Self::Review => SemanticKind::ReviewStateChanged,
            Self::Work => SemanticKind::WorkStateChanged,
        }
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct DomainState {
    pub root: RecordRef,
    pub version: u64,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct RepositoryDomains {
    pub agent: DomainState,
    pub review: DomainState,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub work: Option<DomainState>,
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct DomainRequest {
    pub principal: PrincipalId,
    pub request_id: RequestId,
    pub publication_id: PublicationId,
    pub domain: Domain,
    pub expected: DomainState,
    pub root: RecordRef,
    pub result: Vec<u8>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub facts: Option<DomainFactsV1>,
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct DomainFactsV1 {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub agent: Option<AgentDeltaV1>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub review: Option<ReviewDeltaV1>,
}
impl DomainRequest {
    pub fn new(
        principal: PrincipalId,
        request_id: RequestId,
        publication_id: PublicationId,
        domain: Domain,
        expected: DomainState,
        root: RecordRef,
        result: Vec<u8>,
    ) -> Result<Self> {
        let request = Self {
            principal,
            request_id,
            publication_id,
            domain,
            expected,
            root,
            result,
            facts: None,
        };
        request.validate()?;
        Ok(request)
    }
    pub fn with_agent_facts(mut self, delta: AgentDeltaV1) -> Result<Self> {
        delta.validate().map_err(super::agent::agent_error)?;
        self.facts = Some(DomainFactsV1 {
            agent: Some(delta),
            review: None,
        });
        self.validate()?;
        Ok(self)
    }
    pub fn with_review_facts(mut self, delta: ReviewDeltaV1) -> Result<Self> {
        delta.validate().map_err(super::review::review_error)?;
        self.facts = Some(DomainFactsV1 {
            agent: None,
            review: Some(delta),
        });
        self.validate()?;
        Ok(self)
    }
    fn validate(&self) -> Result<()> {
        if self.result.len() > 1024 * 1024
            || self.root.kind != self.domain.record_kind()
            || self.expected.root.kind != self.domain.record_kind()
            || self.root.attempt.is_some()
            || self.expected.root.attempt.is_some()
            || self.facts.as_ref().is_some_and(|facts| match self.domain {
                Domain::Agent => facts.agent.is_none() || facts.review.is_some(),
                Domain::Review => facts.agent.is_some() || facts.review.is_none(),
                Domain::Work => true,
            })
        {
            return Err(Error::Corrupt("domain publication request"));
        }
        Ok(())
    }
    fn intent(&self) -> Result<IntentDigest> {
        Ok(IntentDigest::new(
            Sha256::digest(records::encode(self, 8 * 1024 * 1024)?).into(),
        ))
    }
}

impl Coordinator {
    pub async fn publish_domain(&self, request: &DomainRequest) -> Result<PublishOutcome> {
        request.validate()?;
        let intent = request.intent()?;
        let unknown = || PublishOutcome::OutcomeUnknown {
            request_id: request.request_id,
            publication_id: request.publication_id,
        };
        match tokio::time::timeout(
            self.limits.timeout,
            self.publish_domain_inner(request, intent),
        )
        .await
        {
            Ok(Ok(receipt)) => Ok(PublishOutcome::Committed(receipt)),
            Ok(Err(Error::OutcomeUnknown)) | Err(_) => Ok(unknown()),
            Ok(Err(error)) => Err(error),
        }
    }

    pub(super) async fn publish_domain_inner(
        &self,
        request: &DomainRequest,
        intent: IntentDigest,
    ) -> Result<Receipt> {
        let native_roots = self.read_domain_root(request.domain, request.root).await?;
        for _ in 0..self.limits.cas_attempts {
            let base = self.read().await?;
            self.guard.authorize_domain(&base, request).await?;
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
                return Ok(receipt);
            }
            if request.domain == Domain::Work && base.manifest.replication_root.is_some() {
                return Err(Error::Corrupt("WorkStore replication unsupported"));
            }
            let mut current = base.manifest.domain_state(request.domain);
            if request.domain == Domain::Work && current.is_none() {
                current = Some(DomainState {
                    root: self.empty_work_store().await?.reference(),
                    version: 0,
                });
            }
            if current != Some(request.expected) {
                return Err(Error::Stale);
            }
            let inventory = if native_roots.is_empty() {
                base.manifest.inventory
            } else {
                let delta = self
                    .verify_delta(base.manifest.inventory, native_roots.iter().copied())
                    .await?;
                self.persist_inventory(base.manifest.inventory, &delta)
                    .await?
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
                code_state_version: base.manifest.code_state_version,
                heads: base.manifest.heads.clone(),
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
            let keys = self.receipt_keys(
                request.principal,
                request.request_id,
                request.publication_id,
            );
            let receipts = self
                .catalog
                .update(
                    base.manifest.receipts,
                    BTreeMap::from([(keys.0, receipt_ref), (keys.1, receipt_ref)]),
                )
                .await?;
            let before_domains = base
                .manifest
                .domains()
                .ok_or(Error::Corrupt("domain roots"))?;
            let mut after_domains = before_domains;
            let selected = DomainState {
                root: request.root,
                version: request
                    .expected
                    .version
                    .checked_add(1)
                    .ok_or(Error::Limit("domain version"))?,
            };
            match request.domain {
                Domain::Agent => after_domains.agent = selected,
                Domain::Review => after_domains.review = selected,
                Domain::Work => after_domains.work = Some(selected),
            }
            let changed_domain = match request.domain {
                Domain::Agent => OriginDomainV1::Agent,
                Domain::Review => OriginDomainV1::Review,
                Domain::Work => OriginDomainV1::Repository,
            };
            let origin = self
                .author_origin(
                    &base,
                    position,
                    request.principal,
                    request.request_id,
                    request.publication_id,
                    intent,
                    BTreeSet::new(),
                    Some(after_domains),
                    inventory,
                    BTreeSet::from([changed_domain]),
                    request.facts.clone(),
                )
                .await?;
            let replication_root = origin
                .as_ref()
                .map(|authored| authored.root)
                .or(base.manifest.replication_root);
            let origins = origin
                .into_iter()
                .map(|authored| authored.receipt)
                .collect();
            let attempt = records::new_attempt()?;
            let transaction = PublishedTransaction {
                attempt,
                repository: self.records.repository,
                before: base.manifest.snapshot(),
                after: after.clone(),
                previous_manifest: base.digest,
                previous: base.manifest.tail,
                receipt: receipt_ref,
                inventory,
                settings: base.manifest.settings,
                kind: request.domain.semantic_kind(),
                published_operations: BTreeSet::new(),
                before_domains: Some(before_domains),
                after_domains: Some(after_domains),
                origins,
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
                format: if after_domains.work.is_some() {
                    3
                } else {
                    base.manifest.format
                },
                work_root: after_domains.work.map(|s| s.root),
                work_state_version: after_domains.work.map_or(0, |s| s.version),
                revision: base
                    .manifest
                    .revision
                    .checked_add(1)
                    .ok_or(Error::Limit("manifest revision"))?,
                published: position,
                tail: Some(tail),
                receipts,
                history,
                inventory,
                agent_root: Some(after_domains.agent.root),
                agent_state_version: after_domains.agent.version,
                review_root: Some(after_domains.review.root),
                review_state_version: after_domains.review.version,
                replication_root,
                ..base.manifest.clone()
            };
            match self.write_manifest(&base, manifest).await {
                Ok(Some(_)) => return Ok(receipt),
                Ok(None) => continue,
                Err(Error::OutcomeUnknown) => {
                    let latest = self.read().await.map_err(|_| Error::OutcomeUnknown)?;
                    self.guard.authorize_domain(&latest, request).await?;
                    return self
                        .lookup_identity(
                            &latest.manifest,
                            request.principal,
                            request.request_id,
                            request.publication_id,
                            intent,
                        )
                        .await?
                        .ok_or(Error::OutcomeUnknown);
                }
                Err(error) => return Err(error),
            }
        }
        Err(Error::Contended)
    }

    pub(super) async fn read_domain_root(
        &self,
        domain: Domain,
        root: RecordRef,
    ) -> Result<BTreeSet<ObjectKey>> {
        match domain {
            Domain::Work => {
                self.work_store(root).await?;
                Ok(BTreeSet::new())
            }
            Domain::Agent => {
                let (_, native) = self.read_agent_root_dependencies(root).await?;
                Ok(native)
            }
            Domain::Review => {
                let (_, native) = self.read_review_root_dependencies(root).await?;
                Ok(native)
            }
        }
    }
}

//! Independent application-domain roots published through the repository CAS.
use super::*;

const MAX_DOMAIN_ROOT: u64 = 4 * 1024 * 1024;

#[derive(Clone, Copy, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum Domain {
    Agent,
    Review,
}
impl Domain {
    fn record_kind(self) -> RecordKind {
        match self {
            Self::Agent => RecordKind::AgentRoot,
            Self::Review => RecordKind::ReviewRoot,
        }
    }
    fn semantic_kind(self) -> SemanticKind {
        match self {
            Self::Agent => SemanticKind::AgentStateChanged,
            Self::Review => SemanticKind::ReviewStateChanged,
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
        };
        request.validate()?;
        Ok(request)
    }
    fn validate(&self) -> Result<()> {
        if self.result.len() > 1024 * 1024
            || self.root.kind != self.domain.record_kind()
            || self.expected.root.kind != self.domain.record_kind()
            || self.root.attempt.is_some()
            || self.expected.root.attempt.is_some()
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

#[derive(Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
struct ReviewRoot {
    format: u32,
    payload: Vec<u8>,
}

impl Coordinator {
    pub async fn stage_review_root(&self, payload: &[u8]) -> Result<RecordRef> {
        if payload.len() as u64 > MAX_DOMAIN_ROOT {
            return Err(Error::Limit("domain root"));
        }
        self.records
            .put(
                RecordKind::ReviewRoot,
                None,
                &ReviewRoot {
                    format: 1,
                    payload: payload.to_vec(),
                },
            )
            .await
    }

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

    async fn publish_domain_inner(
        &self,
        request: &DomainRequest,
        intent: IntentDigest,
    ) -> Result<Receipt> {
        self.read_domain_root(request.domain, request.root).await?;
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
            if base.manifest.domain_state(request.domain) != Some(request.expected) {
                return Err(Error::Stale);
            }
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
            }
            let attempt = records::new_attempt()?;
            let transaction = PublishedTransaction {
                attempt,
                repository: self.records.repository,
                before: base.manifest.snapshot(),
                after: after.clone(),
                previous_manifest: base.digest,
                previous: base.manifest.tail,
                receipt: receipt_ref,
                inventory: base.manifest.inventory,
                settings: base.manifest.settings,
                kind: request.domain.semantic_kind(),
                published_operations: BTreeSet::new(),
                before_domains: Some(before_domains),
                after_domains: Some(after_domains),
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
                agent_root: Some(after_domains.agent.root),
                agent_state_version: after_domains.agent.version,
                review_root: Some(after_domains.review.root),
                review_state_version: after_domains.review.version,
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

    pub(super) async fn read_domain_root(&self, domain: Domain, root: RecordRef) -> Result<()> {
        match domain {
            Domain::Agent => {
                self.read_agent_root(root).await?;
            }
            Domain::Review => {
                let value: ReviewRoot = self.records.get(root, RecordKind::ReviewRoot).await?;
                if value.format != 1 || value.payload.len() as u64 > MAX_DOMAIN_ROOT {
                    return Err(Error::Corrupt("domain root"));
                }
            }
        }
        Ok(())
    }
}

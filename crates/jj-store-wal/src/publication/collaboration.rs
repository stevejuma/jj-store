//! Non-destructive enablement of repository-owned Agent and Review state.
use super::*;

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct CollaborationRequest {
    pub principal: PrincipalId,
    pub request_id: RequestId,
    pub publication_id: PublicationId,
    pub result: Vec<u8>,
}

impl CollaborationRequest {
    pub fn new(
        principal: PrincipalId,
        request_id: RequestId,
        publication_id: PublicationId,
        result: Vec<u8>,
    ) -> Result<Self> {
        let request = Self {
            principal,
            request_id,
            publication_id,
            result,
        };
        request.validate()?;
        Ok(request)
    }

    fn validate(&self) -> Result<()> {
        if self.result.len() > 1024 * 1024 {
            return Err(Error::Limit("collaboration result"));
        }
        Ok(())
    }

    fn intent(&self) -> Result<IntentDigest> {
        let mut hash = Sha256::new_with_prefix(b"jj-store/collaboration-enable/v1\0");
        hash.update(records::encode(self, 2 * 1024 * 1024)?);
        Ok(IntentDigest::new(hash.finalize().into()))
    }
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum CollaborationOutcome {
    Enabled(Receipt),
    AlreadyEnabled(RepositoryDomains),
    OutcomeUnknown {
        request_id: RequestId,
        publication_id: PublicationId,
    },
}

impl Coordinator {
    pub async fn enable_collaboration(
        &self,
        request: &CollaborationRequest,
    ) -> Result<CollaborationOutcome> {
        request.validate()?;
        let intent = request.intent()?;
        let unknown = || CollaborationOutcome::OutcomeUnknown {
            request_id: request.request_id,
            publication_id: request.publication_id,
        };
        match tokio::time::timeout(
            self.limits.timeout,
            self.enable_collaboration_inner(request, intent),
        )
        .await
        {
            Ok(Ok(outcome)) => Ok(outcome),
            Ok(Err(Error::OutcomeUnknown)) | Err(_) => Ok(unknown()),
            Ok(Err(error)) => Err(error),
        }
    }

    async fn enable_collaboration_inner(
        &self,
        request: &CollaborationRequest,
        intent: IntentDigest,
    ) -> Result<CollaborationOutcome> {
        let mut roots = None;
        for _ in 0..self.limits.cas_attempts {
            let base = self.read().await?;
            self.guard.authorize_collaboration(&base, request).await?;
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
                return Ok(CollaborationOutcome::Enabled(receipt));
            }
            if let Some(domains) = base.manifest.domains() {
                return Ok(CollaborationOutcome::AlreadyEnabled(domains));
            }
            if base.manifest.format != 1 {
                return Err(Error::Corrupt("collaboration manifest format"));
            }
            let domains = match roots {
                Some(domains) => domains,
                None => {
                    let domains = RepositoryDomains {
                        work: None,
                        agent: DomainState {
                            root: self.stage_agent_root(&AgentRootV1::empty()).await?,
                            version: 0,
                        },
                        review: DomainState {
                            root: self.stage_review_root(&ReviewRootV1::empty()).await?,
                            version: 0,
                        },
                    };
                    roots = Some(domains);
                    domains
                }
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
                code_state_version: 0,
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
                kind: SemanticKind::CollaborationEnabled,
                published_operations: BTreeSet::new(),
                before_domains: None,
                after_domains: Some(domains),
                origins: Vec::new(),
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
                format: 2,
                revision: base
                    .manifest
                    .revision
                    .checked_add(1)
                    .ok_or(Error::Limit("manifest revision"))?,
                published: position,
                tail: Some(tail),
                receipts,
                history,
                code_state_version: 0,
                agent_root: Some(domains.agent.root),
                agent_state_version: 0,
                review_root: Some(domains.review.root),
                review_state_version: 0,
                domain_history_start: position.0,
                replication_root: base.manifest.replication_root,
                ..base.manifest.clone()
            };
            match self.write_manifest(&base, manifest).await {
                Ok(Some(_)) => return Ok(CollaborationOutcome::Enabled(receipt)),
                Ok(None) => continue,
                Err(Error::OutcomeUnknown) => {
                    let latest = self.read().await.map_err(|_| Error::OutcomeUnknown)?;
                    self.guard.authorize_collaboration(&latest, request).await?;
                    return self
                        .lookup_identity(
                            &latest.manifest,
                            request.principal,
                            request.request_id,
                            request.publication_id,
                            intent,
                        )
                        .await?
                        .map(CollaborationOutcome::Enabled)
                        .ok_or(Error::OutcomeUnknown);
                }
                Err(error) => return Err(error),
            }
        }
        Err(Error::Contended)
    }
}

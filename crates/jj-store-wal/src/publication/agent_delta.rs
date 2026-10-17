//! Stable typed AgentStore deltas rebased through the shared manifest CAS.
use super::agent::agent_error;
use super::*;

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct AgentDeltaRequest {
    pub principal: PrincipalId,
    pub request_id: RequestId,
    pub publication_id: PublicationId,
    pub delta: AgentDeltaV1,
    pub result: Vec<u8>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub thread_binding: Option<AgentThreadBindingV1>,
}
impl AgentDeltaRequest {
    pub fn new(
        principal: PrincipalId,
        request_id: RequestId,
        publication_id: PublicationId,
        delta: AgentDeltaV1,
        result: Vec<u8>,
    ) -> Result<Self> {
        let request = Self {
            principal,
            request_id,
            publication_id,
            delta,
            result,
            thread_binding: None,
        };
        request.validate()?;
        Ok(request)
    }
    pub fn validate(&self) -> Result<()> {
        self.validate_thread_binding()?;
        if self.result.len() > 1024 * 1024 {
            return Err(Error::Limit("agent delta result"));
        }
        if self.delta.edits.iter().any(|edit| {
            edit.replacement
                .is_some_and(|value| value.kind == AgentObjectKind::CatalogConflict)
        }) {
            return Err(Error::Corrupt("authored agent conflict object"));
        }
        self.delta.validate().map_err(agent_error)
    }
    /// Exact original native intent, including IDs, result and Thread binding.
    pub fn intent_digest(&self, repository: RepositoryId) -> Result<IntentDigest> {
        self.validate()?;
        self.intent(repository)
    }
    fn intent(&self, repository: RepositoryId) -> Result<IntentDigest> {
        let mut hash = Sha256::new_with_prefix(b"jj-store/agent-delta/v1\0");
        hash.update(records::encode(&(repository, self), 8 * 1024 * 1024)?);
        Ok(IntentDigest::new(hash.finalize().into()))
    }
}

impl Coordinator {
    pub async fn publish_agent_delta(&self, request: &AgentDeltaRequest) -> Result<PublishOutcome> {
        request.validate()?;
        let intent = request.intent(self.records.repository)?;
        let unknown = || PublishOutcome::OutcomeUnknown {
            request_id: request.request_id,
            publication_id: request.publication_id,
        };
        match tokio::time::timeout(
            self.limits.timeout,
            self.publish_agent_delta_inner(request, intent),
        )
        .await
        {
            Ok(Ok(receipt)) => Ok(PublishOutcome::Committed(receipt)),
            Ok(Err(Error::OutcomeUnknown)) | Err(_) => Ok(unknown()),
            Ok(Err(error)) => Err(error),
        }
    }

    async fn publish_agent_delta_inner(
        &self,
        request: &AgentDeltaRequest,
        intent: IntentDigest,
    ) -> Result<Receipt> {
        for _ in 0..self.limits.cas_attempts {
            let base = self.read().await?;
            let current = base
                .manifest
                .domain_state(Domain::Agent)
                .ok_or(Error::Corrupt("agent domain root"))?;
            let probe = DomainRequest::new(
                request.principal,
                request.request_id,
                request.publication_id,
                Domain::Agent,
                current,
                current.root,
                request.result.clone(),
            )?;
            self.guard.authorize_domain(&base, &probe).await?;
            let thread_expected = if request.thread_binding.is_some() {
                let expected = self.collab_domain_state(&base, Domain::Thread).await?;
                let thread_probe = DomainRequest::new(
                    request.principal,
                    request.request_id,
                    request.publication_id,
                    Domain::Thread,
                    expected,
                    expected.root,
                    Vec::new(),
                )?;
                self.guard.authorize_domain(&base, &thread_probe).await?;
                Some(expected)
            } else {
                None
            };
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
            let pinned = self.agent_store(current.root).await?;
            let root = self.stage_agent_delta(&pinned, &request.delta).await?;
            let domain = DomainRequest::new(
                request.principal,
                request.request_id,
                request.publication_id,
                Domain::Agent,
                current,
                root,
                request.result.clone(),
            )?
            .with_agent_facts(request.delta.clone())?;
            let domain = if let Some(expected) = thread_expected {
                domain.with_additional(self.stage_agent_thread_binding(request, expected).await?)?
            } else {
                domain
            };
            let published = if thread_expected.is_some() {
                self.publish_domain_at_manifest(&domain, intent, Some(base.digest))
                    .await
            } else {
                self.publish_domain_inner(&domain, intent).await
            };
            match published {
                Err(Error::Stale) => continue,
                result => return result,
            }
        }
        Err(Error::Contended)
    }
}

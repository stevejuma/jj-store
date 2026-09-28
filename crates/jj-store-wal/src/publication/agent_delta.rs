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
        };
        request.validate()?;
        Ok(request)
    }
    fn validate(&self) -> Result<()> {
        if self.result.len() > 1024 * 1024 {
            return Err(Error::Limit("agent delta result"));
        }
        self.delta.validate().map_err(agent_error)
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
            )?;
            match self.publish_domain_inner(&domain, intent).await {
                Err(Error::Stale) => continue,
                result => return result,
            }
        }
        Err(Error::Contended)
    }
}

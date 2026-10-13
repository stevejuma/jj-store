//! WorkflowStore commands share domain authorization, CAS and durable receipts.
use super::*;
use std::time::{SystemTime, UNIX_EPOCH};

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct WorkflowCommandRequest {
    pub principal: PrincipalId,
    pub request_id: RequestId,
    pub publication_id: PublicationId,
    pub command: jj_workflow::Command,
}
impl WorkflowCommandRequest {
    pub fn new(
        principal: PrincipalId,
        request_id: RequestId,
        publication_id: PublicationId,
        command: jj_workflow::Command,
    ) -> std::result::Result<Self, WorkflowCommandError> {
        command.validate()?;
        Ok(Self {
            principal,
            request_id,
            publication_id,
            command,
        })
    }
    pub fn intent_digest(
        &self,
        repository: RepositoryId,
    ) -> std::result::Result<IntentDigest, WorkflowCommandError> {
        self.command.validate()?;
        let mut hash = Sha256::new_with_prefix(b"jj-store/workflow-command/v1\0");
        hash.update(records::encode(&(repository, self), 1024 * 1024)?);
        Ok(IntentDigest::new(hash.finalize().into()))
    }
}
#[derive(Debug, thiserror::Error)]
pub enum WorkflowCommandError {
    #[error(transparent)]
    Publication(#[from] Error),
    #[error(transparent)]
    Model(#[from] jj_workflow::Error),
    #[error("WorkflowStore origin replication is unsupported")]
    ReplicationUnsupported,
}
impl Coordinator {
    pub async fn publish_workflow_command(
        &self,
        request: &WorkflowCommandRequest,
    ) -> std::result::Result<PublishOutcome, WorkflowCommandError> {
        let intent = request.intent_digest(self.records.repository)?;
        let at: i64 = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .map_err(|_| Error::Corrupt("workflow time"))?
            .as_millis()
            .try_into()
            .map_err(|_| Error::Limit("workflow time"))?;
        match tokio::time::timeout(
            self.limits.timeout,
            self.publish_workflow_command_inner(request, intent, at),
        )
        .await
        {
            Ok(Ok(receipt)) => Ok(PublishOutcome::Committed(receipt)),
            Ok(Err(WorkflowCommandError::Publication(Error::OutcomeUnknown))) | Err(_) => {
                Ok(PublishOutcome::OutcomeUnknown {
                    request_id: request.request_id,
                    publication_id: request.publication_id,
                })
            }
            Ok(Err(error)) => Err(error),
        }
    }
    async fn publish_workflow_command_inner(
        &self,
        request: &WorkflowCommandRequest,
        intent: IntentDigest,
        at: i64,
    ) -> std::result::Result<Receipt, WorkflowCommandError> {
        for _ in 0..self.limits.cas_attempts {
            let base = self.read().await?;
            let pinned = match base.manifest.workflow_root {
                Some(root) => self.workflow_store(root).await?,
                None => self.empty_workflow_store().await?,
            };
            let current = DomainState {
                root: pinned.reference(),
                version: base.manifest.workflow_state_version,
            };
            let probe = DomainRequest::new(
                request.principal,
                request.request_id,
                request.publication_id,
                Domain::Workflow,
                current,
                current.root,
                Vec::new(),
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
            if base.manifest.work_root.is_none() {
                return Err(Error::Unsupported(
                    "WorkflowStore requires an existing WorkStore home",
                )
                .into());
            }
            if base.manifest.domains().is_none() {
                return Err(Error::Corrupt("enable collaboration before WorkflowStore").into());
            }
            let (_, result) = pinned.state().apply(
                &request.command,
                request.principal,
                request.request_id,
                at,
            )?;
            let root = self.stage_workflow_result(&pinned, &result).await?;
            let bytes = records::encode(&result, 1024 * 1024)?;
            let domain = DomainRequest::new(
                request.principal,
                request.request_id,
                request.publication_id,
                Domain::Workflow,
                current,
                root,
                bytes,
            )?;
            match self.publish_domain_inner(&domain, intent).await {
                Err(Error::Stale) => continue,
                result => return result.map_err(Into::into),
            }
        }
        Err(Error::Contended.into())
    }
}

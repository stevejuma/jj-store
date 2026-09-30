//! WorkStore commands share domain authorization, CAS and durable receipts.
use super::*;
use std::time::{SystemTime, UNIX_EPOCH};

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct WorkCommandRequest {
    pub principal: PrincipalId,
    pub request_id: RequestId,
    pub publication_id: PublicationId,
    pub command: WorkCommandV1,
}
impl WorkCommandRequest {
    pub fn new(
        principal: PrincipalId,
        request_id: RequestId,
        publication_id: PublicationId,
        command: WorkCommandV1,
    ) -> std::result::Result<Self, WorkCommandError> {
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
    ) -> std::result::Result<IntentDigest, WorkCommandError> {
        self.command.validate()?;
        let mut hash = Sha256::new_with_prefix(b"jj-store/work-command/v1\0");
        hash.update(records::encode(&(repository, self), 1024 * 1024)?);
        Ok(IntentDigest::new(hash.finalize().into()))
    }
}
#[derive(Debug, thiserror::Error)]
pub enum WorkCommandError {
    #[error(transparent)]
    Publication(#[from] Error),
    #[error(transparent)]
    Model(#[from] jj_work::Error),
    #[error("WorkStore requires a local home without origin replication")]
    ReplicationUnsupported,
}
impl Coordinator {
    pub async fn publish_work_command(
        &self,
        request: &WorkCommandRequest,
    ) -> std::result::Result<PublishOutcome, WorkCommandError> {
        let intent = request.intent_digest(self.records.repository)?;
        let at: i64 = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .map_err(|_| Error::Corrupt("work time"))?
            .as_millis()
            .try_into()
            .map_err(|_| Error::Limit("work time"))?;
        match tokio::time::timeout(
            self.limits.timeout,
            self.publish_work_command_inner(request, intent, at),
        )
        .await
        {
            Ok(Ok(receipt)) => Ok(PublishOutcome::Committed(receipt)),
            Ok(Err(WorkCommandError::Publication(Error::OutcomeUnknown))) | Err(_) => {
                Ok(PublishOutcome::OutcomeUnknown {
                    request_id: request.request_id,
                    publication_id: request.publication_id,
                })
            }
            Ok(Err(error)) => Err(error),
        }
    }
    async fn publish_work_command_inner(
        &self,
        request: &WorkCommandRequest,
        intent: IntentDigest,
        at: i64,
    ) -> std::result::Result<Receipt, WorkCommandError> {
        for _ in 0..self.limits.cas_attempts {
            let base = self.read().await?;
            let pinned = match base.manifest.work_root {
                Some(root) => self.work_store(root).await?,
                None => self.empty_work_store().await?,
            };
            let current = DomainState {
                root: pinned.reference(),
                version: base.manifest.work_state_version,
            };
            let probe = DomainRequest::new(
                request.principal,
                request.request_id,
                request.publication_id,
                Domain::Work,
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
            if base.manifest.replication_root.is_some() {
                return Err(WorkCommandError::ReplicationUnsupported);
            }
            if base.manifest.domains().is_none() {
                return Err(Error::Corrupt("enable collaboration before WorkStore").into());
            }
            let (_, result) = pinned
                .state()
                .apply(&request.command, request.principal, at)?;
            let root = self.stage_work_result(&pinned, &result).await?;
            let bytes = records::encode(&result, 1024 * 1024)?;
            let domain = DomainRequest::new(
                request.principal,
                request.request_id,
                request.publication_id,
                Domain::Work,
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

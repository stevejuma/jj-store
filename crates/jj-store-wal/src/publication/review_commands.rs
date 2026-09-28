//! Receipt-backed typed ReviewStore command publication.
use super::*;
use jj_review::{ReviewCommandResultV1, ReviewCommandV1, encode_command_result};
use std::time::{SystemTime, UNIX_EPOCH};

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct ReviewCommandRequest {
    pub principal: PrincipalId,
    pub request_id: RequestId,
    pub publication_id: PublicationId,
    pub command: ReviewCommandV1,
}

impl ReviewCommandRequest {
    pub fn new(
        principal: PrincipalId,
        request_id: RequestId,
        publication_id: PublicationId,
        command: ReviewCommandV1,
    ) -> std::result::Result<Self, ReviewCommandError> {
        let request = Self {
            principal,
            request_id,
            publication_id,
            command,
        };
        request.validate()?;
        Ok(request)
    }

    fn validate(&self) -> std::result::Result<(), ReviewCommandError> {
        self.command
            .validate()
            .map_err(|_| ReviewCommandError::Invalid("review command"))
    }

    fn intent(&self, repository: RepositoryId) -> Result<IntentDigest> {
        let mut hash = Sha256::new_with_prefix(b"jj-store/review-command/v1\0");
        hash.update(records::encode(&(repository, self), 8 * 1024 * 1024)?);
        Ok(IntentDigest::new(hash.finalize().into()))
    }

    pub fn intent_digest(
        &self,
        repository: RepositoryId,
    ) -> std::result::Result<IntentDigest, ReviewCommandError> {
        self.validate()?;
        self.intent(repository).map_err(Into::into)
    }
}

#[derive(Debug, thiserror::Error)]
pub enum ReviewCommandError {
    #[error(transparent)]
    Publication(#[from] Error),
    #[error("review command is stale")]
    Stale,
    #[error("review thread is resolved")]
    ThreadResolved,
    #[error("review thread is already open")]
    ThreadOpen,
    #[error("review command target was not found: {0}")]
    NotFound(&'static str),
    #[error("invalid review command: {0}")]
    Invalid(&'static str),
}

pub(super) struct AppliedReviewCommand {
    pub root: RecordRef,
    pub result: ReviewCommandResultV1,
}

impl Coordinator {
    pub async fn publish_review_command(
        &self,
        request: &ReviewCommandRequest,
    ) -> std::result::Result<PublishOutcome, ReviewCommandError> {
        request.validate()?;
        let intent = request.intent(self.records.repository)?;
        let accepted_at_millis = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .map_err(|_| ReviewCommandError::Invalid("review command time"))?
            .as_millis()
            .try_into()
            .map_err(|_| ReviewCommandError::Invalid("review command time"))?;
        let unknown = || PublishOutcome::OutcomeUnknown {
            request_id: request.request_id,
            publication_id: request.publication_id,
        };
        match tokio::time::timeout(
            self.limits.timeout,
            self.publish_review_command_inner(request, intent, accepted_at_millis),
        )
        .await
        {
            Ok(Ok(receipt)) => Ok(PublishOutcome::Committed(receipt)),
            Ok(Err(ReviewCommandError::Publication(Error::OutcomeUnknown))) | Err(_) => {
                Ok(unknown())
            }
            Ok(Err(error)) => Err(error),
        }
    }

    async fn publish_review_command_inner(
        &self,
        request: &ReviewCommandRequest,
        intent: IntentDigest,
        accepted_at_millis: i64,
    ) -> std::result::Result<Receipt, ReviewCommandError> {
        for _ in 0..self.limits.cas_attempts {
            let base = self.read().await?;
            let current = base
                .manifest
                .domain_state(Domain::Review)
                .ok_or(Error::Corrupt("review domain root"))?;
            let probe = DomainRequest::new(
                request.principal,
                request.request_id,
                request.publication_id,
                Domain::Review,
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

            let pinned = self.review_store(current.root).await?;
            let applied = self
                .apply_review_command(&pinned, request, accepted_at_millis)
                .await?;
            let result = encode_command_result(&applied.result)
                .map_err(|_| ReviewCommandError::Invalid("review command result"))?;
            let domain = DomainRequest::new(
                request.principal,
                request.request_id,
                request.publication_id,
                Domain::Review,
                current,
                applied.root,
                result,
            )?;
            match self.publish_domain_inner(&domain, intent).await {
                Err(Error::Stale) => continue,
                Ok(receipt) => return Ok(receipt),
                Err(error) => return Err(error.into()),
            }
        }
        Err(Error::Contended.into())
    }
}

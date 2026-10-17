//! Receipt-backed typed ReviewStore command publication.
use super::*;
use jj_review::{ReviewCommandResultV1, ReviewCommandV1};

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

    pub fn validate(&self) -> std::result::Result<(), ReviewCommandError> {
        self.command
            .validate()
            .map_err(|_| ReviewCommandError::Invalid("review command"))
    }

    pub fn intent(&self, repository: RepositoryId) -> Result<IntentDigest> {
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

/// Immutable projection only; this value is not a publication or receipt.
pub struct AppliedReviewCommand {
    pub root: ReviewRootV1,
    pub delta: ReviewDeltaV1,
    pub result: ReviewCommandResultV1,
}

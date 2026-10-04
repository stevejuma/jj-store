use super::{LocalError, LocalProfile, LocalResult};
use jj_store_jj::local_metadata::{Domain, Receipt};
use jj_store_model as m;
use jj_store_wal::publication::{
    ArtifactTargetV1, ReviewCommandResultV1, ReviewCommandV1, ReviewCommentBodyV1,
};
use serde::{Deserialize, Serialize};
use sha2::{Digest as _, Sha256};

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize, schemars::JsonSchema)]
#[serde(tag = "command", rename_all = "snake_case", deny_unknown_fields)]
pub enum LocalReviewCommand {
    Initialize,
    RepositoryThread { body: ReviewCommentBodyV1 },
    Apply { intent: Box<ReviewCommandV1> },
}
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize, schemars::JsonSchema)]
#[serde(deny_unknown_fields)]
pub struct LocalReviewRequest {
    pub version: u32,
    pub request_id: m::RequestId,
    pub command: LocalReviewCommand,
}
impl LocalReviewRequest {
    pub fn validate(&self) -> LocalResult<()> {
        if self.version != 1 {
            return Err(LocalError::Invalid);
        }
        match &self.command {
            LocalReviewCommand::Initialize => {}
            LocalReviewCommand::RepositoryThread { body } => {
                body.validate().map_err(|_| LocalError::Invalid)?
            }
            LocalReviewCommand::Apply { intent: command } => {
                command.validate().map_err(|_| LocalError::Invalid)?;
                if let ReviewCommandV1::CreateThread { anchor, .. } = command.as_ref()
                    && (!matches!(anchor.target, ArtifactTargetV1::Repository)
                        || !anchor
                            .agent_dependencies()
                            .map_err(|_| LocalError::Invalid)?
                            .is_empty())
                {
                    return Err(LocalError::Unsupported);
                }
            }
        }
        Ok(())
    }
    pub fn intent_for(&self, actor: m::PrincipalId) -> LocalResult<m::IntentDigest> {
        self.validate()?;
        let mut hash = Sha256::new_with_prefix(b"jj-store/local-review-command/v1\0");
        hash.update(actor.as_bytes());
        hash.update(serde_json::to_vec(self).map_err(|_| LocalError::Invalid)?);
        Ok(m::IntentDigest::new(hash.finalize().into()))
    }
}
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize, schemars::JsonSchema)]
#[serde(deny_unknown_fields)]
pub(crate) struct ReviewResult {
    pub root: m::RecordDigest,
    pub accepted_at_millis: i64,
    pub command: Option<ReviewCommandResultV1>,
}
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize, schemars::JsonSchema)]
pub struct LocalReviewReceipt {
    pub version: u32,
    pub profile: LocalProfile,
    pub repository: m::RepositoryId,
    pub actor: m::PrincipalId,
    pub request_id: m::RequestId,
    pub intent: m::IntentDigest,
    pub metadata_sequence: u64,
    pub review_state_version: u64,
    pub review_root: m::RecordDigest,
    pub accepted_at_millis: i64,
    pub command: Option<ReviewCommandResultV1>,
}
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize, schemars::JsonSchema)]
#[serde(tag = "status", rename_all = "snake_case")]
pub enum LocalReviewOutcome {
    Committed { receipt: Box<LocalReviewReceipt> },
    OutcomeUnknown { request_id: m::RequestId },
    NotKnown { request_id: m::RequestId },
}
pub(crate) fn receipt(value: &Receipt) -> LocalResult<LocalReviewOutcome> {
    if value.domain != Domain::Review {
        return Err(LocalError::IntentConflict);
    }
    let result: ReviewResult =
        serde_json::from_slice(&value.result).map_err(|_| LocalError::Corrupt)?;
    if serde_json::to_vec(&result).map_err(|_| LocalError::Corrupt)? != value.result
        || result.accepted_at_millis < 0
    {
        return Err(LocalError::Corrupt);
    }
    if let Some(command) = &result.command {
        command.validate().map_err(|_| LocalError::Corrupt)?;
    }
    Ok(LocalReviewOutcome::Committed {
        receipt: Box::new(LocalReviewReceipt {
            version: 1,
            profile: LocalProfile::LocalJjGit,
            repository: value.repository,
            actor: value.actor,
            request_id: value.request,
            intent: value.intent,
            metadata_sequence: value.sequence,
            review_state_version: value.selection.version,
            review_root: result.root,
            accepted_at_millis: result.accepted_at_millis,
            command: result.command,
        }),
    })
}

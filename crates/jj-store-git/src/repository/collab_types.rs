use super::{LocalError, LocalProfile, LocalResult};
use crate::metadata::{Domain, GitOid, Receipt};
use jj_collab::*;
use jj_store_model as m;
use serde::{Deserialize, Serialize};
use sha2::{Digest as _, Sha256};

pub use jj_collab::{
    CollabCommandResultV1 as LocalCollabResultV1, CollabCommandV1 as LocalCollabCommand,
};
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize, schemars::JsonSchema)]
#[serde(deny_unknown_fields)]
pub struct LocalCollabRequest {
    pub version: u32,
    pub request_id: m::RequestId,
    pub command: LocalCollabCommand,
}
impl LocalCollabRequest {
    pub fn validate(&self) -> LocalResult<()> {
        if self.version != 1 {
            return Err(LocalError::Invalid);
        }
        match &self.command {
            LocalCollabCommand::Thread { intent } => {
                intent.validate()?;
                let anchor = match intent {
                    ThreadCommandV1::Create { baseline, .. } => baseline.as_ref(),
                    ThreadCommandV1::Link {
                        target: ThreadLinkTargetV1::Code { anchor, .. },
                        ..
                    } => Some(anchor),
                    _ => None,
                };
                if matches!(anchor, Some(ThreadCodeAnchorV1::Native(_))) {
                    return Err(LocalError::Unsupported);
                }
            }
            LocalCollabCommand::Reaction { intent } => intent.validate()?,
            LocalCollabCommand::Emoji { intent } => intent.validate()?,
        }
        Ok(())
    }
    pub fn intent_for(&self, actor: m::PrincipalId) -> LocalResult<m::IntentDigest> {
        self.validate()?;
        let mut hash = Sha256::new_with_prefix(b"jj-store/local-collab-command/v1\0");
        hash.update(actor.as_bytes());
        hash.update(serde_json::to_vec(self).map_err(|_| LocalError::Invalid)?);
        Ok(m::IntentDigest::new(hash.finalize().into()))
    }
}
fn result_domain(value: &LocalCollabResultV1) -> Domain {
    match value {
        LocalCollabResultV1::Thread { .. } => Domain::Thread,
        _ => Domain::Reaction,
    }
}
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize, schemars::JsonSchema)]
#[serde(deny_unknown_fields)]
pub struct LocalCollabReceipt {
    pub version: u32,
    pub profile: LocalProfile,
    pub repository: m::RepositoryId,
    pub actor: m::PrincipalId,
    pub request_id: m::RequestId,
    pub intent: m::IntentDigest,
    pub metadata_sequence: u64,
    pub domain_state_version: u64,
    pub domain_object: GitOid,
    pub accepted_at_millis: i64,
    pub result: LocalCollabResultV1,
}
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize, schemars::JsonSchema)]
#[serde(tag = "status", rename_all = "snake_case")]
pub enum LocalCollabOutcome {
    Committed { receipt: Box<LocalCollabReceipt> },
    OutcomeUnknown { request_id: m::RequestId },
    NotKnown { request_id: m::RequestId },
}
#[derive(Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub(crate) struct CollabResult {
    pub accepted_at_millis: i64,
    pub result: LocalCollabResultV1,
}
pub(crate) fn receipt(value: &Receipt) -> LocalResult<LocalCollabOutcome> {
    if !matches!(value.domain, Domain::Thread | Domain::Reaction) || !value.selections.is_empty() {
        return Err(LocalError::IntentConflict);
    }
    let result: CollabResult =
        serde_json::from_slice(&value.result).map_err(|_| LocalError::Corrupt)?;
    if result_domain(&result.result) != value.domain
        || result.accepted_at_millis < 0
        || serde_json::to_vec(&result).map_err(|_| LocalError::Corrupt)? != value.result
    {
        return Err(LocalError::Corrupt);
    }
    result.result.validate().map_err(|_| LocalError::Corrupt)?;
    Ok(LocalCollabOutcome::Committed {
        receipt: Box::new(LocalCollabReceipt {
            version: 1,
            profile: LocalProfile::LocalJjGit,
            repository: value.repository,
            actor: value.actor,
            request_id: value.request,
            intent: value.intent,
            metadata_sequence: value.sequence,
            domain_state_version: value.selection.version,
            domain_object: value.selection.object,
            accepted_at_millis: result.accepted_at_millis,
            result: result.result,
        }),
    })
}

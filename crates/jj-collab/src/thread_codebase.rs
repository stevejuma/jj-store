use crate::*;
use jj_store_model::RepositoryId;
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize, schemars::JsonSchema)]
#[serde(tag = "type", rename_all = "snake_case", deny_unknown_fields)]
pub enum ThreadPurposeV1 {
    Conversation {
        target: ConversationMetadataTargetV1,
    },
    Annotation,
}

pub fn validate_codebase(repositories: &[RepositoryId]) -> Result<()> {
    if repositories.len() > 8 {
        return Err(Error::Limit);
    }
    if repositories
        .iter()
        .copied()
        .collect::<std::collections::BTreeSet<_>>()
        .len()
        != repositories.len()
    {
        return Err(Error::Invalid("duplicate Codebase repository"));
    }
    Ok(())
}

impl ThreadVersionV1 {
    pub fn is_user_thread(&self) -> bool {
        self.purpose.is_none() && self.agent_message.is_none() && self.comment_source.is_none()
    }
}

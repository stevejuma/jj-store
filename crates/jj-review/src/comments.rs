use crate::{
    CommentId, CommentRevisionId, Error, Result, ReviewObjectKind, ReviewObjectRef, ReviewThreadId,
    expect_kind, valid_label,
};
use jj_store_model::PrincipalId;
use serde::{Deserialize, Serialize};
use sha2::{Digest as _, Sha256};
use std::collections::BTreeSet;

const MAX_COMMENT_BYTES: usize = 1024 * 1024;

#[derive(Clone, Copy, Debug, PartialEq, Eq, Serialize, Deserialize, schemars::JsonSchema)]
#[serde(rename_all = "snake_case")]
pub enum ReviewBodyFormatV1 {
    PlainText,
    SafeMarkdown,
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize, schemars::JsonSchema)]
#[serde(deny_unknown_fields)]
pub struct ReviewCommentBodyV1 {
    pub schema_version: u32,
    pub format: ReviewBodyFormatV1,
    pub text: String,
    pub sanitizer_version: String,
}
impl ReviewCommentBodyV1 {
    pub fn validate(&self) -> Result<()> {
        if self.schema_version != 1
            || self.text.is_empty()
            || self.text.len() > MAX_COMMENT_BYTES
            || self
                .text
                .chars()
                .any(|value| value.is_control() && !matches!(value, '\n' | '\r' | '\t'))
            || !valid_label(&self.sanitizer_version, 128)
        {
            return Err(Error::Invalid("review comment body"));
        }
        Ok(())
    }
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize, schemars::JsonSchema)]
#[serde(tag = "state", rename_all = "snake_case", deny_unknown_fields)]
pub enum CommentRevisionContentV1 {
    Visible { body: ReviewObjectRef },
    Tombstoned { reason: Option<String> },
    Hidden { reason: String },
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize, schemars::JsonSchema)]
#[serde(deny_unknown_fields)]
pub struct ReviewCommentRevisionV1 {
    pub schema_version: u32,
    pub thread_id: ReviewThreadId,
    pub comment_id: CommentId,
    pub author: PrincipalId,
    pub actor: PrincipalId,
    pub created_at_millis: i64,
    pub previous_revision: Option<ReviewObjectRef>,
    pub content: CommentRevisionContentV1,
}
impl ReviewCommentRevisionV1 {
    pub fn validate(&self) -> Result<()> {
        if self.schema_version != 1 {
            return Err(Error::Invalid("review comment revision schema"));
        }
        if let Some(previous) = self.previous_revision {
            expect_kind(previous, ReviewObjectKind::CommentRevision)?;
        }
        match &self.content {
            CommentRevisionContentV1::Visible { body } => {
                expect_kind(*body, ReviewObjectKind::CommentBody)?;
            }
            CommentRevisionContentV1::Tombstoned { reason } => {
                if self.previous_revision.is_none()
                    || reason.as_deref().is_some_and(|value| !valid_reason(value))
                {
                    return Err(Error::Invalid("review comment tombstone"));
                }
            }
            CommentRevisionContentV1::Hidden { reason } => {
                if self.previous_revision.is_none() || !valid_reason(reason) {
                    return Err(Error::Invalid("review comment moderation"));
                }
            }
        }
        Ok(())
    }
    pub fn review_dependencies(&self) -> Result<BTreeSet<ReviewObjectRef>> {
        self.validate()?;
        Ok(self
            .previous_revision
            .into_iter()
            .chain(match self.content {
                CommentRevisionContentV1::Visible { body } => Some(body),
                _ => None,
            })
            .collect())
    }
    pub fn revision_id(&self) -> Result<CommentRevisionId> {
        let object = crate::ReviewObjectV1::CommentRevision(Box::new(self.clone()));
        Ok(CommentRevisionId::new(
            Sha256::digest(crate::encode_object(&object)?).into(),
        ))
    }
}

fn valid_reason(value: &str) -> bool {
    valid_label(value, 4096)
}

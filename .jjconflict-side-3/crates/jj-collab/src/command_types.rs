//! Shared typed intents and results; publication and actor admission stay with providers.
use crate::*;
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize, schemars::JsonSchema)]
#[serde(tag = "domain", rename_all = "snake_case", deny_unknown_fields)]
pub enum CollabCommandV1 {
    Thread { intent: ThreadCommandV1 },
    Reaction { intent: ReactionCommandV1 },
    Emoji { intent: CustomEmojiCommandV1 },
}
impl CollabCommandV1 {
    pub fn validate(&self) -> Result<()> {
        match self {
            Self::Thread { intent } => intent.validate(),
            Self::Reaction { intent } => intent.validate(),
            Self::Emoji { intent } => intent.validate(),
        }
    }
}
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize, schemars::JsonSchema)]
#[serde(tag = "domain", rename_all = "snake_case", deny_unknown_fields)]
pub enum CollabCommandResultV1 {
    Thread { result: ThreadCommandResultV1 },
    Reaction { result: ReactionCommandResultV1 },
    Emoji { version: CollabObjectRef },
}
impl CollabCommandResultV1 {
    pub fn validate(&self) -> Result<()> {
        match self {
            Self::Thread { result } => {
                expect(result.version, CollabObjectKind::ThreadVersion)?;
                expect(result.event, CollabObjectKind::ThreadEvent)?;
                if let Some(reference) = result.message {
                    expect(reference, CollabObjectKind::HumanMessageRevision)?;
                }
                if let Some(reference) = result.link {
                    expect(reference, CollabObjectKind::ThreadLink)?;
                }
                Ok(())
            }
            Self::Reaction { result } => result.validate(),
            Self::Emoji { version } => expect(*version, CollabObjectKind::CustomEmojiVersion),
        }
    }
}

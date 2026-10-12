//! Installation profile/library contracts; repository grants remain independent.
use crate::auth::BearerToken;
use jj_store_model::{PrincipalId, RequestId, TokenId};
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};
#[derive(Debug, thiserror::Error)]
pub enum Error {
    #[error("directory access denied")]
    Denied,
    #[error("invalid directory input")]
    Invalid,
    #[error("directory selection changed")]
    Stale,
    #[error("directory limit exceeded")]
    Limit,
    #[error("directory unavailable")]
    Unavailable,
    #[error("directory corrupt")]
    Corrupt,
    #[error("directory outcome unknown")]
    Unknown,
}
pub type Result<T> = std::result::Result<T, Error>;
pub enum Caller {
    Local(PrincipalId),
    Token(BearerToken),
    Operator(PrincipalId),
}
#[derive(Clone, Debug, Default, Serialize, Deserialize, JsonSchema, PartialEq, Eq)]
#[serde(deny_unknown_fields)]
pub struct ProfileDetails {
    pub display_name: String,
    pub description: String,
    pub email: Option<String>,
    /// blob:<sha256>, HTTPS URL, emoji or fixed Community emoji SVG.
    pub avatar: Option<String>,
}
#[derive(Clone, Debug, Serialize, Deserialize, JsonSchema, PartialEq, Eq)]
#[serde(deny_unknown_fields)]
pub struct UserProfile {
    pub principal: PrincipalId,
    pub revision: Option<String>,
    pub details: ProfileDetails,
}
#[derive(Clone, Debug, Serialize, Deserialize, JsonSchema)]
#[serde(deny_unknown_fields)]
pub struct PublicProfile {
    pub principal: PrincipalId,
    pub display_name: String,
    pub description: String,
    pub avatar: Option<String>,
}
#[derive(Clone, Debug, Serialize, Deserialize, JsonSchema, PartialEq, Eq)]
#[serde(deny_unknown_fields)]
pub struct LibraryEmoji {
    pub id: RequestId,
    pub owner: PrincipalId,
    pub revision: String,
    pub shortcode: String,
    pub asset: String,
    pub preview: String,
    pub retired: bool,
}
#[derive(Clone, Debug, Serialize, Deserialize, JsonSchema, PartialEq, Eq)]
#[serde(tag = "type", rename_all = "snake_case", deny_unknown_fields)]
pub enum DirectoryCommand {
    SaveProfile {
        expected: Option<String>,
        details: ProfileDetails,
    },
    Upload {
        bytes: Vec<u8>,
    },
    SaveEmoji {
        id: RequestId,
        expected: Option<String>,
        shortcode: String,
        asset: String,
    },
    RetireEmoji {
        id: RequestId,
        expected: String,
    },
}
#[derive(Clone, Debug, Serialize, Deserialize, JsonSchema)]
#[serde(deny_unknown_fields)]
pub struct DirectoryRequest {
    pub request_id: RequestId,
    pub command: DirectoryCommand,
}
#[derive(Clone, Debug, Serialize, Deserialize, JsonSchema, PartialEq, Eq)]
#[serde(tag = "type", rename_all = "snake_case", deny_unknown_fields)]
pub enum DirectoryResult {
    Profile {
        profile: UserProfile,
    },
    Media {
        asset: String,
        preview: String,
    },
    Emoji {
        emoji: LibraryEmoji,
    },
    Identity {
        token: TokenId,
        principal: PrincipalId,
        revoked: bool,
    },
}
#[derive(Clone, Debug, Serialize, Deserialize, JsonSchema)]
#[serde(deny_unknown_fields)]
pub struct DirectoryReceipt {
    pub request_id: RequestId,
    pub result: DirectoryResult,
}
#[derive(Clone, Debug, Serialize, Deserialize, JsonSchema)]
#[serde(deny_unknown_fields)]
pub struct IdentityBinding {
    pub revoked: bool,
    pub token: TokenId,
    pub principal: PrincipalId,
    pub expires_at: Option<u64>,
}
#[derive(Clone, Debug, Serialize, Deserialize, JsonSchema)]
#[serde(deny_unknown_fields)]
pub struct EmojiSelection {
    pub snapshot: jj_collab::EmojiSnapshotV1,
    pub value: jj_collab::ReactionValueV1,
}
pub struct Media {
    pub bytes: Vec<u8>,
    pub content_type: &'static str,
}
#[async_trait::async_trait]
pub trait Directory: Send + Sync {
    fn freshness(&self) -> Option<tokio::sync::broadcast::Receiver<crate::freshness::Change>> {
        None
    }
    async fn principal(&self, caller: &Caller) -> Result<PrincipalId>;
    async fn profile(&self, caller: &Caller) -> Result<UserProfile>;
    async fn profiles(&self, caller: &Caller, ids: Vec<PrincipalId>) -> Result<Vec<PublicProfile>>;
    async fn emojis(&self, caller: &Caller) -> Result<Vec<LibraryEmoji>>;
    async fn emoji_snapshot(
        &self,
        caller: &Caller,
        id: RequestId,
        revision: &str,
    ) -> Result<EmojiSelection>;
    async fn media(&self, caller: &Caller, digest: &str) -> Result<Media>;
    async fn execute(&self, caller: &Caller, request: DirectoryRequest)
    -> Result<DirectoryReceipt>;
    async fn receipt(
        &self,
        caller: &Caller,
        request: RequestId,
    ) -> Result<Option<DirectoryReceipt>>;
    async fn identities(&self, caller: &Caller) -> Result<Vec<IdentityBinding>>;
    async fn enroll(
        &self,
        caller: &Caller,
        request: RequestId,
        principal: PrincipalId,
        token: &BearerToken,
        expires_at: Option<u64>,
    ) -> Result<DirectoryReceipt>;
    async fn revoke(
        &self,
        caller: &Caller,
        request: RequestId,
        token: TokenId,
    ) -> Result<DirectoryReceipt>;
}

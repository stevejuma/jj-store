//! Owned inner frames. Native caller and cache authority are never request fields.
use crate::{Handle, OperationId, local};
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

#[derive(Deserialize, Serialize, JsonSchema)]
#[serde(deny_unknown_fields)]
pub struct Vault {
    pub vault: String,
}
#[derive(Deserialize, Serialize, JsonSchema)]
#[serde(deny_unknown_fields)]
pub struct Open {
    pub vault: String,
    pub operation: OperationId,
}
#[derive(Deserialize, Serialize, JsonSchema)]
#[serde(deny_unknown_fields)]
pub struct Selection {
    pub vault: String,
    pub handle: Handle,
}
#[derive(Deserialize, Serialize, JsonSchema)]
#[serde(deny_unknown_fields)]
pub struct Read {
    pub vault: String,
    pub handle: Handle,
    pub query: local::Request,
}

#[derive(Clone, Deserialize, Serialize, JsonSchema)]
#[serde(deny_unknown_fields)]
pub struct BlameQuery {
    pub commit: jj_store_model::GitSha1Id,
    pub path: String,
    pub continuation: Option<String>,
}
#[derive(Deserialize, Serialize, JsonSchema)]
#[serde(deny_unknown_fields)]
pub struct Blame {
    pub vault: String,
    pub handle: Handle,
    pub query: BlameQuery,
}

/// Schema-only inventory for independently generated typed clients.
#[derive(JsonSchema)]
pub struct Contract {
    pub heads_request: crate::wire::Frame<Vault>,
    pub open_request: crate::wire::Frame<Open>,
    pub selection_request: crate::wire::Frame<Selection>,
    pub read_request: crate::wire::Frame<Read>,
    pub blame_request: crate::wire::Frame<Blame>,
    pub heads_response: crate::wire::Frame<local::Heads>,
    pub open_response: crate::wire::Frame<Handle>,
    pub status_response: crate::wire::Frame<crate::Status>,
    pub read_response: crate::wire::Frame<local::Response>,
    pub blame_response: crate::wire::Frame<crate::blame::Page>,
    pub close_response: crate::wire::Frame<()>,
}

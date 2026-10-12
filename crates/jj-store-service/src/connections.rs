//! Device-owned connection hints and memory-only authenticated transports.
use jj_store_model::PrincipalId;
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Serialize, Deserialize, JsonSchema, PartialEq, Eq)]
#[serde(deny_unknown_fields)]
pub struct Connection {
    pub id: String,
    pub name: String,
    pub origin: String,
    pub installation: String,
}
#[derive(Deserialize, JsonSchema)]
#[serde(deny_unknown_fields)]
pub struct Connect {
    pub id: Option<String>,
    pub name: String,
    pub origin: String,
    pub token: String,
}
#[derive(Serialize, JsonSchema)]
pub struct Connected {
    pub connection: Connection,
    pub handle: String,
    pub principal: PrincipalId,
}
#[derive(Deserialize, JsonSchema)]
#[serde(tag = "action", rename_all = "snake_case", deny_unknown_fields)]
pub enum Edit {
    Rename { id: String, name: String },
    Remove { id: String },
}
#[derive(Deserialize, JsonSchema)]
#[serde(deny_unknown_fields)]
pub struct Disconnect {
    pub handle: String,
}
pub struct Forward {
    pub connection: String,
    pub handle: String,
    pub method: String,
    pub path: String,
    pub headers: Vec<(String, String)>,
    pub body: Vec<u8>,
}
pub struct Response {
    pub status: u16,
    pub headers: Vec<(String, String)>,
    pub chunks: tokio::sync::mpsc::Receiver<Result<Vec<u8>>>,
}
#[derive(Debug, thiserror::Error)]
pub enum Error {
    #[error("connection denied")]
    Denied,
    #[error("invalid connection")]
    Invalid,
    #[error("connection limit")]
    Limit,
    #[error("server identity changed")]
    Identity,
    #[error("connection unavailable")]
    Unavailable,
}
pub type Result<T> = std::result::Result<T, Error>;
#[async_trait::async_trait]
pub trait Connections: Send + Sync {
    async fn list(&self) -> Result<Vec<Connection>>;
    async fn connect(&self, request: Connect) -> Result<Connected>;
    async fn edit(&self, request: Edit) -> Result<Vec<Connection>>;
    async fn disconnect(&self, handle: &str) -> Result<()>;
    async fn forward(&self, request: Forward) -> Result<Response>;
    async fn events(
        &self,
        connection: &str,
        handle: &str,
        after: Option<String>,
    ) -> Result<tokio::sync::mpsc::Receiver<Result<String>>>;
}

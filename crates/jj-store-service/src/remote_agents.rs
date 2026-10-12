//! Authenticated caller admission. Implementations must expose only execution,
//! never installation configuration or operator-controlled runtime management.
use crate::{
    agent_execution::{AgentManager, Result},
    auth::BearerToken,
};
use jj_store_model::PrincipalId;
#[async_trait::async_trait]
pub trait RemoteAgents: Send + Sync {
    fn freshness(&self) -> Option<tokio::sync::broadcast::Receiver<crate::freshness::Change>> {
        None
    }
    async fn admit(&self, token: BearerToken) -> Result<(PrincipalId, AgentManager)>;
    async fn shutdown(&self) -> Result<()>;
}

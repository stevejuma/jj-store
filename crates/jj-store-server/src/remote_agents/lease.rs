//! No native administrator fallback. The WAL guard invokes this on publication too.
use super::*;
use jj_store_service::{
    access::Delegation,
    auth::{self, Permission},
    directory::Caller,
};
pub(super) struct Lease {
    pub principal: PrincipalId,
    pub token: Mutex<Option<BearerToken>>,
    pub scope: Mutex<Scope>,
    pub directory: Arc<dyn Directory>,
    pub configuration: Arc<std::sync::RwLock<Option<Configuration>>>,
}
impl Lease {
    pub fn scope(&self) -> Result<Scope> {
        self.scope
            .lock()
            .map(|s| s.clone())
            .map_err(|_| Error::Unavailable)
    }
    pub fn revoke(&self) {
        if let Ok(mut token) = self.token.lock() {
            *token = None;
        }
    }
    pub async fn check(&self) -> Result<()> {
        let token = self
            .token
            .lock()
            .map_err(|_| Error::Unavailable)?
            .clone()
            .ok_or(Error::Denied)?;
        if self
            .directory
            .principal(&Caller::Token(token))
            .await
            .map_err(|_| Error::Denied)?
            != self.principal
            || self.scope()?.agents.iter().any(|id| {
                !self
                    .configuration
                    .read()
                    .ok()
                    .and_then(|c| c.as_ref().map(|c| c.allows(id, self.principal)))
                    .unwrap_or(false)
            })
        {
            return Err(Error::Denied);
        }
        Ok(())
    }
}
#[async_trait::async_trait]
impl Delegation for Lease {
    fn credential(&self, repository: RepositoryId) -> auth::Result<BearerToken> {
        if !self
            .scope()
            .map_err(|_| auth::Error::Denied)?
            .repositories
            .contains(&repository)
        {
            return Err(auth::Error::Denied);
        }
        self.token
            .lock()
            .map_err(|_| auth::Error::Denied)?
            .clone()
            .ok_or(auth::Error::Denied)
    }
    async fn principal(
        &self,
        repository: RepositoryId,
        required: Permission,
    ) -> auth::Result<PrincipalId> {
        if !matches!(required, Permission::Read | Permission::Write)
            || !self
                .scope()
                .map_err(|_| auth::Error::Denied)?
                .repositories
                .contains(&repository)
        {
            return Err(auth::Error::Denied);
        }
        self.check().await.map_err(|_| auth::Error::Denied)?;
        Ok(self.principal)
    }
}

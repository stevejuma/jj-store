//! Device-global forge accounts. Only opaque account references cross into UI.
pub mod http;
mod oauth;
pub mod store;
mod types;
use crate::{Error, Result};
use serde_json::{Value, json};
use std::{
    collections::HashMap,
    path::PathBuf,
    sync::{Arc, Mutex, OnceLock},
};
use store::{SecretStore, Store};
pub use types::*;

pub struct Manager {
    pub root: PathBuf,
    pub secrets: Arc<dyn SecretStore>,
    pub transport: Arc<dyn http::Transport>,
    pub client_id: Option<String>,
    flows: Mutex<HashMap<String, oauth::Flow>>,
}
pub fn native() -> Result<&'static Manager> {
    static MANAGER: OnceLock<Manager> = OnceLock::new();
    if let Some(manager) = MANAGER.get() {
        return Ok(manager);
    }
    let root = store::root()?;
    let client_id = std::env::var("JJ_STORE_GITHUB_OAUTH_CLIENT_ID")
        .ok()
        .or_else(|| std::fs::read_to_string(root.join("github-oauth-client-id")).ok())
        .map(|v| v.trim().to_string())
        .filter(|v| {
            !v.is_empty() && v.len() <= 128 && v.bytes().all(|b| b.is_ascii_alphanumeric())
        });
    let manager = Manager::new(
        root,
        Arc::new(store::NativeSecrets),
        Arc::new(http::Http),
        client_id,
    );
    let _ = MANAGER.set(manager);
    MANAGER.get().ok_or(Error::State)
}
pub fn now() -> Result<u64> {
    std::time::SystemTime::now()
        .duration_since(std::time::UNIX_EPOCH)
        .map(|v| v.as_secs())
        .map_err(|_| Error::State)
}
impl Manager {
    pub fn new(
        root: PathBuf,
        secrets: Arc<dyn SecretStore>,
        transport: Arc<dyn http::Transport>,
        client_id: Option<String>,
    ) -> Self {
        Self {
            root,
            secrets,
            transport,
            client_id,
            flows: Mutex::new(HashMap::new()),
        }
    }
    pub fn inventory(&self) -> Result<Inventory> {
        let mut accounts = Store::open(&self.root)?.accounts;
        for account in &mut accounts {
            account.available = self.secrets.get(&account.id)?.is_some();
        }
        Ok(Inventory {
            accounts,
            oauth_available: self.client_id.is_some(),
            keychain_supported: cfg!(any(
                target_os = "macos",
                target_os = "linux",
                target_os = "windows"
            )),
            authorization: None,
        })
    }
    pub fn account(&self, id: &str) -> Result<Account> {
        Store::open(&self.root)?
            .accounts
            .into_iter()
            .find(|a| a.id == id)
            .ok_or(Error::Denied)
    }
    pub async fn credential(&self, id: &str, now: u64) -> Result<Credential> {
        let account = self.account(id)?;
        let value = zeroize::Zeroizing::new(self.secrets.get(id)?.ok_or(Error::Denied)?);
        let credential: Credential = serde_json::from_str(&value).map_err(|_| Error::State)?;
        if credential
            .expires_at
            .is_none_or(|expiry| expiry > now.saturating_add(60))
        {
            return Ok(credential);
        }
        // Serialize refresh with the account store lock: rotating refresh tokens are single-use.
        let _store = Store::open(&self.root)?;
        let current = zeroize::Zeroizing::new(self.secrets.get(id)?.ok_or(Error::Denied)?);
        if *current != *value {
            return Err(Error::Stale);
        }
        let refresh = credential.refresh_token.as_ref().ok_or(Error::Denied)?;
        let result = self
            .transport
            .request(
                "https://github.com/login/oauth/access_token",
                None,
                Some(
                    json!({"client_id":self.client_id.as_ref().ok_or(Error::Unsupported)?,
                "grant_type":"refresh_token", "refresh_token":refresh}),
                ),
            )
            .await?;
        let updated = oauth::credential(&result, now)?;
        let user = self.user(&account.api_url, &updated.access_token).await?;
        if identity(&user)? != account.user_id {
            return Err(Error::Denied);
        }
        let value =
            zeroize::Zeroizing::new(serde_json::to_string(&updated).map_err(|_| Error::State)?);
        self.secrets.set(id, &value)?;
        Ok(updated)
    }
    pub async fn user(&self, api: &str, token: &str) -> Result<Value> {
        self.transport
            .request(&format!("{}/user", api_root(api)?), Some(token), None)
            .await
    }
    pub async fn repository(&self, account: &Account, project: &str, now: u64) -> Result<Value> {
        if project.split('/').count() != 2 || project.split('/').any(|s| !crate::safe_name(s)) {
            return Err(Error::Invalid);
        }
        let credential = self.credential(&account.id, now).await?;
        self.transport
            .request(
                &format!("{}/repos/{project}", account.api_url),
                Some(&credential.access_token),
                None,
            )
            .await
    }
    pub(super) fn persist(
        &self,
        api: String,
        method: Method,
        user: Value,
        credential: &Credential,
    ) -> Result<Account> {
        let user_id = identity(&user)?;
        let login = user
            .get("login")
            .and_then(Value::as_str)
            .filter(|v| crate::safe_name(v))
            .ok_or(Error::Invalid)?
            .to_string();
        let field = |name: &str| {
            user.get(name)
                .and_then(Value::as_str)
                .map(|s| s.chars().take(2048).collect())
        };
        let account = Account {
            id: store::account_id(&api, &user_id, &method),
            api_url: api,
            user_id,
            login,
            name: field("name"),
            email: field("email"),
            avatar_url: field("avatar_url"),
            method,
            revision: 1,
            available: true,
        };
        Store::open(&self.root)?.put(account, credential, self.secrets.as_ref())
    }
    pub async fn execute(&self, command: Command, now: u64) -> Result<Inventory> {
        let authorization = match command {
            Command::AddToken { api_url, token } => {
                let token = zeroize::Zeroizing::new(token);
                if token.trim().is_empty()
                    || token.len() > 4096
                    || token.contains(['\n', '\r', '\0'])
                {
                    return Err(Error::Invalid);
                }
                let api = api_root(&api_url)?;
                let user = self.user(&api, &token).await?;
                let method = if api == "https://api.github.com" {
                    Method::Pat
                } else {
                    Method::Enterprise
                };
                self.persist(
                    api,
                    method,
                    user,
                    &Credential {
                        access_token: token.to_string(),
                        refresh_token: None,
                        expires_at: None,
                    },
                )?;
                None
            }
            Command::Forget { account } => {
                let mut store = Store::open(&self.root)?;
                if store.accounts.iter().any(|a| a.id == account) {
                    self.secrets.remove(&account)?;
                    store.accounts.retain(|a| a.id != account);
                    store.save()?;
                }
                None
            }
            Command::BeginOauth => Some(self.begin(now).await?),
            Command::PollOauth { session } => Some(self.poll(&session, now).await?),
            Command::CancelOauth { session } => {
                self.flows
                    .lock()
                    .map_err(|_| Error::State)?
                    .remove(&session);
                None
            }
        };
        let mut inventory = self.inventory()?;
        inventory.authorization = authorization;
        Ok(inventory)
    }
}
fn identity(user: &Value) -> Result<String> {
    user.get("id")
        .and_then(Value::as_u64)
        .filter(|id| *id > 0)
        .map(|id| id.to_string())
        .ok_or(Error::Invalid)
}

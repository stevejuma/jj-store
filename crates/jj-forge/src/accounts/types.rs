use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize, JsonSchema)]
#[serde(rename_all = "snake_case")]
#[schemars(rename = "ForgeIntegrationMethod")]
pub enum Method {
    Oauth,
    Pat,
    Enterprise,
}

/// Non-secret identity. The numeric provider ID survives username changes.
#[derive(Clone, Debug, Serialize, Deserialize, JsonSchema)]
#[schemars(rename = "ForgeIntegrationAccount")]
pub struct Account {
    pub id: String,
    pub api_url: String,
    pub user_id: String,
    pub login: String,
    pub name: Option<String>,
    pub email: Option<String>,
    pub avatar_url: Option<String>,
    pub method: Method,
    pub revision: u64,
    pub available: bool,
}

/// Sensitive command inputs deliberately do not implement Debug.
#[derive(Serialize, Deserialize, JsonSchema)]
#[serde(tag = "kind", rename_all = "snake_case", deny_unknown_fields)]
#[schemars(rename = "ForgeIntegrationCommand")]
pub enum Command {
    AddToken { api_url: String, token: String },
    Forget { account: String },
    BeginOauth,
    PollOauth { session: String },
    CancelOauth { session: String },
}

#[derive(Clone, Debug, Serialize, Deserialize, JsonSchema)]
#[schemars(rename = "ForgeIntegrationAuthorization")]
pub struct Authorization {
    pub session: String,
    pub user_code: String,
    pub verification_uri: String,
    pub expires_at: u64,
    pub interval: u64,
    pub state: String,
}

#[derive(Clone, Debug, Default, Serialize, Deserialize, JsonSchema)]
#[schemars(rename = "ForgeIntegrationInventory")]
pub struct Inventory {
    pub accounts: Vec<Account>,
    pub oauth_available: bool,
    pub keychain_supported: bool,
    pub authorization: Option<Authorization>,
}

#[derive(Clone, Debug, Serialize, Deserialize, JsonSchema)]
#[schemars(rename = "ForgeIntegrationRemote")]
pub struct Remote {
    pub name: String,
    pub api_url: String,
    pub repository: String,
}
#[derive(Clone, Debug, Default, Serialize, Deserialize, JsonSchema)]
#[schemars(rename = "ForgeIntegrationDetection")]
pub struct Detection {
    pub provider: Option<crate::Provider>,
    pub remotes: Vec<Remote>,
    pub accounts: Vec<Account>,
    pub readiness: String,
    pub account_id: Option<String>,
}

/// Unknown SSH hosts are never assumed to be GitHub Enterprise.
pub fn detect_remote(name: &str, value: &str, accounts: &[Account]) -> Option<Remote> {
    let url = if value.starts_with("git@") && !value.contains("://") {
        let (host, path) = value.strip_prefix("git@")?.split_once(':')?;
        url::Url::parse(&format!("ssh://git@{host}/{path}")).ok()?
    } else {
        url::Url::parse(value).ok()?
    };
    if !matches!(url.scheme(), "https" | "ssh")
        || url.password().is_some()
        || url.query().is_some()
        || url.fragment().is_some()
        || (url.scheme() == "https" && !url.username().is_empty())
    {
        return None;
    }
    let host = url.host_str()?;
    let api = if host == "github.com" && url.port().is_none() {
        "https://api.github.com".into()
    } else {
        accounts
            .iter()
            .find(|a| {
                url::Url::parse(&a.api_url).ok().is_some_and(|api| {
                    api.host_str() == Some(host)
                        && (url.scheme() == "ssh"
                            || api.port_or_known_default() == url.port_or_known_default())
                })
            })?
            .api_url
            .clone()
    };
    let project = url
        .path()
        .trim_start_matches('/')
        .trim_end_matches('/')
        .trim_end_matches(".git");
    if project.split('/').count() != 2 || project.split('/').any(|s| !crate::safe_name(s)) {
        return None;
    }
    Some(Remote {
        name: name.into(),
        api_url: api,
        repository: project.into(),
    })
}

#[derive(Serialize, Deserialize)]
pub struct Credential {
    pub access_token: String,
    pub refresh_token: Option<String>,
    pub expires_at: Option<u64>,
}
impl Drop for Credential {
    fn drop(&mut self) {
        use zeroize::Zeroize;
        self.access_token.zeroize();
        self.refresh_token.zeroize();
    }
}

pub fn api_root(value: &str) -> crate::Result<String> {
    let url = url::Url::parse(value).map_err(|_| crate::Error::Invalid)?;
    if url.scheme() != "https"
        || url.host_str().is_none()
        || !url.username().is_empty()
        || url.password().is_some()
        || url.query().is_some()
        || url.fragment().is_some()
        || url.path().contains("..")
        || (url.host_str() == Some("api.github.com") && !matches!(url.path(), "" | "/"))
        || (url.host_str() != Some("api.github.com")
            && url.path().trim_end_matches('/') != "/api/v3")
    {
        return Err(crate::Error::Invalid);
    }
    Ok(url.as_str().trim_end_matches('/').to_string())
}

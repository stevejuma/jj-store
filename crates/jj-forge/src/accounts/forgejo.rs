//! Explicit native-host Forgejo PAT custody, separate from GitHub device OAuth.
use super::*;
pub fn forgejo_api_root(value: &str) -> Result<String> {
    let url = url::Url::parse(value).map_err(|_| Error::Invalid)?;
    if url.scheme() != "https"
        || url.host_str().is_none()
        || matches!(url.host_str(), Some("github.com" | "api.github.com"))
        || !url.username().is_empty()
        || url.password().is_some()
        || url.query().is_some()
        || url.fragment().is_some()
        || url.path().trim_end_matches('/') != "/api/v1"
    {
        return Err(Error::Invalid);
    }
    Ok(url.as_str().trim_end_matches('/').into())
}
impl Manager {
    /// A downstream host explicitly selects Forgejo and owns caller admission.
    /// Only public identity/availability returns; PAT custody uses its injected
    /// SecretStore and private installation metadata. No GitHub API is called.
    pub async fn add_forgejo_token(&self, api_url: &str, token: String) -> Result<Inventory> {
        let token = zeroize::Zeroizing::new(token);
        if token.trim().is_empty() || token.len() > 4096 || token.contains(['\n', '\r', '\0']) {
            return Err(Error::Invalid);
        }
        let api = forgejo_api_root(api_url)?;
        let user = self
            .transport
            .request(&format!("{api}/user"), Some(&token), None)
            .await?;
        self.persist(
            api,
            Method::Pat,
            user,
            &Credential {
                access_token: token.to_string(),
                refresh_token: None,
                expires_at: None,
            },
        )?;
        self.inventory()
    }
}

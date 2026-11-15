//! An explicit replacement cannot create a different or forgotten account.
use super::*;
use zeroize::Zeroizing;
impl Manager {
    /// Verify the existing provider identity before replacing only its token.
    /// Host admission and pending-publication credential holds remain host-owned.
    pub async fn replace_token(&self, id: &str, token: String) -> Result<Inventory> {
        let token = Zeroizing::new(token);
        if token.trim().is_empty() || token.len() > 4096 || token.chars().any(char::is_control) {
            return Err(Error::Invalid);
        }
        let expected = self.account(id)?;
        let api = api_root(&expected.api_url).or_else(|_| forgejo_api_root(&expected.api_url))?;
        if api != expected.api_url
            || store::account_id(&api, &expected.user_id, &expected.method) != id
        {
            return Err(Error::Denied);
        }
        expected.revision.checked_add(1).ok_or(Error::Limit)?;
        let previous = Zeroizing::new(self.secrets.get(id)?.ok_or(Error::Denied)?);
        let user = self
            .transport
            .request(&format!("{api}/user"), Some(&token), None)
            .await?;
        let account = verified_account(api, expected.method.clone(), user)?;
        if account.id != expected.id {
            return Err(Error::Denied);
        }
        let mut saved = Store::open(&self.root)?;
        let current = saved
            .accounts
            .iter()
            .find(|a| a.id == expected.id)
            .ok_or(Error::Stale)?;
        if current.revision != expected.revision
            || current.api_url != expected.api_url
            || current.user_id != expected.user_id
            || current.method != expected.method
            || self.secrets.get(id)?.as_deref() != Some(previous.as_str())
        {
            return Err(Error::Stale);
        }
        saved.put(
            account,
            &Credential {
                access_token: token.to_string(),
                refresh_token: None,
                expires_at: None,
            },
            self.secrets.as_ref(),
        )?;
        drop(saved);
        self.inventory()
    }
}

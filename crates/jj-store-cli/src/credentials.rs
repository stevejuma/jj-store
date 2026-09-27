//! Credentials are held by the OS facility or injected into this process. There
//! is no file fallback, and endpoint/repository accounts are always distinct.
use crate::binding::Connection;
use jj_store_jj::operation_stores::AuthorityResult;
use jj_store_service::auth::BearerToken;
use sha2::{Digest as _, Sha256};
use zeroize::Zeroizing;

const SERVICE: &str = "md.lapis.jj-store.v1";
pub fn account(connection: &Connection) -> String {
    let endpoint = hex::encode(Sha256::digest(connection.endpoint.as_bytes()));
    format!("{endpoint}/{}", connection.repository)
}
pub fn environment_present() -> bool {
    std::env::var_os("JJ_STORE_TOKEN").is_some()
}
pub fn resolve(connection: &Connection) -> AuthorityResult<BearerToken> {
    if let Some(value) = std::env::var_os("JJ_STORE_TOKEN") {
        let text = Zeroizing::new(
            value
                .into_string()
                .map_err(|_| "JJ_STORE_TOKEN must be valid UTF-8")?,
        );
        return Ok(BearerToken::parse(&text).map_err(|_| "invalid JJ_STORE_TOKEN credential")?);
    }
    let text = Zeroizing::new(entry(connection)?.get_password().map_err(
        |_| "native credential is unavailable; run jj store login or provide JJ_STORE_TOKEN",
    )?);
    Ok(BearerToken::parse(&text)
        .map_err(|_| "stored native credential is invalid; run jj store login")?)
}
pub fn save(connection: &Connection, token: &BearerToken) -> AuthorityResult<()> {
    entry(connection)?
        .set_password(&token.expose())
        .map_err(|_| "OS credential facility could not save the token")?;
    Ok(())
}
pub fn remove(connection: &Connection) -> AuthorityResult<()> {
    match entry(connection)?.delete_credential() {
        Ok(()) | Err(keyring::Error::NoEntry) => Ok(()),
        Err(_) => Err("OS credential facility could not delete the token".into()),
    }
}
fn entry(connection: &Connection) -> AuthorityResult<keyring::Entry> {
    connection.validate()?;
    keyring::Entry::new(SERVICE, &account(connection))
        .map_err(|_| "OS credential facility is unavailable".into())
}

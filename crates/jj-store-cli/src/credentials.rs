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

fn operator_entry(endpoint: &str) -> AuthorityResult<keyring::Entry> {
    let digest = hex::encode(Sha256::digest(endpoint.as_bytes()));
    keyring::Entry::new(SERVICE, &format!("operator/{digest}"))
        .map_err(|_| "OS operator credential facility unavailable".into())
}
pub fn operator_environment() -> AuthorityResult<Option<BearerToken>> {
    let Some(value) = std::env::var_os("JJ_STORE_OPERATOR_TOKEN") else {
        return Ok(None);
    };
    let text = Zeroizing::new(
        value
            .into_string()
            .map_err(|_| "invalid operator credential encoding")?,
    );
    Ok(Some(BearerToken::parse(&text).map_err(
        |_| "invalid JJ_STORE_OPERATOR_TOKEN credential",
    )?))
}
pub fn operator_resolve(endpoint: &str) -> AuthorityResult<BearerToken> {
    if let Some(token) = operator_environment()? {
        return Ok(token);
    }
    let text = Zeroizing::new(operator_entry(endpoint)?.get_password().map_err(
        |_| "operator credential unavailable; provision it or set JJ_STORE_OPERATOR_TOKEN",
    )?);
    Ok(BearerToken::parse(&text).map_err(|_| "invalid stored operator credential")?)
}
/// Explicit provisioning only: reuse an existing account, or durably save a
/// new secret before creating its policy verifier. Never overwrite credentials.
pub fn operator_provision(endpoint: &str) -> AuthorityResult<BearerToken> {
    if let Some(token) = operator_environment()? {
        return Ok(token);
    }
    let directory = dirs::data_local_dir()
        .ok_or("OS credential lock directory unavailable")?
        .join("jj-store/operator-credentials");
    with_operator_lock(&directory, endpoint, || {
        provision_operator_account(endpoint)
    })
}
fn with_operator_lock<T>(
    directory: &std::path::Path,
    endpoint: &str,
    provision: impl FnOnce() -> AuthorityResult<T>,
) -> AuthorityResult<T> {
    let directory = jj_store_server::policy::outside_checkout(directory)?;
    let mut builder = std::fs::DirBuilder::new();
    builder.recursive(true);
    #[cfg(unix)]
    {
        use std::os::unix::fs::DirBuilderExt as _;
        builder.mode(0o700);
    }
    builder.create(&directory)?;
    let digest = hex::encode(Sha256::digest(endpoint.as_bytes()));
    let _lock = jj_store_server::policy::exclusive_lock(&directory.join(format!("{digest}.lock")))?;
    provision()
}
fn provision_operator_account(endpoint: &str) -> AuthorityResult<BearerToken> {
    let entry = operator_entry(endpoint)?;
    match entry.get_password() {
        Ok(text) => Ok(BearerToken::parse(&Zeroizing::new(text))
            .map_err(|_| "invalid stored operator credential")?),
        Err(keyring::Error::NoEntry) => {
            let token = BearerToken::generate()?;
            entry
                .set_password(&token.expose())
                .map_err(|_| "OS operator credential could not be saved")?;
            Ok(token)
        }
        Err(_) => Err("OS operator credential facility unavailable".into()),
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn competing_operator_provision_cannot_replace_the_first_credential() {
        let root = tempfile::tempdir().unwrap();
        let endpoint = "https://operator-test.example.invalid/";
        let (ready, started) = std::sync::mpsc::channel();
        let (release, wait) = std::sync::mpsc::channel();
        let path = root.path().to_owned();
        let first = std::thread::spawn(move || {
            with_operator_lock(&path, endpoint, || {
                ready.send(()).unwrap();
                wait.recv_timeout(std::time::Duration::from_secs(5))
                    .unwrap();
                Ok(())
            })
        });
        started
            .recv_timeout(std::time::Duration::from_secs(5))
            .unwrap();
        assert!(
            with_operator_lock(root.path(), endpoint, || -> AuthorityResult<()> {
                panic!("competing provisioning reached credential write")
            })
            .is_err()
        );
        release.send(()).unwrap();
        first.join().unwrap().unwrap();
        assert!(with_operator_lock(root.path(), endpoint, || Ok(())).is_ok());
    }
}

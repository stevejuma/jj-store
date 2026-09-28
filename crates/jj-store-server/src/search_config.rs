//! Access-specific configuration for rebuildable repository search workers.
use crate::{Error, Result};
use jj_store_model as m;
use jj_store_service::auth::BearerToken;
use jj_store_tantivy::native;
use serde::{Deserialize, Serialize};
use std::collections::BTreeSet;
use zeroize::Zeroizing;

#[derive(Clone, Debug, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct Repository {
    pub repository: m::RepositoryId,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub token_environment: Option<String>,
    #[serde(default)]
    pub source: native::Config,
    #[serde(default = "poll_millis")]
    pub poll_millis: u64,
    #[serde(default = "timeout_seconds")]
    pub timeout_seconds: u64,
    #[serde(default = "index_bytes")]
    pub index_bytes: u64,
    #[serde(default = "snapshot_every")]
    pub snapshot_every: u64,
}
fn poll_millis() -> u64 {
    1000
}
fn timeout_seconds() -> u64 {
    60
}
fn index_bytes() -> u64 {
    512 * 1024 * 1024
}
fn snapshot_every() -> u64 {
    32
}
pub(crate) fn validate(entries: &[Repository], authenticated: bool) -> Result<()> {
    if entries.len() > 8 {
        return Err(Error::Config("search repository limit"));
    }
    let mut repositories = BTreeSet::new();
    let mut credentials = BTreeSet::new();
    for entry in entries {
        let valid_credential = match &entry.token_environment {
            Some(name) if authenticated => {
                let suffix = name.strip_prefix("JJ_STORE_SEARCH_TOKEN_");
                credentials.insert(name)
                    && name.len() <= 128
                    && suffix.is_some_and(|suffix| {
                        !suffix.is_empty()
                            && suffix.bytes().all(|byte| {
                                byte.is_ascii_uppercase() || byte.is_ascii_digit() || byte == b'_'
                            })
                    })
            }
            None if !authenticated => true,
            _ => false,
        };
        if !repositories.insert(entry.repository)
            || !valid_credential
            || !(10..=60000).contains(&entry.poll_millis)
            || !(1..=300).contains(&entry.timeout_seconds)
            || !(1..=2 * 1024 * 1024 * 1024).contains(&entry.index_bytes)
            || entry.snapshot_every > 1_000_000
            || entry.source.identity().is_err()
        {
            return Err(Error::Config(
                "search repository, credential name, source scopes or limits",
            ));
        }
    }
    Ok(())
}

#[derive(Clone, Copy)]
pub enum SearchAccess {
    Authenticated,
    LocalTrusted(m::PrincipalId),
}

pub(crate) fn environment(name: &str) -> Result<BearerToken> {
    let value = Zeroizing::new(
        std::env::var(name).map_err(|_| Error::Config("search credential unavailable"))?,
    );
    BearerToken::parse(&value).map_err(|_| Error::Config("invalid search credential"))
}

//! Owned local stacks protocol. These contracts grant no execution authority.
mod command;
mod content;
mod prepared;
mod receipt;
mod request;
mod selection;
pub use command::*;
pub use content::*;
use jj_store_model as m;
pub use prepared::*;
pub use receipt::*;
pub use request::*;
pub use selection::*;
use sha2::{Digest as _, Sha256};

pub const VERSION: u32 = 1;
pub const MAX_SELECTIONS: usize = 256;
#[derive(Clone, Copy, Debug, PartialEq, Eq, thiserror::Error)]
pub enum Error {
    #[error("invalid stack workspace input")]
    Invalid,
    #[error("stack workspace resource limit")]
    Limit,
    #[error("stack workspace identity profile is unsupported")]
    UnsupportedProfile,
    #[error("stack workspace selection changed")]
    Stale,
    #[error("request identity does not match the retained intent")]
    IntentMismatch,
}
pub type Result<T> = std::result::Result<T, Error>;

fn digest(domain: &[u8], value: &impl serde::Serialize) -> Result<m::RecordDigest> {
    let value = serde_json::to_value(value).map_err(|_| Error::Invalid)?;
    // Canonical key order must not depend on serde_json's preserve_order feature,
    // which is enabled by another package in this workspace.
    fn ordered(value: serde_json::Value) -> serde_json::Value {
        match value {
            serde_json::Value::Object(map) => {
                let ordered: std::collections::BTreeMap<_, _> = map
                    .into_iter()
                    .map(|(key, value)| (key, ordered(value)))
                    .collect();
                serde_json::Value::Object(ordered.into_iter().collect())
            }
            serde_json::Value::Array(values) => {
                serde_json::Value::Array(values.into_iter().map(ordered).collect())
            }
            value => value,
        }
    }
    let bytes = serde_json::to_vec(&ordered(value)).map_err(|_| Error::Invalid)?;
    let mut hash = Sha256::new_with_prefix(domain);
    hash.update(bytes);
    Ok(m::RecordDigest::new(hash.finalize().into()))
}
fn text(value: &str, max: usize) -> Result<()> {
    if value.is_empty() || value.chars().any(char::is_control) {
        return Err(Error::Invalid);
    }
    if value.len() > max {
        return Err(Error::Limit);
    }
    Ok(())
}

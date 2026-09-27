//! Rebuildable projections consume only selected history and cannot publish it.
#![forbid(unsafe_code)]
mod artifacts;
mod context;
mod progress;
mod runner;
mod source;
pub use artifacts::{ArtifactBlob, ArtifactBundle, Artifacts};
pub use context::{Context, Limits};
use jj_store_model as m;
use jj_store_wal::{
    publication::ManifestSnapshot,
    records::{RecordKind, RecordRef},
};
pub use runner::{Health, Projection, Runner};
use serde::{Deserialize, Serialize};
pub use source::{Authorizer, Batch, Objects, Snapshot, Source};
pub use tokio_util::sync::CancellationToken;

#[derive(Clone, Debug, PartialEq, Eq, thiserror::Error)]
pub enum Error {
    #[error("adapter access denied")]
    Denied,
    #[error("invalid adapter configuration or checkpoint")]
    Invalid,
    #[error("adapter source or checkpoint integrity failure")]
    Corrupt,
    #[error("adapter resource limit exceeded")]
    Limit,
    #[error("adapter work canceled")]
    Canceled,
    #[error("adapter writer is already owned or still running")]
    Busy,
    #[error("adapter storage unavailable or checkpoint outcome unknown")]
    Unavailable,
    #[error("adapter cursor was changed by another owner")]
    CursorConflict,
    #[error("projection failed; the source record has not been skipped")]
    Projection,
}
pub type Result<T> = std::result::Result<T, Error>;
impl From<jj_store_object::Error> for Error {
    fn from(value: jj_store_object::Error) -> Self {
        match value {
            jj_store_object::Error::Limit => Self::Limit,
            _ => Self::Unavailable,
        }
    }
}
impl From<jj_store_wal::Error> for Error {
    fn from(value: jj_store_wal::Error) -> Self {
        match value {
            jj_store_wal::Error::Denied => Self::Denied,
            jj_store_wal::Error::Limit(_) => Self::Limit,
            jj_store_wal::Error::Corrupt(_) | jj_store_wal::Error::Json(_) => Self::Corrupt,
            _ => Self::Unavailable,
        }
    }
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct Identity {
    pub name: String,
    pub implementation: String,
    pub schema: u32,
    pub config: m::RecordDigest,
}
impl Identity {
    pub fn validate(&self) -> Result<()> {
        let valid = |s: &str| {
            !s.is_empty()
                && s.len() <= 64
                && s.bytes()
                    .all(|b| b.is_ascii_alphanumeric() || b"-_ .".contains(&b))
        };
        if !valid(&self.name)
            || self.name.contains([' ', '.'])
            || !valid(&self.implementation)
            || self.schema == 0
        {
            return Err(Error::Invalid);
        }
        Ok(())
    }
}
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct Position {
    pub repository: m::RepositoryId,
    pub epoch: m::EpochId,
    pub position: m::WalPosition,
    pub segment: Option<RecordRef>,
}
impl Position {
    fn current(snapshot: &ManifestSnapshot) -> Self {
        let manifest = snapshot.manifest();
        Self {
            repository: manifest.repository,
            epoch: manifest.epoch,
            position: manifest.published,
            segment: manifest.tail,
        }
    }
    fn validate(&self) -> Result<()> {
        if (self.position.0 == 0) != self.segment.is_none()
            || self
                .segment
                .is_some_and(|s| s.kind != RecordKind::Wal || s.attempt.is_none())
        {
            return Err(Error::Corrupt);
        }
        Ok(())
    }
}
/// The artifact identifies a destination-owned durable commit descriptor. The
/// projection must verify the associated files when recovering this checkpoint.
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct Checkpoint {
    pub version: u32,
    pub identity: Identity,
    pub source: Position,
    pub artifact: m::RecordDigest,
}
impl Checkpoint {
    fn validate(&self) -> Result<()> {
        if self.version != 1 {
            return Err(Error::Corrupt);
        }
        self.identity.validate().map_err(|_| Error::Corrupt)?;
        self.source.validate()
    }
}

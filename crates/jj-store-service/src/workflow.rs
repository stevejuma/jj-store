//! Workflow home authority shared by HTTP, RPC and CLI consumers.
mod commands;
mod matching;
mod reads;
mod source;
pub use commands::*;
pub use jj_workflow::*;
pub use reads::*;
pub use source::*;
pub type Result<T> = std::result::Result<T, Error>;
#[derive(Debug, thiserror::Error)]
pub enum Error {
    #[error("workflow access denied")]
    Denied,
    #[error("workflow precondition changed")]
    Stale,
    #[error("workflow request identity conflict")]
    IntentConflict,
    #[error("workflow target missing")]
    NotFound,
    #[error("invalid workflow request")]
    Invalid,
    #[error("workflow integrity failure")]
    Corrupt,
    #[error("workflow limit exceeded")]
    Limit,
    #[error("workflow authority unavailable")]
    Unavailable,
}
impl From<jj_store_wal::Error> for Error {
    fn from(error: jj_store_wal::Error) -> Self {
        use jj_store_wal::Error as E;
        match error {
            E::Denied => Self::Denied,
            E::Stale => Self::Stale,
            E::IntentConflict => Self::IntentConflict,
            E::Limit(_) => Self::Limit,
            E::Corrupt(_) | E::Json(_) => Self::Corrupt,
            E::Unsupported(_) => Self::Invalid,
            _ => Self::Unavailable,
        }
    }
}
impl From<jj_store_wal::publication::WorkflowCommandError> for Error {
    fn from(error: jj_store_wal::publication::WorkflowCommandError) -> Self {
        use jj_store_wal::publication::WorkflowCommandError as E;
        match error {
            E::Publication(e) => e.into(),
            E::ReplicationUnsupported => Self::Invalid,
            E::Model(e) => match e {
                jj_workflow::Error::Denied => Self::Denied,
                jj_workflow::Error::Stale => Self::Stale,
                jj_workflow::Error::NotFound => Self::NotFound,
                jj_workflow::Error::Limit => Self::Limit,
                _ => Self::Invalid,
            },
        }
    }
}

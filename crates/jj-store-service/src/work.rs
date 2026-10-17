//! Home-authorized native project/task reads and receipt-backed commands.
mod activity;
mod browse;
pub(crate) use activity::activity_rows;
pub use activity::{WorkActivityGroup, WorkActivityPage, WorkActivityQuery};
pub(crate) mod projection;
pub(crate) use browse::browse_selected;
mod browse_models;
mod commands;
pub use browse_models::*;
mod cursors;
mod indexes;
mod models;
mod reads;
pub use commands::*;
use jj_store_wal as wal;
pub use models::*;
pub use reads::WorkPager;
pub(crate) use reads::{overhead, take_page};
pub use wal::publication::{
    ProjectId, ProjectV1, TaskId, TaskStateV1, TaskV1, WorkCommandResultV1, WorkCommandV1,
    WorkCommentId, WorkCommentV1,
};
pub type Result<T> = std::result::Result<T, Error>;
#[derive(Debug, Clone, PartialEq, Eq, thiserror::Error)]
pub enum Error {
    #[error("WorkStore access denied")]
    Denied,
    #[error("WorkStore precondition is stale")]
    Stale,
    #[error("request identity has different intent")]
    IntentConflict,
    #[error("WorkStore target not found")]
    NotFound,
    #[error("invalid WorkStore command or query")]
    Invalid,
    #[error("WorkStore integrity failure")]
    Corrupt,
    #[error("WorkStore resource limit exceeded")]
    Limit,
    #[error("WorkStore authority unavailable")]
    Unavailable,
    #[error("WorkStore cursor expired or serving instance changed; refresh the snapshot")]
    CursorReset,
    #[error("WorkStore query or page size changed")]
    InvalidCursor,
    #[error("WorkStore admission is busy")]
    Busy,
    #[error("WorkStore origin replication is unsupported")]
    ReplicationUnsupported,
}
impl From<wal::Error> for Error {
    fn from(error: wal::Error) -> Self {
        match error {
            wal::Error::Denied => Self::Denied,
            wal::Error::Unsupported(_) => Self::ReplicationUnsupported,
            wal::Error::Stale => Self::Stale,
            wal::Error::IntentConflict => Self::IntentConflict,
            wal::Error::Limit(_) => Self::Limit,
            wal::Error::Corrupt(_) | wal::Error::Json(_) => Self::Corrupt,
            _ => Self::Unavailable,
        }
    }
}
impl From<wal::publication::WorkCommandError> for Error {
    fn from(error: wal::publication::WorkCommandError) -> Self {
        use wal::publication::WorkCommandError as E;
        match error {
            E::Publication(error) => error.into(),
            E::ReplicationUnsupported => Self::ReplicationUnsupported,
            E::Model(error) => match error {
                // Use neutral errors through the public owned alias, below.
                wal::publication::WorkModelError::Stale => Self::Stale,
                wal::publication::WorkModelError::NotFound(_) => Self::NotFound,
                wal::publication::WorkModelError::Limit => Self::Limit,
                wal::publication::WorkModelError::Invalid(_) => Self::Invalid,
            },
        }
    }
}

impl From<jj_store_core::Error> for Error {
    fn from(value: jj_store_core::Error) -> Self {
        jj_store_wal::Error::from(value).into()
    }
}

impl From<jj_store_git::work_api::Error> for Error {
    fn from(e: jj_store_git::work_api::Error) -> Self {
        use jj_store_git::work_api::Error as E;
        match e {
            E::Denied => Self::Denied,
            E::Stale => Self::Stale,
            E::IntentConflict => Self::IntentConflict,
            E::NotFound => Self::NotFound,
            E::Invalid => Self::Invalid,
            E::Corrupt => Self::Corrupt,
            E::Limit => Self::Limit,
            E::Unavailable => Self::Unavailable,
            E::Busy => Self::Busy,
        }
    }
}

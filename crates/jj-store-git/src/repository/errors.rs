#[derive(Debug, Clone, PartialEq, Eq, thiserror::Error)]
pub enum LocalError {
    #[error("invalid local collaboration request")]
    Invalid,
    #[error("local collaboration precondition is stale")]
    Stale,
    #[error("local collaboration target was not found")]
    NotFound,
    #[error("request identity has different intent")]
    IntentConflict,
    #[error("local collaboration integrity failure")]
    Corrupt,
    #[error("local collaboration resource limit exceeded")]
    Limit,
    #[error("local collaboration storage unavailable")]
    Unavailable,
    #[error("local collaboration admission is busy")]
    Busy,
    #[error("local collaboration representation is unsupported")]
    Unsupported,
    #[error("review thread is resolved")]
    ThreadResolved,
    #[error("review thread is already open")]
    ThreadOpen,
}
pub type LocalResult<T> = Result<T, LocalError>;
impl From<jj_collab::Error> for LocalError {
    fn from(error: jj_collab::Error) -> Self {
        match error {
            jj_collab::Error::Stale => Self::Stale,
            jj_collab::Error::TargetUnavailable => Self::NotFound,
            jj_collab::Error::ReadUnavailable => Self::Unavailable,
            jj_collab::Error::Limit => Self::Limit,
            _ => Self::Invalid,
        }
    }
}
impl From<jj_store_core::engine::ReviewCommandError> for LocalError {
    fn from(error: jj_store_core::engine::ReviewCommandError) -> Self {
        use jj_store_core::engine::ReviewCommandError as E;
        match error {
            E::Publication(error) => error.into(),
            E::ThreadResolved => Self::ThreadResolved,
            E::ThreadOpen => Self::ThreadOpen,
            E::Stale => Self::Stale,
            E::NotFound(_) => Self::NotFound,
            E::Invalid(_) => Self::Invalid,
        }
    }
}
impl From<crate::metadata::Error> for LocalError {
    fn from(error: crate::metadata::Error) -> Self {
        use crate::metadata::Error as E;
        match error {
            E::Stale => Self::Stale,
            E::IntentConflict => Self::IntentConflict,
            E::Limit => Self::Limit,
            E::Invalid(_) => Self::Corrupt,
            _ => Self::Unavailable,
        }
    }
}

impl From<jj_store_core::Error> for LocalError {
    fn from(e: jj_store_core::Error) -> Self {
        use jj_store_core::Error as E;
        match e {
            E::ThreadResolved => Self::ThreadResolved,
            E::ThreadOpen => Self::ThreadOpen,
            E::Stale => Self::Stale,
            E::IntentConflict => Self::IntentConflict,
            E::NotFound => Self::Unavailable,
            E::Limit(_) => Self::Limit,
            E::Corrupt(_) | E::Json(_) | E::Invalid(_) => Self::Corrupt,
            E::Unsupported(_) => Self::Unsupported,
            E::Contended => Self::Busy,
            _ => Self::Unavailable,
        }
    }
}
impl From<crate::work_api::Error> for LocalError {
    fn from(e: crate::work_api::Error) -> Self {
        use crate::work_api::Error as E;
        match e {
            E::Stale => Self::Stale,
            E::IntentConflict => Self::IntentConflict,
            E::NotFound => Self::NotFound,
            E::Limit => Self::Limit,
            E::Corrupt => Self::Corrupt,
            E::Invalid => Self::Invalid,
            E::Busy => Self::Busy,
            _ => Self::Unavailable,
        }
    }
}

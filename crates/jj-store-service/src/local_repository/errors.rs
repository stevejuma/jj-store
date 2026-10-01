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
impl From<jj_store_wal::publication::ReviewCommandError> for LocalError {
    fn from(error: jj_store_wal::publication::ReviewCommandError) -> Self {
        use jj_store_wal::publication::ReviewCommandError as E;
        match error {
            E::Publication(error) => error.into(),
            E::Stale => Self::Stale,
            E::ThreadResolved => Self::ThreadResolved,
            E::ThreadOpen => Self::ThreadOpen,
            E::NotFound(_) => Self::NotFound,
            E::Invalid(_) => Self::Invalid,
        }
    }
}
impl From<jj_store_wal::Error> for LocalError {
    fn from(error: jj_store_wal::Error) -> Self {
        use jj_store_wal::Error as E;
        match error {
            E::Stale => Self::Stale,
            E::IntentConflict => Self::IntentConflict,
            E::Limit(_) => Self::Limit,
            E::Corrupt(_) | E::Json(_) => Self::Corrupt,
            E::Unsupported(_) => Self::Unsupported,
            E::Contended => Self::Busy,
            _ => Self::Unavailable,
        }
    }
}
impl From<jj_store_jj::local_metadata::Error> for LocalError {
    fn from(error: jj_store_jj::local_metadata::Error) -> Self {
        use jj_store_jj::local_metadata::Error as E;
        match error {
            E::Stale => Self::Stale,
            E::IntentConflict => Self::IntentConflict,
            E::Limit => Self::Limit,
            E::Invalid(_) => Self::Corrupt,
            _ => Self::Unavailable,
        }
    }
}
impl From<crate::work::Error> for LocalError {
    fn from(error: crate::work::Error) -> Self {
        use crate::work::Error as E;
        match error {
            E::Invalid => Self::Invalid,
            E::Stale => Self::Stale,
            E::NotFound => Self::NotFound,
            E::IntentConflict => Self::IntentConflict,
            E::Corrupt => Self::Corrupt,
            E::Limit => Self::Limit,
            E::Busy => Self::Busy,
            _ => Self::Unavailable,
        }
    }
}
impl From<jj_store_object::Error> for LocalError {
    fn from(error: jj_store_object::Error) -> Self {
        match error {
            jj_store_object::Error::Limit => Self::Limit,
            jj_store_object::Error::Response(_) => Self::Corrupt,
            _ => Self::Unavailable,
        }
    }
}

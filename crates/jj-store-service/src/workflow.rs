//! Workflow home authority shared by HTTP, RPC and CLI consumers.
mod commands;
mod matching;
pub(crate) mod projection;
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

/// Shared transport admission; the lease fences revocation through publication.
#[derive(Clone, Debug, Default)]
pub struct HostAdmission {
    values: std::sync::Arc<
        tokio::sync::RwLock<std::collections::BTreeSet<jj_store_model::PrincipalId>>,
    >,
}
impl HostAdmission {
    pub async fn replace(&self, value: std::collections::BTreeSet<jj_store_model::PrincipalId>) {
        *self.values.write().await = value;
    }
    pub async fn replacement(
        &self,
    ) -> tokio::sync::OwnedRwLockWriteGuard<std::collections::BTreeSet<jj_store_model::PrincipalId>>
    {
        self.values.clone().write_owned().await
    }
    async fn authorize(
        &self,
        principal: jj_store_model::PrincipalId,
    ) -> Result<
        tokio::sync::OwnedRwLockReadGuard<std::collections::BTreeSet<jj_store_model::PrincipalId>>,
    > {
        let lease = self.values.clone().read_owned().await;
        if !lease.contains(&principal) {
            return Err(Error::Denied);
        }
        Ok(lease)
    }
}
impl From<std::collections::BTreeSet<jj_store_model::PrincipalId>> for HostAdmission {
    fn from(values: std::collections::BTreeSet<jj_store_model::PrincipalId>) -> Self {
        Self {
            values: std::sync::Arc::new(tokio::sync::RwLock::new(values)),
        }
    }
}
impl<const N: usize> From<[jj_store_model::PrincipalId; N]> for HostAdmission {
    fn from(values: [jj_store_model::PrincipalId; N]) -> Self {
        std::collections::BTreeSet::from(values).into()
    }
}

#[cfg(test)]
mod admission_tests {
    use super::*;
    #[tokio::test]
    async fn replacement_fences_live_publication_and_updates_every_clone() {
        let principal = jj_store_model::PrincipalId::new([42; 16]);
        let admission: HostAdmission = [principal].into();
        let other_transport = admission.clone();
        let lease = admission.authorize(principal).await.unwrap();
        let replace = tokio::spawn(async move {
            other_transport.replace(Default::default()).await;
            other_transport
        });
        tokio::task::yield_now().await;
        assert!(!replace.is_finished());
        drop(lease);
        let other_transport = replace.await.unwrap();
        assert!(matches!(
            admission.authorize(principal).await,
            Err(Error::Denied)
        ));
        assert!(matches!(
            other_transport.authorize(principal).await,
            Err(Error::Denied)
        ));
    }
}

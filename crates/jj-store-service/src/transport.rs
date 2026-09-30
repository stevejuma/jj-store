//! Typed native transport facade. No upstream JJ types or mutable storage keys
//! cross this interface. Object staging cannot advance publication authority.
use crate::{
    administration::{TokenListing, TokenReceipt, TokenRequest},
    auth::{BearerToken, Clock, Permission},
    native::{NativeSession, SessionLimits},
};
use jj_store_jj::{
    objects::{NativeObjects, validate_upload},
    policy::Protection,
};
use jj_store_model as m;
use jj_store_object::{
    Condition, Download, Key, Metadata, ObjectStore, StagedUpload, WriteOutcome,
};
use jj_store_wal as wal;
use jj_store_wal::publication as replication;
pub use jj_store_wal::publication::{
    AgentDeltaRequest, ApplyOriginsTransportRequestV1, ApplyOriginsTransportResponseV1,
    FeedRequestV1, FeedResponseV1, NegotiateReplicaV1, NormalizerProfileV1, OfferOriginsRequestV1,
    OfferOriginsResponseV1, OriginClosureRequestV1, OriginClosureResponseV1, PublishOutcome,
    Receipt, ReplicaCapabilitiesV1, ReplicaNegotiationV1, SanitizedObservationBatchV1, Snapshot,
};
pub use jj_store_wal::records::{MAX_RECORD_BYTES, RecordKind, RecordRef};
use std::{collections::BTreeSet, sync::Arc};

#[derive(Debug, thiserror::Error)]
pub enum Error {
    #[error("request is not authorized")]
    Denied,
    #[error("invalid native request")]
    Invalid,
    #[error("object not found")]
    NotFound,
    #[error("request resource limit exceeded")]
    Limit,
    #[error("publication outcome unknown; retry the original transition")]
    Unknown,
    #[error("publication conflict")]
    Conflict,
    #[error("native service unavailable")]
    Unavailable,
}
pub type Result<T> = std::result::Result<T, Error>;
impl From<wal::Error> for Error {
    fn from(error: wal::Error) -> Self {
        match error {
            wal::Error::Denied => Self::Denied,
            wal::Error::Limit(_) => Self::Limit,
            wal::Error::OutcomeUnknown => Self::Unknown,
            wal::Error::Stale
            | wal::Error::IntentConflict
            | wal::Error::Contended
            | wal::Error::AlreadyExists => Self::Conflict,
            wal::Error::Storage(error) => error.into(),
            _ => Self::Unavailable,
        }
    }
}
impl From<jj_store_object::Error> for Error {
    fn from(error: jj_store_object::Error) -> Self {
        match error {
            jj_store_object::Error::NotFound => Self::NotFound,
            jj_store_object::Error::Limit => Self::Limit,
            jj_store_object::Error::Invalid(_) | jj_store_object::Error::Range => Self::Invalid,
            _ => Self::Unavailable,
        }
    }
}
/// Trusted server configuration. Tokens are supplied separately for each RPC.
#[derive(Clone)]
pub struct NativeEndpoint {
    pub(crate) events: crate::events::Hints,
    repository: m::RepositoryId,
    store: Arc<dyn ObjectStore>,
    objects: Arc<NativeObjects>,
    clock: Arc<dyn Clock>,
    protection: Protection,
    limits: SessionLimits,
    max_blob: u64,
}
impl NativeEndpoint {
    pub fn new(
        repository: m::RepositoryId,
        store: Arc<dyn ObjectStore>,
        clock: Arc<dyn Clock>,
        protection: Protection,
        limits: SessionLimits,
        max_blob: u64,
    ) -> Result<Self> {
        let objects = Arc::new(
            NativeObjects::new(store.clone(), repository, max_blob).map_err(|_| Error::Invalid)?,
        );
        Ok(Self {
            events: crate::events::Hints::default(),
            repository,
            store,
            objects,
            clock,
            protection,
            limits,
            max_blob,
        })
    }
    pub fn repository(&self) -> m::RepositoryId {
        self.repository
    }
    pub fn bind(&self, token: BearerToken) -> Result<NativeRequest> {
        let mut session = NativeSession::new(
            self.store.clone(),
            self.objects.clone(),
            token,
            self.clock.clone(),
            self.protection.clone(),
            self.limits,
        )?;
        session.events = self.events.clone();
        Ok(NativeRequest {
            endpoint: self.clone(),
            session,
        })
    }
    pub fn bind_local_trusted(
        &self,
        principal: m::PrincipalId,
        permission: Permission,
    ) -> Result<NativeRequest> {
        let mut session = NativeSession::new_local_trusted(
            self.store.clone(),
            self.objects.clone(),
            principal,
            permission,
            self.clock.clone(),
            self.protection.clone(),
            self.limits,
        )?;
        session.events = self.events.clone();
        Ok(NativeRequest {
            endpoint: self.clone(),
            session,
        })
    }
}
pub struct NativeRequest {
    endpoint: NativeEndpoint,
    session: NativeSession,
}
impl NativeRequest {
    pub fn into_session(self) -> Arc<NativeSession> {
        Arc::new(self.session)
    }
    pub async fn tokens(&self) -> Result<TokenListing> {
        Ok(self.session.tokens().await?)
    }
    pub async fn change_token(&self, request: &TokenRequest) -> Result<Option<TokenReceipt>> {
        match self.session.change_token(request).await? {
            PublishOutcome::Committed(receipt) => Ok(Some(receipt.try_into()?)),
            PublishOutcome::OutcomeUnknown { .. } => Ok(None),
        }
    }
    pub async fn token_status(&self, request: &TokenRequest) -> Result<Option<TokenReceipt>> {
        self.session
            .token_status(request)
            .await?
            .map(TokenReceipt::try_from)
            .transpose()
    }
    pub async fn prepare_agent_capture(
        &self,
        request_id: m::RequestId,
        publication_id: m::PublicationId,
        profile: &NormalizerProfileV1,
        batch: &SanitizedObservationBatchV1,
        result: Vec<u8>,
    ) -> Result<AgentDeltaRequest> {
        Ok(self
            .session
            .prepare_agent_capture(request_id, publication_id, profile, batch, result)
            .await?)
    }
    pub async fn publish_agent_delta(&self, request: &AgentDeltaRequest) -> Result<PublishOutcome> {
        Ok(self.session.publish_agent_delta(request).await?)
    }
    pub async fn authorize(&self, write: bool) -> Result<()> {
        self.session
            .principal(if write {
                Permission::Write
            } else {
                Permission::Read
            })
            .await?;
        Ok(())
    }
    pub async fn negotiate_replica(
        &self,
        request: &replication::NegotiateReplicaV1,
    ) -> Result<replication::ReplicaNegotiationV1> {
        Ok(self.session.negotiate_replica(request).await?)
    }
    pub async fn replication_feed(
        &self,
        request: &replication::FeedRequestV1,
    ) -> Result<replication::FeedResponseV1> {
        Ok(self.session.replication_feed(request).await?)
    }
    pub async fn offer_origins(
        &self,
        request: &replication::OfferOriginsRequestV1,
    ) -> Result<replication::OfferOriginsResponseV1> {
        Ok(self.session.offer_origins(request).await?)
    }
    pub async fn origin_closure(
        &self,
        request: &replication::OriginClosureRequestV1,
    ) -> Result<replication::OriginClosureResponseV1> {
        Ok(self.session.origin_closure(request).await?)
    }
    pub async fn apply_origins(
        &self,
        request: &replication::ApplyOriginsTransportRequestV1,
    ) -> Result<replication::ApplyOriginsTransportResponseV1> {
        Ok(self.session.apply_origins(request).await?)
    }
    pub async fn read_replication_record(
        &self,
        reference: wal::records::RecordRef,
    ) -> Result<Vec<u8>> {
        Ok(self.session.read_replication_record(reference).await?)
    }
    pub async fn write_replication_record(
        &self,
        reference: wal::records::RecordRef,
        bytes: &[u8],
    ) -> Result<WriteOutcome> {
        Ok(self
            .session
            .write_replication_record(reference, bytes)
            .await?)
    }
    fn path(&self, key: m::ObjectKey) -> Key {
        Key::new(key.storage_path(self.endpoint.repository)).expect("typed object path")
    }
    pub async fn head(&self, key: m::ObjectKey) -> Result<Metadata> {
        self.authorize(false).await?;
        let value = self.endpoint.store.head(&self.path(key)).await?;
        self.authorize(false).await?;
        Ok(value)
    }
    pub async fn read(&self, key: m::ObjectKey) -> Result<Download> {
        self.authorize(false).await?;
        let value = self.endpoint.store.get(&self.path(key), None).await?;
        self.authorize(false).await?;
        Ok(value)
    }
    pub async fn upload(&self, key: m::ObjectKey, upload: &StagedUpload) -> Result<WriteOutcome> {
        self.authorize(true).await?;
        validate_upload(key, upload, self.endpoint.max_blob)
            .await
            .map_err(|_| Error::Invalid)?;
        self.authorize(true).await?;
        Ok(self
            .endpoint
            .store
            .put(&self.path(key), Condition::Absent, upload)
            .await?)
    }
    pub async fn heads(&self) -> Result<BTreeSet<m::OperationId>> {
        Ok(self.session.snapshot().await?.manifest().heads.clone())
    }
    pub async fn inventory(&self, copies: bool, limit: usize) -> Result<Vec<Vec<u8>>> {
        if limit == 0 || limit > self.endpoint.limits.inventory_nodes {
            return Err(Error::Limit);
        }
        let keys = self.session.inventory().await?;
        let ids: Vec<_> = keys
            .into_iter()
            .filter(|key| {
                matches!(
                    (copies, key),
                    (true, m::ObjectKey::Copy(_)) | (false, m::ObjectKey::Operation(_))
                )
            })
            .map(|key| key.as_bytes().to_vec())
            .collect();
        if ids.len() > limit {
            return Err(Error::Limit);
        }
        Ok(ids)
    }
    pub async fn publish(&self, old: BTreeSet<m::OperationId>, new: m::OperationId) -> Result<()> {
        let request = self.session.transition_request(old, new).await?;
        match self.session.publish_request(&request).await? {
            PublishOutcome::Committed(_) => Ok(()),
            PublishOutcome::OutcomeUnknown { .. } => Err(Error::Unknown),
        }
    }
}

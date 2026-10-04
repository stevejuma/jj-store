//! Remote native stores registered through the same upstream factory boundary.
use jj_store_jj::{
    backend::{BackendAuthority, BackendLimits},
    factories::{NativeAuthority, NativeStores},
    objects::NativeObjects,
    operation_stores::{AuthorityResult, OperationAuthority, OperationLimits},
};
use jj_store_model as m;
use jj_store_rpc::client::{NativeClient, RemoteObjects};
use std::{
    collections::BTreeSet,
    sync::{Arc, Mutex},
};

#[derive(Clone)]
pub struct BridgeContext {
    pub client: NativeClient,
    pending: Arc<Mutex<Option<jj_store_service::git_bridge::Request>>>,
}
pub struct PendingGit(BridgeContext);
impl Drop for PendingGit {
    fn drop(&mut self) {
        if let Ok(mut pending) = self.0.pending.lock() {
            *pending = None;
        }
    }
}
impl BridgeContext {
    pub fn new(client: NativeClient) -> Self {
        Self {
            client,
            pending: Arc::default(),
        }
    }
    pub fn begin(
        &self,
        request: jj_store_service::git_bridge::Request,
    ) -> AuthorityResult<PendingGit> {
        let mut pending = self
            .pending
            .lock()
            .map_err(|_| "Git publication lock poisoned")?;
        if pending.is_some() {
            return Err("Git publication already bound".into());
        }
        *pending = Some(request);
        Ok(PendingGit(self.clone()))
    }
}
pub struct RemoteAuthority(BridgeContext);
#[async_trait::async_trait]
impl BackendAuthority for RemoteAuthority {
    async fn copy_family(
        &self,
        seeds: &BTreeSet<m::CopyId>,
        required: &BTreeSet<m::CopyId>,
        limits: jj_store_jj::query::copy_index::LookupLimits,
        deadline: tokio::time::Instant,
    ) -> AuthorityResult<Option<jj_store_jj::query::copy_index::Family>> {
        match self
            .0
            .client
            .copy_family(
                seeds,
                required,
                jj_store_service::transport::CopyFamilyLimits {
                    max_nodes: limits.max_nodes,
                    max_work: limits.max_work,
                    max_bytes: limits.max_bytes,
                },
                deadline,
            )
            .await
        {
            Ok(family) => Ok(Some(jj_store_jj::query::copy_index::Family {
                ids: family.ids,
                work: family.work,
            })),
            // Compatibility is explicit; limit/auth/corruption failures never
            // trigger broader work or a different selected lookup.
            Err(error) if error.code() == tonic::Code::Unimplemented => Ok(None),
            Err(error) => Err(error.into()),
        }
    }
    async fn authorize_read(&self) -> AuthorityResult<()> {
        Ok(self.0.client.authorize(false).await?)
    }
    async fn authorize_write(&self) -> AuthorityResult<()> {
        Ok(self.0.client.authorize(true).await?)
    }
    async fn copy_ids(&self, limit: usize) -> AuthorityResult<BTreeSet<m::CopyId>> {
        Ok(self
            .0
            .client
            .inventory(true, limit)
            .await?
            .into_iter()
            .map(m::CopyId::new)
            .collect())
    }
}
#[async_trait::async_trait]
impl OperationAuthority for RemoteAuthority {
    async fn authorize_read(&self) -> AuthorityResult<()> {
        BackendAuthority::authorize_read(self).await
    }
    async fn authorize_write(&self) -> AuthorityResult<()> {
        BackendAuthority::authorize_write(self).await
    }
    async fn heads(&self) -> AuthorityResult<BTreeSet<m::OperationId>> {
        Ok(self.0.client.heads().await?)
    }
    async fn operation_ids(&self, limit: usize) -> AuthorityResult<BTreeSet<m::OperationId>> {
        Ok(self
            .0
            .client
            .inventory(false, limit)
            .await?
            .into_iter()
            .map(m::OperationId::new)
            .collect())
    }
    async fn publish(
        &self,
        old: BTreeSet<m::OperationId>,
        new: m::OperationId,
    ) -> AuthorityResult<()> {
        // A failed response never becomes success. Deterministic server-side
        // transition identities preserve receipt recovery after reconnection.
        let pending = self
            .0
            .pending
            .lock()
            .map_err(|_| "Git publication lock poisoned")?
            .clone();
        if let Some(mut request) = pending {
            request.transition = Some(jj_store_service::git_bridge::NativeTransition {
                remove: old,
                add: new,
            });
            match self.0.client.accept_git_bridge(&request).await? {
                jj_store_service::git_bridge::Outcome::Committed { .. } => Ok(()),
                jj_store_service::git_bridge::Outcome::OutcomeUnknown { request_id } => Err(
                    format!("Git native acceptance outcome unknown; request {request_id}").into(),
                ),
            }
        } else {
            Ok(self.0.client.publish(old, new).await?)
        }
    }
}
impl NativeAuthority for RemoteAuthority {
    fn repository(&self) -> m::RepositoryId {
        self.0.client.repository()
    }
}
pub fn stores(
    client: NativeClient,
    max_blob: u64,
    backend: BackendLimits,
    operations: OperationLimits,
) -> AuthorityResult<NativeStores> {
    stores_with_context(BridgeContext::new(client), max_blob, backend, operations)
}
pub fn stores_with_context(
    context: BridgeContext,
    max_blob: u64,
    backend: BackendLimits,
    operations: OperationLimits,
) -> AuthorityResult<NativeStores> {
    let client = context.client.clone();
    let objects = Arc::new(NativeObjects::new(
        Arc::new(RemoteObjects(client.clone())),
        client.repository(),
        max_blob,
    )?);
    NativeStores::new(
        objects,
        Arc::new(RemoteAuthority(context)),
        backend,
        operations,
    )
}

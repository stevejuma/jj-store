//! Remote native stores registered through the same upstream factory boundary.
use jj_store_jj::{
    backend::{BackendAuthority, BackendLimits},
    factories::{NativeAuthority, NativeStores},
    objects::NativeObjects,
    operation_stores::{AuthorityResult, OperationAuthority, OperationLimits},
};
use jj_store_model as m;
use jj_store_rpc::client::{NativeClient, RemoteObjects};
use std::{collections::BTreeSet, sync::Arc};

pub struct RemoteAuthority(pub NativeClient);
#[async_trait::async_trait]
impl BackendAuthority for RemoteAuthority {
    async fn authorize_read(&self) -> AuthorityResult<()> {
        Ok(self.0.authorize(false).await?)
    }
    async fn authorize_write(&self) -> AuthorityResult<()> {
        Ok(self.0.authorize(true).await?)
    }
    async fn copy_ids(&self, limit: usize) -> AuthorityResult<BTreeSet<m::CopyId>> {
        Ok(self
            .0
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
        Ok(self.0.heads().await?)
    }
    async fn operation_ids(&self, limit: usize) -> AuthorityResult<BTreeSet<m::OperationId>> {
        Ok(self
            .0
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
        Ok(self.0.publish(old, new).await?)
    }
}
impl NativeAuthority for RemoteAuthority {
    fn repository(&self) -> m::RepositoryId {
        self.0.repository()
    }
}
pub fn stores(
    client: NativeClient,
    max_blob: u64,
    backend: BackendLimits,
    operations: OperationLimits,
) -> AuthorityResult<NativeStores> {
    let objects = Arc::new(NativeObjects::new(
        Arc::new(RemoteObjects(client.clone())),
        client.repository(),
        max_blob,
    )?);
    NativeStores::new(
        objects,
        Arc::new(RemoteAuthority(client)),
        backend,
        operations,
    )
}

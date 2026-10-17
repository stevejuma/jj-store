//! Application-owned admission for library store providers.
use super::*;
use jj_store_core::{self as c, *};
use jj_store_native::NativeProvider;
use std::time::{SystemTime, UNIX_EPOCH};
struct SessionHost {
    access: Arc<RepositoryAccess>,
    coordinator: Coordinator,
    actor: m::PrincipalId,
    epoch: m::EpochId,
    preparation_time: Option<i64>,
}
#[async_trait::async_trait]
impl StoreHost for SessionHost {
    async fn authorize(&self, _: StoreKind, access: Access) -> c::Result<()> {
        let permission = match access {
            Access::Read => Permission::Read,
            Access::Write => Permission::Write,
        };
        if self.access.authorize_current(permission).await?.id() != self.actor {
            return Err(c::Error::Denied);
        }
        if self.coordinator.read().await?.manifest().epoch != self.epoch {
            return Err(c::Error::Stale);
        }
        if self.access.authorize_current(permission).await?.id() != self.actor {
            return Err(c::Error::Denied);
        }
        Ok(())
    }
    fn actor(&self, _: bool) -> c::Result<Option<m::PrincipalId>> {
        Ok(Some(self.actor))
    }
    fn now_millis(&self) -> c::Result<i64> {
        if let Some(time) = self.preparation_time {
            return Ok(time);
        }
        SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .map_err(|_| c::Error::Unavailable("clock".into()))?
            .as_millis()
            .try_into()
            .map_err(|_| c::Error::Limit("clock"))
    }
}
impl NativeSession {
    /// Default native profile, composed through the same public named factories
    /// available to independent library hosts. Each handle retains fresh policy.
    pub async fn collaboration_stores(&self) -> c::Result<RepositoryStores> {
        self.collaboration_stores_at(None).await
    }
    pub(crate) async fn collaboration_stores_at(
        &self,
        preparation_time: Option<i64>,
    ) -> c::Result<RepositoryStores> {
        let principal = self.principal(Permission::Read).await?.id();
        let snapshot = self.snapshot().await?;
        let host = Arc::new(SessionHost {
            access: self.access.clone(),
            coordinator: self.coordinator.clone(),
            actor: principal,
            epoch: snapshot.manifest().epoch,
            preparation_time,
        });
        let provider = NativeProvider::new(self.coordinator.clone(), "application")?;
        let mut factories = StoreFactories::empty();
        provider.register(&mut factories, "native")?;
        factories.loader().open(
            &StoreConfig::all("native"),
            StoreContext::new(provider.scope(), host),
        )
    }
}
pub(crate) fn native_receipt<R>(value: StoreReceipt<R>) -> wal::Result<Receipt> {
    let receipt: Receipt = serde_json::from_slice(&value.provider_receipt)?;
    if receipt.request_id != value.request_id
        || receipt.principal != value.actor
        || serde_json::to_vec(&receipt)? != value.provider_receipt
    {
        return Err(wal::Error::Corrupt("native provider receipt adaptation"));
    }
    Ok(receipt)
}
pub(crate) fn publication_outcome<R>(
    value: StoreOutcome<R>,
    publication_id: m::PublicationId,
) -> wal::Result<PublishOutcome> {
    Ok(match value {
        StoreOutcome::Committed(value) => {
            let receipt = native_receipt(value)?;
            if receipt.publication_id != publication_id {
                return Err(wal::Error::IntentConflict);
            }
            PublishOutcome::Committed(receipt)
        }
        StoreOutcome::Unknown { request_id } => PublishOutcome::OutcomeUnknown {
            request_id,
            publication_id,
        },
        StoreOutcome::NotKnown { .. } => {
            return Err(wal::Error::Corrupt("unattempted native publication"));
        }
    })
}

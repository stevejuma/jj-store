//! Injectable native stores retain the coordinator's manifest/WAL authority.
#![forbid(unsafe_code)]
mod agent;
mod commands;
mod snapshots;
pub use agent::{from_agent_delta, to_agent_delta};
use jj_store_core::{self as c, *};
use jj_store_model::{self as m, RequestId};
use jj_store_wal::{self as wal, publication as p};
use std::sync::Arc;

#[derive(Clone)]
pub struct NativeProvider {
    coordinator: p::Coordinator,
    scope: PublicationScope,
    group: Arc<PublicationGroup>,
}
impl NativeProvider {
    /// The host names its object-storage authority; no repository marker is read or written.
    pub fn new(coordinator: p::Coordinator, authority: &str) -> c::Result<Self> {
        if authority.is_empty() {
            return Err(c::Error::Invalid("empty native authority"));
        }
        Ok(Self {
            scope: PublicationScope {
                repository: Some(coordinator.repository_id()),
                authority: format!("native:{authority}:{}", coordinator.repository_id()),
            },
            coordinator,
            group: PublicationGroup::new(),
        })
    }
    pub fn scope(&self) -> PublicationScope {
        self.scope.clone()
    }
    fn bind(&self, name: &str, context: &StoreContext) -> c::Result<Arc<NativeStore>> {
        if context.scope != self.scope {
            return Err(c::Error::Invalid("native provider binding"));
        }
        Ok(Arc::new(NativeStore {
            coordinator: self.coordinator.clone(),
            context: context.clone(),
            name: name.into(),
            group: self.group.clone(),
        }))
    }
}
macro_rules! register {
    ($method:ident) => {
        impl NativeProvider {
            pub fn $method(&self, factories: &mut StoreFactories, name: &str) -> c::Result<()> {
                let provider = self.clone();
                let implementation = name.to_owned();
                factories.$method(name, move |context| {
                    Ok(provider.bind(&implementation, context)?)
                })
            }
        }
    };
}
register!(add_agent_store);
register!(add_work_store);
register!(add_review_store);
register!(add_thread_store);
register!(add_reaction_store);
impl NativeProvider {
    pub fn register(&self, factories: &mut StoreFactories, name: &str) -> c::Result<()> {
        self.add_agent_store(factories, name)?;
        self.add_work_store(factories, name)?;
        self.add_review_store(factories, name)?;
        self.add_thread_store(factories, name)?;
        self.add_reaction_store(factories, name)
    }
}
struct NativeStore {
    coordinator: p::Coordinator,
    context: StoreContext,
    name: String,
    group: Arc<PublicationGroup>,
}
impl StoreBinding for NativeStore {
    fn name(&self) -> &str {
        &self.name
    }
    fn scope(&self) -> &PublicationScope {
        &self.context.scope
    }
    fn publication_group(&self) -> Option<Arc<PublicationGroup>> {
        Some(self.group.clone())
    }
}
impl NativeStore {
    fn actor(&self, writing: bool) -> c::Result<m::PrincipalId> {
        self.context.host.actor(writing)?.ok_or(c::Error::Denied)
    }
    fn publication<C>(&self, request: &StoreRequest<C>) -> m::PublicationId {
        request
            .publication_id
            .unwrap_or_else(|| m::PublicationId::new(*request.request_id.as_bytes()))
    }
    async fn pin(&self, kind: StoreKind) -> c::Result<p::ManifestSnapshot> {
        self.context.admit(kind, Access::Read).await?;
        Ok(self.coordinator.read().await?)
    }
    async fn pinned<T>(
        &self,
        kind: StoreKind,
        pin: &p::ManifestSnapshot,
        data: T,
    ) -> c::Result<Pinned<T>> {
        let domain = pin.manifest().domain_state(domain(kind));
        self.context.admit(kind, Access::Read).await?;
        Ok(Pinned {
            scope: self.scope().clone(),
            version: domain.map_or(0, |d| d.version),
            selection: serde_json::to_vec(&domain.map(|d| d.root))?,
            data,
        })
    }
    async fn status_raw(
        &self,
        kind: StoreKind,
        request_id: RequestId,
    ) -> c::Result<StoreOutcome<Vec<u8>>> {
        self.context.admit(kind, Access::Write).await?;
        let actor = self.actor(false)?;
        let current = self.coordinator.read().await?;
        let receipt = self
            .coordinator
            .receipt_at(&current, actor, request_id)
            .await?;
        match receipt {
            Some(receipt) => self.committed(kind, receipt).await,
            None => {
                self.context.admit(kind, Access::Write).await?;
                Ok(StoreOutcome::NotKnown { request_id })
            }
        }
    }
    async fn outcome(
        &self,
        kind: StoreKind,
        value: p::PublishOutcome,
    ) -> c::Result<StoreOutcome<Vec<u8>>> {
        match value {
            p::PublishOutcome::Committed(receipt) => self.committed(kind, receipt).await,
            p::PublishOutcome::OutcomeUnknown { request_id, .. } => {
                self.context.admit(kind, Access::Write).await?;
                Ok(StoreOutcome::Unknown { request_id })
            }
        }
    }
    async fn committed(
        &self,
        kind: StoreKind,
        receipt: p::Receipt,
    ) -> c::Result<StoreOutcome<Vec<u8>>> {
        let request_id = receipt.request_id;
        match self.committed_inner(kind, receipt).await {
            Err(c::Error::Unavailable(_) | c::Error::OutcomeUnknown) => {
                Ok(StoreOutcome::Unknown { request_id })
            }
            result => result,
        }
    }
    async fn committed_inner(
        &self,
        kind: StoreKind,
        receipt: p::Receipt,
    ) -> c::Result<StoreOutcome<Vec<u8>>> {
        let current = self.coordinator.read().await?;
        let selected = self
            .coordinator
            .receipt_domain_at(&current, &receipt, domain(kind))
            .await?;
        self.context.admit(kind, Access::Write).await?;
        if self.actor(false)? != receipt.principal {
            return Err(c::Error::Denied);
        }
        Ok(StoreOutcome::Committed(StoreReceipt {
            request_id: receipt.request_id,
            actor: receipt.principal,
            repository: self.coordinator.repository_id(),
            version: selected.version,
            selection: serde_json::to_vec(&selected.root)?,
            provider_receipt: serde_json::to_vec(&receipt)?,
            result: receipt.result,
        }))
    }
}
fn domain(kind: StoreKind) -> p::Domain {
    match kind {
        StoreKind::Agent => p::Domain::Agent,
        StoreKind::Work => p::Domain::Work,
        StoreKind::Review => p::Domain::Review,
        StoreKind::Thread => p::Domain::Thread,
        StoreKind::Reaction => p::Domain::Reaction,
    }
}
fn map_outcome<R>(
    value: StoreOutcome<Vec<u8>>,
    decode: impl FnOnce(&StoreReceipt<Vec<u8>>) -> c::Result<R>,
) -> c::Result<StoreOutcome<R>> {
    Ok(match value {
        StoreOutcome::Unknown { request_id } => StoreOutcome::Unknown { request_id },
        StoreOutcome::NotKnown { request_id } => StoreOutcome::NotKnown { request_id },
        StoreOutcome::Committed(r) => {
            let result = decode(&r)?;
            StoreOutcome::Committed(StoreReceipt {
                request_id: r.request_id,
                actor: r.actor,
                repository: r.repository,
                version: r.version,
                selection: r.selection,
                provider_receipt: r.provider_receipt,
                result,
            })
        }
    })
}
fn collab_error(error: p::CollabCommandError) -> c::Error {
    match error {
        p::CollabCommandError::Publication(e) => e.into(),
        p::CollabCommandError::Model(e) => match e {
            jj_collab::Error::Stale => c::Error::Stale,
            jj_collab::Error::Limit => c::Error::Limit("collaboration command"),
            jj_collab::Error::TargetUnavailable => c::Error::NotFound,
            jj_collab::Error::ReadUnavailable => {
                c::Error::Unavailable("collaboration source".into())
            }
            _ => c::Error::Invalid("collaboration command"),
        },
    }
}

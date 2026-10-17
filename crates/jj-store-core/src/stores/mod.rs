//! Typed host-owned store construction. Loading never initializes repository state.
mod commands;
mod factories;
mod traits;
use crate::{Error, Result};
pub use commands::*;
pub use factories::*;
use jj_store_model::{PrincipalId, RepositoryId, RequestId};
use std::{
    collections::{BTreeMap, BTreeSet},
    sync::{Arc, OnceLock},
};
pub use traits::*;
mod sources;
pub use sources::*;

#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum StoreKind {
    Agent,
    Work,
    Review,
    Thread,
    Reaction,
}
impl StoreKind {
    pub const ALL: [Self; 5] = [
        Self::Agent,
        Self::Work,
        Self::Review,
        Self::Thread,
        Self::Reaction,
    ];
}
/// A provider-owned publication boundary, never an authorization grant.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct PublicationScope {
    pub repository: Option<RepositoryId>,
    pub authority: String,
}
impl PublicationScope {
    pub fn require_same(&self, other: &Self) -> Result<()> {
        if self != other {
            return Err(Error::Unsupported("different publication authorities"));
        }
        Ok(())
    }
}
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Access {
    Read,
    Write,
}
/// Trusted embedding-host policy. Actor identity never comes from a command body.
#[async_trait::async_trait]
pub trait StoreHost: Send + Sync {
    async fn authorize(&self, store: StoreKind, access: Access) -> Result<()>;
    /// May create device provenance only when called for an explicit write.
    fn actor(&self, writing: bool) -> Result<Option<PrincipalId>>;
    fn now_millis(&self) -> Result<i64>;
}
#[derive(Clone)]
pub struct StoreContext {
    pub scope: PublicationScope,
    pub host: Arc<dyn StoreHost>,
    enabled: BTreeSet<StoreKind>,
    participants: Arc<OnceLock<BTreeMap<StoreKind, Option<Arc<PublicationGroup>>>>>,
}
impl StoreContext {
    pub fn new(scope: PublicationScope, host: Arc<dyn StoreHost>) -> Self {
        Self {
            scope,
            host,
            enabled: BTreeSet::new(),
            participants: Arc::new(OnceLock::new()),
        }
    }

    pub async fn admit(&self, kind: StoreKind, access: Access) -> Result<()> {
        if !self.enabled.contains(&kind) {
            return Err(Error::Unsupported("store disabled"));
        }
        self.host.authorize(kind, access).await
    }

    /// Admit only source implementations selected by this loader assembly.
    pub async fn admit_sources<I>(
        &self,
        group: &Arc<PublicationGroup>,
        sources: I,
        access: Access,
    ) -> Result<()>
    where
        I: IntoIterator<Item = StoreKind> + Send,
        I::IntoIter: Send,
    {
        let selected = self
            .participants
            .get()
            .ok_or(Error::Unsupported("unbound source participants"))?;
        for kind in sources {
            match selected.get(&kind).and_then(Option::as_ref) {
                Some(source) if Arc::ptr_eq(source, group) => {}
                _ => return Err(Error::Unsupported("incompatible or disabled source store")),
            }
            self.admit(kind, access).await?;
        }
        Ok(())
    }
}
#[derive(Clone, Debug)]
pub struct StoreRequest<C> {
    /// Preserve an existing native publication identity when adapting a durable request.
    pub publication_id: Option<jj_store_model::PublicationId>,
    pub request_id: RequestId,
    pub command: C,
}
#[derive(Clone, Debug)]
pub struct StoreReceipt<R> {
    pub request_id: RequestId,
    pub actor: PrincipalId,
    pub repository: RepositoryId,
    pub version: u64,
    /// Exact provider-owned selection; Git and native IDs are never reinterpreted.
    pub selection: Vec<u8>,
    /// Exact canonical provider receipt, retained for existing service and CLI contracts.
    pub provider_receipt: Vec<u8>,
    pub result: R,
}
#[derive(Clone, Debug)]
pub enum StoreOutcome<R> {
    Committed(StoreReceipt<R>),
    Unknown { request_id: RequestId },
    NotKnown { request_id: RequestId },
}
#[derive(Clone)]
pub struct Pinned<T> {
    pub scope: PublicationScope,
    pub version: u64,
    pub selection: Vec<u8>,
    pub data: T,
}

impl<C> StoreRequest<C> {
    pub fn new(request_id: RequestId, command: C) -> Self {
        Self {
            request_id,
            command,
            publication_id: None,
        }
    }
    pub fn with_publication_id(mut self, id: jj_store_model::PublicationId) -> Self {
        self.publication_id = Some(id);
        self
    }
}
/// Runtime identity for one provider's atomic publication participants.
#[derive(Debug, Default)]
pub struct PublicationGroup(());
impl PublicationGroup {
    pub fn new() -> Arc<Self> {
        Arc::new(Self(()))
    }
}

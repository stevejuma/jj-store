//! Typed host-owned store construction. Loading never initializes repository state.
mod commands;
mod factories;
mod traits;
use crate::{Error, Result};
pub use commands::*;
pub use factories::*;
use jj_store_model::{PrincipalId, RepositoryId, RequestId};
use std::{collections::BTreeSet, sync::Arc};
pub use traits::*;

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
    pub enabled: BTreeSet<StoreKind>,
}
impl StoreContext {
    pub async fn admit(&self, kind: StoreKind, access: Access) -> Result<()> {
        if !self.enabled.contains(&kind) {
            return Err(Error::Unsupported("store disabled"));
        }
        self.host.authorize(kind, access).await
    }
}
#[derive(Clone, Debug)]
pub struct StoreRequest<C> {
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

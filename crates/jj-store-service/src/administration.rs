//! Typed provisioning and policy mutations. Authentication is always current;
//! retry identity is reconstructed from retained manifest-selected settings.
use crate::{
    auth::{BearerToken, Clock, Permission, Scope, TokenDescription, TokenPolicy},
    native::{NativeSession, SessionLimits, create_repository},
    transport::NativeEndpoint,
};
use jj_store_jj::{objects::NativeObjects, policy::Protection};
use jj_store_model as m;
use jj_store_object::ObjectStore;
pub use jj_store_wal::publication::Snapshot as AdministrationSnapshot;
use jj_store_wal::{self as wal, publication::*};
use serde::{Deserialize, Serialize};
use sha2::{Digest as _, Sha256};
use std::sync::Arc;

/// Server configuration supplies the operator policy on each request. A file
/// provider can observe replacement/revocation without caching secret material.
#[async_trait::async_trait]
pub trait OperatorPolicies: Send + Sync {
    async fn current(&self) -> crate::auth::Result<TokenPolicy>;
}
#[derive(Clone)]
pub struct RepositoryHost {
    events: crate::events::Hints,
    store: Arc<dyn ObjectStore>,
    operators: Arc<dyn OperatorPolicies>,
    clock: Arc<dyn Clock>,
    protection: Protection,
    limits: SessionLimits,
    max_blob: u64,
}
impl RepositoryHost {
    pub fn new(
        store: Arc<dyn ObjectStore>,
        operators: Arc<dyn OperatorPolicies>,
        clock: Arc<dyn Clock>,
        protection: Protection,
        limits: SessionLimits,
        max_blob: u64,
    ) -> crate::transport::Result<Self> {
        // Validate the composition before accepting requests, without storage I/O.
        NativeEndpoint::new(
            m::RepositoryId::new([0; 16]),
            store.clone(),
            clock.clone(),
            protection.clone(),
            limits,
            max_blob,
        )?;
        Ok(Self {
            events: crate::events::Hints::default(),
            store,
            operators,
            clock,
            protection,
            limits,
            max_blob,
        })
    }
    /// Lazy binding needs no mutable repository directory or bucket listing.
    /// The selected manifest policy establishes existence and access together.
    pub fn endpoint(
        &self,
        repository: m::RepositoryId,
    ) -> crate::transport::Result<NativeEndpoint> {
        let mut endpoint = NativeEndpoint::new(
            repository,
            self.store.clone(),
            self.clock.clone(),
            self.protection.clone(),
            self.limits,
            self.max_blob,
        )?;
        endpoint.events = self.events.clone();
        Ok(endpoint)
    }
    pub async fn create(
        &self,
        operator: &BearerToken,
        repository: m::RepositoryId,
        epoch: m::EpochId,
        admin: &BearerToken,
    ) -> wal::Result<Snapshot> {
        tokio::time::timeout(self.limits.timeout, async {
            let operators = self
                .operators
                .current()
                .await
                .map_err(|_| wal::Error::Denied)?;
            if operators.scope() != Scope::Operator {
                return Err(wal::Error::Denied);
            }
            let principal = operators
                .authorize(
                    operator,
                    Permission::CreateRepository,
                    self.clock.unix_seconds().map_err(|_| wal::Error::Denied)?,
                )
                .map_err(|_| wal::Error::Denied)?;
            let mut policy = TokenPolicy::new(Scope::Repository(repository));
            policy
                .grant(admin, principal.id(), Permission::Admin, None)
                .map_err(|_| wal::Error::Denied)?;
            let objects = Arc::new(
                NativeObjects::new(self.store.clone(), repository, self.max_blob)
                    .map_err(|_| wal::Error::Limit("native blob limit"))?,
            );
            let created = create_repository(
                self.store.clone(),
                objects,
                &operators,
                operator,
                &policy,
                epoch,
                self.clock.as_ref(),
                self.limits.publication,
            )
            .await?;
            Ok(created.manifest().snapshot())
        })
        .await
        .map_err(|_| wal::Error::OutcomeUnknown)?
    }
    pub async fn create_local_trusted(
        &self,
        repository: m::RepositoryId,
        epoch: m::EpochId,
        permission: Permission,
    ) -> wal::Result<Snapshot> {
        tokio::time::timeout(self.limits.timeout, async {
            let objects = Arc::new(
                NativeObjects::new(self.store.clone(), repository, self.max_blob)
                    .map_err(|_| wal::Error::Limit("native blob limit"))?,
            );
            let created = crate::native::create_repository_local_trusted(
                self.store.clone(),
                objects,
                permission,
                epoch,
                self.limits.publication,
            )
            .await?;
            Ok(created.manifest().snapshot())
        })
        .await
        .map_err(|_| wal::Error::OutcomeUnknown)?
    }
}

#[derive(Clone, Debug)]
pub enum TokenChange {
    Grant {
        credential: BearerToken,
        principal: m::PrincipalId,
        permission: Permission,
        expires_at: Option<u64>,
    },
    Revoke {
        token: m::TokenId,
    },
}
#[derive(Clone, Debug)]
pub struct TokenRequest {
    pub request_id: m::RequestId,
    pub expected: Snapshot,
    pub change: TokenChange,
}
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(tag = "action", rename_all = "snake_case", deny_unknown_fields)]
pub enum TokenResult {
    Granted { token: TokenDescription },
    Revoked { token: m::TokenId },
}
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct TokenListing {
    pub snapshot: Snapshot,
    pub tokens: Vec<TokenDescription>,
}
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct TokenReceipt {
    pub request_id: m::RequestId,
    pub publication_id: m::PublicationId,
    pub accepted: AdministrationSnapshot,
    pub result: TokenResult,
}
impl TryFrom<Receipt> for TokenReceipt {
    type Error = crate::transport::Error;
    fn try_from(receipt: Receipt) -> Result<Self, Self::Error> {
        Ok(Self {
            request_id: receipt.request_id,
            publication_id: receipt.publication_id,
            accepted: receipt.accepted,
            result: serde_json::from_slice(&receipt.result)
                .map_err(|_| Self::Error::Unavailable)?,
        })
    }
}
impl NativeSession {
    pub async fn tokens(&self) -> wal::Result<TokenListing> {
        tokio::time::timeout(self.limits.timeout, async {
            self.principal(Permission::Admin).await?;
            let snapshot = self.coordinator.read().await?;
            let bytes = self.coordinator.settings(&snapshot).await?;
            let policy = TokenPolicy::decode(&bytes, Scope::Repository(self.repository))
                .map_err(|_| wal::Error::Corrupt("repository token policy"))?;
            self.principal(Permission::Admin).await?;
            Ok(TokenListing {
                snapshot: snapshot.manifest().snapshot(),
                tokens: policy.tokens(),
            })
        })
        .await
        .map_err(|_| wal::Error::Limit("token listing time"))?
    }
    async fn token_request(&self, request: &TokenRequest) -> wal::Result<Request> {
        let principal = self.principal(Permission::Admin).await?;
        let current = self.coordinator.read().await?;
        let bytes = self
            .coordinator
            .settings_at(&current, &request.expected)
            .await?;
        let mut policy = TokenPolicy::decode(&bytes, Scope::Repository(self.repository))
            .map_err(|_| wal::Error::Corrupt("historical token policy"))?;
        let result = match &request.change {
            TokenChange::Grant {
                credential,
                principal,
                permission,
                expires_at,
            } => {
                policy
                    .grant(credential, *principal, *permission, *expires_at)
                    .map_err(|_| wal::Error::IntentConflict)?;
                TokenResult::Granted {
                    token: TokenDescription {
                        id: credential.id(),
                        principal: *principal,
                        permission: *permission,
                        expires_at: *expires_at,
                    },
                }
            }
            TokenChange::Revoke { token } => {
                policy.revoke(*token);
                TokenResult::Revoked { token: *token }
            }
        };
        let mut hash = Sha256::new_with_prefix(b"jj-store/token-administration/v1");
        hash.update(self.repository.as_bytes());
        hash.update(principal.id().as_bytes());
        hash.update(request.request_id.as_bytes());
        let mut id = [0; 16];
        id.copy_from_slice(&hash.finalize()[..16]);
        Ok(Request {
            principal: principal.id(),
            request_id: request.request_id,
            publication_id: m::PublicationId::new(id),
            transition: Some(Transition::Strict {
                expected: request.expected.clone(),
                heads: request.expected.heads.clone(),
            }),
            settings: Some(
                policy
                    .encode()
                    .map_err(|_| wal::Error::Limit("token policy"))?,
            ),
            result: serde_json::to_vec(&result)?,
            kind: SemanticKind::SettingsChanged,
        })
    }
    pub async fn change_token(&self, request: &TokenRequest) -> wal::Result<PublishOutcome> {
        let outcome = tokio::time::timeout(self.limits.timeout, async {
            let prepared = self.token_request(request).await?;
            self.publish_request(&prepared).await
        })
        .await;
        // Never imply rejection if cancellation could have raced the manifest CAS.
        outcome.map_err(|_| wal::Error::OutcomeUnknown)?
    }
    /// None means no receipt observed; it is not proof the request was rejected.
    pub async fn token_status(&self, request: &TokenRequest) -> wal::Result<Option<Receipt>> {
        tokio::time::timeout(self.limits.timeout, async {
            let prepared = self.token_request(request).await?;
            self.coordinator.status(&prepared).await
        })
        .await
        .map_err(|_| wal::Error::Limit("token status time"))?
    }
}

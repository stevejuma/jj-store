//! Authenticated, data-only native Git acceptance. Transport remains CLI-owned.
use crate::{auth::Permission, native::NativeSession};
use jj_store_jj::{git_bridge, objects::NativeObjects};
use jj_store_model as m;
pub use jj_store_wal::publication::Receipt;
use jj_store_wal::{self as wal, publication::*};
use serde::{Deserialize, Serialize};
use sha2::{Digest as _, Sha256};
use std::{collections::BTreeSet, sync::Arc};

#[derive(Clone, Debug, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct NativeTransition {
    pub remove: BTreeSet<m::OperationId>,
    pub add: m::OperationId,
}
#[derive(Clone, Debug, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct Request {
    pub version: u32,
    pub request_id: m::RequestId,
    pub records: Vec<GitRecordV1>,
    pub transition: Option<NativeTransition>,
}
#[derive(Clone, Debug, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct State {
    pub version: u32,
    pub code: Snapshot,
    pub activation_available: bool,
    pub git: Option<GitStoreSnapshot>,
}
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(tag = "status", rename_all = "snake_case")]
pub enum Outcome {
    Committed { receipt: Box<Receipt> },
    OutcomeUnknown { request_id: m::RequestId },
}
fn publication_id(
    repository: m::RepositoryId,
    principal: m::PrincipalId,
    request: m::RequestId,
) -> m::PublicationId {
    let mut hash = Sha256::new_with_prefix(b"jj-store/git-publication/v1\0");
    hash.update(repository.as_bytes());
    hash.update(principal.as_bytes());
    hash.update(request.as_bytes());
    let mut id = [0; 16];
    id.copy_from_slice(&hash.finalize()[..16]);
    m::PublicationId::new(id)
}
pub async fn read(session: Arc<NativeSession>) -> wal::Result<State> {
    let before = session.snapshot().await?;
    let git = match before.manifest().git_root {
        Some(root) => Some(session.coordinator.git_store(root).await?),
        None => None,
    };
    session.principal(Permission::Read).await?;
    Ok(State {
        version: 1,
        code: before.manifest().snapshot(),
        activation_available: before.manifest().work_root.is_some()
            || before.manifest().replication_root.is_none(),
        git,
    })
}
pub async fn accept(session: Arc<NativeSession>, request: &Request) -> wal::Result<Outcome> {
    if request.version != 1
        || request.records.len() > 100_000
        || request
            .transition
            .as_ref()
            .is_some_and(|t| t.remove.is_empty() || t.remove.len() > 256)
    {
        return Err(wal::Error::Corrupt("Git acceptance request"));
    }
    let principal = session.principal(Permission::Write).await?.id();
    let before = session.snapshot().await?;
    let current = match before.manifest().git_root {
        Some(root) => session.coordinator.git_store(root).await?,
        None => session.coordinator.empty_git_store().await?,
    };
    let staged = session
        .coordinator
        .stage_git_records(&current, &request.records)
        .await?;
    let native = Arc::new(
        NativeObjects::new(
            session.object_store.clone(),
            session.repository,
            32 * 1024 * 1024,
        )
        .map_err(|_| wal::Error::Limit("Git native object configuration"))?,
    );
    tokio::time::timeout(
        session.limits.timeout,
        git_bridge::verify_delta(native, &staged, Some(&current)),
    )
    .await
    .map_err(|_| wal::Error::Limit("Git verification time"))?
    .map_err(|_| wal::Error::Corrupt("Git representation verification"))?;
    session.principal(Permission::Write).await?;
    let guarded = wal::publication::Request {
        principal,
        request_id: request.request_id,
        publication_id: publication_id(session.repository, principal, request.request_id),
        transition: request.transition.as_ref().map(|t| Transition::Native {
            remove: t.remove.clone(),
            add: t.add,
        }),
        settings: None,
        result: Vec::new(),
        kind: SemanticKind::GitStateChanged,
    };
    match session
        .coordinator
        .publish_git(&guarded, &request.records)
        .await?
    {
        PublishOutcome::Committed(receipt) => {
            session.events.published(session.repository);
            session.principal(Permission::Write).await?;
            Ok(Outcome::Committed {
                receipt: Box::new(receipt),
            })
        }
        PublishOutcome::OutcomeUnknown { .. } => Ok(Outcome::OutcomeUnknown {
            request_id: request.request_id,
        }),
    }
}
pub async fn status(session: Arc<NativeSession>, id: m::RequestId) -> wal::Result<Option<Receipt>> {
    let principal = session.principal(Permission::Write).await?.id();
    let snapshot = session.snapshot().await?;
    let receipt = session
        .coordinator
        .receipt_at(&snapshot, principal, id)
        .await?;
    session.principal(Permission::Write).await?;
    if receipt
        .as_ref()
        .is_some_and(|r| r.publication_id != publication_id(session.repository, principal, id))
    {
        return Err(wal::Error::IntentConflict);
    }
    Ok(receipt)
}

//! Internal server monitoring of committed metadata. Delivery requires current subscriber authorization.
use super::{Change, Domain};
use jj_store_model::RepositoryId;
use jj_store_object::ObjectStore;
use jj_store_wal::{self as wal, publication::ManifestSnapshot};
use std::sync::Arc;

pub(crate) async fn observe(
    store: Arc<dyn ObjectStore>,
    repository: RepositoryId,
) -> wal::Result<Vec<Change>> {
    let captured = ManifestSnapshot::load(store, repository).await?;
    let m = captured.manifest();
    let epoch = m.epoch.to_string();
    let values = [
        (Domain::Repository, epoch.clone()),
        (
            Domain::Code,
            serde_json::to_string(&(m.code_state_version, &m.heads))?,
        ),
        (
            Domain::Threads,
            serde_json::to_string(&(
                m.thread_root,
                m.reaction_root,
                m.thread_state_version,
                m.reaction_state_version,
            ))?,
        ),
        (
            Domain::Agents,
            serde_json::to_string(&(m.agent_root, m.agent_state_version))?,
        ),
        (
            Domain::Reviews,
            serde_json::to_string(&(m.review_root, m.review_state_version))?,
        ),
        (
            Domain::Work,
            serde_json::to_string(&(m.work_root, m.work_state_version))?,
        ),
        (
            Domain::Storage,
            serde_json::to_string(&(m.revision, m.published, &m.checkpoint))?,
        ),
        (Domain::Sync, serde_json::to_string(&m.replication_root)?),
    ];
    Ok(values
        .into_iter()
        .map(|(domain, value)| Change {
            repository: Some(repository),
            domain,
            witness: format!("{epoch}:{value}"),
            identity: None,
            audience: None,
        })
        .collect())
}

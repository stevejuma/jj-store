//! Independent conversion archives and typed reference translation. No publication authority.
use jj_store_model as m;
use jj_store_wal::publication::*;
use serde::{Deserialize, Serialize};
use std::collections::{BTreeMap, BTreeSet};
mod agent;
pub(crate) mod history;
pub use history::{CollaborationSelectionV1, RepositoryArchiveV1, WorkSnapshotArchiveV1};
mod collab;
pub use collab::{CollabGraphArchiveV1, transfer_collab};
mod review;
pub type Result<T> = std::result::Result<T, Box<dyn std::error::Error + Send + Sync>>;
#[derive(Clone, Debug, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct AgentGraphV1 {
    pub root: AgentRootV1,
    pub objects: Vec<(AgentObjectRef, AgentObjectV1)>,
}
#[derive(Clone, Debug, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct ReviewGraphV1 {
    pub root: ReviewRootV1,
    pub objects: Vec<(ReviewObjectRef, ReviewObjectV1)>,
}
fn mapped<K: Ord + Copy, V: Copy>(values: &BTreeMap<K, V>, key: K) -> Result<V> {
    values
        .get(&key)
        .copied()
        .ok_or_else(|| "incomplete conversion reference mapping".into())
}
pub async fn archive_agent(source: &dyn AgentSnapshotRead) -> Result<AgentGraphV1> {
    let mut pending = source.root().dependencies()?;
    let mut objects = BTreeMap::new();
    while let Some(reference) = pending.pop() {
        if objects.contains_key(&reference) {
            continue;
        }
        if objects.len() >= 4096 {
            return Err("agent conversion object limit".into());
        }
        let value = source.read_object(reference).await?;
        agent::admit(&value)?;
        pending.extend(value.dependencies()?);
        objects.insert(reference, value);
    }
    Ok(AgentGraphV1 {
        root: source.root().clone(),
        objects: objects.into_iter().collect(),
    })
}
pub async fn archive_review(source: &dyn ReviewSnapshotRead) -> Result<ReviewGraphV1> {
    let mut pending = source.root().dependencies()?;
    let mut objects = BTreeMap::new();
    while let Some(reference) = pending.pop() {
        if objects.contains_key(&reference) {
            continue;
        }
        if objects.len() >= 100_000 {
            return Err("review conversion object limit".into());
        }
        let value = source.read_object(reference).await?;
        review::admit(&value)?;
        pending.extend(value.review_dependencies()?);
        objects.insert(reference, value);
    }
    Ok(ReviewGraphV1 {
        root: source.root().clone(),
        objects: objects.into_iter().collect(),
    })
}
pub use agent::transfer_agent;
pub use review::transfer_review;

use super::*;
use crate::{auth::Permission, native::NativeSession};
use jj_store_model as m;
use serde::{Deserialize, Serialize};
use std::sync::Arc;

// Bound encoded pages as well as row counts, so artifacts remain inspectable
// through the shared eight-MiB RPC envelope.
fn bounded<T: Serialize>(
    items: impl IntoIterator<Item = T>,
    limit: usize,
    id: fn(&T) -> String,
) -> Result<(Vec<T>, Option<String>)> {
    let mut page = Vec::new();
    let mut bytes = 0;
    for item in items {
        let size = serde_json::to_vec(&item).map_err(|_| Error::Corrupt)?.len();
        if page.len() == limit || bytes + size > 3 * 1024 * 1024 {
            let cursor = id(page.last().ok_or(Error::Limit)?);
            return Ok((page, Some(cursor)));
        }
        bytes += size;
        page.push(item);
    }
    Ok((page, None))
}

#[derive(Clone, Debug, Default, Serialize, Deserialize, schemars::JsonSchema)]
#[serde(deny_unknown_fields)]
#[schemars(rename = "WorkflowQuery")]
pub struct Query {
    pub workflow: Option<WorkflowId>,
    pub run: Option<RunId>,
    pub at: Option<m::WalPosition>,
    pub after: Option<String>,
    pub limit: Option<usize>,
    pub evidence_after: Option<u64>,
    pub attempt: Option<ExecutionAttemptId>,
    pub pending_only: Option<bool>,
}
#[derive(Clone, Debug, Serialize, Deserialize, schemars::JsonSchema)]
#[schemars(rename = "WorkflowPage")]
pub struct Page {
    pub version: u32,
    pub repository: m::RepositoryId,
    pub epoch: m::EpochId,
    pub position: m::WalPosition,
    pub root: Option<m::RecordDigest>,
    pub workflows: Vec<DefinitionVersion>,
    pub runs: Vec<Run>,
    pub evidence: Vec<Evidence>,
    pub next: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub event: Option<SourceEvent>,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub step_reports: Vec<Evidence>,
}
impl WorkflowService {
    pub async fn page(&self, session: Arc<NativeSession>, query: Query) -> Result<Page> {
        tokio::time::timeout(session.limits.timeout, self.page_inner(session, query))
            .await
            .map_err(|_| Error::Limit)?
    }
    async fn page_inner(&self, session: Arc<NativeSession>, query: Query) -> Result<Page> {
        let limit = query.limit.unwrap_or(50);
        if !(1..=256).contains(&limit) {
            return Err(Error::Limit);
        }
        if query
            .after
            .as_ref()
            .is_some_and(|after| after.parse::<WorkflowId>().is_err())
        {
            return Err(Error::Invalid);
        }
        session.principal(Permission::Read).await?;
        let snapshot = session.snapshot().await?;
        let position = query.at.unwrap_or(snapshot.manifest().published);
        if position > snapshot.manifest().published {
            return Err(Error::Invalid);
        }
        let root = if position == snapshot.manifest().published {
            snapshot.manifest().workflow_root
        } else if position.0 == 0 {
            None
        } else {
            let records = session
                .coordinator
                .committed(&snapshot, m::WalPosition(position.0 - 1), 1)
                .await?;
            records
                .first()
                .ok_or(Error::NotFound)?
                .after_domains
                .and_then(|d| d.workflow)
                .map(|d| d.root)
        };
        let data = match root {
            Some(root) => {
                let value = session
                    .read_indexes
                    .cache
                    .get(&session, snapshot.manifest().epoch, root)
                    .await
                    .map_err(|e| match e {
                        crate::collab_mutations::Error::Denied => Error::Denied,
                        crate::collab_mutations::Error::Corrupt => Error::Corrupt,
                        crate::collab_mutations::Error::Limit => Error::Limit,
                        _ => Error::Unavailable,
                    })?;
                let crate::collab_reads::materialization::Data::Workflow(data) = value.as_ref()
                else {
                    return Err(Error::Corrupt);
                };
                Some(data.clone())
            }
            None => None,
        };
        let empty = State::default();
        let state = data.as_ref().map_or(&empty, |d| &d.state);
        let mut page = Page {
            version: 1,
            repository: session.repository,
            epoch: snapshot.manifest().epoch,
            position,
            root: root.map(|r| r.digest),
            workflows: Vec::new(),
            runs: Vec::new(),
            evidence: Vec::new(),
            next: None,
            event: None,
            step_reports: Vec::new(),
        };
        if let Some(id) = query.run {
            if let Some(run) = state.runs.get(&id) {
                page.event = run
                    .event
                    .and_then(|id| state.evaluations.get(&id))
                    .map(|e| e.event.clone());
                let selected_attempt = query.attempt.unwrap_or(run.current().id);
                let projection = data.as_ref().ok_or(Error::Corrupt)?;
                let step_ids = projection.evidence_ids(id, Some(selected_attempt), true);
                page.step_reports = step_ids
                    .iter()
                    .map(|id| {
                        state
                            .evidence
                            .get(&id.parse().map_err(|_| Error::Corrupt)?)
                            .cloned()
                            .ok_or(Error::Corrupt)
                    })
                    .collect::<Result<_>>()?;
                if serde_json::to_vec(&page.step_reports)
                    .map_err(|_| Error::Corrupt)?
                    .len()
                    > 3 * 1024 * 1024
                {
                    return Err(Error::Limit);
                }
                page.runs.push(run.clone());
                page.workflows.push(
                    state
                        .definitions
                        .get(&run.version)
                        .ok_or(Error::Corrupt)?
                        .clone(),
                );
                let ids = projection.evidence_ids(id, query.attempt, false);
                let selected = ids
                    .iter()
                    .map(|id| {
                        state
                            .evidence
                            .get(&id.parse().map_err(|_| Error::Corrupt)?)
                            .ok_or(Error::Corrupt)
                    })
                    .collect::<Result<Vec<_>>>()?;
                let selected: Vec<_> = selected
                    .into_iter()
                    .filter(|e| query.evidence_after.is_none_or(|after| e.sequence >= after))
                    .collect();
                let offset = match query.after.as_deref() {
                    Some(after) => {
                        selected
                            .iter()
                            .position(|e| e.id.to_string() == after)
                            .ok_or(Error::Invalid)?
                            + 1
                    }
                    None => 0,
                };
                (page.evidence, page.next) =
                    bounded(selected.into_iter().skip(offset).cloned(), limit, |e| {
                        e.id.to_string()
                    })?;
            }
        } else if let Some(id) = query.workflow {
            if let Some(version) = state.workflows.get(&id) {
                page.workflows.push(
                    state
                        .definitions
                        .get(version)
                        .ok_or(Error::Corrupt)?
                        .clone(),
                );
            }
            if data.is_none() && query.after.is_some() {
                return Err(Error::Invalid);
            }
            if let Some(projection) = &data {
                let ids = projection.run_ids(id);
                let offset = match query.after.as_deref() {
                    Some(after) => {
                        ids.iter()
                            .position(|id| id == after)
                            .ok_or(Error::Invalid)?
                            + 1
                    }
                    None => 0,
                };
                let runs = ids.iter().skip(offset).map(|id| {
                    state
                        .runs
                        .get(&id.parse().expect("validated run ID"))
                        .expect("indexed source run")
                        .clone()
                });
                (page.runs, page.next) = bounded(runs, limit, |r| r.id.to_string())?;
            }
        } else if query.pending_only == Some(true) {
            if let Some(projection) = &data {
                let ids = projection.pending_ids(true);
                let runs = ids
                    .iter()
                    .filter(|id| query.after.as_ref().is_none_or(|after| *id > after))
                    .map(|id| {
                        state
                            .runs
                            .get(&id.parse().expect("validated run ID"))
                            .expect("indexed source run")
                            .clone()
                    });
                (page.runs, page.next) = bounded(runs, limit, |r| r.id.to_string())?;
            }
        } else {
            let values = state
                .workflows
                .iter()
                .filter(|(id, _)| {
                    query
                        .after
                        .as_ref()
                        .is_none_or(|after| id.to_string() > *after)
                })
                .map(|(_, version)| {
                    state
                        .definitions
                        .get(version)
                        .expect("validated definition selection")
                        .clone()
                });
            (page.workflows, page.next) = bounded(values, limit, |v| v.id.to_string())?;
            // Pending execution discovery is bounded; historical details are separate.
            if let Some(projection) = &data {
                let ids = projection.pending_ids(false);
                let runs = ids.iter().map(|id| {
                    state
                        .runs
                        .get(&id.parse().expect("validated run ID"))
                        .expect("indexed source run")
                        .clone()
                });
                page.runs = bounded(runs, limit, |r| r.id.to_string())?.0;
            }
        }
        session.principal(Permission::Read).await?;
        if session.snapshot().await?.manifest().epoch != page.epoch {
            return Err(Error::Stale);
        }
        if serde_json::to_vec(&page).map_err(|_| Error::Corrupt)?.len() > 8 * 1024 * 1024 {
            return Err(Error::Limit);
        }
        Ok(page)
    }
}

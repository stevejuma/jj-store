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
        let state = match root {
            Some(root) => session
                .coordinator
                .workflow_store(root)
                .await?
                .state()
                .clone(),
            None => jj_workflow::State::default(),
        };
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
                page.step_reports = state
                    .evidence
                    .values()
                    .filter(|e| {
                        e.run == id
                            && e.attempt == selected_attempt
                            && matches!(e.data, EvidenceData::Step { .. })
                    })
                    .cloned()
                    .collect();
                page.step_reports.sort_by_key(|e| e.sequence);
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
                page.evidence = state
                    .evidence
                    .values()
                    .filter(|e| {
                        e.run == id
                            && query.attempt.is_none_or(|attempt| e.attempt == attempt)
                            && query.evidence_after.is_none_or(|after| e.sequence >= after)
                    })
                    .cloned()
                    .collect();
                page.evidence.sort_by_key(|e| {
                    (
                        run.attempts.iter().position(|a| a.id == e.attempt),
                        e.sequence,
                    )
                });
                if let Some(after) = query.after.as_deref() {
                    let index = page
                        .evidence
                        .iter()
                        .position(|e| e.id.to_string() == after)
                        .ok_or(Error::Invalid)?;
                    page.evidence.drain(..=index);
                }
                (page.evidence, page.next) = bounded(page.evidence, limit, |e| e.id.to_string())?;
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
            let mut runs: Vec<_> = state
                .runs
                .values()
                .filter(|r| r.workflow == id)
                .cloned()
                .collect();
            runs.sort_by_key(|r| std::cmp::Reverse((r.created_at, r.id)));
            if let Some(after) = query.after.as_deref() {
                let index = runs
                    .iter()
                    .position(|r| r.id.to_string() == after)
                    .ok_or(Error::Invalid)?;
                runs.drain(..=index);
            }
            (page.runs, page.next) = bounded(runs, limit, |r| r.id.to_string())?;
        } else if query.pending_only == Some(true) {
            let runs = state
                .runs
                .values()
                .filter(|r| {
                    r.current().status == RunStatus::Pending
                        && query
                            .after
                            .as_ref()
                            .is_none_or(|after| r.id.to_string() > *after)
                })
                .cloned();
            (page.runs, page.next) = bounded(runs, limit, |r| r.id.to_string())?;
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
                        .cloned()
                        .ok_or(Error::Corrupt)
                })
                .collect::<Result<Vec<_>>>()?;
            (page.workflows, page.next) = bounded(values, limit, |v| v.id.to_string())?;
            // Pending execution discovery is bounded; historical details are separate.
            let runs = state
                .runs
                .values()
                .filter(|r| !r.current().status.terminal())
                .cloned();
            page.runs = bounded(runs, limit, |r| r.id.to_string())?.0;
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

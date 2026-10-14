//! Audited source state shared by all pages; secondary indexes never select a root.
use super::*;
use crate::native::NativeSession;
use jj_store_read_index::{Index, Row};
use jj_store_wal::records::RecordRef;

pub(crate) struct Projection {
    pub state: State,
    pub runs: Option<Index>,
    pub pending: Option<Index>,
    pub evidence: Option<Index>,
    pub bytes: usize,
    pub objects: usize,
}
pub(crate) fn source<T: serde::Serialize>(value: &T) -> Result<String> {
    use sha2::{Digest, Sha256};
    Ok(hex::encode(Sha256::digest(
        serde_json::to_vec(value).map_err(|_| Error::Corrupt)?,
    )))
}
pub(crate) fn newest(time: i64) -> Vec<u8> {
    (!(time as u64 ^ (1 << 63))).to_be_bytes().to_vec()
}
impl Projection {
    pub async fn load(session: &NativeSession, root: RecordRef) -> Result<Self> {
        let store = session
            .coordinator
            .workflow_store_bounded(root, session.limits.inventory_nodes, 64 * 1024 * 1024)
            .await?;
        let state = store.state();
        let runs = state
            .runs
            .values()
            .map(|r| {
                let mut order = newest(r.created_at);
                order.extend(r.id.as_bytes().iter().map(|byte| !byte));
                Ok(Row::new(r.id.to_string(), source(r)?, order).facet("workflow", r.workflow))
            })
            .collect::<Result<Vec<_>>>()?;
        let pending = state
            .runs
            .values()
            .map(|r| {
                let mut row = Row::new(r.id.to_string(), source(r)?, r.id.as_bytes().to_vec());
                if r.current().status == RunStatus::Pending {
                    row = row.facet("pending", "true");
                }
                if !r.current().status.terminal() {
                    row = row.facet("active", "true");
                }
                Ok(row)
            })
            .collect::<Result<Vec<_>>>()?;
        let evidence = state
            .evidence
            .values()
            .map(|e| {
                let run = state.runs.get(&e.run).ok_or(Error::Corrupt)?;
                let attempt = run
                    .attempts
                    .iter()
                    .position(|a| a.id == e.attempt)
                    .ok_or(Error::Corrupt)?;
                let mut order = (attempt as u64).to_be_bytes().to_vec();
                order.extend(e.sequence.to_be_bytes());
                let mut row = Row::new(e.id.to_string(), source(e)?, order)
                    .facet("run", e.run)
                    .facet("attempt", e.attempt);
                if matches!(e.data, EvidenceData::Step { .. }) {
                    row = row.facet("step", "true");
                }
                Ok(row)
            })
            .collect::<Result<Vec<_>>>()?;
        let epoch = session.snapshot().await?.manifest().epoch.to_string();
        let prepare = |name, rows| {
            session.read_indexes.prepare(
                session.repository.to_string(),
                epoch.clone(),
                root.digest.to_string(),
                name,
                rows,
            )
        };
        let runs = prepare("workflow-runs-v1", runs).await.ok();
        let pending = prepare("workflow-pending-v1", pending).await.ok();
        let evidence = prepare("workflow-evidence-v1", evidence).await.ok();
        let bytes = serde_json::to_vec(state).map_err(|_| Error::Corrupt)?.len()
            + runs.as_ref().map_or(0, Index::bytes)
            + pending.as_ref().map_or(0, Index::bytes)
            + evidence.as_ref().map_or(0, Index::bytes);
        Ok(Self {
            state: state.clone(),
            runs,
            pending,
            evidence,
            bytes,
            objects: store.object_count(),
        })
    }
    pub fn run_ids(&self, workflow: WorkflowId) -> Vec<String> {
        if let Some(index) = &self.runs {
            return index.select(&[("workflow", workflow.to_string())], "").ids;
        }
        let mut runs: Vec<_> = self
            .state
            .runs
            .values()
            .filter(|r| r.workflow == workflow)
            .collect();
        runs.sort_by_key(|r| std::cmp::Reverse((r.created_at, r.id)));
        runs.into_iter().map(|r| r.id.to_string()).collect()
    }
    pub fn pending_ids(&self, only_pending: bool) -> Vec<String> {
        if let Some(index) = &self.pending {
            return index
                .select(
                    &[(
                        if only_pending { "pending" } else { "active" },
                        "true".into(),
                    )],
                    "",
                )
                .ids;
        }
        self.state
            .runs
            .values()
            .filter(|r| {
                if only_pending {
                    r.current().status == RunStatus::Pending
                } else {
                    !r.current().status.terminal()
                }
            })
            .map(|r| r.id.to_string())
            .collect()
    }
    pub fn evidence_ids(
        &self,
        run: RunId,
        attempt: Option<ExecutionAttemptId>,
        steps: bool,
    ) -> Vec<String> {
        if let Some(index) = &self.evidence {
            let mut filters = vec![("run", run.to_string())];
            if let Some(attempt) = attempt {
                filters.push(("attempt", attempt.to_string()));
            }
            if steps {
                filters.push(("step", "true".into()));
            }
            return index.select(&filters, "").ids;
        }
        let Some(run) = self.state.runs.get(&run) else {
            return vec![];
        };
        let mut records: Vec<_> = self
            .state
            .evidence
            .values()
            .filter(|e| {
                e.run == run.id
                    && attempt.is_none_or(|a| e.attempt == a)
                    && (!steps || matches!(e.data, EvidenceData::Step { .. }))
            })
            .collect();
        records.sort_by_key(|e| {
            (
                run.attempts.iter().position(|a| a.id == e.attempt),
                e.sequence,
            )
        });
        records.into_iter().map(|e| e.id.to_string()).collect()
    }
}

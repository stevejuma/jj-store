//! Device-private intent journal. Only the coordinator can select checkpoints.
mod journal;
use jj_store_model::{RepositoryId, RequestId};
use jj_store_service::storage_management::*;
use std::{collections::BTreeSet, path::PathBuf, sync::Arc};
use tokio::sync::Mutex;
struct Inner {
    root: PathBuf,
    backend: Arc<dyn StorageBackend>,
    active: Mutex<BTreeSet<(RepositoryId, RequestId)>>,
}
#[derive(Clone)]
pub struct Manager(Arc<Inner>);
impl Manager {
    pub fn new(root: PathBuf, backend: Arc<dyn StorageBackend>) -> crate::Result<Self> {
        let root = crate::policy::outside_checkout(&root)?;
        if !root.is_dir() {
            return Err(crate::Error::Config("storage journal directory"));
        }
        Ok(Self(Arc::new(Inner {
            root,
            backend,
            active: Mutex::new(BTreeSet::new()),
        })))
    }
    async fn report(
        &self,
        repo: RepositoryId,
        entry: &journal::Entry,
        active: bool,
    ) -> Result<StorageOperation, Error> {
        let mut report = entry.report.clone();
        if report.phase == StoragePhase::Running && !active {
            report.phase = if report.kind == StorageKind::Check {
                StoragePhase::Interrupted
            } else {
                StoragePhase::Unknown
            };
            report.message = Some("Service stopped before the operation report was saved.".into());
        }
        if report.kind == StorageKind::Checkpoint
            && matches!(report.phase, StoragePhase::Unknown | StoragePhase::Complete)
        {
            let selection = self.0.backend.selection(repo, &entry.capture).await?;
            report.selection = Some(selection);
            if selection != CheckpointSelection::NotCovered {
                report.phase = StoragePhase::Complete;
                report.message = None;
            }
        }
        Ok(report)
    }
    async fn execute(
        &self,
        repo: RepositoryId,
        id: RequestId,
        capture: StorageCapture,
        kind: StorageKind,
    ) {
        let result = if kind == StorageKind::Check {
            self.0.backend.check(repo, &capture).await
        } else {
            self.0.backend.publish(repo, &capture).await
        };
        let mut active = self.0.active.lock().await;
        // A lost report is recovered as interrupted/unknown from the durable running intent.
        if let Ok(mut journal) = journal::read(&self.0.root, repo)
            && let Some(entry) = journal
                .entries
                .iter_mut()
                .find(|e| e.report.request_id == id)
        {
            entry.report.phase = match &result {
                Ok(()) => StoragePhase::Complete,
                Err(_) if kind == StorageKind::Checkpoint => StoragePhase::Unknown,
                Err(Error::Incomplete) => StoragePhase::Incomplete,
                Err(_) => StoragePhase::Failed,
            };
            entry.report.message = result.err().map(|e| e.to_string());
            if kind == StorageKind::Checkpoint
                && let Ok(selection) = self.0.backend.selection(repo, &capture).await
            {
                entry.report.selection = Some(selection);
                if selection != CheckpointSelection::NotCovered {
                    entry.report.phase = StoragePhase::Complete;
                    entry.report.message = None;
                }
            }
            let _ = journal::save(&self.0.root, repo, &journal);
        }
        active.remove(&(repo, id));
    }
}
#[async_trait::async_trait]
impl StorageManager for Manager {
    async fn overview(&self, repo: RepositoryId) -> Result<StorageOverview, Error> {
        let mut overview = self.0.backend.overview(repo).await?;
        let active = self.0.active.lock().await;
        let journal = journal::read(&self.0.root, repo)?;
        if let Some(entry) = journal.entries.last() {
            overview.latest = Some(
                self.report(
                    repo,
                    entry,
                    active.contains(&(repo, entry.report.request_id)),
                )
                .await?,
            );
        }
        Ok(overview)
    }
    async fn operation(
        &self,
        repo: RepositoryId,
        id: RequestId,
    ) -> Result<StorageOperation, Error> {
        let active = self.0.active.lock().await;
        let journal = journal::read(&self.0.root, repo)?;
        let entry = journal
            .entries
            .iter()
            .find(|e| e.report.request_id == id)
            .ok_or(Error::NotFound)?;
        self.report(repo, entry, active.contains(&(repo, id))).await
    }
    async fn command(
        &self,
        repo: RepositoryId,
        command: StorageCommand,
    ) -> Result<StorageOperation, Error> {
        if command.version != 1 {
            return Err(Error::Invalid);
        }
        let mut active = self.0.active.lock().await;
        let mut journal = journal::read(&self.0.root, repo)?;
        let id = command.request_id;
        let index = if let Some(index) = journal
            .entries
            .iter()
            .position(|e| e.report.request_id == id)
        {
            let entry = &journal.entries[index];
            let expected = if command.action == StorageAction::Check {
                StorageKind::Check
            } else {
                StorageKind::Checkpoint
            };
            if entry.report.kind != expected {
                return Err(Error::Conflict);
            }
            let report = self
                .report(repo, entry, active.contains(&(repo, id)))
                .await?;
            if command.action == StorageAction::CancelCheckpoint {
                if report.phase == StoragePhase::Canceled {
                    return Ok(report);
                }
                if report.phase != StoragePhase::Prepared {
                    return Err(Error::Conflict);
                }
                journal.entries[index].report.phase = StoragePhase::Canceled;
                let report = journal.entries[index].report.clone();
                journal::save(&self.0.root, repo, &journal)?;
                return Ok(report);
            }
            if command.action != StorageAction::PublishCheckpoint
                || !matches!(report.phase, StoragePhase::Prepared | StoragePhase::Unknown)
            {
                return Ok(report);
            }
            index
        } else {
            if matches!(
                command.action,
                StorageAction::PublishCheckpoint | StorageAction::CancelCheckpoint
            ) {
                return Err(Error::NotFound);
            }
            if journal.entries.len() >= 128 {
                return Err(Error::Limit);
            }
            if let Some(last) = journal.entries.last() {
                let report = self
                    .report(repo, last, active.contains(&(repo, last.report.request_id)))
                    .await?;
                if matches!(
                    report.phase,
                    StoragePhase::Prepared | StoragePhase::Running | StoragePhase::Unknown
                ) {
                    return Err(Error::Busy);
                }
            }
            let capture = self.0.backend.capture(repo).await?;
            let kind = if command.action == StorageAction::Check {
                StorageKind::Check
            } else {
                StorageKind::Checkpoint
            };
            journal.entries.push(journal::Entry {
                report: StorageOperation {
                    version: 1,
                    request_id: id,
                    kind,
                    phase: StoragePhase::Prepared,
                    target: capture.position.clone(),
                    limits: capture.limits.clone(),
                    selection: None,
                    message: None,
                },
                capture,
            });
            journal.entries.len() - 1
        };
        // Bound service-owned work even when the operator switches repositories.
        if matches!(
            command.action,
            StorageAction::Check | StorageAction::PublishCheckpoint
        ) && !active.is_empty()
        {
            return Err(Error::Busy);
        }
        let entry = &mut journal.entries[index];
        if command.action != StorageAction::PrepareCheckpoint {
            entry.report.phase = StoragePhase::Running;
            entry.report.message = None;
        }
        let entry = entry.clone();
        journal::save(&self.0.root, repo, &journal)?;
        if entry.report.phase == StoragePhase::Running {
            active.insert((repo, id));
            let this = self.clone();
            tokio::spawn(async move {
                this.execute(repo, id, entry.capture, entry.report.kind)
                    .await;
            });
        }
        Ok(entry.report)
    }
}

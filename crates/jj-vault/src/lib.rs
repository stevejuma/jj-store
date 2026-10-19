//! Automatic checkpoint ownership, independent of any desktop, watcher or UI.
#![forbid(unsafe_code)]
mod adoption;
mod native;
mod remote;
mod restore;
mod schedule;
pub mod search;
mod sync;
pub use jj_store_jj::vault_workspace::{
    AdoptionRequest, Binding, Capture, Checkpoint, ExistingHistory, FetchedHead, GitCredentials,
    PendingPublication, PendingStorage, PushIntent, PushOutcome, RemoteTarget, RemoteTransfer,
    RestorePreview, RestoreRequest, Storage, StorageLocation, StorageProgress, StorageStatus,
    SyncConflict, SyncProposal, SyncResolution, SyncTerm, VaultRestorePreview, VaultRestoreRequest,
};
pub use native::NativeRepository;
pub use schedule::Timing;
use serde::{Deserialize, Serialize};
use std::{
    sync::{
        Arc,
        atomic::{AtomicBool, Ordering},
        mpsc,
    },
    thread,
    time::{Duration, Instant},
};
use tokio::sync::{oneshot, watch};

/// Runs only on the owned backup worker. No caller UI thread performs native IO.
pub trait Repository: Send + 'static {
    fn adopt_history(&mut self, _request: &AdoptionRequest) -> Result<Checkpoint, String> {
        Err("History adoption is unavailable".into())
    }
    fn stop_adoption(&mut self, _request: &str) -> Result<(), String> {
        Err("History adoption is unavailable".into())
    }
    fn capture(&mut self) -> Result<Capture, String>;
    fn prepare_sync(&mut self, _incoming: &str) -> Result<SyncProposal, String> {
        Err("Synchronization is unavailable".into())
    }
    fn preview_sync(&mut self, _candidate: &str) -> Result<VaultRestorePreview, String> {
        Err("Synchronization is unavailable".into())
    }
    fn apply_sync(&mut self, _request: &VaultRestoreRequest) -> Result<Capture, String> {
        Err("Synchronization is unavailable".into())
    }
    fn sync_conflict(&mut self, _candidate: &str, _path: &str) -> Result<SyncConflict, String> {
        Err("Synchronization is unavailable".into())
    }
    fn resolve_sync(
        &mut self,
        _candidate: &str,
        _path: &str,
        _resolution: SyncResolution,
    ) -> Result<Checkpoint, String> {
        Err("Synchronization is unavailable".into())
    }
    fn preview_restore(
        &mut self,
        _checkpoint: &str,
        _path: &str,
    ) -> Result<RestorePreview, String> {
        Err("Restore is unavailable for this provider".into())
    }
    fn restore(&mut self, _request: &RestoreRequest) -> Result<Capture, String> {
        Err("Restore is unavailable for this provider".into())
    }
    fn preview_vault_restore(&mut self, _checkpoint: &str) -> Result<VaultRestorePreview, String> {
        Err("Vault restore is unavailable for this provider".into())
    }
    fn restore_vault(&mut self, _request: &VaultRestoreRequest) -> Result<Capture, String> {
        Err("Vault restore is unavailable for this provider".into())
    }
    fn import_remote(&mut self, _head: &FetchedHead) -> Result<Checkpoint, String> {
        Err("Remote import is unavailable for this provider".into())
    }
    fn restore_remote(&mut self, _request: &VaultRestoreRequest) -> Result<Capture, String> {
        Err("Remote restore is unavailable for this provider".into())
    }
}
#[derive(Clone, Debug, Serialize, Deserialize, PartialEq, Eq)]
#[serde(rename_all = "snake_case")]
pub enum Phase {
    Saving,
    Ready,
    Failed,
    Closed,
}
#[derive(Clone, Debug, Serialize)]
pub struct Status {
    pub phase: Phase,
    pub checkpoint: Option<Checkpoint>,
    pub error: Option<String>,
}
enum Command {
    Wake,
    Capture(oneshot::Sender<Result<Capture, String>>),
    Preview(
        String,
        String,
        oneshot::Sender<Result<RestorePreview, String>>,
    ),
    Restore(RestoreRequest, oneshot::Sender<Result<Capture, String>>),
    PreviewVault(String, oneshot::Sender<Result<VaultRestorePreview, String>>),
    RestoreVault(
        VaultRestoreRequest,
        oneshot::Sender<Result<Capture, String>>,
    ),
    Remote(remote::Request),
    Sync(sync::Request),
    Adoption(adoption::Request),
    Close(oneshot::Sender<Result<(), String>>),
}
#[derive(Clone)]
pub struct Changes {
    commands: mpsc::SyncSender<Command>,
    dirty: Arc<AtomicBool>,
}
impl Changes {
    pub fn changed(&self) {
        self.dirty.store(true, Ordering::Release);
        let _ = self.commands.try_send(Command::Wake);
    }
}
pub struct Backup {
    commands: mpsc::SyncSender<Command>,
    changed: Arc<AtomicBool>,
    state: watch::Receiver<Status>,
    worker: Option<thread::JoinHandle<()>>,
}
impl Backup {
    pub fn start(repository: impl Repository, timing: Timing) -> Result<Self, String> {
        Self::launch(repository, timing, true)
    }
    /// Hosts attach watching before explicitly requesting the initial capture.
    pub fn start_idle(repository: impl Repository, timing: Timing) -> Result<Self, String> {
        Self::launch(repository, timing, false)
    }
    fn launch(
        repository: impl Repository,
        timing: Timing,
        immediate: bool,
    ) -> Result<Self, String> {
        let now = Instant::now();
        let mut schedule = schedule::Schedule::new(now, timing)?;
        if !immediate {
            schedule.completed(now);
        }
        let (sender, receiver) = mpsc::sync_channel(32);
        let (state_sender, state) = watch::channel(Status {
            phase: Phase::Saving,
            checkpoint: None,
            error: None,
        });
        let changed = Arc::new(AtomicBool::new(false));
        let dirty = changed.clone();
        let worker = thread::Builder::new()
            .name("vault-backup".into())
            .spawn(move || {
                run(
                    Box::new(repository),
                    receiver,
                    dirty,
                    state_sender,
                    schedule,
                );
            })
            .map_err(|e| e.to_string())?;
        Ok(Self {
            commands: sender,
            changed,
            state,
            worker: Some(worker),
        })
    }
    /// Overflow only coalesces wakeups; the dirty flag cannot be dropped.
    pub fn changed(&self) {
        self.changes().changed();
    }
    pub fn changes(&self) -> Changes {
        Changes {
            commands: self.commands.clone(),
            dirty: self.changed.clone(),
        }
    }
    pub fn subscribe(&self) -> watch::Receiver<Status> {
        self.state.clone()
    }
    pub async fn capture(&self) -> Result<Capture, String> {
        let (sender, receiver) = oneshot::channel();
        self.commands
            .try_send(Command::Capture(sender))
            .map_err(|_| "Vault backup is busy or closed".to_string())?;
        receiver
            .await
            .map_err(|_| "Vault backup stopped".to_string())?
    }
    pub async fn close(mut self) -> Result<(), String> {
        let (sender, receiver) = oneshot::channel();
        // A close barrier must not be dropped if the bounded command queue is full.
        let commands = self.commands.clone();
        tokio::task::spawn_blocking(move || {
            commands
                .send(Command::Close(sender))
                .map_err(|_| "Vault backup worker is closed".to_owned())
        })
        .await
        .map_err(|e| e.to_string())?
        .map_err(|e| e.to_string())?;
        let result = receiver
            .await
            .map_err(|_| "Vault backup stopped".to_string())?;
        if let Some(worker) = self.worker.take() {
            tokio::task::spawn_blocking(move || worker.join())
                .await
                .map_err(|e| e.to_string())?
                .map_err(|_| "Vault backup worker failed".to_string())?;
        }
        result
    }
}
fn capture(
    repository: &mut dyn Repository,
    state: &watch::Sender<Status>,
) -> Result<Capture, String> {
    mutation(state, || repository.capture())
}
fn mutation(
    state: &watch::Sender<Status>,
    action: impl FnOnce() -> Result<Capture, String>,
) -> Result<Capture, String> {
    state.send_modify(|s| {
        s.phase = Phase::Saving;
        s.error = None;
    });
    let result = action();
    state.send_modify(|s| match &result {
        Ok(value) => {
            s.phase = Phase::Ready;
            if value.checkpoint.is_some() {
                s.checkpoint = value.checkpoint.clone();
            }
            s.error = None;
        }
        Err(error) => {
            s.phase = Phase::Failed;
            s.error = Some(error.clone());
        }
    });
    result
}
fn run(
    mut repository: Box<dyn Repository>,
    receiver: mpsc::Receiver<Command>,
    dirty: Arc<AtomicBool>,
    state: watch::Sender<Status>,
    mut schedule: schedule::Schedule,
) {
    loop {
        if dirty.swap(false, Ordering::AcqRel) {
            schedule.changed(Instant::now());
        }
        let timeout = schedule
            .due()
            .saturating_duration_since(Instant::now())
            .min(Duration::from_secs(60));
        match receiver.recv_timeout(timeout) {
            Ok(Command::Wake) => {}
            Ok(Command::Adoption(request)) => {
                adoption::run(request, repository.as_mut(), &state, &mut schedule)
            }
            Ok(Command::Sync(request)) => {
                sync::run(request, repository.as_mut(), &state, &mut schedule)
            }
            Ok(Command::Remote(request)) => {
                remote::run(request, repository.as_mut(), &state, &mut schedule)
            }
            Ok(Command::Preview(checkpoint, path, reply)) => {
                let _ = reply.send(repository.preview_restore(&checkpoint, &path));
            }
            Ok(Command::Restore(request, reply)) => {
                let result = mutation(&state, || repository.restore(&request));
                if result.is_ok() {
                    schedule.completed(Instant::now());
                } else {
                    schedule.failed(Instant::now());
                }
                let _ = reply.send(result);
            }
            Ok(Command::PreviewVault(checkpoint, reply)) => {
                let _ = reply.send(repository.preview_vault_restore(&checkpoint));
            }
            Ok(Command::RestoreVault(request, reply)) => {
                let result = mutation(&state, || repository.restore_vault(&request));
                if result.is_ok() {
                    schedule.completed(Instant::now());
                } else {
                    schedule.failed(Instant::now());
                }
                let _ = reply.send(result);
            }
            Ok(Command::Capture(reply)) => {
                let result = capture(repository.as_mut(), &state);
                if result.is_ok() {
                    schedule.completed(Instant::now());
                } else {
                    schedule.failed(Instant::now());
                }
                let _ = reply.send(result);
            }
            Ok(Command::Close(reply)) => {
                let result = capture(repository.as_mut(), &state).map(|_| ());
                state.send_modify(|s| s.phase = Phase::Closed);
                let _ = reply.send(result);
                return;
            }
            Err(mpsc::RecvTimeoutError::Timeout) => {
                if capture(repository.as_mut(), &state).is_ok() {
                    schedule.completed(Instant::now());
                } else {
                    schedule.failed(Instant::now());
                }
            }
            Err(mpsc::RecvTimeoutError::Disconnected) => return,
        }
    }
}

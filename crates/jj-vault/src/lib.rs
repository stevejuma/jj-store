//! Automatic checkpoint ownership, independent of any desktop, watcher or UI.
#![forbid(unsafe_code)]
mod native;
mod schedule;
pub use jj_store_jj::vault_workspace::{Binding, Capture, Checkpoint};
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
    fn capture(&mut self) -> Result<Capture, String>;
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
        tokio::task::spawn_blocking(move || commands.send(Command::Close(sender)))
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
    state.send_modify(|s| {
        s.phase = Phase::Saving;
        s.error = None;
    });
    let result = repository.capture();
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

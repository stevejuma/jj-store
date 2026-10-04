//! Explicit owned lifecycle; configured process startup remains separately gated.
use crate::{
    Error, Result,
    capture_supervisor::{CodexCaptureSupervisor, CodexSupervisorPoll, CodexSupervisorStatus},
    capture_worker::CaptureWorkerPhase,
};
use std::time::Duration;
use tokio::{sync::watch, task::JoinHandle};
mod control;
mod imports;
mod rescan;
mod sources;
mod targets;
pub use control::CaptureRuntimeControl;

#[derive(Clone, Copy, PartialEq, Eq)]
enum Command {
    Running,
    Paused,
    Stopped,
}

/// Cached device-memory observation; no path, body, credential or ingress proof.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct CaptureRuntimeSnapshot {
    pub phase: CaptureWorkerPhase,
    pub status: CodexSupervisorStatus,
}
impl CaptureRuntimeSnapshot {
    fn read(supervisor: &CodexCaptureSupervisor) -> Self {
        Self {
            phase: supervisor.phase(),
            status: supervisor.status(),
        }
    }
}
pub struct RunningCodexCapture {
    control: CaptureRuntimeControl,
    task: Option<JoinHandle<Result<()>>>,
    completed: Option<bool>,
}
impl RunningCodexCapture {
    /// Bind storage ownership to every lane before dispatch; admission stays separate.
    pub fn start_with_storage(
        mut supervisor: CodexCaptureSupervisor,
        lifetime: crate::storage::CaptureStorageLifetime,
    ) -> Result<Self> {
        supervisor.retain_storage(lifetime)?;
        Self::start(supervisor)
    }
    /// Resources are already explicitly prepared; preserve a settled supervisor pause.
    pub fn start(supervisor: CodexCaptureSupervisor) -> Result<Self> {
        let runtime = tokio::runtime::Handle::try_current()
            .map_err(|_| Error::Service("capture runtime unavailable"))?;
        let initial = if supervisor.phase() == CaptureWorkerPhase::Paused {
            Command::Paused
        } else {
            Command::Running
        };
        let (command, receiver) = watch::channel(initial);
        let (updates, snapshot) = watch::channel(CaptureRuntimeSnapshot::read(&supervisor));
        let (source_updates, source_snapshot) =
            watch::channel(std::sync::Arc::new(supervisor.source_catalog()));
        let (target_updates, target_snapshot) =
            watch::channel(std::sync::Arc::new(supervisor.import_targets()));
        let rescan = std::sync::Arc::new(rescan::RescanSignal::default());
        let (import_commands, imports) = tokio::sync::mpsc::channel(1);
        let configured_roots = supervisor.configured_roots();
        let task = runtime.spawn(drive(
            supervisor,
            receiver,
            command.clone(),
            RuntimeUpdates {
                updates,
                sources: source_updates,
                targets: target_updates,
            },
            initial,
            rescan.clone(),
            imports,
        ));
        Ok(Self {
            control: CaptureRuntimeControl {
                configured_roots,
                command,
                snapshot,
                rescan,
                sources: source_snapshot,
                targets: target_snapshot,
                imports: import_commands,
            },
            task: Some(task),
            completed: None,
        })
    }
    pub fn snapshot(&self) -> CaptureRuntimeSnapshot {
        *self.control.snapshot.borrow()
    }
    /// Shares controls and bounded private target metadata; this owner alone awaits drain.
    pub fn controls(&self) -> CaptureRuntimeControl {
        self.control.clone()
    }
    pub fn pause(&self) -> Result<()> {
        self.control.pause()
    }
    pub fn resume(&self) -> Result<()> {
        self.control.resume()
    }
    fn stop(&self) {
        self.control.stop();
    }
    /// Timeout retains the exact draining owner. Call again before storage teardown.
    pub async fn shutdown(&mut self) -> Result<()> {
        self.stop();
        let Some(task) = self.task.as_mut() else {
            return if self.completed == Some(true) {
                Ok(())
            } else {
                Err(Error::Service("capture runtime shutdown failed"))
            };
        };
        let joined = tokio::time::timeout(Duration::from_secs(5), task)
            .await
            .map_err(|_| Error::Service("capture runtime is still draining"))?;
        let result = joined
            .map_err(|_| Error::Service("capture runtime owner lost"))
            .and_then(|result| result);
        self.completed = Some(result.is_ok());
        self.task = None;
        result
    }
}
impl Drop for RunningCodexCapture {
    fn drop(&mut self) {
        // Dropping JoinHandle detaches this owner; it must finish drain itself.
        self.stop();
    }
}
struct RuntimeUpdates {
    updates: watch::Sender<CaptureRuntimeSnapshot>,
    sources: watch::Sender<std::sync::Arc<crate::capture_supervisor::CodexSourceCatalog>>,
    targets: watch::Sender<std::sync::Arc<targets::ImportTargets>>,
}
async fn drive(
    mut supervisor: CodexCaptureSupervisor,
    mut commands: watch::Receiver<Command>,
    terminal: watch::Sender<Command>,
    updates: RuntimeUpdates,
    initial: Command,
    rescan: std::sync::Arc<rescan::RescanSignal>,
    mut imports: tokio::sync::mpsc::Receiver<imports::ImportCommand>,
) -> Result<()> {
    let mut applied = initial;
    let mut interval = tokio::time::interval(Duration::from_millis(10));
    interval.set_missed_tick_behavior(tokio::time::MissedTickBehavior::Skip);
    let fault = loop {
        tokio::select! {
            biased;
            changed = commands.changed() => {
                let requested = if changed.is_err() { Command::Stopped } else { *commands.borrow_and_update() };
                if requested == Command::Stopped { break Ok(()); }
                match requested {
                    Command::Paused => supervisor.pause(),
                    Command::Running => {
                        if let Err(error) = supervisor.resume() { break Err(error); }
                    }
                    Command::Stopped => unreachable!(),
                }
                applied = requested;
            }
            _ = interval.tick() => {
                let settled = matches!(supervisor.poll().await, CodexSupervisorPoll::Completed(_));
                // Keep stop publication and synchronous dispatch in one order.
                let requested = commands.borrow();
                if *requested == Command::Stopped { break Ok(()); }
                // Publish settlement for one quantum before starting another turn.
                if applied == Command::Running && *requested == Command::Running && !settled
                    && let Err(error) = supervisor.start_step() { break Err(error); }
            }
            _ = rescan::requested(&rescan) => {
                if rescan::take(&rescan) && applied == Command::Running
                    && *commands.borrow() == Command::Running {
                    // A lost lane stays closed; a hint cannot replace it.
                    let _ = supervisor.rescan();
                }
            }
            Some(import) = imports.recv() => {
                let result = if applied == Command::Running && *commands.borrow() == Command::Running {
                    supervisor.schedule_import(import.dispatch)
                } else { Err(Error::Service("capture import runtime unavailable")) };
                // Caller loss cannot cancel the command now retained by the source slot.
                let _ = import.settled.send(result);
            }
        }
        publish(&supervisor, &updates);
    };
    rescan.clear();
    terminal.send_if_modified(|state| {
        let changed = *state != Command::Stopped;
        *state = Command::Stopped;
        changed
    });
    supervisor.stop();
    publish(&supervisor, &updates);
    let result = supervisor.shutdown().await;
    // A bounded caller may time out. This task still owns every blocked lease,
    // buffer and publication await until the supervisor proves all lanes stopped.
    while supervisor.phase() != CaptureWorkerPhase::Stopped {
        supervisor.poll().await;
        publish(&supervisor, &updates);
        interval.tick().await;
    }
    publish(&supervisor, &updates);
    fault.and(result)
}
fn publish(supervisor: &CodexCaptureSupervisor, updates: &RuntimeUpdates) {
    let observed = supervisor.source_catalog();
    updates.sources.send_if_modified(|current| {
        if **current == observed {
            return false;
        }
        *current = std::sync::Arc::new(observed);
        true
    });
    let targets = supervisor.import_targets();
    updates.targets.send_if_modified(|current| {
        if current.generation == targets.generation
            && current.slots.len() == targets.slots.len()
            && current
                .slots
                .iter()
                .zip(&targets.slots)
                .all(|((a, q), (b, r))| a == b && q.fingerprint() == r.fingerprint())
        {
            return false;
        }
        *current = std::sync::Arc::new(targets);
        true
    });
    updates
        .updates
        .send_replace(CaptureRuntimeSnapshot::read(supervisor));
}

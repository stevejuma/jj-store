use super::*;
use std::time::Instant;
use tokio::{sync::watch, task::JoinHandle};

#[async_trait::async_trait]
pub(crate) trait OwnedWork: Send + 'static {
    type Command: Send + 'static;
    type Output: Send + 'static;
    async fn step(&mut self, command: Self::Command) -> Self::Output;
    /// The step future has ended; retain state while releasing only step-local gates.
    fn settled(&mut self) {}
}
pub(crate) enum Completion<T> {
    Idle,
    Running,
    Completed(T),
    Cancelled,
    Deadline,
    Lost,
}
// Field order also retains storage while a lost task drops its source state.
struct RetainedWork<T: OwnedWork> {
    state: T,
    _storage: Option<crate::storage::CaptureStorageLifetime>,
}
struct Flight<T: OwnedWork> {
    task: JoinHandle<(RetainedWork<T>, Completion<T::Output>)>,
    cancel: watch::Sender<bool>,
    expires: Instant,
}
pub(crate) struct OwnedWorker<T: OwnedWork> {
    idle: Option<T>,
    flight: Option<Flight<T>>,
    deadline: Duration,
    paused: bool,
    closed: bool,
    storage: Option<crate::storage::CaptureStorageLifetime>,
}
impl<T: OwnedWork> OwnedWorker<T> {
    pub(crate) fn new(state: T, deadline: Duration) -> Result<Self> {
        if deadline.is_zero() || deadline > Duration::from_secs(300) {
            return Err(Error::Config(
                "capture worker deadline must be positive and at most 300 seconds",
            ));
        }
        Ok(Self {
            idle: Some(state),
            flight: None,
            deadline,
            paused: false,
            closed: false,
            storage: None,
        })
    }
    pub(crate) fn retain_storage(&mut self, lifetime: crate::storage::CaptureStorageLifetime) {
        self.storage = Some(lifetime);
    }
    pub(crate) fn phase(&self) -> CaptureWorkerPhase {
        if let Some(flight) = &self.flight {
            if self.paused || self.closed {
                CaptureWorkerPhase::Draining
            } else if Instant::now() >= flight.expires {
                CaptureWorkerPhase::Overdue
            } else {
                CaptureWorkerPhase::Running
            }
        } else if self.closed {
            CaptureWorkerPhase::Stopped
        } else if self.paused {
            CaptureWorkerPhase::Paused
        } else {
            CaptureWorkerPhase::Ready
        }
    }
    pub(crate) fn idle_mut(&mut self) -> Option<&mut T> {
        self.idle.as_mut()
    }
    #[cfg(test)]
    pub(crate) fn abort_queued_task_for_test(&self) {
        self.flight.as_ref().expect("queued task").task.abort();
    }
    pub(crate) fn start(&mut self, command: T::Command) -> Result<()> {
        if self.closed || self.paused || self.flight.is_some() || self.idle.is_none() {
            return Err(Error::Service(
                "capture worker cannot dispatch while paused, stopped or busy",
            ));
        }
        let runtime = tokio::runtime::Handle::try_current()
            .map_err(|_| Error::Service("capture worker runtime unavailable"))?;
        let mut retained = RetainedWork {
            state: self.idle.take().expect("idle owner"),
            _storage: self.storage.as_ref().map(|s| s.fork()),
        };
        let expires = Instant::now() + self.deadline;
        let timer = tokio::time::Instant::from_std(expires);
        let (cancel, mut stopped) = watch::channel(false);
        let task = tokio::task::spawn_blocking(move || {
            let result = runtime.block_on(async {
                tokio::select! {
                    biased;
                    _ = stopped.changed() => Completion::Cancelled,
                    _ = tokio::time::sleep_until(timer) => Completion::Deadline,
                    result = retained.state.step(command) => Completion::Completed(result),
                }
            });
            retained.state.settled();
            (retained, result)
        });
        self.flight = Some(Flight {
            task,
            cancel,
            expires,
        });
        Ok(())
    }
    pub(crate) fn pause(&mut self) {
        self.paused = true;
        self.cancel();
    }
    pub(crate) fn resume(&mut self) -> Result<()> {
        if self.closed {
            return Err(Error::Service("capture worker stopped"));
        }
        self.paused = false;
        Ok(())
    }
    fn cancel(&self) {
        if let Some(flight) = &self.flight {
            flight.cancel.send_replace(true);
        }
    }
    pub(crate) fn stop(&mut self) {
        self.closed = true;
        self.cancel();
        self.idle = None;
        self.storage = None;
    }
    pub(crate) async fn shutdown(&mut self) -> Result<()> {
        self.stop();
        tokio::time::timeout(Duration::from_secs(5), async {
            while matches!(self.poll().await, Completion::Running) {
                tokio::time::sleep(Duration::from_millis(5)).await;
            }
        })
        .await
        .map_err(|_| Error::Service("capture worker is still draining"))
    }
    pub(crate) async fn poll(&mut self) -> Completion<T::Output> {
        let Some(flight) = &self.flight else {
            return Completion::Idle;
        };
        if !flight.task.is_finished() {
            return Completion::Running;
        }
        let flight = self.flight.take().expect("finished flight");
        match flight.task.await {
            Ok((retained, completion)) => {
                if !self.closed {
                    self.idle = Some(retained.state);
                }
                completion
            }
            Err(_) => {
                self.closed = true;
                self.storage = None;
                Completion::Lost
            }
        }
    }
}
impl<T: OwnedWork> Drop for OwnedWorker<T> {
    fn drop(&mut self) {
        self.cancel();
        if let Some(flight) = &self.flight {
            flight.task.abort();
        }
    }
}
#[cfg(test)]
pub(crate) mod faults;
#[cfg(test)]
mod tests;

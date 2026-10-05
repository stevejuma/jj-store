use super::*;
use std::sync::{
    Arc,
    atomic::{AtomicBool, Ordering},
};
#[derive(Default)]
pub(super) struct RescanSignal {
    pending: AtomicBool,
    wake: tokio::sync::Notify,
}
impl RescanSignal {
    pub fn clear(&self) {
        self.pending.store(false, Ordering::SeqCst);
    }
    pub fn pending(&self) -> bool {
        self.pending.load(Ordering::SeqCst)
    }
}
impl CaptureRuntimeControl {
    /// A coalesced discovery hint; no policy, task or source eligibility is created.
    pub fn rescan(&self) -> Result<()> {
        let observed = self.snapshot()?;
        if observed.status.discovery.phase == CaptureWorkerPhase::Stopped
            || self.command.is_closed()
            || self.snapshot.has_changed().is_err()
        {
            return Err(Error::Service("capture discovery unavailable"));
        }
        let mut accepted = false;
        self.command.send_if_modified(|state| {
            if *state == Command::Running {
                self.rescan.pending.store(true, Ordering::SeqCst);
                accepted = true;
            }
            false
        });
        if !accepted || self.command.is_closed() || self.snapshot.has_changed().is_err() {
            return Err(Error::Service("capture discovery paused or unavailable"));
        }
        self.rescan.wake.notify_one();
        Ok(())
    }
}
pub(super) async fn requested(signal: &Arc<RescanSignal>) {
    signal.wake.notified().await;
}
pub(super) fn take(signal: &Arc<RescanSignal>) -> bool {
    signal.pending.swap(false, Ordering::SeqCst)
}

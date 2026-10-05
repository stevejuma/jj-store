//! One optional body-free lane, governed by the supervisor's normal owner lifetime.
use super::*;
use crate::capture_management::CaptureImportRecovery;
use std::time::Instant;
pub(super) struct RecoveryLane {
    pub owner: OwnedWorker<CaptureImportRecovery>,
    next: Instant,
}
impl CodexCaptureSupervisor {
    /// Attach once before dispatch, including an initially paused retained supervisor.
    pub fn retain_import_recovery(&mut self, recovery: CaptureImportRecovery) -> Result<()> {
        if self.dispatched || self.closed || self.recovery.is_some() {
            return Err(Error::Service(
                "capture import recovery must be attached before dispatch",
            ));
        }
        let mut owner = OwnedWorker::new(recovery, self.deadline)?;
        if let Some(storage) = &self.storage {
            owner.retain_storage(storage.fork());
        }
        if self.paused {
            owner.pause();
        }
        self.recovery = Some(RecoveryLane {
            owner,
            next: Instant::now(),
        });
        Ok(())
    }
    pub(super) fn start_recovery(&mut self) -> Result<()> {
        if self.slots.is_empty() {
            return Ok(());
        }
        if let Some(lane) = &mut self.recovery
            && lane.owner.phase() == CaptureWorkerPhase::Ready
            && Instant::now() >= lane.next
        {
            lane.next = Instant::now() + Duration::from_secs(1);
            lane.owner.start(())?;
        }
        Ok(())
    }
    pub(super) async fn poll_recovery(&mut self) -> bool {
        let Some(lane) = &mut self.recovery else {
            return false;
        };
        match lane.owner.poll().await {
            Completion::Completed(Ok(Some(dispatch))) => {
                // A changed/paused/retired owner withholds this offer. Never replace it.
                let _ = self.schedule_import(dispatch);
                true
            }
            Completion::Completed(
                Ok(None) | Err(jj_store_service::capture_management::Error::Busy),
            ) => true,
            Completion::Completed(Err(_)) => {
                self.unavailable += 1;
                true
            }
            Completion::Lost => {
                self.unavailable += 1;
                true
            }
            Completion::Cancelled | Completion::Deadline => true,
            _ => false,
        }
    }
}

use super::*;
impl CodexCaptureSupervisor {
    pub(crate) fn retain_storage(
        &mut self,
        lifetime: crate::storage::CaptureStorageLifetime,
    ) -> Result<()> {
        if self.dispatched || self.closed || self.paused || self.storage.is_some() {
            return Err(Error::Service(
                "capture storage lifetime must be attached once before dispatch",
            ));
        }
        self.discovery.retain_storage(lifetime.fork());
        self.delivery.retain_storage(lifetime.fork());
        if let Some(api) = &mut self.api {
            api.retain_storage(&lifetime);
        }
        if let Some(lane) = &mut self.recovery {
            lane.owner.retain_storage(lifetime.fork());
        }
        self.storage = Some(lifetime);
        Ok(())
    }
    pub fn phase(&self) -> CaptureWorkerPhase {
        let phases = std::iter::once(self.discovery.phase())
            .chain(std::iter::once(self.delivery.phase()))
            .chain(self.recovery.iter().map(|lane| lane.owner.phase()))
            .chain(self.slots.iter().map(|s| s.owner.phase()));
        let phases = phases.chain(self.api.iter().flat_map(|api| api.phases()));
        let mut running = false;
        for phase in phases {
            if phase == CaptureWorkerPhase::Draining {
                return phase;
            }
            if phase == CaptureWorkerPhase::Overdue {
                return phase;
            }
            running |= phase == CaptureWorkerPhase::Running;
        }
        if running {
            CaptureWorkerPhase::Running
        } else if self.closed {
            CaptureWorkerPhase::Stopped
        } else if self.paused {
            CaptureWorkerPhase::Paused
        } else {
            CaptureWorkerPhase::Ready
        }
    }
}

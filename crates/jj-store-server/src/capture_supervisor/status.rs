use super::*;
mod harness;
pub use harness::CodexHarnessStatus;
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct CodexDiscoveryStatus {
    pub phase: CaptureWorkerPhase,
    pub roots: usize,
    pub unchecked_roots: usize,
    pub available_roots: usize,
    pub unavailable_roots: usize,
    pub rescan_pending: bool,
}
impl Default for CodexDiscoveryStatus {
    fn default() -> Self {
        Self {
            phase: CaptureWorkerPhase::Stopped,
            roots: 0,
            unchecked_roots: 0,
            available_roots: 0,
            unavailable_roots: 0,
            rescan_pending: false,
        }
    }
}
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub struct CodexSupervisorStatus {
    pub harnesses: [Option<CodexHarnessStatus>; 5],
    pub discovery: CodexDiscoveryStatus,
    pub degraded_sources: usize,
    pub pending_capacity_bytes: u64,
    pub ingress_capacity_refusals: usize,
    pub ingress_required_capacity_bytes: Option<u64>,
    pub delivery_required_capacity_bytes: Option<u64>,
    pub tracked: usize,
    pub observed_sources: usize,
    pub awaiting_activity: usize,
    pub caught_up: usize,
    pub running_sources: usize,
    pub resident_source_frames: usize,
    pub resident_delivery_frames: usize,
    pub pending_batches: usize,
    pub unavailable: usize,
    pub source_limit_reached: bool,
}

impl CodexCaptureSupervisor {
    pub fn status(&self) -> CodexSupervisorStatus {
        let stable_queue = self.stable_queue();
        let mut status = CodexSupervisorStatus {
            harnesses: self.harnesses(stable_queue),
            discovery: CodexDiscoveryStatus {
                phase: self.discovery.phase(),
                roots: self.discovery_coverage.roots,
                unchecked_roots: self.discovery_coverage.unchecked,
                available_roots: self.discovery_coverage.available,
                unavailable_roots: self.discovery_coverage.unavailable,
                rescan_pending: self.rescan_requested
                    || (!self.discard_rescan && self.discovery_coverage.rescan_pending),
            },
            degraded_sources: self
                .slots
                .iter()
                .filter(|s| {
                    s.snapshot.unavailable
                        || (!self.closed && s.owner.phase() == CaptureWorkerPhase::Stopped)
                })
                .count(),
            pending_capacity_bytes: self.config.max_pending_bytes,
            ingress_capacity_refusals: self
                .slots
                .iter()
                .filter(|s| {
                    s.owner.phase() != CaptureWorkerPhase::Stopped
                        && s.snapshot.capacity_refusal.is_some()
                })
                .count(),
            ingress_required_capacity_bytes: self
                .slots
                .iter()
                .filter(|s| s.owner.phase() != CaptureWorkerPhase::Stopped)
                .filter_map(|s| s.snapshot.capacity_refusal.map(|r| r.required_bytes))
                .max(),
            delivery_required_capacity_bytes: self
                .delivery_capacity_refusal
                .filter(|_| self.delivery.phase() != CaptureWorkerPhase::Stopped)
                .map(|r| r.required_bytes),
            tracked: self.slots.len(),
            observed_sources: self.slots.iter().filter(|s| s.snapshot.observed).count(),
            awaiting_activity: self
                .slots
                .iter()
                .filter(|s| s.snapshot.awaiting_activity)
                .count(),
            caught_up: self
                .slots
                .iter()
                .filter(|s| stable_queue && s.snapshot.caught_up && !s.occupied)
                .count(),
            running_sources: self
                .slots
                .iter()
                .filter(|s| s.occupied && s.owner.phase() != CaptureWorkerPhase::Stopped)
                .count(),
            resident_source_frames: self
                .slots
                .iter()
                .filter(|s| s.raw && s.owner.phase() != CaptureWorkerPhase::Stopped)
                .count(),
            resident_delivery_frames: match self.delivery.phase() {
                CaptureWorkerPhase::Running
                | CaptureWorkerPhase::Overdue
                | CaptureWorkerPhase::Draining => self.config.max_readers * 2,
                CaptureWorkerPhase::Stopped => 0,
                _ => self.delivery_frames,
            },
            pending_batches: self.pending,
            unavailable: self.unavailable,
            source_limit_reached: self.limit_reached,
        };
        if let Some(api) = &self.api {
            let harness = api.harness(stable_queue, self.closed);
            status.degraded_sources += harness.degraded_sources;
            status.tracked += harness.tracked;
            status.observed_sources += harness.observed_sources;
            status.awaiting_activity += harness.awaiting_activity;
            status.caught_up += harness.caught_up;
            status.running_sources += harness.running_sources;
            status.resident_source_frames +=
                if self.closed && self.phase() == CaptureWorkerPhase::Stopped {
                    0
                } else {
                    api.raw()
                };
            status.ingress_capacity_refusals += api.capacity_refusals();
            status.delivery_required_capacity_bytes = status
                .delivery_required_capacity_bytes
                .into_iter()
                .chain(api.required_delivery_capacity())
                .max();
            status.ingress_required_capacity_bytes = status
                .ingress_required_capacity_bytes
                .into_iter()
                .chain(api.required_capacity())
                .max();
        }
        status
    }
    pub(super) fn stable_queue(&self) -> bool {
        self.queue_epoch == Some(self.epoch)
            && self.pending == 0
            && self.delivery.phase() == CaptureWorkerPhase::Ready
            && !self.slots.iter().any(|s| s.occupied)
            && self.api.as_ref().is_none_or(|api| api.stable())
    }
}

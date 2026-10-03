use super::*;
use jj_store_capture::profile::CaptureProfile;

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct CodexHarnessStatus {
    pub profile: CaptureProfile,
    pub roots: usize,
    pub unchecked_roots: usize,
    pub available_roots: usize,
    pub unavailable_roots: usize,
    pub tracked: usize,
    pub observed_sources: usize,
    pub awaiting_activity: usize,
    pub caught_up: usize,
    pub running_sources: usize,
    pub degraded_sources: usize,
}
impl CodexCaptureSupervisor {
    pub(super) fn harnesses(&self, stable_queue: bool) -> [Option<CodexHarnessStatus>; 3] {
        let files = self.discovery_coverage.harnesses.map(|group| {
            group.map(|group| {
                let slots = || {
                    self.slots
                        .iter()
                        .filter(|s| s.qualified.profile() == group.profile)
                };
                CodexHarnessStatus {
                    profile: group.profile,
                    roots: group.roots,
                    unchecked_roots: group.unchecked,
                    available_roots: group.available,
                    unavailable_roots: group.unavailable,
                    tracked: slots().count(),
                    observed_sources: slots().filter(|s| s.snapshot.observed).count(),
                    awaiting_activity: slots().filter(|s| s.snapshot.awaiting_activity).count(),
                    caught_up: slots()
                        .filter(|s| stable_queue && s.snapshot.caught_up && !s.occupied)
                        .count(),
                    running_sources: slots()
                        .filter(|s| s.occupied && s.owner.phase() != CaptureWorkerPhase::Stopped)
                        .count(),
                    degraded_sources: slots()
                        .filter(|s| {
                            s.snapshot.unavailable
                                || (!self.closed && s.owner.phase() == CaptureWorkerPhase::Stopped)
                        })
                        .count(),
                }
            })
        });
        [
            files[0],
            files[1],
            self.api
                .as_ref()
                .map(|api| api.harness(stable_queue, self.closed)),
        ]
    }
}

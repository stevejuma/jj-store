use super::*;
use crate::{opencode_read_api::OpenCodeReadApi, opencode_source::OpenCodePublicationProof};
mod delivery;
mod discovery;
mod scheduling;
mod source;
#[cfg(all(test, unix))]
mod tests;
use delivery::ApiDeliveryWork;
use discovery::ApiDiscoveryWork;
use source::ApiSourceWork;
pub(super) struct ApiSlot {
    selector: u64,
    native_session: String,
    owner: OwnedWorker<ApiSourceWork>,
    snapshot: SourceSnapshot,
    proofs: Vec<OpenCodePublicationProof>,
    publication_query: Option<jj_store_capture::reader_publication::SnapshotPublicationQuery>,
    raw: bool,
    occupied: bool,
    rescan: bool,
}
pub(super) struct ApiLane {
    api: OpenCodeReadApi,
    discovery: OwnedWorker<ApiDiscoveryWork>,
    delivery: OwnedWorker<ApiDeliveryWork>,
    slots: Vec<ApiSlot>,
    after: usize,
    rescan: bool,
    delivery_capacity_refusal: Option<jj_store_capture::agent_capture::CaptureCapacityRefusal>,
}
impl ApiLane {
    pub fn new(
        api: OpenCodeReadApi,
        resources: &CodexSupervisorResources,
        interval: Duration,
        deadline: Duration,
    ) -> Result<Self> {
        Ok(Self {
            discovery: OwnedWorker::new(
                ApiDiscoveryWork::new(api.clone(), resources.fork(), interval),
                deadline,
            )?,
            delivery: OwnedWorker::new(ApiDeliveryWork::new(resources.fork()), deadline)?,
            api,
            slots: Vec::new(),
            after: 0,
            rescan: false,
            delivery_capacity_refusal: None,
        })
    }
    pub fn pause(&mut self) {
        self.rescan = false;
        self.discovery.pause();
        self.delivery.pause();
        for slot in &mut self.slots {
            slot.rescan = false;
            slot.owner.pause();
        }
    }
    pub fn resume(&mut self) -> Result<()> {
        if self.discovery.phase() != CaptureWorkerPhase::Stopped {
            self.discovery.resume()?;
        }
        if self.delivery.phase() != CaptureWorkerPhase::Stopped {
            self.delivery.resume()?;
        }
        for slot in &mut self.slots {
            if slot.owner.phase() != CaptureWorkerPhase::Stopped {
                slot.owner.resume()?;
            }
        }
        Ok(())
    }
    pub fn stop(&mut self) {
        self.rescan = false;
        self.discovery.stop();
        self.delivery.stop();
        for slot in &mut self.slots {
            slot.owner.stop();
            slot.proofs.clear();
        }
    }
    pub fn retain_storage(&mut self, lifetime: &crate::storage::CaptureStorageLifetime) {
        self.discovery.retain_storage(lifetime.fork());
        self.delivery.retain_storage(lifetime.fork());
    }
    pub fn phases(&self) -> impl Iterator<Item = CaptureWorkerPhase> + '_ {
        [self.discovery.phase(), self.delivery.phase()]
            .into_iter()
            .chain(self.slots.iter().map(|s| s.owner.phase()))
    }
    pub fn tracked(&self) -> usize {
        self.slots.len()
    }
    pub fn occupied(&self) -> usize {
        self.slots.iter().filter(|s| s.occupied).count()
    }
    pub fn raw(&self) -> usize {
        self.slots.iter().filter(|s| s.raw).count()
    }
    pub fn stable(&self) -> bool {
        self.delivery.phase() == CaptureWorkerPhase::Ready && self.occupied() == 0
    }
    pub fn rescan(&mut self) {
        self.rescan = true;
        for slot in &mut self.slots {
            slot.rescan = true;
        }
    }
    pub fn observations(
        &self,
        stable: bool,
        closed: bool,
    ) -> impl Iterator<Item = super::sources::CodexSourceObservation> + '_ {
        self.slots.iter().map(move |s| {
            let phase = s.owner.phase();
            super::sources::CodexSourceObservation {
                import_request: None,
                snapshot_publication: s.publication_query,
                selector: s.selector,
                profile: jj_store_capture::profile::CaptureProfile::OpencodeReadOnlyApiV1,
                phase,
                observed: s.snapshot.observed,
                awaiting_activity: s.snapshot.awaiting_activity,
                caught_up: stable && s.snapshot.caught_up && !s.occupied,
                running: s.occupied && phase != CaptureWorkerPhase::Stopped,
                degraded: s.snapshot.unavailable
                    || (!closed && phase == CaptureWorkerPhase::Stopped),
                last_bound_repository: s.snapshot.anchor.as_ref().map(|a| a.workspace.repository),
                ingress_required_capacity_bytes: (phase != CaptureWorkerPhase::Stopped)
                    .then_some(s.snapshot.capacity_refusal)
                    .flatten()
                    .map(|r| r.required_bytes),
            }
        })
    }
    pub fn harness(&self, stable: bool, closed: bool) -> CodexHarnessStatus {
        let observations: Vec<_> = self.observations(stable, closed).collect();
        CodexHarnessStatus {
            profile: jj_store_capture::profile::CaptureProfile::OpencodeReadOnlyApiV1,
            roots: 0,
            unchecked_roots: 0,
            available_roots: 0,
            unavailable_roots: 0,
            tracked: self.slots.len(),
            observed_sources: observations.iter().filter(|s| s.observed).count(),
            awaiting_activity: observations.iter().filter(|s| s.awaiting_activity).count(),
            caught_up: observations.iter().filter(|s| s.caught_up).count(),
            running_sources: observations.iter().filter(|s| s.running).count(),
            degraded_sources: observations.iter().filter(|s| s.degraded).count(),
        }
    }
    pub fn required_capacity(&self) -> Option<u64> {
        self.slots
            .iter()
            .filter(|s| s.owner.phase() != CaptureWorkerPhase::Stopped)
            .filter_map(|s| s.snapshot.capacity_refusal.map(|r| r.required_bytes))
            .max()
    }
    pub fn required_delivery_capacity(&self) -> Option<u64> {
        self.delivery_capacity_refusal
            .filter(|_| self.delivery.phase() != CaptureWorkerPhase::Stopped)
            .map(|r| r.required_bytes)
    }
    pub fn capacity_refusals(&self) -> usize {
        self.slots
            .iter()
            .filter(|s| {
                s.owner.phase() != CaptureWorkerPhase::Stopped
                    && s.snapshot.capacity_refusal.is_some()
            })
            .count()
    }
}

use super::*;
use jj_store_capture::profile::CaptureProfile;
use jj_store_model::RepositoryId;

pub(super) struct SourceSelectors {
    generation: [u8; 16],
    high_water: u64,
}
impl SourceSelectors {
    pub fn new() -> Result<Self> {
        let mut generation = [0; 16];
        getrandom::fill(&mut generation)
            .map_err(|_| Error::Service("capture source selector unavailable"))?;
        if generation == [0; 16] {
            return Err(Error::Service("capture source selector unavailable"));
        }
        Ok(Self {
            generation,
            high_water: 0,
        })
    }
    pub fn allocate(&mut self) -> Result<u64> {
        self.high_water = self
            .high_water
            .checked_add(1)
            .ok_or(Error::Service("capture source selectors exhausted"))?;
        Ok(self.high_water)
    }
}
/// Body-free metadata only; no source binding, identity, lease or task owner.
#[derive(Clone, Debug, PartialEq, Eq)]
pub(crate) struct CodexSourceCatalog {
    pub generation: [u8; 16],
    pub high_water: u64,
    pub phase: CaptureWorkerPhase,
    pub sources: Vec<CodexSourceObservation>,
}
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub(crate) struct CodexSourceObservation {
    pub import_request: Option<(jj_store_model::PrincipalId, jj_store_model::RequestId)>,
    pub snapshot_publication:
        Option<jj_store_capture::reader_publication::SnapshotPublicationQuery>,
    pub selector: u64,
    pub profile: CaptureProfile,
    pub phase: CaptureWorkerPhase,
    pub observed: bool,
    pub awaiting_activity: bool,
    pub caught_up: bool,
    pub running: bool,
    pub degraded: bool,
    pub last_bound_repository: Option<RepositoryId>,
    pub ingress_required_capacity_bytes: Option<u64>,
}
impl CodexCaptureSupervisor {
    pub(crate) fn source_generation(&self) -> [u8; 16] {
        self.selectors.generation
    }
    pub(crate) fn retained_import_slots(&self) -> Vec<(u64, QualifiedCodexSource)> {
        self.slots
            .iter()
            .filter(|slot| {
                slot.owner.phase() != CaptureWorkerPhase::Stopped
                    && !slot.qualified.profile().is_cursor()
            })
            .map(|slot| (slot.selector, slot.qualified.clone()))
            .collect()
    }
    pub(crate) fn source_catalog(&self) -> CodexSourceCatalog {
        let stable = self.stable_queue();
        let mut catalog = CodexSourceCatalog {
            generation: self.selectors.generation,
            high_water: self.selectors.high_water,
            phase: self.phase(),
            sources: self
                .slots
                .iter()
                .map(|slot| {
                    let phase = slot.owner.phase();
                    CodexSourceObservation {
                        import_request: slot.import.as_ref().map(|d| d.request_identity()),
                        snapshot_publication: None,
                        selector: slot.selector,
                        profile: slot.qualified.profile(),
                        phase,
                        observed: slot.snapshot.observed,
                        awaiting_activity: slot.snapshot.awaiting_activity,
                        caught_up: stable && slot.snapshot.caught_up && !slot.occupied,
                        running: slot.occupied && phase != CaptureWorkerPhase::Stopped,
                        degraded: slot.snapshot.unavailable
                            || (!self.closed && phase == CaptureWorkerPhase::Stopped),
                        last_bound_repository: slot
                            .snapshot
                            .anchor
                            .as_ref()
                            .map(|a| a.workspace.repository),
                        ingress_required_capacity_bytes: (phase != CaptureWorkerPhase::Stopped)
                            .then_some(slot.snapshot.capacity_refusal)
                            .flatten()
                            .map(|r| r.required_bytes),
                    }
                })
                .collect(),
        };
        if let Some(api) = &self.api {
            catalog
                .sources
                .extend(api.observations(stable, self.closed));
        }
        catalog.sources.sort_by_key(|s| s.selector);
        catalog
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn selector_generations_change_and_ordinals_never_wrap_or_reuse() {
        let mut first = SourceSelectors::new().unwrap();
        let second = SourceSelectors::new().unwrap();
        assert_ne!(first.generation, [0; 16]);
        assert_ne!(first.generation, second.generation);
        assert_eq!(first.allocate().unwrap(), 1);
        assert_eq!(first.allocate().unwrap(), 2);
        first.high_water = u64::MAX;
        assert!(first.allocate().is_err());
        assert_eq!(first.high_water, u64::MAX);
    }
}

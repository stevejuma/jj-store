use super::*;
use crate::capture_supervisor::CodexSourceCatalog;
use std::sync::Arc;

impl CaptureRuntimeControl {
    pub(crate) fn source_snapshot(&self) -> Result<Arc<CodexSourceCatalog>> {
        let observed = self.sources.borrow().clone();
        if self.sources.has_changed().is_err() && observed.phase != CaptureWorkerPhase::Stopped {
            return Err(Error::Service("capture source observation unavailable"));
        }
        Ok(observed)
    }
}
#[cfg(test)]
pub(super) fn test_observation(
    phase: CaptureWorkerPhase,
) -> watch::Receiver<Arc<CodexSourceCatalog>> {
    let (_updates, sources) = watch::channel(Arc::new(CodexSourceCatalog {
        generation: [1; 16],
        high_water: 0,
        phase,
        sources: vec![],
    }));
    sources
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn source_producer_loss_withholds_nonterminal_cache_but_preserves_stopped_cache() {
        for phase in [
            CaptureWorkerPhase::Running,
            CaptureWorkerPhase::Paused,
            CaptureWorkerPhase::Stopped,
        ] {
            let (command, _receiver) = watch::channel(Command::Running);
            let (_updates, snapshot) = watch::channel(CaptureRuntimeSnapshot {
                phase,
                status: Default::default(),
            });
            let (source_updates, sources) = watch::channel(Arc::new(CodexSourceCatalog {
                generation: [1; 16],
                high_water: 1,
                phase,
                sources: vec![],
            }));
            let control = CaptureRuntimeControl {
                configured_roots: Default::default(),
                command,
                snapshot,
                sources,
                rescan: Default::default(),
                targets: super::super::targets::empty(),
                imports: super::super::imports::empty(),
            };
            let retained = control.source_snapshot().unwrap();
            assert_eq!(retained.high_water, 1);
            drop(source_updates);
            if phase == CaptureWorkerPhase::Stopped {
                assert!(Arc::ptr_eq(&retained, &control.source_snapshot().unwrap()));
            } else {
                assert!(control.source_snapshot().is_err());
            }
            assert_eq!(retained.high_water, 1);
        }
    }
}

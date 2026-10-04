//! Project meaningful capture progress while suppressing unchanged scheduling observations.
use super::*;
use jj_store_service::freshness::{Domain, Signals};
fn phase(value: CaptureWorkerPhase) -> CaptureWorkerPhase {
    match value {
        CaptureWorkerPhase::Running => CaptureWorkerPhase::Ready,
        other => other,
    }
}
fn stable(mut value: CaptureRuntimeSnapshot) -> CaptureRuntimeSnapshot {
    value.phase = phase(value.phase);
    value.status.discovery.phase = phase(value.status.discovery.phase);
    value.status.running_sources = 0;
    value.status.caught_up = 0;
    value.status.resident_source_frames = 0;
    value.status.resident_delivery_frames = 0;
    for harness in value.status.harnesses.iter_mut().flatten() {
        harness.running_sources = 0;
        harness.caught_up = 0;
    }
    value
}
fn sources(
    value: &crate::capture_supervisor::CodexSourceCatalog,
) -> crate::capture_supervisor::CodexSourceCatalog {
    let mut value = value.clone();
    value.phase = phase(value.phase);
    for source in &mut value.sources {
        source.phase = phase(source.phase);
        source.running = false;
        source.caught_up = false;
    }
    value
}
impl CaptureRuntimeControl {
    pub(crate) fn follow_freshness(&self, signals: Signals) {
        let mut snapshot = self.snapshot.clone();
        let mut catalog = self.sources.clone();
        let mut targets = self.targets.clone();
        let mut command = self.command.subscribe();
        tokio::spawn(async move {
            let mut previous = (
                stable(*snapshot.borrow()),
                sources(&catalog.borrow()),
                *command.borrow(),
            );
            loop {
                let target_changed;
                tokio::select! {
                    result = snapshot.changed() => { if result.is_err() { signals.notify(None, Domain::Capture, None); return; } target_changed = false; }
                    result = catalog.changed() => { if result.is_err() { return; } target_changed = false; }
                    result = command.changed() => { if result.is_err() { return; } target_changed = false; }
                    result = targets.changed() => { if result.is_err() { return; } target_changed = true; }
                }
                let current = (
                    stable(*snapshot.borrow()),
                    sources(&catalog.borrow()),
                    *command.borrow(),
                );
                if target_changed || previous.0 != current.0 || previous.2 != current.2 {
                    signals.notify(None, Domain::Capture, Some("status".into()));
                }
                if previous.1 != current.1 {
                    signals.notify(None, Domain::Capture, Some("sources".into()));
                }
                previous = current;
            }
        });
    }
}
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn scheduling_only_capture_changes_are_not_semantic_freshness() {
        let baseline = CaptureRuntimeSnapshot {
            phase: CaptureWorkerPhase::Ready,
            status: Default::default(),
        };
        let mut scheduled = baseline;
        scheduled.phase = CaptureWorkerPhase::Running;
        scheduled.status.running_sources = 1;
        scheduled.status.caught_up = 2;
        scheduled.status.resident_source_frames = 1;
        assert_eq!(stable(baseline), stable(scheduled));
        scheduled.status.pending_batches = 1;
        assert_ne!(stable(baseline), stable(scheduled));
        scheduled = baseline;
        scheduled.phase = CaptureWorkerPhase::Paused;
        assert_ne!(stable(baseline), stable(scheduled));
    }
}

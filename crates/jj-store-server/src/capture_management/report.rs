use super::*;
use crate::{capture_runtime::CaptureRuntimeSnapshot, capture_worker::CaptureWorkerPhase};

pub(super) fn policy(
    snapshot: crate::capture_control_state::CaptureControlSnapshot,
) -> CapturePolicy {
    CapturePolicy {
        generation: hex::encode(snapshot.generation),
        revision: snapshot.revision,
        paused: snapshot.paused,
    }
}
pub(super) fn runtime(
    snapshot: CaptureRuntimeSnapshot,
    roots: &[(
        jj_store_capture::profile::CaptureProfile,
        std::path::PathBuf,
    )],
) -> CaptureRuntimeStatus {
    let status = snapshot.status;
    CaptureRuntimeStatus {
        harnesses: status
            .harnesses
            .into_iter()
            .flatten()
            .map(|status| harness(status, roots))
            .collect(),
        phase: phase(snapshot.phase),
        discovery: CaptureDiscoveryStatus {
            phase: phase(status.discovery.phase),
            roots: status.discovery.roots,
            unchecked_roots: status.discovery.unchecked_roots,
            available_roots: status.discovery.available_roots,
            unavailable_roots: status.discovery.unavailable_roots,
            rescan_pending: status.discovery.rescan_pending,
        },
        degraded_sources: status.degraded_sources,
        pending_capacity_bytes: status.pending_capacity_bytes,
        ingress_capacity_refusals: status.ingress_capacity_refusals,
        ingress_required_capacity_bytes: status.ingress_required_capacity_bytes,
        delivery_required_capacity_bytes: status.delivery_required_capacity_bytes,
        tracked: status.tracked,
        observed_sources: status.observed_sources,
        awaiting_activity: status.awaiting_activity,
        caught_up: status.caught_up,
        running_sources: status.running_sources,
        resident_source_frames: status.resident_source_frames,
        resident_delivery_frames: status.resident_delivery_frames,
        pending_batches: status.pending_batches,
        unavailable: status.unavailable,
        source_limit_reached: status.source_limit_reached,
    }
}
fn harness(
    status: crate::capture_supervisor::CodexHarnessStatus,
    roots: &[(
        jj_store_capture::profile::CaptureProfile,
        std::path::PathBuf,
    )],
) -> CaptureHarnessCoverage {
    CaptureHarnessCoverage {
        configured_roots: Some(
            roots
                .iter()
                .filter(|(profile, _)| *profile == status.profile)
                .map(|(_, path)| {
                    CaptureRootPath(
                        path.to_str()
                            .expect("startup-qualified UTF-8 root")
                            .to_owned(),
                    )
                })
                .collect(),
        ),
        profile: profile(status.profile),
        product_version: status.profile.product_version().into(),
        roots: status.roots,
        unchecked_roots: status.unchecked_roots,
        available_roots: status.available_roots,
        unavailable_roots: status.unavailable_roots,
        tracked: status.tracked,
        observed_sources: status.observed_sources,
        awaiting_activity: status.awaiting_activity,
        caught_up: status.caught_up,
        running_sources: status.running_sources,
        degraded_sources: status.degraded_sources,
    }
}
pub(super) fn profile(profile: jj_store_capture::profile::CaptureProfile) -> CaptureHarnessProfile {
    use jj_store_capture::profile::CaptureProfile as P;
    match profile {
        P::CodexCliJsonlV1 => CaptureHarnessProfile::CodexCliJsonlV1,
        P::CodexDesktopJsonlV1 => CaptureHarnessProfile::CodexDesktopJsonlV1,
        P::CursorDesktopTranscriptV1 => CaptureHarnessProfile::CursorDesktopTranscriptV1,
        P::CursorCliTranscriptV1 => CaptureHarnessProfile::CursorCliTranscriptV1,
        P::OpencodeReadOnlyApiV1 => CaptureHarnessProfile::OpencodeReadOnlyApiV1,
        P::OpencodeExportJsonV1 => CaptureHarnessProfile::OpencodeExportJsonV1,
    }
}
pub(super) fn phase(value: CaptureWorkerPhase) -> CaptureRuntimePhase {
    match value {
        CaptureWorkerPhase::Ready => CaptureRuntimePhase::Ready,
        CaptureWorkerPhase::Running => CaptureRuntimePhase::Running,
        CaptureWorkerPhase::Overdue => CaptureRuntimePhase::Overdue,
        CaptureWorkerPhase::Draining => CaptureRuntimePhase::Draining,
        CaptureWorkerPhase::Paused => CaptureRuntimePhase::Paused,
        CaptureWorkerPhase::Stopped => CaptureRuntimePhase::Stopped,
    }
}
pub(super) fn failure(error: crate::capture_control_state::Error) -> Error {
    use crate::capture_control_state::Error as E;
    match error {
        E::Invalid => Error::Invalid,
        E::Conflict => Error::Conflict,
        E::StaleRevision | E::ForeignGeneration => Error::Stale,
        E::Capacity => Error::Capacity,
        E::OutcomeUnknown => Error::Unknown,
        E::Unavailable => Error::Unavailable,
    }
}

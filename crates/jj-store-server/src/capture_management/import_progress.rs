use super::*;
use crate::{
    capture_import_state::*, capture_supervisor::CodexSourceCatalog,
    capture_worker::CaptureWorkerPhase,
};
use jj_store_model::RequestId;

pub(super) fn command(intent: &CaptureImportIntent) -> CaptureImportCommand {
    CaptureImportCommand {
        version: 1,
        request_id: intent.request,
        generation: hex::encode(intent.generation),
        expected_revision: intent.expected_revision,
        selector: intent.selector.clone(),
    }
}
fn acceptance(receipt: &CaptureImportReceipt) -> CaptureImportAcceptance {
    CaptureImportAcceptance {
        command: command(&receipt.intent),
        ordinal: receipt.ordinal,
        profile: report::profile(receipt.source.profile),
        product_version: receipt.source.product_version.clone(),
        complete_byte_offset: receipt.source.complete_byte_offset,
        complete_records: receipt.source.complete_records,
    }
}
pub(super) fn project(
    actor: PrincipalId,
    request: RequestId,
    paused: bool,
    snapshot: Option<&CodexSourceCatalog>,
    resources: &CaptureImportManagement,
    selected: Option<(&CaptureImportReceipt, Option<&CaptureImportReaderBinding>)>,
) -> CaptureImportStatus {
    use CaptureImportPhase as P;
    let principal = selected.map_or(actor, |(r, _)| r.intent.principal);
    let row = snapshot.and_then(|s| {
        s.sources
            .iter()
            .find(|r| r.import_request == Some((principal, request)))
    });
    let tail_phase = if paused {
        P::Withheld
    } else {
        match (snapshot, row) {
            (Some(s), _)
                if matches!(
                    s.phase,
                    CaptureWorkerPhase::Paused | CaptureWorkerPhase::Draining
                ) =>
            {
                P::Withheld
            }
            (Some(s), _) if s.phase == CaptureWorkerPhase::Stopped => P::Unavailable,
            (_, Some(r)) if r.degraded => P::Withheld,
            (_, Some(r)) if r.running => P::Running,
            (_, Some(_)) => P::Pending,
            _ => P::Unavailable,
        }
    };
    let mut progress = CaptureImportProgress {
        phase: tail_phase,
        boundary_available: false,
        captured_byte_offset: 0,
        committed_byte_offset: 0,
        tail_phase,
        tail: None,
    };
    if let Some((receipt, binding)) = selected {
        if tail_phase == P::Unavailable
            && snapshot.is_some_and(|s| s.phase != CaptureWorkerPhase::Stopped)
        {
            progress.phase = P::Selected;
        }
        if let Some(binding) = binding.filter(|b| b.prefix_verified) {
            match resources.spool.inspect_reader_publication(
                jj_store_service::collaboration::AgentIntegrationIdentity::Codex,
                binding.source,
                binding.generation_digest,
                receipt.source.complete_byte_offset,
            ) {
                Ok(Some(coverage)) => {
                    progress.boundary_available = coverage.boundary_available;
                    progress.captured_byte_offset = coverage.captured_byte_offset;
                    progress.committed_byte_offset = coverage.committed_byte_offset;
                    progress.tail = coverage.tail.map(|t| CaptureImportTail {
                        history_available: t.history_available,
                        captured_byte_offset: t.captured_byte_offset,
                        committed_byte_offset: t.committed_byte_offset,
                        outstanding_ranges: t.outstanding_ranges,
                        receipts: t.receipts,
                    });
                    if coverage.completed {
                        progress.phase = P::Completed;
                    } else if !coverage.boundary_available {
                        progress.phase = P::Unavailable;
                    }
                }
                Ok(None) => {}
                Err(_) => {
                    progress.phase = P::Unavailable;
                    progress.tail_phase = P::Unavailable;
                }
            }
        } else {
            progress.phase = P::Unavailable;
        }
    }
    CaptureImportStatus {
        version: 1,
        request_id: request,
        receipt: selected.map(|(r, _)| acceptance(r)),
        progress,
    }
}

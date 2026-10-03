import type {
  CaptureImport,
  CaptureImportOutcome,
  CaptureSourcePage,
  CaptureStatus,
} from "../../web/src/lib/controller/capture-types.js";
export const captureGeneration = "ab".repeat(16);
export function captureStatus(
  paused = false,
  revision: number | bigint = 1,
): CaptureStatus {
  return {
    version: 1,
    enabled: true,
    policy: { generation: captureGeneration, revision, paused },
    runtime: {
      phase: paused ? "paused" : "running",
      discovery: {
        phase: paused ? "paused" : "running",
        roots: 1,
        unchecked_roots: 0,
        available_roots: 1,
        unavailable_roots: 0,
        rescan_pending: false,
      },
      degraded_sources: 0,
      harnesses: [
        {
          profile: "codex-cli-jsonl-v1",
          product_version: "0.156.1",
          roots: 1,
          unchecked_roots: 0,
          available_roots: 1,
          unavailable_roots: 0,
          tracked: 1,
          observed_sources: 1,
          awaiting_activity: 1,
          caught_up: 0,
          running_sources: 0,
          degraded_sources: 0,
        },
      ],
      pending_capacity_bytes: 4096,
      ingress_capacity_refusals: 0,
      ingress_required_capacity_bytes: null,
      delivery_required_capacity_bytes: null,
      tracked: 1,
      observed_sources: 1,
      awaiting_activity: 1,
      caught_up: 0,
      running_sources: 0,
      resident_source_frames: 0,
      resident_delivery_frames: 0,
      pending_batches: 0,
      unavailable: 0,
      source_limit_reached: false,
    },
  };
}
export function captureSources(): CaptureSourcePage {
  return {
    version: 1,
    enabled: true,
    runtime_generation: captureGeneration,
    phase: "running",
    total: 1,
    sources: [
      {
        selector: captureGeneration + ":0000000000000001",
        profile: "codex-cli-jsonl-v1",
        product_version: "0.156.1",
        phase: "running",
        observed: true,
        awaiting_activity: true,
        caught_up: false,
        running: false,
        degraded: false,
        last_bound_repository: "ef".repeat(16),
        ingress_required_capacity_bytes: null,
      },
    ],
    next_cursor: null,
  };
}
export function captureReceipt(
  command: CaptureImport = {
    version: 1,
    request_id: "cd".repeat(16),
    generation: captureGeneration,
    expected_revision: 1,
    selector: captureSources().sources[0].selector,
  },
): CaptureImportOutcome {
  return {
    version: 1,
    replayed: false,
    receipt: {
      command,
      ordinal: 1,
      profile: "codex-cli-jsonl-v1",
      product_version: "0.156.1",
      complete_byte_offset: 1024,
      complete_records: 3,
    },
    progress: {
      phase: "completed",
      boundary_available: true,
      captured_byte_offset: 1024,
      committed_byte_offset: 1024,
      tail_phase: "running",
      tail: {
        history_available: true,
        captured_byte_offset: 2048,
        committed_byte_offset: 1024,
        outstanding_ranges: 1,
        receipts: 1,
      },
    },
  };
}

import type { Model } from "../api/client.js";
import {
  requireCapture,
  object,
  bool,
  uint,
  count,
} from "./capture-response-validation.js";
import { validateSnapshotPublication } from "./capture-snapshot-validation.js";
export { requireCapture } from "./capture-response-validation.js";
import {
  captureProfiles,
  type CaptureControl,
  type CaptureImport,
} from "./capture-types.js";

export const captureId = (v: unknown): v is string =>
  typeof v === "string" && /^[0-9a-f]{32}$/.test(v) && v !== "0".repeat(32);
const selection = (v: unknown): v is string =>
  typeof v === "string" &&
  /^[0-9a-f]{32}:[0-9a-f]{16}$/.test(v) &&
  captureId(v.slice(0, 32));
const runtimePhase = (v: unknown) =>
  ["ready", "running", "overdue", "draining", "paused", "stopped"].includes(
    v as string,
  );
const importPhase = (v: unknown) =>
  [
    "selected",
    "pending",
    "running",
    "withheld",
    "completed",
    "unavailable",
  ].includes(v as string);
const counts = (v: Record<string, unknown>, fields: string[]) =>
  fields.every((f) => count(v[f]));
const coverageCounts = [
  "roots",
  "unchecked_roots",
  "available_roots",
  "unavailable_roots",
  "tracked",
  "observed_sources",
  "awaiting_activity",
  "caught_up",
  "running_sources",
  "degraded_sources",
];
const profile = (v: unknown) =>
  typeof v === "string" && Object.hasOwn(captureProfiles, v);
const productVersion = (v: unknown) =>
  typeof v === "string" && /^[0-9A-Za-z.+_-]{1,128}$/.test(v);

export function validateCaptureStatus(
  v: unknown,
): asserts v is Model<"CaptureStatus"> {
  requireCapture(object(v) && v.version === 1 && bool(v.enabled));
  requireCapture(
    v.policy === null ||
      (object(v.policy) &&
        captureId(v.policy.generation) &&
        uint(v.policy.revision) &&
        bool(v.policy.paused)),
  );
  if (v.runtime === null) return;
  const r = v.runtime;
  requireCapture(object(r) && runtimePhase(r.phase) && object(r.discovery));
  requireCapture(
    runtimePhase(r.discovery.phase) &&
      bool(r.discovery.rescan_pending) &&
      counts(r.discovery, coverageCounts.slice(0, 4)),
  );
  requireCapture(
    counts(r, [
      "degraded_sources",
      "tracked",
      "observed_sources",
      "awaiting_activity",
      "caught_up",
      "running_sources",
      "resident_source_frames",
      "resident_delivery_frames",
    ]),
  );
  requireCapture(
    uint(r.ingress_capacity_refusals) &&
      uint(r.unavailable) &&
      uint(r.pending_batches) &&
      uint(r.pending_capacity_bytes) &&
      bool(r.source_limit_reached),
  );
  requireCapture(
    [
      r.ingress_required_capacity_bytes,
      r.delivery_required_capacity_bytes,
    ].every((v) => v === null || uint(v)),
  );
  requireCapture(Array.isArray(r.harnesses) && r.harnesses.length <= 5);
  const profiles = new Set<string>();
  for (const h of r.harnesses) {
    requireCapture(
      object(h) &&
        profile(h.profile) &&
        productVersion(h.product_version) &&
        counts(h, coverageCounts),
    );
    requireCapture(!profiles.has(h.profile as string));
    profiles.add(h.profile as string);
  }
}
export function validateCaptureSources(
  v: unknown,
): asserts v is Model<"CaptureSourcePage"> {
  requireCapture(
    object(v) && v.version === 1 && bool(v.enabled) && count(v.total),
  );
  requireCapture(
    v.runtime_generation === null || captureId(v.runtime_generation),
  );
  requireCapture(v.phase === null || runtimePhase(v.phase));
  requireCapture(
    Array.isArray(v.sources) &&
      v.sources.length <= 100 &&
      BigInt(v.total as number | bigint) >= BigInt(v.sources.length),
  );
  requireCapture(
    v.next_cursor === null ||
      (selection(v.next_cursor) &&
        v.next_cursor.slice(0, 32) === v.runtime_generation),
  );
  const seen = new Set<string>();
  for (const s of v.sources) {
    requireCapture(
      object(s) &&
        selection(s.selector) &&
        s.selector.slice(0, 32) === v.runtime_generation &&
        !seen.has(s.selector),
    );
    seen.add(s.selector);
    requireCapture(
      profile(s.profile) &&
        productVersion(s.product_version) &&
        runtimePhase(s.phase),
    );
    requireCapture(
      [
        s.observed,
        s.awaiting_activity,
        s.caught_up,
        s.running,
        s.degraded,
      ].every(bool),
    );
    requireCapture(
      s.last_bound_repository === null || captureId(s.last_bound_repository),
    );
    requireCapture(
      s.ingress_required_capacity_bytes === null ||
        uint(s.ingress_required_capacity_bytes),
    );
    if (s.snapshot_publication != null) {
      requireCapture(s.profile === "opencode-read-only-api-v1");
      validateSnapshotPublication(s.snapshot_publication);
    }
  }
  requireCapture(
    v.enabled ||
      (!v.sources.length &&
        v.runtime_generation === null &&
        v.next_cursor === null &&
        v.phase === null &&
        v.total === 0),
  );
}
function validateProgress(v: unknown) {
  requireCapture(
    object(v) &&
      importPhase(v.phase) &&
      importPhase(v.tail_phase) &&
      bool(v.boundary_available),
  );
  requireCapture(
    uint(v.captured_byte_offset) &&
      uint(v.committed_byte_offset) &&
      BigInt(v.committed_byte_offset as number | bigint) <=
        BigInt(v.captured_byte_offset as number | bigint),
  );
  if (v.tail === null) return;
  const t = v.tail;
  requireCapture(
    object(t) &&
      bool(t.history_available) &&
      uint(t.captured_byte_offset) &&
      uint(t.committed_byte_offset) &&
      count(t.outstanding_ranges) &&
      uint(t.receipts),
  );
  requireCapture(
    BigInt(t.committed_byte_offset as number | bigint) <=
      BigInt(t.captured_byte_offset as number | bigint),
  );
}
function validateReceipt(v: unknown) {
  requireCapture(object(v) && object(v.command));
  const c = v.command;
  requireCapture(
    c.version === 1 &&
      captureId(c.request_id) &&
      captureId(c.generation) &&
      uint(c.expected_revision) &&
      selection(c.selector),
  );
  requireCapture(
    uint(v.ordinal) &&
      profile(v.profile) &&
      productVersion(v.product_version) &&
      uint(v.complete_byte_offset) &&
      uint(v.complete_records),
  );
}
export function sameImport(a: CaptureImport, b: CaptureImport) {
  return (
    a.version === b.version &&
    a.request_id === b.request_id &&
    a.generation === b.generation &&
    BigInt(a.expected_revision) === BigInt(b.expected_revision) &&
    a.selector === b.selector
  );
}
export function validateControlOutcome(
  v: unknown,
  c: CaptureControl,
): asserts v is Model<"CaptureControlOutcome"> {
  requireCapture(
    object(v) &&
      v.version === 1 &&
      bool(v.replayed) &&
      bool(v.runtime_requested) &&
      object(v.receipt),
  );
  validateCaptureStatus(v.status);
  const r = v.receipt;
  requireCapture(
    r.request_id === c.request_id &&
      r.generation === c.generation &&
      uint(r.expected_revision) &&
      BigInt(r.expected_revision as number | bigint) ===
        BigInt(c.expected_revision),
  );
  requireCapture(
    uint(r.revision) &&
      BigInt(r.revision as number | bigint) ===
        BigInt(c.expected_revision) + 1n &&
      r.paused === (c.action === "pause"),
  );
}
export function validateImportOutcome(
  v: unknown,
  c: CaptureImport,
): asserts v is Model<"CaptureImportOutcome"> {
  requireCapture(object(v) && v.version === 1 && bool(v.replayed));
  validateReceipt(v.receipt);
  requireCapture(
    sameImport((v.receipt as Model<"CaptureImportAcceptance">).command, c),
  );
  validateProgress(v.progress);
  validateCompletion(v as Model<"CaptureImportOutcome">);
}
function validateCompletion(result: {
  receipt: Model<"CaptureImportAcceptance"> | null;
  progress: Model<"CaptureImportProgress">;
}) {
  if (result.progress.phase === "completed")
    requireCapture(
      result.receipt !== null &&
        result.progress.boundary_available &&
        BigInt(result.progress.committed_byte_offset) >=
          BigInt(result.receipt.complete_byte_offset),
    );
}
export function validateImportStatus(
  v: unknown,
  id: string,
): asserts v is Model<"CaptureImportStatus"> {
  requireCapture(object(v) && v.version === 1 && v.request_id === id);
  if (v.receipt !== null) {
    validateReceipt(v.receipt);
    requireCapture(
      (v.receipt as Model<"CaptureImportAcceptance">).command.request_id === id,
    );
  }
  validateProgress(v.progress);
  validateCompletion(v as Model<"CaptureImportStatus">);
}
export function validateRescan(
  v: unknown,
): asserts v is Model<"CaptureRescanOutcome"> {
  requireCapture(object(v) && v.version === 1 && bool(v.accepted));
  validateCaptureStatus(v.status);
}

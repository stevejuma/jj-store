import type { ForgeCheck } from "./types.js";
/** Only provider start/completion facts establish elapsed time. No wall-clock estimates. */
export function checkLabel(check: ForgeCheck) {
  const label = {
    running: "In progress",
    queued: "Queued",
    manual: "Action required",
    failed: "Failing",
    passed: "Successful",
    skipped: "Skipped",
    cancelled: "Cancelled",
    unknown: "Unknown",
  }[check.state];
  if (!["passed", "failed"].includes(check.state)) return label;
  const iso =
    /^\d{4}-\d{2}-\d{2}T\d{2}:\d{2}:\d{2}(?:\.\d+)?(?:Z|[+-]\d{2}:\d{2})$/;
  const start = check.started_at,
    end = check.completed_at;
  if (!start || !end || !iso.test(start) || !iso.test(end)) return label;
  const elapsed = Date.parse(end) - Date.parse(start);
  if (!Number.isFinite(elapsed) || elapsed < 0) return label;
  const seconds = Math.floor(elapsed / 1000);
  const time =
    seconds >= 60
      ? `${Math.floor(seconds / 60)}m ${seconds % 60}s`
      : `${seconds}s`;
  return `${label} ${check.state === "failed" ? "after" : "in"} ${time}`;
}

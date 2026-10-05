import type { CaptureStatus } from "./capture-types.js";
export function captureSummary(value: CaptureStatus) {
  if (!value.enabled) return "Disabled";
  if (value.policy?.paused) return "Paused";
  const runtime = value.runtime;
  if (!runtime || runtime.phase === "stopped") return "Capture unavailable";
  if (runtime.pending_batches || runtime.running_sources > runtime.caught_up)
    return "Running · Catching up";
  if (
    !runtime.observed_sources ||
    runtime.awaiting_activity === runtime.observed_sources
  )
    return "Running · Waiting for linked agent activity";
  if (runtime.caught_up) return "Running · Published captures are up to date";
  return "Running · Observing linked agent sessions";
}

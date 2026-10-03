import { ApiError } from "../api/client.js";

export const captureRefusals = new Set([
  "capture_management_denied",
  "capture_disabled",
  "capture_paused",
  "invalid_capture_control",
  "capture_control_intent_conflict",
  "capture_policy_changed",
  "capture_source_selection_changed",
  "capture_control_busy",
  "capture_control_capacity",
  "capture_management_unavailable",
]);
export function captureError(cause: unknown) {
  const code = cause instanceof ApiError ? cause.code : "";
  if (code === "capture_management_denied")
    return "Local capture access was denied.";
  if (code === "capture_policy_changed")
    return "Capture policy changed. Refresh capture before a new command.";
  if (code === "capture_source_selection_changed")
    return "Source selection changed. Refresh capture before a new import.";
  if (code === "capture_control_capacity")
    return "Capture state is full. Retained evidence has been preserved.";
  return "Capture management is unavailable. Refresh capture before a new command.";
}

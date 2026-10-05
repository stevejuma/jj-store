import { selectedWorkspace } from "./presentation.js";
import type { WorkspaceCommand, WorkspaceSnapshot } from "./types.js";
export function workspaceCommandEligibility(
  snapshot: WorkspaceSnapshot | null,
  kind: WorkspaceCommand["kind"],
) {
  if (!snapshot || snapshot.availability !== "ready" || !snapshot.folder)
    return {
      allowed: false,
      reason: "This command requires an available local checkout.",
    };
  if (!snapshot.capabilities.includes(kind))
    return {
      allowed: false,
      reason:
        snapshot.capabilityReasons?.[kind] ??
        "This command is not available for this local checkout.",
    };
  if (!snapshot.target && kind !== "switch_workspace")
    return {
      allowed: false,
      reason: "Choose a workspace target branch first.",
    };
  if (
    !selectedWorkspace(snapshot) &&
    kind !== "create_workspace" &&
    kind !== "switch_workspace"
  )
    return { allowed: false, reason: "Set up a named stack workspace first." };
  return { allowed: true, reason: "" };
}

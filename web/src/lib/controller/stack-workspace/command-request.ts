import type { StackWorkspaceController } from "./controller.svelte.js";
import {
  cloneWorkspace,
  type WorkspaceCommand,
  type WorkspaceRequest,
  type WorkspaceSnapshot,
} from "./types.js";

export function workspaceCommandRequest(
  snapshot: WorkspaceSnapshot,
  command: WorkspaceCommand,
): WorkspaceRequest {
  return {
    id: globalThis.crypto.randomUUID(),
    repository: snapshot.repository,
    revision: snapshot.revision,
    ...(snapshot.readKey ? { readKey: snapshot.readKey } : {}),
    command: cloneWorkspace(command),
  };
}

/** Membership is a single-click command; the executor still validates its exact pin. */
export async function applyWorkspaceMembership(
  controller: StackWorkspaceController,
  command: WorkspaceCommand,
) {
  if (command.kind !== "apply_stack" && command.kind !== "unapply_stack")
    return;
  const snapshot = controller.snapshot;
  if (!snapshot || controller.locked) return;
  controller.error = controller.validation = controller.outcome = "";
  const eligibility = controller.eligibility(command.kind);
  if (!eligibility.allowed) {
    controller.validation = eligibility.reason;
    return;
  }
  await controller.apply(workspaceCommandRequest(snapshot, command));
}

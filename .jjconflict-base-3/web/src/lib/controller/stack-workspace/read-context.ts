import { WorkspaceCommandError, type WorkspaceSnapshot } from "./types.js";
export function stackReadContext(
  snapshot: WorkspaceSnapshot | null,
  request: { repository: string; revision: number; readKey?: string },
  closed: AbortSignal,
) {
  closed.throwIfAborted();
  if (
    !snapshot ||
    request.repository !== snapshot.repository ||
    request.revision !== snapshot.revision ||
    request.readKey !== snapshot.readKey
  )
    throw new WorkspaceCommandError(
      "stale",
      "The stack read scope changed. Refresh and select again.",
    );
  return snapshot;
}
export function unavailableStackCommand(): Promise<never> {
  return Promise.reject(
    new WorkspaceCommandError(
      "invalid",
      "The durable stack command executor is not available.",
    ),
  );
}

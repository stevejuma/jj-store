import { ApiError, Client, fullId, pretty } from "../api/client.js";
import type { AgentTarget } from "./agent-execution-types.js";
import { HttpLocalIndexDataSource } from "./http-local-index-data-source.js";
import { HttpStoreWriteDataSource } from "./http-store-write-data-source.js";

export function needsAgentRevisionIndex(cause: unknown, target: AgentTarget) {
  if (!(cause instanceof ApiError) || cause.code !== "index_rebuild_required")
    return false;
  const snapshot = cause.localSnapshot ?? cause.snapshot;
  if (!snapshot) return true;
  return (
    ("repository" in snapshot ? snapshot.repository : snapshot.repo_id) ===
      target.repository &&
    ("operation" in snapshot ? snapshot.operation : snapshot.operation_id) ===
      target.operation
  );
}

/** Explicit bounded preparation using the existing profile-owned task transports. */
export async function rebuildAgentRevisionIndex(
  target: AgentTarget,
  fetcher: typeof fetch,
  signal: AbortSignal,
) {
  const client =
    target.profile === "native"
      ? new Client(target.repository, null, () => {}, fetcher)
      : undefined;
  const source = client
    ? new HttpStoreWriteDataSource(client)
    : new HttpLocalIndexDataSource(target.repository, fetcher);
  const started = Date.now();
  try {
    signal.throwIfAborted();
    let task = await source.prepareIndex(target.operation, signal);
    const id = fullId(task.id, 32),
      pin = pretty(task.snapshot);
    while (true) {
      signal.throwIfAborted();
      const snapshot = task.snapshot;
      if (
        task.version !== 1 ||
        task.id !== id ||
        pretty(snapshot) !== pin ||
        ("repository" in snapshot ? snapshot.repository : snapshot.repo_id) !==
          target.repository ||
        ("operation" in snapshot
          ? snapshot.operation
          : snapshot.operation_id) !== target.operation
      )
        throw new ApiError("snapshot_mismatch");
      if (task.status === "ready") return;
      if (!["running", "canceling"].includes(task.status))
        throw new ApiError("index_preparation_failed");
      if (Date.now() - started >= 300_000)
        throw new ApiError("index_preparation_still_running");
      await new Promise<void>((resolve, reject) => {
        const done = () => {
          signal.removeEventListener("abort", abort);
          resolve();
        };
        const timer = setTimeout(done, 500);
        const abort = () => {
          clearTimeout(timer);
          reject(signal.reason);
        };
        signal.addEventListener("abort", abort, { once: true });
      });
      task = await source.indexTask(id, signal);
    }
  } finally {
    if (source instanceof HttpLocalIndexDataSource) source.close();
    client?.close();
  }
}

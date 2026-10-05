import { ApiError, type Model } from "../../api/client.js";
import type { LocalStacksTransport } from "./local-transport.js";
import type { StackCommit } from "./types.js";
/** Inspect only already-loaded rows at one operation, never repository history. */
export async function inspectLocalCommits(
  transport: LocalStacksTransport | undefined,
  state: Model<"LocalStacksState"> | null,
  generation: number | bigint,
  commits: StackCommit[],
  signal: AbortSignal,
) {
  if (!transport?.inspect || !state?.active) return;
  const unique = [...new Set(commits.map((commit) => commit.id))];
  for (let offset = 0; offset < unique.length; offset += 500) {
    signal.throwIfAborted();
    const ids = unique.slice(offset, offset + 500);
    const inspected = await transport.inspect({
      version: 1,
      binding_generation: generation,
      operation: state.operation,
      revision: state.revision,
      commits: ids,
    });
    signal.throwIfAborted();
    const entries = new Map(
      inspected.commits.map((entry) => [entry.commit, entry]),
    );
    if (entries.size !== ids.length || ids.some((id) => !entries.has(id)))
      throw new ApiError("invalid_stack_inspection");
    for (const commit of commits) {
      const result = entries.get(commit.id);
      if (result) {
        commit.immutable = !result.mutable;
        commit.immutableReason = result.reason ?? undefined;
      }
    }
  }
}

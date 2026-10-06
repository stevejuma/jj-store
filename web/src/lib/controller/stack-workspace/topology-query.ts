import { ApiError, type Model } from "../../api/client.js";
import type { LocalStacksTransport } from "./local-transport.js";
import { projectStackTopology } from "./topology-projection.js";
import type { WorkspaceSnapshot } from "./types.js";

/** Single-operation cache for target browsing before any workspace is configured. */
export class StackTopologyQuery {
  #cached: { key: string; value: Model<"LocalStacksTopology"> } | null = null;
  async project(
    snapshot: WorkspaceSnapshot,
    context: {
      source: { stacks?: LocalStacksTransport };
      binding: { generation: number | bigint };
      pin(): { operation: string };
      signal: AbortSignal;
    },
  ) {
    const {
      source: { stacks: transport },
      binding: { generation },
      signal,
    } = context;
    const operation = context.pin().operation;
    if (snapshot.topology || !snapshot.target || !transport?.topology) return;
    const target = snapshot.target;
    const key = JSON.stringify([operation, target.name, target.commit]);
    let value = this.#cached?.key === key ? this.#cached.value : undefined;
    if (!value) {
      const response = await transport.topology({
        version: 1,
        binding_generation: generation,
        operation,
        target: target.name,
        target_commit: target.commit,
      });
      signal.throwIfAborted();
      if (
        response.version !== 1 ||
        response.operation !== operation ||
        response.target !== target.name ||
        response.target_commit !== target.commit
      )
        throw new ApiError("snapshot_mismatch");
      value = response.topology;
      this.#cached = { key, value };
    }
    projectStackTopology(snapshot, value);
  }
}

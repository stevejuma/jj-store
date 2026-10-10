import {
  isAgentCommandRefusal,
  agentCommandError,
} from "./agent-execution-errors.js";
import { LocalClient } from "../api/local-client.js";
import type { operations } from "../api/generated.js";
export type ThreadWorkspaceRequest =
  operations["local_threads_configure"]["requestBody"]["content"]["application/json"];
import { ApiError, type Model } from "../api/client.js";

/** Session-owned frozen configuration requests, including uncertain publication. */
export class ThreadWorkspaces {
  busy = $state(false);
  error = $state("");
  pending = $state<ThreadWorkspaceRequest | null>(null);
  constructor(
    readonly client: LocalClient,
    readonly published: (
      value: Model<"ThreadWorkspaceOutcome">,
    ) => Promise<void>,
  ) {}
  async submit(command: ThreadWorkspaceRequest["command"]) {
    if (this.busy || this.pending)
      throw new Error("Resolve the pending Thread action first.");
    this.pending = {
      version: 1,
      request_id: crypto.randomUUID().replaceAll("-", ""),
      command,
    };
    return this.retry();
  }
  async retry() {
    if (!this.pending || this.busy) return;
    this.busy = true;
    this.error = "";
    try {
      const result = await this.client.request<Model<"ThreadWorkspaceOutcome">>(
        "threads",
        this.pending,
      );
      this.pending = null;
      await this.published(result);
      return result;
    } catch (cause) {
      // Proven refusals are editable; unknown outcomes retain exact intent.
      if (
        isAgentCommandRefusal(cause) ||
        (cause instanceof ApiError &&
          [
            "access_denied",
            "invalid_request",
            "conflict",
            "stale",
            "busy",
            "invalid",
            "denied",
          ].includes(cause.code))
      )
        this.pending = null;
      this.error = isAgentCommandRefusal(cause)
        ? agentCommandError(cause)
        : cause instanceof Error
          ? cause.message
          : "Thread publication outcome is unknown.";
      throw cause;
    } finally {
      this.busy = false;
    }
  }
  destroy() {
    this.client.close();
  }
}

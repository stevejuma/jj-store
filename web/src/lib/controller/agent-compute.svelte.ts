import type { Exact, Model } from "../api/client.js";
import type { operations } from "../api/generated.js";
import { ApiError } from "../api/client.js";
import type { LocalClient } from "../api/local-client.js";
export type ComputeValue = Model<"ComputeStatus">;
export type ComputeAction = Exact<
  operations["local_compute_command"]["requestBody"]["content"]["application/json"]["action"]
>;
export class AgentCompute {
  value = $state<ComputeValue | null>(null);
  draft = $state<Model<"ComputeSettings"> | null>(null);
  pending = $state<Record<string, unknown> | null>(null);
  busy = $state(false);
  error = $state("");
  #timer?: ReturnType<typeof setTimeout>;
  #active = false;
  constructor(readonly client: LocalClient) {}
  start() {
    this.#active = true;
    void this.refresh();
  }
  stop() {
    this.#active = false;
    clearTimeout(this.#timer);
  }
  async refresh() {
    try {
      this.value = await this.client.request<ComputeValue>("compute");
      if (this.pending) {
        const outcome = await this.client.request<
          Model<"AgentExecutionOutcome">
        >(`agents/outcome?request=${this.pending.request_id}`);
        if (
          ["completed", "failed", "cancelled", "outcome_unknown"].includes(
            outcome.state,
          )
        ) {
          if (outcome.error) this.error = outcome.error;
          this.pending = null;
        }
      }
    } catch (cause) {
      this.error = cause instanceof Error ? cause.message : String(cause);
    } finally {
      clearTimeout(this.#timer);
      if (this.#active)
        this.#timer = setTimeout(() => void this.refresh(), 2000);
    }
  }
  async command(action: ComputeAction) {
    if (!this.value || this.busy || (this.pending && action.action !== "stop"))
      return false;
    this.busy = true;
    this.error = "";
    const body = {
      version: 1,
      request_id: crypto.randomUUID().replaceAll("-", ""),
      expected_revision: this.value.settings.revision,
      action,
    };
    this.pending = body;
    try {
      const outcome = await this.client.request<Model<"AgentExecutionOutcome">>(
        "compute",
        body,
      );
      if (outcome.error) this.error = outcome.error;
      if (["completed", "failed"].includes(outcome.state)) this.pending = null;
    } catch (cause) {
      if (
        cause instanceof ApiError &&
        [
          "invalid_agent_execution",
          "agent_selection_changed",
          "agent_execution_busy",
          "agent_execution_denied",
        ].includes(cause.code)
      )
        this.pending = null;
      this.error = cause instanceof Error ? cause.message : String(cause);
    } finally {
      this.busy = false;
      await this.refresh();
    }
    // A configure acknowledgement must settle before a dependent operation.
    if (action.action === "configure") {
      const deadline = Date.now() + 20000;
      while (
        this.pending?.request_id === body.request_id &&
        Date.now() < deadline
      ) {
        await new Promise((resolve) => setTimeout(resolve, 100));
        await this.refresh();
      }
    }
    return !this.pending && !this.error;
  }
}

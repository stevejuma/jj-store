import { watchAgentChanges } from "./agent-freshness.js";
import { RetainedReads } from "./retained-reads.js";
import type { ApplicationFreshness } from "./application-freshness.js";
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
  modelPicking = $state(false);
  modelPickerError = $state("");
  #picker?: AbortController;
  #lifetime = new AbortController();
  readonly #reads = new RetainedReads(() => this.refresh());
  readonly #watch: ReturnType<typeof watchAgentChanges>;
  #active = false;
  constructor(
    readonly client: LocalClient,
    readonly freshness?: ApplicationFreshness,
  ) {
    this.#watch = watchAgentChanges(freshness, {
      active: () => this.#active,
      reset: () => this.#reads.invalidate(),
      matches: (c) => {
        const matches = c.repository === null && c.domain === "agent_execution";
        if (matches) this.#reads.invalidate();
        return matches;
      },
      refresh: async () => (await this.#reads.ensure("compute")) !== false,
    });
  }
  destroy() {
    this.#lifetime.abort();
    this.stop();
    this.#watch.stop();
  }
  start() {
    this.#active = true;
    void this.#reads.ensure("compute");
    this.#watch.request();
  }
  stop() {
    this.#active = false;
    this.#picker?.abort();
    this.#picker = undefined;
    this.modelPicking = false;
  }
  async pickModelFile() {
    if (
      !this.#active ||
      !this.value?.model_file_picker ||
      this.modelPicking ||
      this.busy ||
      this.pending ||
      this.value.state === "serving"
    )
      return null;
    const request = new AbortController();
    this.#picker = request;
    this.modelPicking = true;
    this.modelPickerError = "";
    try {
      const selected = await this.client.request<Model<"ModelFileSelection">>(
        "compute/pick",
        {},
        request.signal,
      );
      if (request.signal.aborted || this.#picker !== request) return null;
      if (
        selected.version !== 1 ||
        (selected.path !== null &&
          (typeof selected.path !== "string" ||
            !selected.path.startsWith("/") ||
            selected.path.includes("\0") ||
            new TextEncoder().encode(selected.path).length > 4096 ||
            !selected.path.toLowerCase().endsWith(".gguf")))
      )
        throw new Error("Invalid model path");
      return selected.path;
    } catch {
      if (!request.signal.aborted && this.#picker === request)
        this.modelPickerError =
          "Could not open the model file picker. Enter an absolute GGUF path or try Browse again.";
      return null;
    } finally {
      if (this.#picker === request) {
        this.#picker = undefined;
        this.modelPicking = false;
      }
    }
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
      return true;
    } catch (cause) {
      this.error = cause instanceof Error ? cause.message : String(cause);
      return false;
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
    let observed = this.freshness?.journal.revision ?? 0;
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
    if (action.action === "configure" && this.freshness?.supported) {
      const deadline = new AbortController();
      const timer = setTimeout(() => deadline.abort(), 20000);
      const signal = AbortSignal.any([deadline.signal, this.#lifetime.signal]);
      try {
        while (
          this.pending?.request_id === body.request_id &&
          !signal.aborted
        ) {
          await this.freshness.journal.wait({
            after: observed,
            matches: (c) =>
              c.repository === null && c.domain === "agent_execution",
            healthy: () => true,
            listen: (listener) => this.freshness!.listen(listener),
            signal,
            delay: 0,
          });
          if (signal.aborted) break;
          observed = this.freshness.journal.revision;
          await this.#reads.ensure("compute");
        }
      } finally {
        clearTimeout(timer);
      }
    }
    return !this.pending && !this.error;
  }
}

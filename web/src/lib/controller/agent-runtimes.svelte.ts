import { ApiError, type Model } from "../api/client.js";
import { LocalClient } from "../api/local-client.js";
export type RuntimeStatus = Model<"AgentRuntimeStatus">;
export type RuntimeCommand = NonNullable<
  import("../api/generated.js").operations["local_agents_runtime_install"]["requestBody"]
>["content"]["application/json"];
export type RuntimeOutcome = Model<"AgentRuntimeOutcome">;
export type Runtime = Model<"AgentRuntime">;
/** Tooling state has independent receipts; it never touches session command state. */
export class AgentRuntimes {
  value = $state<RuntimeStatus | null>(null);
  refreshing = $state(false);
  saving = $state(false);
  error = $state("");
  commandError = $state("");
  pending = $state<string | null>(null);
  outcome = $state<RuntimeOutcome | null>(null);
  #active = false;
  #alive = true;
  #timer?: ReturnType<typeof setTimeout>;
  readonly client: Pick<LocalClient, "request" | "close">;
  constructor(
    readonly available: () => boolean,
    fetcher?: typeof fetch,
    client?: Pick<LocalClient, "request" | "close">,
  ) {
    this.client = client ?? new LocalClient("", fetcher);
  }
  start() {
    this.#active = true;
    void this.refresh();
  }
  stop() {
    this.#active = false;
    clearTimeout(this.#timer);
  }
  async refresh() {
    if (!this.#alive || this.refreshing || !this.available()) return;
    this.refreshing = true;
    try {
      if (this.pending) await this.checkOutcome();
      const value = await this.client.request<RuntimeStatus>("agents/runtimes");
      if (!this.#alive) return;
      this.value = value;
      if (!this.pending && value.pending)
        this.pending = value.pending.request_id;
      this.error = "";
    } catch (cause) {
      if (this.#alive) this.error = errorText(cause);
    } finally {
      this.refreshing = false;
      clearTimeout(this.#timer);
      if (this.#alive && this.#active)
        this.#timer = setTimeout(
          () => void this.refresh(),
          this.pending ? 1000 : 15000,
        );
    }
  }
  async install(runtime: Runtime) {
    if (
      !this.#alive ||
      !this.available() ||
      this.saving ||
      this.pending ||
      this.error ||
      !this.value?.install_supported ||
      !runtime.action ||
      runtime.state === "ready"
    )
      return;
    const body: RuntimeCommand = {
      version: 1,
      request_id: crypto.randomUUID().replaceAll("-", ""),
      runtime_id: runtime.id,
      action: runtime.action,
    };
    this.pending = body.request_id;
    this.saving = true;
    this.commandError = "";
    this.outcome = null;
    try {
      const result = await this.client.request<RuntimeOutcome>(
        "agents/runtimes/install",
        body,
      );
      if (this.#alive) this.accept(result, body.request_id);
    } catch (cause) {
      if (!this.#alive) return;
      // Structured admission refusal proves no install was dispatched. Transport loss does not.
      if (
        cause instanceof ApiError &&
        [
          "invalid_agent_execution",
          "agent_execution_denied",
          "agent_execution_busy",
          "agent_execution_unavailable",
          "agent_execution_limit",
          "agent_request_identity_conflict",
        ].includes(cause.code)
      )
        this.pending = null;
      this.commandError = this.pending
        ? "Installation outcome is unknown. Check the original request; do not install again."
        : errorText(cause);
    } finally {
      this.saving = false;
      if (this.#alive) await this.refresh();
    }
  }
  async checkOutcome() {
    const request = this.pending;
    if (!request || !this.#alive) return;
    try {
      const result = await this.client.request<RuntimeOutcome>(
        `agents/runtimes/outcome?request=${encodeURIComponent(request)}`,
      );
      if (this.#alive) this.accept(result, request);
    } catch (cause) {
      if (this.#alive)
        this.commandError = `Could not check the original installation: ${errorText(cause)}`;
    }
  }
  private accept(result: RuntimeOutcome, request: string) {
    if (result.request_id !== request || result.runtime_id !== "codex")
      throw new Error(
        "Installation receipt did not match the original request.",
      );
    this.outcome = result;
    if (["completed", "failed", "interrupted"].includes(result.state)) {
      this.pending = null;
      this.commandError = result.error ? runtimeError(result.error) : "";
    } else this.commandError = result.error ? runtimeError(result.error) : "";
  }
  destroy() {
    this.#alive = false;
    this.stop();
    this.client.close();
  }
}
function runtimeError(code: string) {
  const messages: Record<string, string> = {
    runtime_receipt_unavailable_restart:
      "Installation finished but its durable receipt is unconfirmed. Restart the local service, then check this original request before continuing.",
    runtime_install_failed_check_network:
      "Installation failed. Check network access to npm, then try again.",
    runtime_install_timeout:
      "Installation timed out. The previous runtime is still selected. Try again when the network is available.",
    runtime_verification_failed:
      "The installed tooling failed verification. The previous runtime is still selected.",
    runtime_install_size_limit:
      "Installation exceeded the device tooling size limit.",
    installation_interrupted_check_again:
      "Installation was interrupted. Check again, then explicitly start a new installation if needed.",
  };
  return (
    messages[code] ??
    `Runtime installation failed (${code}). Check again before retrying.`
  );
}
function errorText(cause: unknown) {
  if (cause instanceof ApiError && cause.code === "agent_execution_busy")
    return "End the connected agent session before installing or updating tooling.";
  return cause instanceof Error ? cause.message : "Runtime status unavailable";
}

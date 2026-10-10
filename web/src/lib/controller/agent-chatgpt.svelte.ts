import { ApiError, type Exact, type Model } from "../api/client.js";
import type { LocalClient } from "../api/local-client.js";
import type { operations } from "../api/generated.js";
export type ChatGptStatus = Model<"AgentChatGptStatus">;
type Command = Exact<
  operations["local_agents_chatgpt_command"]["requestBody"]["content"]["application/json"]
>;
export class AgentChatGpt {
  value = $state<Model<"AgentChatGptStatus"> | null>(null);
  error = $state("");
  busy = $state(false);
  pending = $state<Command | null>(null);
  #alive = true;
  #timer?: ReturnType<typeof setTimeout>;
  #reading?: Promise<boolean>;
  constructor(
    readonly client: LocalClient,
    readonly changed: () => void,
  ) {}
  dispose() {
    this.#alive = false;
    clearTimeout(this.#timer);
  }
  refresh(): Promise<boolean> {
    if (!this.#alive) return Promise.resolve(true);
    return (this.#reading ??= this.read().finally(() => {
      this.#reading = undefined;
    }));
  }
  private async read() {
    try {
      const value =
        await this.client.request<Model<"AgentChatGptStatus">>(
          "agents/chatgpt",
        );
      if (this.#alive) this.accept(value);
      return true;
    } catch (cause) {
      if (this.#alive)
        this.error = cause instanceof Error ? cause.message : String(cause);
      this.schedule();
      return false;
    }
  }
  private schedule() {
    clearTimeout(this.#timer);
    if (
      this.#alive &&
      this.value?.attempts.some((a) => a.state === "authorizing")
    )
      this.#timer = setTimeout(() => void this.refresh(), 1000);
  }
  private accept(value: Model<"AgentChatGptStatus">) {
    if (
      !value ||
      typeof value.supported !== "boolean" ||
      !Array.isArray(value.accounts) ||
      !Array.isArray(value.attempts) ||
      value.attempts.some(
        (a) => !a || typeof a.id !== "string" || typeof a.state !== "string",
      ) ||
      value.accounts.some(
        (a) =>
          !a ||
          typeof a.id !== "string" ||
          typeof a.connected !== "boolean" ||
          typeof a.plan_enabled !== "boolean",
      )
    )
      throw new Error("Invalid ChatGPT account status");
    if (JSON.stringify(this.value?.accounts) !== JSON.stringify(value.accounts))
      this.changed();
    this.value = value;
    this.error = "";
    this.schedule();
  }
  connect(account: string | null = null) {
    if (this.pending) return Promise.resolve(false);
    this.pending = {
      action: "connect",
      request_id: crypto.randomUUID().replaceAll("-", ""),
      account,
    };
    return this.retry();
  }
  cancel(attempt: string) {
    return this.send({ action: "cancel", attempt });
  }
  signOut(account: string) {
    return this.send({ action: "sign_out", account });
  }
  private send(command: Command) {
    if (this.pending) return Promise.resolve(false);
    this.pending = command;
    return this.retry();
  }
  async retry() {
    if (!this.pending || this.busy || !this.#alive) return false;
    this.busy = true;
    this.error = "";
    try {
      const value = await this.client.request<Model<"AgentChatGptStatus">>(
        "agents/chatgpt",
        $state.snapshot(this.pending),
      );
      if (this.#alive) {
        this.accept(value);
        this.pending = null;
      }
      return true;
    } catch (cause) {
      if (this.#alive) {
        this.error = cause instanceof Error ? cause.message : String(cause);
        if (
          cause instanceof ApiError &&
          [
            "invalid_request",
            "access_denied",
            "local_access_denied",
            "invalid_agent_execution",
            "agent_execution_denied",
            "agent_selection_changed",
            "agent_request_identity_conflict",
            "agent_execution_busy",
            "agent_execution_limit",
          ].includes(cause.code)
        )
          this.pending = null;
      }
      return false;
    } finally {
      this.busy = false;
    }
  }
}

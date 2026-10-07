import { isAgentCommandRefusal } from "./agent-execution-errors.js";
import type { Model } from "../api/client.js";
import type { LocalClient } from "../api/local-client.js";
export type AgentSettingsValue = Model<"AgentSettings">;
export type AgentEnvironment = NonNullable<
  Model<"AgentRuntimeOptions">["environment"]
>;
export class AgentSettings {
  value = $state<AgentSettingsValue | null>(null);
  draft = $state<AgentSettingsValue | null>(null);
  secretDrafts = $state<Record<string, string>>({});
  setupSecretDrafts = $state<Record<string, Record<string, string>>>({});
  busy = $state(false);
  error = $state("");
  pending = $state<Record<string, unknown> | null>(null);
  #reading = false;
  constructor(readonly client: LocalClient) {}
  dispose() {
    this.draft = null;
    this.secretDrafts = {};
    this.setupSecretDrafts = {};
  }
  async refresh() {
    if (this.#reading) return;
    this.#reading = true;
    try {
      this.value =
        await this.client.request<AgentSettingsValue>("agents/settings");
    } catch (cause) {
      this.error = cause instanceof Error ? cause.message : String(cause);
    } finally {
      this.#reading = false;
    }
  }
  async saveSecrets(values: Record<string, string>) {
    for (const [id, value] of Object.entries(values)) {
      if (!value) continue;
      await this.client.request("agents/secrets", { id, value });
      delete values[id];
    }
  }
  async save(value: AgentSettingsValue, secrets: Record<string, string> = {}) {
    if (this.busy || this.pending) return false;
    this.busy = true;
    this.error = "";
    try {
      await this.saveSecrets(secrets);
      this.pending = {
        version: 1,
        request_id: crypto.randomUUID().replaceAll("-", ""),
        expected_revision: value.revision,
        settings: structuredClone($state.snapshot(value)),
      };
      return await this.retry();
    } catch (cause) {
      this.error = cause instanceof Error ? cause.message : String(cause);
      return false;
    } finally {
      this.busy = false;
    }
  }
  async retry() {
    if (!this.pending) return false;
    try {
      this.value = await this.client.request<AgentSettingsValue>(
        "agents/settings",
        this.pending,
      );
      this.pending = null;
      this.error = "";
      return true;
    } catch (cause) {
      this.error = cause instanceof Error ? cause.message : String(cause);
      if (isAgentCommandRefusal(cause)) this.pending = null;
      return false;
    }
  }
}

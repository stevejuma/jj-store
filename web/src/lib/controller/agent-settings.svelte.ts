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
  #reading?: Promise<boolean>;
  #alive = true;
  #mentionRevision: AgentSettingsValue["revision"] | null = null;
  constructor(readonly client: LocalClient) {}
  dispose() {
    this.#alive = false;
    this.draft = null;
    this.secretDrafts = {};
    this.setupSecretDrafts = {};
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
        await this.client.request<AgentSettingsValue>("agents/settings");
      if (this.#alive) this.accept(value);
      return true;
    } catch (cause) {
      if (this.#alive)
        this.error = cause instanceof Error ? cause.message : String(cause);
      return false;
    }
  }
  private accept(value: AgentSettingsValue) {
    if (this.value && BigInt(value.revision) < BigInt(this.value.revision))
      return;
    this.value = value;
    if (this.draft)
      this.draft.automatically_mention = value.automatically_mention;
  }
  async setAutomaticallyMention(enabled: boolean) {
    if (!this.value || this.busy || this.pending) return false;
    if (this.value.automatically_mention === enabled) return true;
    this.#mentionRevision = this.value.revision;
    return this.save({
      ...$state.snapshot(this.value),
      automatically_mention: enabled,
    });
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
      const value = await this.client.request<AgentSettingsValue>(
        "agents/settings",
        this.pending,
      );
      this.accept(value);
      // Only our preference-only write can safely advance an unrelated draft.
      if (this.draft && this.draft.revision === this.#mentionRevision)
        this.draft.revision = value.revision;
      this.#mentionRevision = null;
      this.pending = null;
      this.error = "";
      return true;
    } catch (cause) {
      this.error = cause instanceof Error ? cause.message : String(cause);
      if (isAgentCommandRefusal(cause)) {
        this.pending = null;
        this.#mentionRevision = null;
      }
      return false;
    }
  }
}

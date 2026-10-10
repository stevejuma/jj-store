import type { ApplicationFreshness } from "./application-freshness.js";
import type { AgentExecution } from "./agent-execution.svelte.js";
import type { AgentStatus, AgentTarget } from "./agent-execution-types.js";
import { watchAgentChanges } from "./agent-freshness.js";
import { syncAgentPower } from "./agent-power.js";
import { RetainedReads } from "./retained-reads.js";
/** Installation readers share the application's feed and keep separate invalidation scopes. */
export class AgentExecutionRefresh {
  #alive = true;
  #reads = new Map<string, Promise<boolean>>();
  readonly retained = new RetainedReads((key) =>
    key === "status"
      ? this.status()
      : key === "settings"
        ? this.settings()
        : this.targets(),
  );
  #watches: ReturnType<typeof watchAgentChanges>[];
  constructor(
    private manager: AgentExecution,
    freshness?: ApplicationFreshness,
  ) {
    const active = () => this.#alive && manager.available();
    this.#watches = [
      watchAgentChanges(freshness, {
        active,
        refresh: () =>
          this.retained.ensure("status").then((ok) => ok !== false),
        matches: (c) =>
          this.invalidateWhen(
            "status",
            (c.repository === null &&
              ["agent_execution", "discovery", "workspaces"].includes(
                c.domain,
              )) ||
              c.domain === "repository",
          ),
      }),
      watchAgentChanges(freshness, {
        active,
        refresh: () =>
          this.retained.ensure("settings").then((ok) => ok !== false),
        matches: (c) =>
          this.invalidateWhen(
            "settings",
            c.repository === null && c.domain === "agent_settings",
          ),
      }),
      watchAgentChanges(freshness, {
        active,
        refresh: () =>
          this.retained.ensure("targets").then((ok) => ok !== false),
        matches: (c) =>
          this.invalidateWhen(
            "targets",
            ["discovery", "workspaces", "repository", "code"].includes(
              c.domain,
            ),
          ),
        reset: () => this.retained.invalidate(),
      }),
    ];
  }
  private invalidateWhen(key: string, matches: boolean) {
    if (matches) this.retained.invalidate(key);
    return matches;
  }
  ensure() {
    return Promise.all(
      ["status", "settings", "targets"].map((key) => this.retained.ensure(key)),
    );
  }
  async refresh() {
    this.retained.invalidate();
    const results = await Promise.all([
      this.retained.ensure("status"),
      this.retained.ensure("settings"),
      this.retained.ensure("targets"),
    ]);
    results.forEach((accepted, index) => {
      if (!accepted) this.#watches[index]!.request();
    });
  }
  private read(key: string, load: () => Promise<boolean>) {
    if (!this.#alive || !this.manager.available()) return Promise.resolve(true);
    const current = this.#reads.get(key);
    if (current) return current;
    const read = load().finally(() => this.#reads.delete(key));
    this.#reads.set(key, read);
    return read;
  }
  status() {
    return this.read("status", async () => {
      const m = this.manager;
      try {
        const status = await m.client.request<AgentStatus>("agents");
        if (!this.#alive) return true;
        m.value = status;
        void syncAgentPower(status, m.settings.value?.keep_awake === true);
        if (m.pending) await m.checkOutcome();
        if (!this.#alive) return true;
        m.connectionError = "";
        m.invalidateHistory();
        m.chat.invalidate();
        return true;
      } catch (cause) {
        if (!this.#alive) return true;
        void syncAgentPower(null, false);
        this.retained.invalidate("status");
        m.connectionError =
          cause instanceof Error
            ? cause.message
            : "Agent execution unavailable";
        this.#watches[0]!.request();
        return false;
      }
    });
  }
  settings() {
    return this.read("settings", async () => {
      const accepted = await this.manager.settings.refresh();
      void this.manager.chatgpt.refresh();
      if (this.#alive && accepted)
        void syncAgentPower(
          this.manager.connectionError ? null : this.manager.value,
          this.manager.settings.value?.keep_awake === true,
        );
      return accepted;
    });
  }
  targets() {
    return this.read("targets", async () => {
      try {
        const targets =
          await this.manager.client.request<AgentTarget[]>("agents/targets");
        if (!this.#alive) return true;
        this.manager.targets = targets;
        this.manager.targetError = "";
        return true;
      } catch {
        if (!this.#alive) return true;
        this.manager.targets = [];
        this.manager.targetError =
          "Repository workspaces unavailable. Retry to refresh selections.";
        return false;
      }
    });
  }
  destroy() {
    this.#alive = false;
    for (const watch of this.#watches) watch.stop();
  }
}

import type { LocalClient } from "../api/local-client.js";
import type { Model } from "../api/client.js";
/** Editor-only no-prompt discovery; failure never changes agent/session status. */
export class AgentModelCatalog {
  values = $state<Record<string, Model<"AgentExecutionCatalog">>>({});
  errors = $state<Record<string, string>>({});
  loading = $state<Record<string, boolean>>({});
  #alive = true;
  #loadedAt = new Map<string, number>();
  constructor(readonly client: LocalClient) {}
  async load(runner: string, retry = false) {
    if (
      !this.#alive ||
      this.loading[runner] ||
      (!retry &&
        (this.errors[runner] ||
          Date.now() - (this.#loadedAt.get(runner) ?? 0) < 300000))
    )
      return;
    this.loading[runner] = true;
    this.errors[runner] = "";
    try {
      const value = await this.client.request<Model<"AgentExecutionCatalog">>(
        `agents/catalog?runner=${encodeURIComponent(runner)}`,
      );
      if (value.runner !== runner)
        throw new Error("Model catalog did not match the selected harness.");
      if (this.#alive) {
        this.values[runner] = value;
        this.#loadedAt.set(runner, Date.now());
      }
    } catch {
      if (this.#alive)
        this.errors[runner] =
          "Models could not be loaded. Check the runtime and try again.";
    } finally {
      if (this.#alive) this.loading[runner] = false;
    }
  }
  destroy() {
    this.#alive = false;
  }
}

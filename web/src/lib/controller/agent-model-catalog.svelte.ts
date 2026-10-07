import type { LocalClient } from "../api/local-client.js";
import type { Model } from "../api/client.js";
/** Editor-only no-prompt discovery; failure never changes agent/session status. */
export class AgentModelCatalog {
  values = $state<Record<string, Model<"AgentExecutionCatalog">>>({});
  errors = $state<Record<string, string>>({});
  loading = $state<Record<string, boolean>>({});
  #alive = true;
  #epoch = 0;
  #loadedAt = new Map<string, number>();
  constructor(readonly client: LocalClient) {}
  key(runner: string, setup?: string) {
    return setup ? `${runner}:${setup}` : runner;
  }
  invalidate() {
    this.#epoch++;
    this.values = {};
    this.errors = {};
    this.loading = {};
    this.#loadedAt.clear();
  }
  async load(runner: string, retry = false, setup?: string) {
    const key = this.key(runner, setup);
    if (
      !this.#alive ||
      this.loading[key] ||
      (!retry &&
        (this.errors[key] ||
          Date.now() - (this.#loadedAt.get(key) ?? 0) < 300000))
    )
      return;
    this.loading[key] = true;
    this.errors[key] = "";
    const epoch = this.#epoch;
    try {
      const value = await this.client.request<Model<"AgentExecutionCatalog">>(
        `agents/catalog?runner=${encodeURIComponent(runner)}${setup ? `&setup=${encodeURIComponent(setup)}` : ""}`,
      );
      if (value.runner !== runner)
        throw new Error("Model catalog did not match the selected harness.");
      if (this.#alive && epoch === this.#epoch) {
        this.values[key] = value;
        this.#loadedAt.set(key, Date.now());
      }
    } catch {
      if (this.#alive && epoch === this.#epoch)
        this.errors[key] =
          "Models could not be loaded. Check the runtime and try again.";
    } finally {
      if (this.#alive && epoch === this.#epoch) this.loading[key] = false;
    }
  }
  destroy() {
    this.#alive = false;
  }
}

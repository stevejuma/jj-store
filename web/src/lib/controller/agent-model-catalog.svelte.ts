import type { LocalClient } from "../api/local-client.js";
import type { Model } from "../api/client.js";
/** Editor-only no-prompt discovery; failure never changes agent/session status. */
export class AgentModelCatalog {
  values = $state<Record<string, Model<"AgentExecutionCatalog">>>({});
  errors = $state<Record<string, string>>({});
  loading = $state<Record<string, boolean>>({});
  #previews = new Map<
    string,
    { signature: string; controller: AbortController }
  >();
  #alive = true;
  #epoch = 0;
  #loadedAt = new Map<string, number>();
  constructor(readonly client: LocalClient) {}
  key(runner: string, setup?: string) {
    return setup ? `${runner}:${setup}` : runner;
  }
  invalidate() {
    for (const preview of this.#previews.values()) preview.controller.abort();
    this.#previews.clear();
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
  async preview(
    scope: string,
    draft: import("./agent-configuration.js").AgentConfiguration,
    secrets: Record<string, string>,
    generation: string,
    retry = false,
  ) {
    const used = new Set(
      Object.values(draft.options.environment ?? {}).flatMap((entry) =>
        entry.source === "keychain" ? [entry.value] : [],
      ),
    );
    const body: import("../api/generated.js").operations["local_agents_catalog_preview"]["requestBody"]["content"]["application/json"] =
      {
        version: 1,
        runner: draft.runner,
        preferences: structuredClone(draft.preferences),
        options: structuredClone(draft.options),
        secrets: Object.fromEntries(
          Object.entries(secrets).filter(
            ([id, value]) => used.has(id) && !!value,
          ),
        ),
      };
    const signature = JSON.stringify([body, generation]);
    const previous = this.#previews.get(scope);
    if (!this.#alive || (!retry && previous?.signature === signature)) return;
    previous?.controller.abort();
    const controller = new AbortController();
    this.#previews.set(scope, { signature, controller });
    delete this.values[scope];
    this.errors[scope] = "";
    this.loading[scope] = true;
    try {
      const result = await this.client.request<Model<"AgentExecutionCatalog">>(
        "agents/catalog",
        body,
        controller.signal,
      );
      if (result.runner !== draft.runner) throw new Error("Mismatched catalog");
      if (!controller.signal.aborted && this.#alive)
        this.values[scope] = result;
    } catch {
      if (!controller.signal.aborted && this.#alive)
        this.errors[scope] =
          "Models could not be loaded. Check the provider configuration and try again.";
    } finally {
      if (!controller.signal.aborted && this.#alive)
        this.loading[scope] = false;
    }
  }
  cancelPreview(scope: string) {
    this.#previews.get(scope)?.controller.abort();
    this.#previews.delete(scope);
    delete this.values[scope];
    delete this.errors[scope];
    delete this.loading[scope];
  }
  destroy() {
    for (const preview of this.#previews.values()) preview.controller.abort();
    this.#previews.clear();
    this.#alive = false;
  }
}

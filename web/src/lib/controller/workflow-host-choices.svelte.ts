import { LocalClient } from "../api/local-client.js";
import type { Model } from "../api/client.js";
/** Read-only attachment discovery; selection never registers or grants access. */
export function createWorkflowHostChoices(options: {
  available(): boolean;
  client?: Pick<LocalClient, "request" | "close">;
}) {
  const client = options.client ?? new LocalClient("");
  const state = $state({
    bindings: [] as Model<"LocalBinding">[],
    loading: false,
    error: "",
  });
  let closed = false,
    pending: Promise<void> | undefined;
  function refresh() {
    if (closed || !options.available()) return Promise.resolve();
    if (pending) return pending;
    state.loading = true;
    pending = (async () => {
      try {
        const value =
          await client.request<Model<"LocalBindings">>("repositories");
        if (
          !Array.isArray(value.bindings) ||
          value.bindings.length > 256 ||
          value.bindings.some(
            (b) =>
              !/^[0-9a-f]{32}$/.test(b.repository) || !b.path.startsWith("/"),
          )
        )
          throw Error("Invalid attachment directory");
        if (!closed) {
          state.bindings = value.bindings;
          state.error = "";
        }
      } catch {
        if (!closed)
          state.error =
            "Repository attachments could not be loaded. Your host configuration is retained.";
      } finally {
        state.loading = false;
        pending = undefined;
      }
    })();
    return pending;
  }
  return {
    state,
    refresh,
    destroy() {
      closed = true;
      client.close();
    },
  };
}
export type WorkflowHostChoices = ReturnType<typeof createWorkflowHostChoices>;

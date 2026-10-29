import type { StoreController } from "../../web/src/lib/controller/contracts.js";
import type { StackWorkspaceController } from "../../web/src/lib/controller/stack-workspace/controller.svelte.js";
/** Retain the controller's live getters while adapting a fixture-owned binding. */
export function applicationController(
  controller: StoreController,
  workspace: () => StackWorkspaceController | undefined,
): StoreController {
  return new Proxy(controller, {
    get(target, key) {
      return key === "stackWorkspace"
        ? (workspace() ?? target.stackWorkspace)
        : Reflect.get(target, key, target);
    },
  });
}

import { untrack } from "svelte";
import type { StoreController } from "../controller/contracts.js";
import type { NotificationContext } from "./notification-context.svelte.js";

/** Recovery follows controller state even while its presentation is unmounted. */
export function observeNotificationRecovery(
  context: NotificationContext,
  controller: () => StoreController,
) {
  const observed = new Map<string, Set<string>>();
  $effect(() => {
    const { settings, status } = controller().state;
    const conditions: [string, string | null | undefined, boolean][] = [
      [
        "repository-request",
        status.error,
        status.navigation === "loading" || status.navigation === "refreshing",
      ],
      [
        "graph-index",
        settings.indexPreparationError,
        settings.indexPreparation === "loading",
      ],
      [
        "search-health",
        settings.searchStatusError,
        settings.searchStatusState === "loading" ||
          settings.searchStatusState === "refreshing",
      ],
      [
        "search-settings",
        settings.searchSettings.error || settings.searchSettings.value?.error,
        settings.searchSettings.changing,
      ],
      [
        "storage",
        settings.storageSettings.error,
        settings.storageSettings.changing ||
          !!settings.storageSettings.pending ||
          ["loading", "refreshing"].includes(settings.storageSettings.state),
      ],
      [
        "sync",
        settings.syncSettings.error,
        settings.syncSettings.changing ||
          ["loading", "refreshing"].includes(settings.syncSettings.state),
      ],
      [
        "sync-wait",
        settings.syncSettings.wait.error,
        ["capturing", "waiting"].includes(settings.syncSettings.wait.state),
      ],
      [
        "folder-picker",
        settings.workspacePickerError,
        settings.workspacePicking,
      ],
      [
        "folders",
        settings.workspaceLinksError,
        settings.workspaceLinking ||
          ["loading", "refreshing"].includes(settings.workspaceLinksState),
      ],
    ];
    const scope = context.scope;
    untrack(() => {
      for (const [key, message, pending] of conditions) {
        if (message) {
          const scopes = observed.get(key) ?? new Set<string>();
          scopes.add(scope);
          observed.set(key, scopes);
        } else if (!pending) {
          for (const previous of observed.get(key) ?? [])
            context.service.resolve(previous, key);
          context.service.resolve(scope, key);
          observed.delete(key);
        }
      }
    });
  });
}

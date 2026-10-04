import { untrack } from "svelte";
import type { StoreController } from "../controller/contracts.js";
import type { NotificationContext } from "./notification-context.svelte.js";

/** Recovery follows controller state even while its presentation is unmounted. */
export function observeNotificationRecovery(
  context: NotificationContext,
  controller: () => StoreController,
) {
  const directoryKeys = new Set<string>();
  $effect(() => {
    const entries = (controller().projects?.repositories.entries ?? []).map(
      (entry) => ({
        key: `repository-summary:${entry.repository.id}`,
        error: entry.error,
        loading: entry.loading,
      }),
    );
    const current = new Set(entries.map((entry) => entry.key));
    untrack(() => {
      for (const key of directoryKeys)
        if (!current.has(key))
          context.service.resolve("repository-directory", key);
      directoryKeys.clear();
      for (const entry of entries) {
        const key = entry.key;
        directoryKeys.add(key);
        if (!entry.error && !entry.loading)
          context.service.resolve("repository-directory", key);
      }
    });
  });
  $effect(() => {
    const state = controller().capture?.state;
    const pending = state?.busy || !!state?.pending;
    if (!state?.error && !pending)
      untrack(() =>
        context.service.resolve("installation-capture", "capture-request"),
      );
  });
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

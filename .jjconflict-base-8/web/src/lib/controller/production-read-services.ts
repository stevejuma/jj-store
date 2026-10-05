import { watchCodeIndexRecovery } from "./managed-index-recovery.svelte.js";
import { createProductionStoreSearch } from "./production-store-search-controller.js";
import { createProductionStoreSettings } from "./production-store-index.js";
export function createProductionReadServices(
  options: Omit<
    Parameters<typeof createProductionStoreSettings>[0],
    "route" | "operation" | "restore"
  > & {
    history: Parameters<typeof createProductionStoreSearch>[0]["history"];
    restoreRoute(): Promise<void>;
    restore(): Promise<void>;
  },
) {
  const settings = createProductionStoreSettings({
    ...options,
    route: () => options.history.route,
  });
  watchCodeIndexRecovery(() => settings.recoverCodeIndex());
  return {
    search: createProductionStoreSearch({
      data: options.data,
      history: options.history,
      source: options.source,
      isCurrent: (active) => options.current(options.version(), active),
      restoreRoute: options.restoreRoute,
    }),
    settings,
  };
}

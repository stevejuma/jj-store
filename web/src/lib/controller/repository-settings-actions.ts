import type { createStoreRouteHistory } from "./store-route-history.svelte.js";
export function repositorySettingsActions(
  history: ReturnType<typeof createStoreRouteHistory>,
  restore: () => Promise<void>,
) {
  return {
    setNavigationGuard: history.setNavigationGuard,
    openRepositorySettings: history.repositorySettings.open,
    selectRepositorySettings: history.repositorySettings.select,
    async leaveStacks() {
      history.stacks.leave();
      await restore();
    },
    async leaveRepositorySettings() {
      history.repositorySettings.leave();
      await restore();
    },
  };
}

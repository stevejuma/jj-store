import type { StoreController } from "../../controller/contracts.js";

export function observeRepositoryThreadNavigation(
  controller: () => StoreController,
) {
  $effect(() => {
    const current = controller();
    const projects = current.projects?.navigation;
    if (!current.freshness || !projects) return;
    const watch = current.freshness.watch({
      initial: false,
      active: () => projects.visible,
      reset: () => projects.invalidate(),
      matches: (change) => {
        if (!["work", "repository", "discovery"].includes(change.domain))
          return false;
        projects.invalidate(
          change.repository ? new Set([change.repository]) : undefined,
        );
        return true;
      },
      refresh: async () => {
        await projects.load();
        return !projects.dirty && !projects.homes.some((home) => home.error);
      },
    });
    return watch.stop;
  });
  $effect(() => {
    const current = controller();
    if (!current.freshness || !current.threadNavigation) return;
    const watch = current.freshness.watch({
      initial: false,
      active: () => !!current.threadNavigation!.visible.size,
      reset: () => current.threadNavigation!.invalidate(),
      matches: (change) => {
        if (
          ["threads", "repository"].includes(change.domain) &&
          change.repository
        ) {
          current.threadNavigation!.invalidate(new Set([change.repository]));
          return true;
        }
        if (
          change.repository === null &&
          ["repository", "discovery"].includes(change.domain)
        ) {
          current.threadNavigation!.invalidate();
          return true;
        }
        return false;
      },
      refresh: async () => {
        const batch = current.threadNavigation!.visible;
        await current.threadNavigation!.load(false, batch);
        const failed = Object.entries(current.threadNavigation!.entries).some(
          ([id, entry]) => batch.has(id) && entry.error,
        );
        return !failed && !current.threadNavigation!.hasDirty(batch);
      },
    });
    return watch.stop;
  });
}

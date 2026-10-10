import type { StoreController } from "./contracts.js";
import { createAutomaticRefresh } from "./automatic-refresh.js";
export type SettingsRefreshScope =
  "repository" | "storage" | "sync" | "search" | "search_health";
/** The rendered settings boundary owns its reads; unrelated panels remain dormant. */
export function observeStoreSettings(
  controller: StoreController,
  section: string,
) {
  const repository = controller.state.route.repository;
  const domain =
    section === "search_health"
      ? "search"
      : section === "repository"
        ? "workspaces"
        : section;
  const configuration = section !== "search" && section !== "search_health";
  let configurationRevision = configuration ? 1 : 0,
    sectionRevision = 1;
  let acceptedConfiguration = 0,
    acceptedSection = 0;
  const refresh = async () => {
    if (controller.freshness && !controller.freshness.healthy) {
      configurationRevision++;
      sectionRevision++;
    }
    const config = configurationRevision,
      selected = sectionRevision;
    if (config > acceptedConfiguration)
      await controller.configuration?.observe();
    if (
      selected > acceptedSection &&
      controller.state.status.connection === "connected" &&
      ["repository", "storage", "sync", "search", "search_health"].includes(
        section,
      )
    )
      await controller.actions.refreshSettings(section as SettingsRefreshScope);
    const successful =
      (!configuration || !controller.configuration?.state.error) &&
      ![
        controller.state.settings.storageSettings.state,
        controller.state.settings.syncSettings.state,
        controller.state.settings.searchSettings.state,
        controller.state.settings.searchStatusState,
        controller.state.settings.workspaceLinksState,
      ]
        .filter(
          (_, index) =>
            index ===
            (
              {
                storage: 0,
                sync: 1,
                search: 2,
                search_health: 3,
                repository: 4,
              } as Record<string, number>
            )[section],
        )
        .includes("failed");
    if (successful) {
      acceptedConfiguration = config;
      acceptedSection = selected;
    }
    return successful;
  };
  if (controller.freshness) {
    const watch = controller.freshness.watch({
      reset: () => {
        configurationRevision++;
        sectionRevision++;
      },
      matches: (change) => {
        const config = configuration && change.domain === "configuration";
        const selected =
          (change.repository === null || change.repository === repository) &&
          ["repository", domain].includes(change.domain);
        if (config) configurationRevision++;
        if (selected || config) sectionRevision++;
        return config || selected;
      },
      refresh,
    });
    return watch.stop;
  }
  const abort = new AbortController();
  const automatic = createAutomaticRefresh({
    active: () => true,
    refresh: () => {
      configurationRevision++;
      sectionRevision++;
      return refresh();
    },

    report: () => {},
    signal: abort.signal,
  });
  automatic.request();
  return () => abort.abort();
}

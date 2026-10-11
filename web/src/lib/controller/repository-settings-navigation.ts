import type { StoreRoute } from "./store-route-types.js";
import { storePageRoute } from "./store-stacks-navigation.js";

export const repositorySettingsTabs = {
  general: [],
  folders: [],
  search: ["configuration", "scopes", "advanced", "health"],
  storage: ["overview", "maintenance"],
  sync: ["overview", "progress", "configuration"],
  advanced: [],
} as const;
export type RepositorySettingsSection = keyof typeof repositorySettingsTabs;
export type RepositorySettingsTab =
  (typeof repositorySettingsTabs)[RepositorySettingsSection][number];
export function validRepositorySettings(section: string, tab?: string | null) {
  if (!Object.hasOwn(repositorySettingsTabs, section)) return false;
  return (
    !tab ||
    (
      repositorySettingsTabs[
        section as RepositorySettingsSection
      ] as readonly string[]
    ).includes(tab)
  );
}

/** Session navigation owns the return route; configuration remains controller-owned. */
export function createRepositorySettingsNavigation(options: {
  route(): StoreRoute;
  push(route: StoreRoute): void;
  replace(route: StoreRoute): void;
}) {
  const returns = new Map<string, StoreRoute>();
  return {
    open(section: RepositorySettingsSection = "general") {
      const current = options.route();
      if (
        !current.repository ||
        current.projectsHub ||
        current.agentsHub ||
        current.threadsHub
      )
        return;
      if (current.page !== "settings")
        returns.set(current.repository, { ...current });
      options.push({
        ...storePageRoute(current, "settings"),
        workspace: null,
        repositorySettingsSection: section,
      });
    },
    select(
      section: RepositorySettingsSection,
      tab?: RepositorySettingsTab,
      replace = false,
    ) {
      if (
        options.route().page !== "settings" ||
        !validRepositorySettings(section, tab)
      )
        return;
      options[replace ? "replace" : "push"]({
        ...options.route(),
        repositorySettingsSection: section,
        repositorySettingsTab: tab,
      });
    },
    leave() {
      const current = options.route();
      if (current.page !== "settings") return;
      options.push(
        returns.get(current.repository) ?? {
          ...storePageRoute(current, "files"),
          workspace: null,
        },
      );
    },
  };
}

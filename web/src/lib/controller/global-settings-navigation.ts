import type { StoreRoute } from "./store-route-types.js";

export const globalSettingsTabs = {
  profile: ["details"],
  "custom-emoji": ["library"],
  appearance: ["theme", "preferences"],
  service: ["overview", "limits", "launch", "protection", "storage"],
  agents: [
    "runtimes",
    "capture",
    "configuration",
    "sources",
    "imports",
    "diagnostics",
  ],
  compute: ["models", "resources", "sharing"],
} as const;
export type GlobalSettingsSection = keyof typeof globalSettingsTabs;
export type GlobalSettingsTab =
  (typeof globalSettingsTabs)[GlobalSettingsSection][number];
export type GlobalSettingsDestination = {
  section: GlobalSettingsSection;
  tab: GlobalSettingsTab;
};
export function globalSettingsDestination(
  section: string,
  tab?: string | null,
): GlobalSettingsDestination | null {
  if (!Object.hasOwn(globalSettingsTabs, section)) return null;
  const selected = section as GlobalSettingsSection;
  const tabs: readonly string[] = globalSettingsTabs[selected];
  if (tab != null && !tabs.includes(tab)) return null;
  return { section: selected, tab: (tab ?? tabs[0]) as GlobalSettingsTab };
}
export function withoutGlobalSettings(route: StoreRoute): StoreRoute {
  const { globalSettings: _, ...rest } = route;
  return rest;
}
/** Keep the exact underlying route in session history, never in device storage. */
export function createGlobalSettingsNavigation(options: {
  route(): StoreRoute;
  push(route: StoreRoute): void;
  replace(route: StoreRoute): void;
}) {
  return {
    open(section: GlobalSettingsSection = "service", tab?: string) {
      const destination = globalSettingsDestination(section, tab);
      if (destination)
        options.push({ ...options.route(), globalSettings: destination });
    },
    select(section: GlobalSettingsSection, tab?: string, replace = false) {
      if (!options.route().globalSettings) return;
      const destination = globalSettingsDestination(section, tab);
      if (destination)
        options[replace ? "replace" : "push"]({
          ...options.route(),
          globalSettings: destination,
        });
    },
    leave() {
      if (options.route().globalSettings)
        options.push(withoutGlobalSettings(options.route()));
    },
  };
}

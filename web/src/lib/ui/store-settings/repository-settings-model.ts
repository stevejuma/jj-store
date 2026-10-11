import type { StoreController } from "../../controller/contracts.js";
import type {
  RepositorySettingsSection,
  RepositorySettingsTab,
} from "../../controller/repository-settings-navigation.js";

export function repositorySettingsSections(view: StoreController["state"]) {
  const capabilities = view.settings.capabilities;
  const admin =
    view.accessMode === "local_trusted" &&
    view.repository.permission === "admin";
  const native = view.repository.profile !== "git_sha1_jj045";
  return [
    { value: "general", label: "General" },
    ...((!native && view.accessMode === "local_trusted") ||
    (admin && capabilities?.local_workspace_links)
      ? [{ value: "folders", label: "Local folders" }]
      : []),
    ...(capabilities?.search || (admin && capabilities?.local_search_management)
      ? [{ value: "search", label: "Search" }]
      : []),
    ...(native && admin && capabilities?.local_storage_management
      ? [{ value: "storage", label: "Storage" }]
      : []),
    ...(native && admin && capabilities?.local_sync_management
      ? [{ value: "sync", label: "Sync" }]
      : []),
    { value: "advanced", label: "Advanced" },
  ] as readonly { value: RepositorySettingsSection; label: string }[];
}
export function supportedSettingsTabs(
  view: StoreController["state"],
  section: RepositorySettingsSection,
  configuration = false,
) {
  const capabilities = view.settings.capabilities;
  const admin =
    view.accessMode === "local_trusted" &&
    view.repository.permission === "admin";
  if (section === "search")
    return [
      ...(admin && capabilities?.local_search_management
        ? ["configuration", "scopes", "advanced"]
        : []),
      ...(capabilities?.search ? ["health"] : []),
    ] as RepositorySettingsTab[];
  if (section === "storage")
    return ["overview", "maintenance"] as RepositorySettingsTab[];
  if (section === "sync")
    return [
      "overview",
      ...(view.settings.syncSettings.value?.worker ? ["progress"] : []),
      ...(configuration ? ["configuration"] : []),
    ] as RepositorySettingsTab[];
  return [];
}
export function supportedSettingsDestination(
  view: StoreController["state"],
  configuration = false,
) {
  const desiredSection = view.route.repositorySettingsSection ?? "general";
  const section = repositorySettingsSections(view).some(
    (s) => s.value === desiredSection,
  )
    ? desiredSection
    : "general";
  const tabs = supportedSettingsTabs(view, section, configuration);
  const desiredTab = view.route.repositorySettingsTab;
  const tab = desiredTab && tabs.includes(desiredTab) ? desiredTab : tabs[0];
  return {
    section,
    tab,
    unsupported:
      section !== desiredSection || Boolean(desiredTab && desiredTab !== tab),
  };
}

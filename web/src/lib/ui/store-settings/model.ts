export type StoreSettingsSection =
  import("../../controller/global-settings-navigation.js").GlobalSettingsSection;
export type StoreSettingsPageSection =
  | StoreSettingsSection
  | "repository"
  | "folders"
  | "advanced"
  | "search"
  | "storage"
  | "sync";

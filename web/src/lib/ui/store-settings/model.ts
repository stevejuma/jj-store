export type StoreSettingsSection =
  "appearance" | "service" | "agents" | "compute";
export type StoreSettingsPageSection =
  | StoreSettingsSection
  | "repository"
  | "folders"
  | "advanced"
  | "search"
  | "storage"
  | "sync";

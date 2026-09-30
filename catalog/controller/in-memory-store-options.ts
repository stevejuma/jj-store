export interface InMemoryStoreControllerOptions {
  large?: boolean;
  mutationOutcome?: "committed" | "unknown";
  mutationFailure?: string;
  reviewOutcome?: "committed" | "unknown";
  settingsSearch?: import("./in-memory-store-settings.svelte.js").CatalogSearchSettings;
}

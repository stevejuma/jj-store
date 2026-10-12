export interface InMemoryStoreControllerOptions {
  accessMode?: "authenticated" | "local_trusted";
  workingCopies?: readonly import("../../web/src/lib/projects/repository-log.js").RepositoryWorkingCopy[];
  large?: boolean;
  mutationOutcome?: "committed" | "unknown";
  mutationFailure?: string;
  reviewOutcome?: "committed" | "unknown";
  settingsSearch?: import("./in-memory-store-settings.svelte.js").CatalogSearchSettings;
}

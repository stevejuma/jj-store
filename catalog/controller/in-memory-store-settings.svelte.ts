import { emptySearchSettings } from "../../web/src/lib/controller/search-settings.js";
import { emptyStorageSettings } from "../../web/src/lib/controller/storage-settings.js";
import { emptySyncSettings } from "../../web/src/lib/controller/sync-settings.js";
import { emptyWorkspaceLinks } from "../../web/src/lib/controller/workspace-link-types.js";
import type {
  StoreController,
  StoreLoadStatus,
  StoreSettingsState,
} from "../../web/src/lib/controller/contracts.js";

type Capabilities = NonNullable<StoreSettingsState["capabilities"]>;
type SearchStatus = NonNullable<StoreSettingsState["searchStatus"]>;

export type CatalogSearchSettings =
  "fresh" | "degraded" | "unavailable" | "failed";

const capabilities = (search: boolean): Capabilities => ({
  installation_directory: false,
  server_connections: false,
  authenticated_agent_execution: false,
  version: 1,
  native_profile: "jj-store-native-v1",
  authentication: "repository_bearer",
  read_api: true,
  browsing_windows: true,
  index_tasks: true,
  events: true,
  search,
  typed_mutations: ["new", "describe", "edit_file"],
  mutation_request_bytes: 1_048_576,
  conflict_upload_bytes: 262_144,
  conflict_resolution_actions: ["select_term", "delete", "uploaded_file"],
  event_limits: {
    streams: 8,
    replay_window: 1_024,
    page: 100,
    poll_millis: 500,
  },
  limits: {
    requests: 16,
    workers: 4,
    deadline_millis: 30_000,
    json_bytes: 1_048_576,
    page: 100,
    index_commits: 5_000,
    inline_index_commits: 250,
    file_bytes: 262_144,
  },
  log_context: {
    at_time_millis: "required",
    workspace: "explicit when needed",
    user_email: "explicit for identity-relative expressions",
  },
});

export function createInMemoryStoreSettings(options: {
  repository: () => string;
  heads: () => readonly string[];
  operation: () => string | null;
  mode?: CatalogSearchSettings;
}) {
  const mode = options.mode ?? "fresh";
  let searchStatusState = $state<StoreLoadStatus>(
    mode === "unavailable"
      ? "unavailable"
      : mode === "failed"
        ? "failed"
        : "ready",
  );
  let searchStatusError = $state(
    mode === "failed" ? "Search status could not be loaded." : "",
  );
  let indexPreparation = $state<StoreLoadStatus>("idle");

  const status = (): SearchStatus | null =>
    mode === "unavailable" || mode === "failed"
      ? null
      : {
          version: 1,
          repository: options.repository(),
          epoch: "e1".repeat(16),
          repository_position: 18,
          durable_position: 18,
          indexed_position: mode === "degraded" ? 15 : 18,
          index_generation: "a4".repeat(32),
          is_stale: mode === "degraded",
          degraded: mode === "degraded",
        };

  return {
    get state(): StoreSettingsState {
      const search = mode !== "unavailable";
      return {
        ...emptyWorkspaceLinks(),
        searchSettings: emptySearchSettings(),
        storageSettings: emptyStorageSettings(),
        syncSettings: emptySyncSettings(),
        capabilities: capabilities(search),
        snapshot: {
          version: 1,
          repo_id: options.repository(),
          epoch: "e1".repeat(16),
          published_seq: 18,
          manifest_revision: 9,
          code_state_version: 6,
          heads: [...options.heads()],
        },
        searchStatus: status(),
        searchStatusState,
        searchStatusError,
        indexPreparation,
        indexPreparationError: "",
        indexTaskStatus: indexPreparation === "loading" ? "running" : null,
        indexOperation: options.operation(),
      };
    },
    async refresh() {
      if (mode === "unavailable") return;
      searchStatusState = "loading";
      await Promise.resolve();
      searchStatusState = mode === "failed" ? "failed" : "ready";
      searchStatusError =
        mode === "failed" ? "Search status could not be loaded." : "";
    },
    async cancelIndex() {
      indexPreparation = "idle";
    },
    async prepareIndex() {
      if (!options.operation()) return;
      indexPreparation = "loading";
      await Promise.resolve();
      indexPreparation = "ready";
    },
  };
}

export const unavailableManagementActions = {
  async captureReconciliation(): ReturnType<
    StoreController["actions"]["captureReconciliation"]
  > {
    throw new Error("Reconciliation capture unavailable");
  },
  async refreshWorkspaceLinks() {},
  setWorkspacePath() {},
  async linkWorkspace() {},
  async pickWorkspaceFolder() {},
  async waitForSync() {},
  stopSyncWait() {},
  async refreshSyncSettings() {},
  async controlSync() {},
  async refreshStorage() {},
  async startStorage() {},
  async retryStorage() {},
  setSearchDraft() {},
  cancelSearchDraft() {},
  async refreshSearchSettings() {},
  async saveSearchSettings() {},
  async controlSearch() {},
};

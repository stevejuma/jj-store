import type { Model, Task } from "../api/client.js";
import type { StoreLoadStatus } from "./contracts.js";
import type {
  RepositoryHead as Head,
  RepositoryCapabilities,
} from "./repository-profile.js";
import type { WorkspaceLinkSettings } from "./workspace-link-types.js";
import type { IndexPreparationKind } from "./native-data-source.js";

export interface StoreSettingsState extends WorkspaceLinkSettings {
  readonly searchSettings: import("./search-settings.js").SearchSettings;
  readonly storageSettings: import("./storage-settings.js").StorageSettings;
  readonly syncSettings: import("./sync-settings.js").SyncSettings;
  readonly capabilities: RepositoryCapabilities | null;
  readonly snapshot: Head | null;
  readonly searchStatus: Model<"SearchStatus"> | null;
  readonly searchStatusState: StoreLoadStatus;
  readonly searchStatusError: string;
  readonly indexPreparation: StoreLoadStatus;
  readonly indexPreparationKind?: IndexPreparationKind;
  readonly indexProgress?: Model<"KindPreparationProgress"> | null;
  readonly indexCancellationUnconfirmed?: boolean;
  readonly indexPreparationError: string;
  readonly indexTaskStatus: Task["status"] | null;
  readonly indexOperation: string | null;
}

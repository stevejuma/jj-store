import type { StoreSearchState } from "./store-search-types.js";
export type {
  StoreSearchResultKind,
  StoreSearchResult,
  StoreSearchState,
} from "./store-search-types.js";
import type { RepositoryProfile } from "./repository-profile.js";
import type { StoreSettingsState } from "./store-settings-types.js";
export type { StoreSettingsState } from "./store-settings-types.js";
import type { FileListingViewMode } from "@lapismd/design-core/diff";
import type { Model } from "../api/client.js";
export type StoreConflictDescriptor =
  import("./repository-read-types.js").CodeConflict;
export type StoreConflictUpload = Model<"ConflictUpload">;
import type { CommunityMemberModel } from "../ui/community-workspace/model.js";
import type {
  RepositoryLogGraph,
  RepositoryLogRevision,
  RepositoryCommitSummary,
  RepositoryWorkingCopy,
} from "../projects/repository-log.js";
import type { Bookmark, RevisionDetail } from "../projects/presentation.js";
import type { StoreMutationState } from "./store-mutation-types.js";
export interface RepositoryBlob {
  path: string;
  contents?: string;
  binary?: boolean;
  too_large?: boolean;
}
export interface RepositorySnapshot {
  pathChanges?: Readonly<Record<string, RepositoryCommitSummary | undefined>>;
  id: string;
  files: readonly RepositoryBlob[];
  directories?: readonly { path: string }[];
}
export interface RepositoryChoice {
  value: string;
  label: string;
  kind: "bookmark" | "change";
  searchTerms?: readonly string[];
  targets: readonly string[];
}
import type { StoreRoute } from "./store-route-types.js";
export type { StoreRoute } from "./store-route-types.js";
export type StoreLoadStatus =
  | "idle"
  | "loading"
  | "refreshing"
  | "ready"
  | "empty"
  | "failed"
  | "unavailable";
export interface StoreEvolutionState {
  readonly graph: RepositoryLogGraph;
  readonly status: StoreLoadStatus;
  readonly error: string;
  readonly canLoadMore: boolean;
  readonly predecessorsUnknown: boolean;
}
export interface StoreControllerStatus {
  readonly connection:
    "disconnected" | "connecting" | "connected" | "denied" | "failed";
  readonly navigation: StoreLoadStatus;
  readonly repositories: StoreLoadStatus;
  readonly bookmarks: StoreLoadStatus;
  readonly graph: StoreLoadStatus;
  readonly files: StoreLoadStatus;
  readonly detail: StoreLoadStatus;
  readonly search: StoreLoadStatus;
  readonly agents: StoreLoadStatus;
  readonly transcript: StoreLoadStatus;
  readonly reviews: StoreLoadStatus;
  readonly reviewAction: StoreLoadStatus;
  readonly live: "disconnected" | "connecting" | "live" | "reconnecting";
  readonly refreshAvailable: boolean;
  readonly error: string;
}
export interface StoreConnection {
  repository: string;
  token: string | null;
}
export interface StoreRepositorySummary {
  readonly id: string;
  readonly name: string;
  readonly profile?: RepositoryProfile;
  readonly availability?: "ready" | "unavailable" | "identity_changed";
  readonly permission: Model<"EffectivePermission"> | null;
  readonly principal?: string;
}
export interface StoreControllerState {
  readonly installationReady?: boolean;
  readonly nativeRepositoryCreation?: boolean;
  readonly repositoryConversion?: boolean;
  readonly accessMode: "bootstrapping" | "authenticated" | "local_trusted";
  readonly storageReady: boolean;
  readonly backups: readonly Model<"BackupStatus">[];
  readonly route: StoreRoute;
  readonly canNavigateBack: boolean;
  readonly canNavigateForward: boolean;
  readonly repositories: readonly StoreRepositorySummary[];
  readonly repository: StoreRepositorySummary;
  readonly operationHeads: readonly string[];
  readonly comparisonParents: readonly string[];
  readonly conflictCount: number | null;
  readonly conflicts: readonly StoreConflictDescriptor[];
  readonly conflictResolutionActions: readonly string[];
  readonly conflictUploadBytes: number | bigint;
  readonly unavailableComparisonCount: number;
  readonly canLoadMoreBookmarks: boolean;
  readonly canLoadMoreGraph: boolean;
  readonly canLoadMoreFiles: boolean;
  readonly canLoadMoreDetail: boolean;
  readonly canLoadMoreAgents: boolean;
  readonly canLoadMoreTranscript: boolean;
  readonly canLoadMoreReviews: boolean;
  readonly fileViewMode: FileListingViewMode;
  readonly collapsedDirectoryPaths: ReadonlySet<string>;
  readonly graph: RepositoryLogGraph;
  readonly evolution: StoreEvolutionState;
  readonly now: number;
  readonly detail: RevisionDetail | undefined;
  readonly fileRevision: RepositoryCommitSummary | undefined;
  readonly fileRevisionStatus?: StoreLoadStatus;
  readonly bookmarks: readonly Bookmark[];
  readonly choices: readonly RepositoryChoice[];
  readonly workingCopiesStatus?: StoreLoadStatus;
  readonly canLoadMoreWorkingCopies?: boolean;
  readonly workingCopies: readonly RepositoryWorkingCopy[];
  readonly targets: readonly string[];
  readonly snapshot: RepositorySnapshot | undefined;
  readonly blob: RepositoryBlob | undefined;
  readonly search: StoreSearchState;
  readonly settings: StoreSettingsState;
  readonly mutation: StoreMutationState;
  readonly agentSessions: readonly Model<"AgentSessionSummary">[];
  readonly agentSession:
    import("./repository-read-types.js").AgentDetail | undefined;
  readonly agentRelations: readonly Model<"AgentRelationLink">[];
  readonly agentTranscript: readonly Model<"AgentTranscriptChunk">[];
  readonly reviewThreads: readonly Model<"ReviewThreadSummary">[];
  readonly reviewThread:
    import("./repository-read-types.js").ReviewDetail | undefined;
  readonly reviewContext?: import("./native-activity-types.js").NativeReviewContext;
  readonly reviewPatch:
    import("./review-anchor-patch.js").ReviewInlinePatch | undefined;
  readonly reviewActionMessage: string;
  readonly reviewActionRequestId: string | null;
  readonly committedReviewRequestId?: string | null;
  readonly authenticated: boolean;
  readonly member: CommunityMemberModel;
  readonly status: StoreControllerStatus;
}
/** Community's state/actions contract shape, scoped to local JJ presentation. */
export interface StoreController {
  readonly blame?: import("./blame-types.js").BlameController;
  /** Workspace binding discovery can continue after repository connection. */
  readonly stackWorkspaceLoading?: boolean;
  readonly stackWorkspace?: import("./stack-workspace/controller.svelte.js").StackWorkspaceController;
  readonly freshness?: import("./application-freshness.js").ApplicationFreshness;
  readonly historyQuery?: import("./history-query-types.js").HistoryQueryController;
  readonly configuration?: import("./server-settings.svelte.js").ServerSettingsManagement;
  readonly capture?: import("./capture-types.js").CaptureManagement;
  readonly referenceSearch?: import("./global-search-types.js").GlobalSearchController;
  readonly globalSearch?: import("./global-search-types.js").GlobalSearchController;
  readonly threads?: import("./thread-types.js").ThreadsController;
  readonly threadNavigation?: import("./repository-thread-navigation.svelte.js").RepositoryThreadNavigation;
  readonly projects?: import("./projects-hub-types.js").ProjectsHubController;
  readonly work?: import("./work-types.js").WorkController;
  readonly state: StoreControllerState;
  readonly actions: import("./controller-actions.js").StoreControllerActions;
}

/** Host-owned transport and routing inputs; defaults retain ordinary browser startup. */
export type ProductionStoreControllerOptions = {
  fetcher?: typeof fetch;
  now?: () => number;
  browser?: import("./store-route-history.svelte.js").StoreRouteBrowser;
};

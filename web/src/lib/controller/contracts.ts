import type {
  StoreSearchResult,
  StoreSearchState,
} from "./store-search-types.js";
export type {
  StoreSearchResultKind,
  StoreSearchResult,
  StoreSearchState,
} from "./store-search-types.js";
import type {
  RepositoryHead as Head,
  RepositoryCapabilities,
  RepositoryProfile,
} from "./repository-profile.js";
import type { WorkspaceLinkSettings } from "./workspace-link-types.js";
import type { FileListingViewMode } from "@lapismd/design-core/diff";
import type { FileDiffLineContext } from "@lapismd/design-core/diff";
import type { Model } from "../api/client.js";
import type { ReviewDetail } from "./repository-read-types.js";
export type StoreConflictDescriptor =
  import("./repository-read-types.js").CodeConflict;
export type StoreConflictUpload = Model<"ConflictUpload">;
import type { CommunityMemberModel } from "../ui/community-workspace/model.js";
import type { StorePage } from "../ui/community-navigation/model.js";
import type {
  RepositoryLogGraph,
  RepositoryLogRevision,
  RepositoryWorkingCopy,
} from "../projects/repository-log.js";
import type { Bookmark, RevisionDetail } from "../projects/presentation.js";
import type {
  StoreMutationCommand,
  StoreMutationState,
} from "./store-mutation-types.js";
export interface RepositoryBlob {
  path: string;
  contents?: string;
  binary?: boolean;
  too_large?: boolean;
}
export interface RepositorySnapshot {
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
export interface StoreSettingsState extends WorkspaceLinkSettings {
  readonly searchSettings: import("./search-settings.js").SearchSettings;
  readonly syncSettings: import("./sync-settings.js").SyncSettings;
  readonly capabilities: RepositoryCapabilities | null;
  readonly snapshot: Head | null;
  readonly searchStatus: Model<"SearchStatus"> | null;
  readonly searchStatusState: StoreLoadStatus;
  readonly searchStatusError: string;
  readonly indexPreparation: StoreLoadStatus;
  readonly indexPreparationError: string;
  readonly indexOperation: string | null;
}
export interface StoreControllerState {
  readonly installationReady?: boolean;
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
  readonly fileRevision: RepositoryLogRevision | undefined;
  readonly bookmarks: readonly Bookmark[];
  readonly choices: readonly RepositoryChoice[];
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
  readonly historyQuery?: import("./history-query-types.js").HistoryQueryController;
  readonly referenceSearch?: import("./global-search-types.js").GlobalSearchController;
  readonly globalSearch?: import("./global-search-types.js").GlobalSearchController;
  readonly threads?: import("./thread-types.js").ThreadsController;
  readonly threadNavigation?: import("./repository-thread-navigation.svelte.js").RepositoryThreadNavigation;
  readonly projects?: import("./projects-hub-types.js").ProjectsHubController;
  readonly work?: import("./work-types.js").WorkController;
  readonly state: StoreControllerState;
  readonly actions: {
    connect(connection: StoreConnection): Promise<void>;
    disconnect(): void;
    destroy(): void;
    refresh(): Promise<void>;
    selectOperation(id: string): Promise<void>;
    selectComparisonParent(id: string): Promise<void>;
    refreshSettings(): Promise<void>;
    setRefreshPaused?(key: string, paused: boolean): void;
    refreshWorkspaceLinks(): Promise<void>;
    setWorkspacePath(path: string): void;
    linkWorkspace(): Promise<void>;
    pickWorkspaceFolder(): Promise<void>;
    prepareIndex(): Promise<void>;
    setSearchDraft(
      patch: Partial<import("./search-settings.js").SearchDraft>,
    ): void;
    cancelSearchDraft(): void;
    refreshSearchSettings(): Promise<void>;
    saveSearchSettings(): Promise<void>;
    controlSearch(
      action: import("./search-settings.js").SearchAction,
    ): Promise<void>;
    refreshSyncSettings(): Promise<void>;
    controlSync(action: import("./sync-settings.js").SyncAction): Promise<void>;
    loadMore(
      collection:
        | "bookmarks"
        | "graph"
        | "evolution"
        | "files"
        | "detail"
        | "search"
        | "agents"
        | "transcript"
        | "reviews",
    ): Promise<void>;
    search(query: string): Promise<void>;
    selectSearchResult(result: StoreSearchResult): Promise<void>;
    previewMutation(command: StoreMutationCommand): Promise<void>;
    applyMutation(): Promise<void>;
    checkMutationStatus(): Promise<void>;
    resetMutation(): void;
    uploadConflict(
      commit: string,
      path: string,
      fingerprint: string,
      bytes: Uint8Array,
    ): Promise<StoreConflictUpload>;
    openWorkingFile?(
      path: string,
    ): Promise<import("./working-copy-types.js").WorkingFile>;
    saveWorkingFile?(
      selection: import("./working-copy-types.js").WorkingFileSelection,
      contents: string,
    ): Promise<import("./working-copy-types.js").WorkingFile>;
    setFileViewMode(mode: FileListingViewMode): void | Promise<void>;
    toggleDirectory(path: string): void | Promise<void>;
    openRoute?(route: StoreRoute): Promise<void>;
    openThreads?(id?: string | null, repository?: string): Promise<void>;
    openReview?(repository: string, thread: string): Promise<void>;
    openRepository(id: string): void | Promise<void>;
    selectPage(page: StorePage): void | Promise<void>;
    navigate(delta: -1 | 1): void | Promise<void>;
    selectBookmark(name: string): void | Promise<void>;
    selectWorkingCopy(commit: string, workspace?: string): void | Promise<void>;
    selectWorkspace(workspace: string | null): void | Promise<void>;
    selectRevision(id: string | null): void | Promise<void>;
    selectEvolution(id: string | null): void | Promise<void>;
    selectFile(path: string | null): void | Promise<void>;
    selectDirectory(path: string): void | Promise<void>;
    selectAgentSession(id: string | null): void | Promise<void>;
    selectReviewThread(id: string | null): void | Promise<void>;
    createRepositoryReview?(text: string): Promise<void>;
    readAgentToolReview?(toolCallId: string): Promise<ReviewDetail | undefined>;
    replyToReview(text: string, parentCommentId?: string): Promise<void>;
    editReviewComment(
      commentId: string,
      expectedRevisionId: string,
      text: string,
    ): Promise<void>;
    commentOnDiffLine(
      context: FileDiffLineContext,
      text: string,
    ): Promise<void>;
    commentOnAgentTool(toolCallId: string, text: string): Promise<void>;
    readAgentSessionReview?(): Promise<ReviewDetail | undefined>;
    commentOnAgentSession?(text: string): Promise<void>;
    setReviewResolved(resolved: boolean): Promise<void>;
    checkReviewAction(): Promise<void>;
    selectRef(value: string): void | Promise<void>;
    signIn(): void | Promise<void>;
    signOut(): void | Promise<void>;
  };
}

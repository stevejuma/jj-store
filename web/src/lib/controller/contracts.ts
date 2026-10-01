import type { WorkspaceLinkSettings } from "./workspace-link-types.js";
import type { FileListingViewMode } from "@lapismd/design-core/diff";
import type { FileDiffLineContext } from "@lapismd/design-core/diff";
import type { Head, Model } from "../api/client.js";
export type StoreConflictDescriptor = Model<"ConflictDescriptor">;
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
export type StoreSearchResultKind = "operation" | "commit" | "file";
export interface StoreSearchResult {
  id: string;
  kind: StoreSearchResultKind;
  title: string;
  description: string;
  author: string;
  timestamp: number;
  operationId: string | null;
  commitId: string | null;
  changeId: string | null;
  scope: string | null;
  path: string | null;
  hasConflict: boolean;
}
export interface StoreSearchState {
  readonly available: boolean;
  readonly query: string;
  readonly results: readonly StoreSearchResult[];
  readonly status: StoreLoadStatus;
  readonly error: string;
  readonly canLoadMore: boolean;
  readonly isStale: boolean;
  readonly degraded: boolean;
  readonly truncated: boolean;
}
export interface StoreRoute {
  projectsHub?: boolean;
  workTab?: import("./projects-hub-types.js").ProjectsTab;
  workQuery?: string;
  workActivity?: import("./native-activity-filter.js").NativeActivityFilter;
  workState?: "active" | "all" | "open" | "in_progress" | "blocked" | "done";
  workRepository?: string;
  workHome?: string;
  page: StorePage;
  repository: string;
  bookmark: string;
  workspace: string | null;
  snapshotOperationId: string | null;
  inspectedOperationId: string | null;
  commitId: string | null;
  evolutionCommitId: string | null;
  changeId: string | null;
  comparisonParentId: string | null;
  comparisonGroupId: string | null;
  path: string | null;
  directory: string;
  agentSessionId: string | null;
  reviewThreadId: string | null;
  projectId?: string | null;
  taskId?: string | null;
}
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
  readonly permission: Model<"EffectivePermission"> | null;
  readonly principal?: string;
}
export interface StoreSettingsState extends WorkspaceLinkSettings {
  readonly capabilities: Model<"Capabilities"> | null;
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
  readonly agentSession: Model<"AgentSessionDetail"> | undefined;
  readonly agentRelations: readonly Model<"AgentRelationLink">[];
  readonly agentTranscript: readonly Model<"AgentTranscriptChunk">[];
  readonly reviewThreads: readonly Model<"ReviewThreadSummary">[];
  readonly reviewThread: Model<"ReviewThreadDetail"> | undefined;
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
    refreshWorkspaceLinks(): Promise<void>;
    setWorkspacePath(path: string): void;
    linkWorkspace(): Promise<void>;
    pickWorkspaceFolder(): Promise<void>;
    prepareIndex(): Promise<void>;
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
    setFileViewMode(mode: FileListingViewMode): void | Promise<void>;
    toggleDirectory(path: string): void | Promise<void>;
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
    setReviewResolved(resolved: boolean): Promise<void>;
    checkReviewAction(): Promise<void>;
    selectRef(value: string): void | Promise<void>;
    signIn(): void | Promise<void>;
    signOut(): void | Promise<void>;
  };
}

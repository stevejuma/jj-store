import type { FileListingViewMode } from "@lapismd/design-core/diff";
import type { Model } from "../api/client.js";
export type StoreConflictDescriptor = Model<"ConflictDescriptor">;
export type StoreConflictUpload = Model<"ConflictUpload">;
import type { CommunityMemberModel } from "../ui/community-workspace/model.js";
import type { StorePage } from "../ui/community-navigation/model.js";
import type {
  RepositoryLogGraph,
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
  page: StorePage;
  repository: string;
  bookmark: string;
  snapshotOperationId: string | null;
  inspectedOperationId: string | null;
  commitId: string | null;
  changeId: string | null;
  comparisonParentId: string | null;
  comparisonGroupId: string | null;
  path: string | null;
  directory: string;
  agentSessionId: string | null;
  reviewThreadId: string | null;
}
export type StoreLoadStatus =
  | "idle"
  | "loading"
  | "refreshing"
  | "ready"
  | "empty"
  | "failed"
  | "unavailable";
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
export interface StoreControllerState {
  readonly accessMode: "bootstrapping" | "authenticated" | "local_trusted";
  readonly storageReady: boolean;
  readonly backups: readonly Model<"BackupStatus">[];
  readonly route: StoreRoute;
  readonly canNavigateBack: boolean;
  readonly canNavigateForward: boolean;
  readonly repositories: readonly { id: string; name: string }[];
  readonly repository: { id: string; name: string };
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
  readonly now: number;
  readonly detail: RevisionDetail | undefined;
  readonly bookmarks: readonly Bookmark[];
  readonly choices: readonly RepositoryChoice[];
  readonly workingCopies: readonly RepositoryWorkingCopy[];
  readonly targets: readonly string[];
  readonly snapshot: RepositorySnapshot | undefined;
  readonly blob: RepositoryBlob | undefined;
  readonly search: StoreSearchState;
  readonly mutation: StoreMutationState;
  readonly agentSessions: readonly Model<"AgentSessionSummary">[];
  readonly agentSession: Model<"AgentSessionDetail"> | undefined;
  readonly agentTranscript: readonly Model<"AgentTranscriptChunk">[];
  readonly reviewThreads: readonly Model<"ReviewThreadSummary">[];
  readonly reviewThread: Model<"ReviewThreadDetail"> | undefined;
  readonly reviewActionMessage: string;
  readonly reviewActionRequestId: string | null;
  readonly authenticated: boolean;
  readonly member: CommunityMemberModel;
  readonly status: StoreControllerStatus;
}
/** Community's state/actions contract shape, scoped to local JJ presentation. */
export interface StoreController {
  readonly state: StoreControllerState;
  readonly actions: {
    connect(connection: StoreConnection): Promise<void>;
    disconnect(): void;
    destroy(): void;
    refresh(): Promise<void>;
    selectOperation(id: string): Promise<void>;
    selectComparisonParent(id: string): Promise<void>;
    prepareIndex(): Promise<void>;
    loadMore(
      collection:
        | "bookmarks"
        | "graph"
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
    openRepository(id: string): void | Promise<void>;
    selectPage(page: StorePage): void | Promise<void>;
    navigate(delta: -1 | 1): void | Promise<void>;
    selectBookmark(name: string): void | Promise<void>;
    selectWorkingCopy(commit: string): void | Promise<void>;
    selectRevision(id: string | null): void | Promise<void>;
    selectFile(path: string | null): void | Promise<void>;
    selectDirectory(path: string): void | Promise<void>;
    selectAgentSession(id: string | null): void | Promise<void>;
    selectReviewThread(id: string | null): void | Promise<void>;
    replyToReview(text: string, parentCommentId?: string): Promise<void>;
    setReviewResolved(resolved: boolean): Promise<void>;
    checkReviewAction(): Promise<void>;
    selectRef(value: string): void | Promise<void>;
    signIn(): void | Promise<void>;
    signOut(): void | Promise<void>;
  };
}

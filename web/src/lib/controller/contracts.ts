import type { FileListingViewMode } from "@lapismd/design-core/diff";
import type { CommunityMemberModel } from "../ui/community-workspace/model.js";
import type { StorePage } from "../ui/community-navigation/model.js";
import type { RepositoryLogGraph } from "../projects/repository-log.js";
import type { Bookmark, RevisionDetail } from "../projects/presentation.js";
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
  readonly live: "disconnected" | "connecting" | "live" | "reconnecting";
  readonly refreshAvailable: boolean;
  readonly error: string;
}
export interface StoreConnection {
  repository: string;
  token: string;
}
export interface StoreControllerState {
  readonly route: StoreRoute;
  readonly canNavigateBack: boolean;
  readonly canNavigateForward: boolean;
  readonly repositories: readonly { id: string; name: string }[];
  readonly repository: { id: string; name: string };
  readonly operationHeads: readonly string[];
  readonly comparisonParents: readonly string[];
  readonly conflictCount: number | null;
  readonly unavailableComparisonCount: number;
  readonly canLoadMoreBookmarks: boolean;
  readonly canLoadMoreGraph: boolean;
  readonly canLoadMoreFiles: boolean;
  readonly fileViewMode: FileListingViewMode;
  readonly collapsedDirectoryPaths: ReadonlySet<string>;
  readonly graph: RepositoryLogGraph;
  readonly now: number;
  readonly detail: RevisionDetail | undefined;
  readonly bookmarks: readonly Bookmark[];
  readonly choices: readonly RepositoryChoice[];
  readonly targets: readonly string[];
  readonly snapshot: RepositorySnapshot | undefined;
  readonly blob: RepositoryBlob | undefined;
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
    loadMore(collection: "bookmarks" | "graph" | "files"): Promise<void>;
    setFileViewMode(mode: FileListingViewMode): void | Promise<void>;
    toggleDirectory(path: string): void | Promise<void>;
    openRepository(id: string): void | Promise<void>;
    selectPage(page: StorePage): void | Promise<void>;
    navigate(delta: -1 | 1): void | Promise<void>;
    selectBookmark(name: string): void | Promise<void>;
    selectRevision(id: string | null): void | Promise<void>;
    selectFile(path: string | null): void | Promise<void>;
    selectDirectory(path: string): void | Promise<void>;
    selectRef(value: string): void | Promise<void>;
    signIn(): void | Promise<void>;
    signOut(): void | Promise<void>;
  };
}

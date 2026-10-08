import type {
  StoreConflictUpload,
  StoreRoute,
  StoreConnection,
} from "./contracts.js";
import type { StoreSearchResult } from "./store-search-types.js";
import type { StoreMutationCommand } from "./store-mutation-types.js";
import type {
  FileListingViewMode,
  FileDiffLineContext,
} from "@lapismd/design-core/diff";
import type { StorePage } from "../ui/community-navigation/model.js";
import type { ReviewDetail } from "./repository-read-types.js";
import type { Model } from "../api/client.js";
export type StoreControllerActions = {
  connect(connection: StoreConnection): Promise<void>;
  disconnect(): void;
  destroy(): void;
  refresh(): Promise<void>;
  selectOperation(id: string): Promise<void>;
  selectComparisonParent(id: string): Promise<void>;
  refreshSettings(
    section?: import("./settings-refresh.js").SettingsRefreshScope,
  ): Promise<void>;
  setRefreshPaused?(key: string, paused: boolean): void;
  refreshWorkspaceLinks(): Promise<void>;
  setWorkspacePath(path: string): void;
  linkWorkspace(): Promise<void>;
  pickWorkspaceFolder(): Promise<void>;
  prepareKindIndex?(kind: Model<"KindPreparationKind">): Promise<void>;
  prepareIndex(): Promise<void>;
  cancelIndex(): Promise<void>;
  setSearchDraft(
    patch: Partial<import("./search-settings.js").SearchDraft>,
  ): void;
  cancelSearchDraft(): void;
  refreshSearchSettings(): Promise<void>;
  saveSearchSettings(): Promise<void>;
  controlSearch(
    action: import("./search-settings.js").SearchAction,
  ): Promise<void>;
  waitForSync(capture: boolean): Promise<void>;
  stopSyncWait(): void;
  refreshSyncSettings(): Promise<void>;
  refreshStorage(): Promise<void>;
  startStorage(
    action: import("./storage-settings.js").StorageAction,
  ): Promise<void>;
  retryStorage(): Promise<void>;
  controlSync(action: import("./sync-settings.js").SyncAction): Promise<void>;
  loadMore(
    collection:
      | "workspaces"
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
  captureReconciliation(): Promise<import("../api/client.js").Head>;
  previewMutation(
    command: StoreMutationCommand,
    captured?: import("../api/client.js").Head,
  ): Promise<void>;
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
  setFileDisplay?(mode: "source" | "preview" | "blame"): void;
  openRoute?(route: StoreRoute): Promise<void>;
  openThreads?(id?: string | null, repository?: string): Promise<void>;
  openReview?(repository: string, thread: string): Promise<void>;
  openRepository(id: string): void | Promise<void>;
  selectPage(page: StorePage): void | Promise<void>;
  selectStacksView(
    view: import("./store-stacks-navigation.js").StoreStacksView,
    historyMode?: "live" | "pinned",
  ): void | Promise<void>;
  leaveStacks(): void | Promise<void>;
  navigate(delta: -1 | 1): void | Promise<void>;
  selectBookmark(name: string): void | Promise<void>;
  selectWorkingCopy(commit: string, workspace?: string): void | Promise<void>;
  selectWorkspace(workspace: string | null): void | Promise<void>;
  selectRevision(id: string | null): void | Promise<void>;
  selectEvolution(id: string | null): void | Promise<void>;
  selectFile(path: string | null): void | Promise<void>;
  selectDirectory(path: string): void | Promise<void>;
  refreshAgentConversation?(): Promise<void>;
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
  commentOnDiffLine(context: FileDiffLineContext, text: string): Promise<void>;
  commentOnAgentTool(toolCallId: string, text: string): Promise<void>;
  readAgentSessionReview?(): Promise<ReviewDetail | undefined>;
  commentOnAgentSession?(text: string): Promise<void>;
  setReviewResolved(resolved: boolean): Promise<void>;
  checkReviewAction(): Promise<void>;
  selectRef(value: string): void | Promise<void>;
  signIn(): void | Promise<void>;
  signOut(): void | Promise<void>;
};

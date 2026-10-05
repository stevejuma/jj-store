/** Frontend-only boundary. An adapter owns repository execution and durability. */
export type WorkspaceHunk = { id: string; label: string; patch: string };
export type WorkspaceFile = {
  path: string;
  hunks: WorkspaceHunk[];
  /** Real reads distinguish an unloaded patch from an empty text diff. */
  content?:
    | { status: "unloaded" | "ready" }
    | {
        status: "unavailable";
        reason: Extract<
          import("../repository-read-types.js").CodeDiffChange["content"],
          { status: "unavailable" }
        >["reason"];
      };
  /** Preserve modes, ordered conflict terms and rename/copy provenance. */
  change?: Pick<
    import("../repository-read-types.js").CodeDiffChange,
    "before" | "after" | "sources"
  >;
};
export type StackCommit = {
  id: string;
  changeId: string;
  message: string;
  author: string;
  createdAt: number;
  email: string;
  immutable: boolean;
  /** Provider graph metadata; absence is reserved for the linear mock. */
  graph?: Pick<
    import("../../projects/repository-log.js").RepositoryLogRevision,
    "parents" | "bookmarks" | "workspaces" | "conflicted" | "divergent"
  > & {
    edges: { target: string; kind: "direct" | "indirect" | "missing" }[];
  };
  files: WorkspaceFile[];
};
export type StackHead = { name: string; commit: string };
export type WorkspaceTargetBranch = StackHead &
  Pick<StackCommit, "message" | "author" | "email" | "createdAt">;
export type WorkspaceHistoryRequest = {
  readKey?: string;
  repository: string;
  revision: number;
  branch: string;
  head: string;
  limit: number;
  cursor?: string;
};
export type WorkspaceHistoryPage = {
  /** Newest first, pinned to the requested branch head. */
  commits: StackCommit[];
  nextCursor?: string;
  total?: number;
};
export type WorkspaceStackHistoryRequest = WorkspaceHistoryRequest & {
  readKey?: string;
  stack: string;
  base: string;
};
export type WorkspaceStack = {
  id: string;
  name: string;
  base: string;
  heads: StackHead[];
  /** Oldest to newest, as in the Changeyard stack model. */
  commits: StackCommit[];
  conflicts: boolean;
  /** Explicit provider segments; their displayed rows are newest first. */
  segments?: {
    head: StackHead;
    baseCommit: string;
    commits: StackCommit[];
    nextCursor?: string;
  }[];
};
export type NamedWorkspace = {
  id: string;
  name: string;
  applied: string[];
  unassigned: WorkspaceFile[];
  mergeCommit?: string;
  wipCommit?: string;
};
export type WorkspaceConflict = {
  path: string;
  left: string;
  base: string;
  right: string;
};
export type WorkspaceSnapshot = {
  repository: string;
  /** Physical folder on this device; null means no local checkout link. */
  folder: string | null;
  profile: "native" | "git_sha1_jj045";
  availability: "ready" | "unlinked" | "unavailable";
  revision: number;
  /** Opaque read scope; numeric revision remains a UI counter. */
  readKey?: string;
  base: string | null;
  active: string | null;
  target: WorkspaceTargetBranch | null;
  /** Exact, unambiguous heads available within this read scope. */
  targetOptions?: WorkspaceTargetBranch[];
  loadedTargetHistory?: WorkspaceHistoryPage;
  /** Provider reference metadata retains conflicted/deleted choices for browsing. */
  bookmarks?: import("../../projects/presentation.js").Bookmark[];
  branchesNextCursor?: string;
  branchUnavailableReasons?: Record<string, string>;
  workspaces: NamedWorkspace[];
  stacks: WorkspaceStack[];
  conflicts: WorkspaceConflict[];
  capabilities: WorkspaceCommand["kind"][];
  capabilityReasons?: Partial<Record<WorkspaceCommand["kind"], string>>;
  canUndo: boolean;
  workspaceWarning?: string;
};
/** Stateful catalog execution fixtures always begin with explicit setup. */
export type ConfiguredWorkspaceSnapshot = WorkspaceSnapshot & {
  base: string;
  active: string;
  target: WorkspaceTargetBranch;
};
export type WorkspaceSelection = {
  /** A commit identity, or the active combination's unassigned edits. */
  source: string | "unassigned";
  path: string;
  /** Omitted means the complete file. */
  hunks?: string[];
};
/** One independent file view per originating canvas column. */
export type WorkspaceOpenDiff = WorkspaceSelection & { origin: string };
export type WorkspaceCommand =
  | {
      kind: "apply_stack" | "unapply_stack" | "delete_stack" | "rebase";
      stack: string;
    }
  | {
      kind: "create_stack";
      name: string;
      parent?: string;
      /** Retained UI selections to stage after confirmed branch creation. */
      selections?: WorkspaceSelection[];
    }
  | { kind: "create_workspace"; name: string }
  | {
      kind: "create_branch";
      stack: string;
      commit: string;
      placement: "above" | "below";
      name: string;
    }
  | { kind: "rename_branch"; stack: string; branch: string; name: string }
  | { kind: "delete_branch"; stack: string; name: string }
  | { kind: "squash_branch"; stack: string; name: string }
  | {
      kind: "add_empty_commit";
      commit: string;
      placement: "above" | "below";
      message: string;
    }
  | { kind: "delete_commit"; commit: string }
  | {
      kind: "edit_commit";
      commit: string;
      message: string;
      selections?: WorkspaceSelection[];
    }
  | { kind: "rename_stack"; stack: string; name: string }
  | { kind: "switch_workspace" | "delete_workspace"; workspace: string }
  | {
      kind: "update_workspace";
      workspace: string;
      name: string;
      targetBranch: { name: string; commit: string };
    }
  | {
      kind: "create_commit";
      stack: string;
      message: string;
      selection?: WorkspaceSelection;
      selections?: WorkspaceSelection[];
    }
  | { kind: "reword"; commit: string; message: string }
  | { kind: "move_commit"; commit: string; stack: string; branch?: string }
  | { kind: "reorder_commit"; commit: string; direction: "up" | "down" }
  | {
      kind: "reorder_commit";
      commit: string;
      target: string;
      placement: "above" | "below";
    }
  | { kind: "squash"; commit: string; target: string }
  | {
      kind: "split";
      commit: string;
      message: string;
      selection: WorkspaceSelection;
    }
  | {
      kind: "transfer";
      selection: WorkspaceSelection;
      selections?: WorkspaceSelection[];
      target: string;
    }
  | { kind: "uncommit"; commit: string }
  | { kind: "resolve_conflict"; path: string; content: string }
  | { kind: "undo" };
export type WorkspaceRequest = {
  readKey?: string;
  id: string;
  repository: string;
  revision: number;
  command: WorkspaceCommand;
};
export type WorkspacePreview = {
  request: WorkspaceRequest;
  title: string;
  summary: string;
  conflicts: string[];
};
export type WorkspaceOutcome =
  | { status: "committed"; snapshot: WorkspaceSnapshot }
  | { status: "unknown" }
  | { status: "refreshed"; snapshot: WorkspaceSnapshot };
export type WorkspaceSourceRequest = {
  repository: string;
  revision: number;
  readKey?: string;
  source: string;
  cursor?: string;
  signal?: AbortSignal;
};
export type WorkspaceFilePage = { files: WorkspaceFile[]; nextCursor?: string };
export interface StackWorkspaceAdapter {
  /** Ordinary JJ commands must not be retried after an unknown response. */
  readonly retryPending?: boolean;
  /** Current checkout evidence; never the historical read pin or complete Unstaged. */
  observeWorkingTree?(
    signal?: AbortSignal,
  ): Promise<import("../working-tree-types.js").WorkingTreeObservation>;
  close?(): void;
  read(): Promise<WorkspaceSnapshot>;
  readBranches?(request: {
    repository: string;
    revision: number;
    readKey?: string;
    cursor: string;
    signal?: AbortSignal;
  }): Promise<WorkspaceSnapshot>;
  chooseTarget?(request: {
    repository: string;
    revision: number;
    readKey?: string;
    target: StackHead;
  }): Promise<WorkspaceSnapshot>;
  readTargetHistory(
    request: WorkspaceHistoryRequest,
    signal?: AbortSignal,
  ): Promise<WorkspaceHistoryPage>;
  readSourceFiles?(request: WorkspaceSourceRequest): Promise<WorkspaceFilePage>;
  readStackHistory?(
    request: WorkspaceStackHistoryRequest,
    signal?: AbortSignal,
  ): Promise<WorkspaceHistoryPage>;
  readFileDiff?(
    request: WorkspaceSourceRequest & { path: string },
  ): Promise<WorkspaceFile>;
  preview(request: WorkspaceRequest): Promise<WorkspacePreview>;
  apply(request: WorkspaceRequest): Promise<WorkspaceOutcome>;
  status(requestId: string): Promise<WorkspaceOutcome>;
}
export class WorkspaceCommandError extends Error {
  constructor(
    readonly code: "stale" | "invalid" | "failed",
    message: string,
  ) {
    super(message);
  }
}
export const cloneWorkspace = <T>(value: T): T =>
  JSON.parse(JSON.stringify(value)) as T;

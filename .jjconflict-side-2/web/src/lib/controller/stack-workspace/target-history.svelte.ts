import {
  cloneWorkspace,
  type StackWorkspaceAdapter,
  type StackCommit,
  type WorkspaceHistoryRequest,
  type WorkspaceOpenDiff,
  type WorkspaceSnapshot,
  type WorkspaceTargetBranch,
  type WorkspaceFile,
} from "./types.js";

export const TARGET_HISTORY_ORIGIN = "target-history";
export const TARGET_HISTORY_PAGE_SIZE = 40;

/** Read-only, operation/head-pinned history; never added to applied stacks. */
export class WorkspaceTargetHistory {
  active = $state(false);
  branch = $state<WorkspaceTargetBranch | null>(null);
  commits = $state<StackCommit[]>([]);
  selectedId = $state<string | null>(null);
  diff = $state<WorkspaceOpenDiff | null>(null);
  loading = $state(false);
  error = $state("");
  hasMore = $state(true);
  total = $state<number | undefined>();
  #request: WorkspaceHistoryRequest | null = null;
  #cursor: string | undefined;
  #cursors = new Set<string>();
  #epoch = 0;
  #closed = false;
  #readKey: string | undefined;
  #abort = new AbortController();
  constructor(
    readonly adapter: StackWorkspaceAdapter,
    readonly details?: {
      files(source: string): WorkspaceFile[];
      selectCommit(source: string): void;
      selectFile(source: string, path: string): void;
    },
    readonly report?: (cause: unknown) => boolean,
  ) {}

  sync(snapshot: WorkspaceSnapshot) {
    if (!snapshot.target) {
      this.#epoch++;
      this.#abort.abort();
      this.#abort = new AbortController();
      this.#request = null;
      this.#readKey = undefined;
      this.#cursor = undefined;
      this.#cursors.clear();
      this.total = undefined;
      this.branch = null;
      this.commits = [];
      this.active = this.loading = this.hasMore = false;
      this.selectedId = this.diff = null;
      this.error = "";
      return;
    }
    const next: WorkspaceHistoryRequest = {
      repository: snapshot.repository,
      revision: snapshot.revision,
      ...(snapshot.readKey ? { readKey: snapshot.readKey } : {}),
      branch: snapshot.target.name,
      head: snapshot.target.commit,
      limit: TARGET_HISTORY_PAGE_SIZE,
    };
    if (
      JSON.stringify(next) === JSON.stringify(this.#request) &&
      snapshot.readKey === this.#readKey
    )
      return;
    const loaded = snapshot.loadedTargetHistory;
    const selected =
      this.#request?.head === next.head &&
      this.#request.repository === next.repository &&
      loaded?.commits.some((commit) => commit.id === this.selectedId)
        ? this.selectedId
        : null;
    const retainedDiff =
      selected && this.diff?.source === selected ? this.diff : null;
    this.#readKey = snapshot.readKey;
    this.#epoch++;
    this.#abort.abort();
    this.#abort = new AbortController();
    this.#request = next;
    this.branch = cloneWorkspace(snapshot.target);
    this.commits = cloneWorkspace(loaded?.commits ?? []);
    this.selectedId = selected;
    this.diff = retainedDiff;
    this.#cursor = loaded?.nextCursor;
    this.#cursors.clear();
    this.loading = false;
    this.error = "";
    this.hasMore = loaded ? Boolean(loaded.nextCursor) : true;
    this.total = loaded?.total;
    if (this.active && !loaded) void this.loadMore();
  }
  async select() {
    if (this.#closed) return;
    this.active = true;
    if (!this.commits.length) await this.loadMore();
  }
  async loadMore() {
    if (
      this.#closed ||
      !this.active ||
      !this.#request ||
      this.loading ||
      !this.hasMore
    )
      return;
    const epoch = this.#epoch;
    const request = {
      ...this.#request,
      ...(this.#cursor ? { cursor: this.#cursor } : {}),
    };
    this.loading = true;
    this.error = "";
    try {
      const page = await this.adapter.readTargetHistory(
        request,
        this.#abort.signal,
      );
      if (this.#closed || epoch !== this.#epoch) return;
      if (
        page.commits.length > request.limit ||
        (page.nextCursor &&
          (page.nextCursor === request.cursor ||
            this.#cursors.has(page.nextCursor)))
      )
        throw new Error(
          "The branch history page did not advance within its bound.",
        );
      const ids = new Set(this.commits.map((commit) => commit.id));
      const added = page.commits.filter((commit) => {
        if (ids.has(commit.id)) return false;
        ids.add(commit.id);
        return true;
      });
      if (!added.length && page.nextCursor)
        throw new Error("The branch history page did not advance.");
      this.commits = [
        ...this.commits,
        ...cloneWorkspace(added).map((commit) => ({
          ...commit,
          immutable: true,
        })),
      ];
      if (request.cursor) this.#cursors.add(request.cursor);
      this.#cursor = page.nextCursor;
      this.hasMore = Boolean(page.nextCursor);
      this.total = page.total;
    } catch (error) {
      if (!this.#closed && epoch === this.#epoch)
        this.error = this.report?.(error)
          ? ""
          : error instanceof Error
            ? error.message
            : String(error);
    } finally {
      if (!this.#closed && epoch === this.#epoch) this.loading = false;
    }
  }
  selectCommit(id: string) {
    if (this.commits.some((commit) => commit.id === id)) {
      this.selectedId = id;
      this.details?.selectCommit(id);
    }
  }
  selectFile(source: string, path: string) {
    const files =
      this.details?.files(source) ??
      this.commits.find((commit) => commit.id === source)?.files;
    if (!files?.some((file) => file.path === path)) return;
    this.diff = { origin: TARGET_HISTORY_ORIGIN, source, path };
    this.details?.selectFile(source, path);
  }
  close() {
    this.#closed = true;
    this.#epoch++;
    this.#abort.abort();
  }
}

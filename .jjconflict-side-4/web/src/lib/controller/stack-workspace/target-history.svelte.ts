import {
  cloneWorkspace,
  type StackWorkspaceAdapter,
  type StackCommit,
  type WorkspaceHistoryRequest,
  type WorkspaceOpenDiff,
  type WorkspaceSnapshot,
  type WorkspaceTargetBranch,
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
  constructor(readonly adapter: StackWorkspaceAdapter) {}

  sync(snapshot: WorkspaceSnapshot) {
    const next: WorkspaceHistoryRequest = {
      repository: snapshot.repository,
      revision: snapshot.revision,
      branch: snapshot.target.name,
      head: snapshot.target.commit,
      limit: TARGET_HISTORY_PAGE_SIZE,
    };
    if (JSON.stringify(next) === JSON.stringify(this.#request)) return;
    this.#epoch++;
    this.#request = next;
    this.branch = cloneWorkspace(snapshot.target);
    this.commits = [];
    this.selectedId = this.diff = null;
    this.#cursor = undefined;
    this.#cursors.clear();
    this.loading = false;
    this.error = "";
    this.hasMore = true;
    this.total = undefined;
    if (this.active) void this.loadMore();
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
      const page = await this.adapter.readTargetHistory(request);
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
        this.error = error instanceof Error ? error.message : String(error);
    } finally {
      if (!this.#closed && epoch === this.#epoch) this.loading = false;
    }
  }
  selectCommit(id: string) {
    if (this.commits.some((commit) => commit.id === id)) this.selectedId = id;
  }
  selectFile(source: string, path: string) {
    if (
      !this.commits
        .find((commit) => commit.id === source)
        ?.files.some((file) => file.path === path)
    )
      return;
    this.diff = { origin: TARGET_HISTORY_ORIGIN, source, path };
  }
  close() {
    this.#closed = true;
    this.#epoch++;
  }
}

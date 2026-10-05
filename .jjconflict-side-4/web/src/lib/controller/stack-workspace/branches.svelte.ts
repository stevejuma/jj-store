import type { StackWorkspaceAdapter, WorkspaceSnapshot } from "./types.js";

/** Branch metadata pages own their retry and cancellation independently of histories. */
export class WorkspaceBranches {
  loading = $state(false);
  error = $state("");
  #snapshot: WorkspaceSnapshot | null = null;
  #scope = "";
  #owner: AbortController | null = null;
  #closed = false;
  constructor(
    readonly adapter: StackWorkspaceAdapter,
    readonly accept: (snapshot: WorkspaceSnapshot) => void,
    readonly report?: (cause: unknown) => boolean,
  ) {}
  sync(snapshot: WorkspaceSnapshot) {
    const scope = JSON.stringify([
      snapshot.repository,
      snapshot.revision,
      snapshot.readKey,
    ]);
    this.#snapshot = snapshot;
    if (scope === this.#scope) return;
    this.#scope = scope;
    this.#owner?.abort();
    this.#owner = null;
    this.loading = false;
    this.error = "";
  }
  async load() {
    const snapshot = this.#snapshot;
    if (
      this.#closed ||
      this.loading ||
      !snapshot?.branchesNextCursor ||
      !this.adapter.readBranches
    )
      return;
    const owner = new AbortController(),
      scope = this.#scope;
    this.#owner = owner;
    this.loading = true;
    this.error = "";
    try {
      const next = await this.adapter.readBranches({
        repository: snapshot.repository,
        revision: snapshot.revision,
        readKey: snapshot.readKey,
        cursor: snapshot.branchesNextCursor,
        signal: owner.signal,
      });
      if (!this.#closed && !owner.signal.aborted && scope === this.#scope)
        this.accept(next);
    } catch (error) {
      if (!this.#closed && !owner.signal.aborted && scope === this.#scope)
        this.error = this.report?.(error)
          ? ""
          : error instanceof Error
            ? error.message
            : String(error);
    } finally {
      if (this.#owner === owner) {
        this.#owner = null;
        this.loading = false;
      }
    }
  }
  close() {
    this.#closed = true;
    this.#owner?.abort();
  }
}

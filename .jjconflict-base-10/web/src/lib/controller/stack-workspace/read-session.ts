import type {
  StackWorkspaceAdapter,
  WorkspaceSnapshot,
  WorkspaceTargetBranch,
} from "./types.js";

/** Refresh and target choices share one epoch so late results cannot replace a newer scope. */
export class WorkspaceReadSession {
  #epoch = 0;
  #closed = false;
  constructor(
    readonly state: { loading: boolean; error: string },
    readonly accept: (snapshot: WorkspaceSnapshot) => void,
    readonly report?: (cause: unknown) => boolean,
  ) {}
  async run(read: () => Promise<WorkspaceSnapshot>) {
    if (this.#closed) return;
    const epoch = ++this.#epoch;
    this.state.loading = true;
    this.state.error = "";
    try {
      const snapshot = await read();
      if (!this.#closed && epoch === this.#epoch) this.accept(snapshot);
    } catch (error) {
      if (!this.#closed && epoch === this.#epoch && !this.report?.(error))
        this.state.error =
          error instanceof Error ? error.message : String(error);
    } finally {
      if (!this.#closed && epoch === this.#epoch) this.state.loading = false;
    }
  }
  choose(
    adapter: StackWorkspaceAdapter,
    snapshot: WorkspaceSnapshot,
    target: WorkspaceTargetBranch,
  ) {
    return this.run(() =>
      adapter.chooseTarget!({
        repository: snapshot.repository,
        revision: snapshot.revision,
        ...(snapshot.readKey ? { readKey: snapshot.readKey } : {}),
        target: { name: target.name, commit: target.commit },
      }),
    );
  }
  close() {
    this.#closed = true;
    this.#epoch++;
  }
}

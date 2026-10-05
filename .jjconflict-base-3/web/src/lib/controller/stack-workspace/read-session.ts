import type { WorkspaceSnapshot } from "./types.js";

/** Refresh and target choices share one epoch so late results cannot replace a newer scope. */
export class WorkspaceReadSession {
  #epoch = 0;
  #closed = false;
  constructor(
    readonly state: { loading: boolean; error: string },
    readonly accept: (snapshot: WorkspaceSnapshot) => void,
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
      if (!this.#closed && epoch === this.#epoch)
        this.state.error =
          error instanceof Error ? error.message : String(error);
    } finally {
      if (!this.#closed && epoch === this.#epoch) this.state.loading = false;
    }
  }
  close() {
    this.#closed = true;
    this.#epoch++;
  }
}

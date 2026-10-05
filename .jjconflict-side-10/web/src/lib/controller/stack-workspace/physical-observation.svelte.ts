import type { WorkingTreeObservation } from "../working-tree-types.js";
import type { StackWorkspaceAdapter } from "./types.js";

/** Observation failure never changes historical selections or grants assignment. */
export class WorkspacePhysicalObservation {
  observation = $state<WorkingTreeObservation | null>(null);
  loading = $state(false);
  error = $state("");
  #pending: AbortController | null = null;
  #epoch = 0;
  #closed = false;
  constructor(readonly adapter: StackWorkspaceAdapter) {}
  async refresh() {
    if (this.#closed || !this.adapter.observeWorkingTree) return;
    const ticket = ++this.#epoch,
      abort = new AbortController();
    this.#pending?.abort();
    this.#pending = abort;
    this.loading = true;
    this.error = "";
    this.observation = null;
    try {
      const value = await this.adapter.observeWorkingTree(abort.signal);
      if (!this.#closed && ticket === this.#epoch && !abort.signal.aborted)
        this.observation = value;
    } catch (cause) {
      if (!this.#closed && ticket === this.#epoch && !abort.signal.aborted)
        this.error = cause instanceof Error ? cause.message : String(cause);
    } finally {
      if (!this.#closed && ticket === this.#epoch) {
        this.loading = false;
        this.#pending = null;
      }
    }
  }
  close() {
    this.#closed = true;
    this.#epoch++;
    this.#pending?.abort();
    this.#pending = null;
    this.observation = null;
    this.loading = false;
  }
}

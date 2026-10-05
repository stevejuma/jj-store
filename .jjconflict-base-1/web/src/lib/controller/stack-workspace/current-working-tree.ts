import { ApiError, type Model } from "../../api/client.js";
import type { StoreDataSource } from "../native-data-source.js";
import { awaitPhysicalRead as wait } from "../working-tree-read.js";
import { isLocal, type RepositoryHead } from "../repository-profile.js";
import {
  validateWorkingTreeObservation,
  validateWorkingTreeSelection,
} from "../working-tree-validation.js";

/** Current disk evidence has its own lifetime and never advances historical selections. */
export class CurrentWorkingTreeReads {
  readonly #closed = new AbortController();
  readonly #binding: Model<"LocalBinding">;
  constructor(
    readonly source: StoreDataSource,
    binding: Model<"LocalBinding">,
  ) {
    this.#binding = structuredClone(binding);
  }
  #selection(captured: RepositoryHead) {
    const binding = this.#binding;
    if (
      this.source.profile !== "git_sha1_jj045" ||
      !isLocal(captured) ||
      captured.repository !== this.source.repository ||
      binding.repository !== this.source.repository ||
      binding.availability !== "ready" ||
      !binding.path ||
      String(captured.generation) !== String(binding.generation)
    )
      throw new ApiError("snapshot_mismatch");
    if (captured.heads.length !== 1)
      throw new ApiError("working_tree_operation_unavailable");
    const selection = {
      bindingGeneration: binding.generation,
      operation: captured.heads[0]!,
    };
    validateWorkingTreeSelection(selection);
    return selection;
  }
  async observe(input?: AbortSignal) {
    const signal = AbortSignal.any([
      this.#closed.signal,
      ...(input ? [input] : []),
    ]);
    signal.throwIfAborted();
    if (!this.source.workingTree)
      throw new ApiError("working_tree_unavailable");
    const before = this.#selection(
      await wait(this.source.snapshot(true), signal),
    );
    const value = await wait(
      this.source.workingTree.observe(before, signal),
      signal,
    );
    const result = validateWorkingTreeObservation(
      value,
      this.source.repository,
      before,
    );
    const after = this.#selection(
      await wait(this.source.snapshot(true), signal),
    );
    if (
      after.operation !== before.operation ||
      String(after.bindingGeneration) !== String(before.bindingGeneration)
    )
      throw new ApiError("stale_working_tree");
    signal.throwIfAborted();
    return structuredClone(result);
  }
  close() {
    this.#closed.abort();
  }
}

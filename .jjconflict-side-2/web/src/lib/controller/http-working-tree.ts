import type { LocalClient } from "../api/local-client.js";
import { awaitPhysicalRead } from "./working-tree-read.js";
import {
  validateWorkingTreeObservation,
  validateWorkingTreeSelection,
} from "./working-tree-validation.js";
import type {
  WorkingTreeSelection,
  WorkingTreeTransport,
} from "./working-tree-types.js";

/** Uncached current physical reads; owned by the repository transport. */
export class HttpWorkingTree implements WorkingTreeTransport {
  constructor(readonly client: LocalClient) {}
  async observe(selection: WorkingTreeSelection, signal?: AbortSignal) {
    const pin = { ...selection };
    validateWorkingTreeSelection(pin);
    const lifetime = AbortSignal.any([
      this.client.closed.signal,
      ...(signal ? [signal] : []),
    ]);
    lifetime.throwIfAborted();
    const value = await awaitPhysicalRead(
      this.client.request<unknown>(
        "working-tree/observe",
        {
          version: 1,
          binding_generation: pin.bindingGeneration,
          operation: pin.operation,
        },
        lifetime,
      ),
      lifetime,
    );
    return validateWorkingTreeObservation(value, this.client.repository, pin);
  }
}

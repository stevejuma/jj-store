import type { Model } from "../api/client.js";

/** Current checkout evidence, separate from historical code and combination WIP. */
export type WorkingTreeObservation = Model<"ObservedWorkingTree">;
export type WorkingTreeSelection = {
  bindingGeneration: number | bigint;
  operation: string;
};
export interface WorkingTreeTransport {
  observe(
    selection: WorkingTreeSelection,
    signal?: AbortSignal,
  ): Promise<WorkingTreeObservation>;
}

import type { WorkflowHistoryEntry } from "./workflow-history.svelte.js";

export interface WorkflowEntryActions {
  onOpen(entry: WorkflowHistoryEntry | null, duplicate?: boolean): void;
  onToggle(entry: WorkflowHistoryEntry): void;
  onRemove(entry: WorkflowHistoryEntry): void;
  onRun(entry: WorkflowHistoryEntry): void;
}

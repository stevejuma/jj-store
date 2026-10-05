import type { StackWorkspaceController } from "./controller.svelte.js";
import type { WorkspaceSnapshot } from "./types.js";
import { retainedWorkspaceDiffs } from "./retained-diffs.js";
import { retainedStackSelections } from "./files.js";
import { activateWorkspaceDiff } from "./diff-selection.js";
export function updateWorkspaceControllerSnapshot(
  state: StackWorkspaceController,
  snapshot: WorkspaceSnapshot,
) {
  state.branches.sync(snapshot);
  state.histories.sync(snapshot);
  state.details.sync(snapshot);
  const diffs = retainedWorkspaceDiffs(
    snapshot,
    state.snapshot?.active,
    state.diffs,
    state.details.lazy
      ? (source) => state.details.files[source] ?? []
      : undefined,
  );
  state.stackSelections = retainedStackSelections(
    snapshot,
    state.stackSelections,
  );
  state.snapshot = snapshot;
  state.targetHistory.sync(snapshot);
  state.diffs = diffs;
  const active = state.diffs.find(
    (diff) => diff.source === state.source && diff.path === state.selectedPath,
  );
  if (active) activateWorkspaceDiff(state, active);
  else {
    state.selectedPath = null;
    state.selection = null;
  }
}

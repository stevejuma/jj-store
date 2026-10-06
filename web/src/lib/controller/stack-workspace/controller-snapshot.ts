import type { StackWorkspaceController } from "./controller.svelte.js";
import type { WorkspaceSnapshot } from "./types.js";
import { retainedWorkspaceDiffs } from "./retained-diffs.js";
import { retainedStackSelections } from "./files.js";
import { activateWorkspaceDiff } from "./diff-selection.js";
import { projectWorkspaceFiles } from "./controller-files.js";
export function updateWorkspaceControllerSnapshot(
  state: StackWorkspaceController,
  snapshot: WorkspaceSnapshot,
) {
  state.details.sync(snapshot);
  if (state.details.lazy)
    for (const [source, files] of Object.entries(state.details.files))
      projectWorkspaceFiles(snapshot, source, files);
  state.branches.sync(snapshot);
  state.histories.sync(snapshot);
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
  const operationChanged = state.snapshot?.operation !== snapshot.operation;
  state.snapshot = snapshot;
  state.snapshotLoaded = true;
  if (operationChanged && snapshot.operation)
    state.options.acceptedOperation?.(snapshot.repository, snapshot.operation);
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

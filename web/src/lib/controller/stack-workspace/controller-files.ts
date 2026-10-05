import type { StackWorkspaceController } from "./controller.svelte.js";
import { workspaceSourceFiles } from "./files.js";
import { reconcileSourceDiffs } from "./retained-diffs.js";
import { activateWorkspaceDiff } from "./diff-selection.js";
import {
  cloneWorkspace,
  type WorkspaceFile,
  type WorkspaceSnapshot,
} from "./types.js";

/** Reapply only the lazy cache retained by the exact current read scope. */
export function projectWorkspaceFiles(
  snapshot: WorkspaceSnapshot,
  source: string,
  files: WorkspaceFile[],
) {
  if (source === "unassigned") {
    const workspace = snapshot.workspaces.find((w) => w.id === snapshot.active);
    if (workspace) workspace.unassigned = cloneWorkspace(files);
  }
  for (const stack of snapshot.stacks)
    for (const commit of stack.commits)
      if (commit.id === source) commit.files = cloneWorkspace(files);
}

export function acceptWorkspaceFiles(
  controller: StackWorkspaceController,
  source: string,
  files: WorkspaceFile[],
  complete: boolean,
) {
  if (controller.snapshot)
    projectWorkspaceFiles(controller.snapshot, source, files);
  controller.diffs = reconcileSourceDiffs(
    controller.diffs,
    source,
    files,
    complete,
  );
  if (controller.targetHistory.diff)
    controller.targetHistory.diff =
      reconcileSourceDiffs(
        [controller.targetHistory.diff],
        source,
        files,
        complete,
      )[0] ?? null;
  const active = controller.diffs.find(
    (diff) =>
      diff.source === controller.source &&
      diff.path === controller.selectedPath,
  );
  if (active) activateWorkspaceDiff(controller, active);
}

/** Lazy provider files always win over presentation unions. */
export function workspaceControllerFiles(
  controller: Pick<
    StackWorkspaceController,
    "details" | "targetHistory" | "snapshot"
  >,
  source: string,
) {
  if (controller.details.lazy) return controller.details.files[source] ?? [];
  return (
    controller.targetHistory.commits.find((commit) => commit.id === source)
      ?.files ??
    (controller.snapshot
      ? workspaceSourceFiles(controller.snapshot, source)
      : [])
  );
}

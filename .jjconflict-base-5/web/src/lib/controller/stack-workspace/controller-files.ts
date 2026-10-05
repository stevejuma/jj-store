import type { StackWorkspaceController } from "./controller.svelte.js";
import { workspaceSourceFiles } from "./files.js";
import { reconcileSourceDiffs } from "./retained-diffs.js";
import { activateWorkspaceDiff } from "./diff-selection.js";
import { cloneWorkspace, type WorkspaceFile } from "./types.js";

export function acceptWorkspaceFiles(
  controller: StackWorkspaceController,
  source: string,
  files: WorkspaceFile[],
  complete: boolean,
) {
  if (source === "unassigned" && controller.snapshot) {
    const workspace = controller.snapshot.workspaces.find(
      (w) => w.id === controller.snapshot?.active,
    );
    if (workspace) workspace.unassigned = cloneWorkspace(files);
  }
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

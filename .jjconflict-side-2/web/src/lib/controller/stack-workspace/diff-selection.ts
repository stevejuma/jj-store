import { workspaceAggregateSource, workspaceSourceStack } from "./files.js";
import type { StackWorkspaceController } from "./controller.svelte.js";
import type { WorkspaceOpenDiff, WorkspaceCommand } from "./types.js";
/** Adjacent file views retain independent read cancellation and selection. */
export function selectWorkspaceFile(
  controller: StackWorkspaceController,
  source: string,
  path: string,
  hunks?: string[],
) {
  if (controller.locked) return;
  const snapshot = controller.snapshot;
  if (
    !snapshot ||
    !controller.filesFor(source).some((file) => file.path === path)
  )
    return;
  const origin =
    source === "unassigned" || source === "combined"
      ? "working"
      : `stack-${workspaceSourceStack(snapshot, source)!.id}`;
  const diff: WorkspaceOpenDiff = {
    origin,
    source,
    path,
    ...(hunks ? { hunks: [...hunks] } : {}),
  };
  const index = controller.diffs.findIndex((item) => item.origin === origin);
  if (index < 0) controller.diffs = [...controller.diffs, diff];
  else {
    const old = controller.diffs[index]!;
    if (old.source !== source || old.path !== path)
      controller.details.cancelDiff(old.source, old.path);
    controller.diffs = controller.diffs.map((item, i) =>
      i === index ? diff : item,
    );
  }
  activateWorkspaceDiff(controller, diff);
  void controller.details.loadDiff(source, path);
}
export function closeWorkspaceDiff(
  controller: StackWorkspaceController,
  origin: string,
) {
  const old =
    origin === "target-history"
      ? controller.targetHistory.diff
      : controller.diffs.find((diff) => diff.origin === origin);
  if (old) controller.details.cancelDiff(old.source, old.path);
  if (origin === "target-history") {
    controller.targetHistory.diff = null;
    return;
  }
  controller.diffs = controller.diffs.filter((diff) => diff.origin !== origin);
  if (
    !controller.diffs.some(
      (diff) =>
        diff.source === controller.source &&
        diff.path === controller.selectedPath,
    )
  ) {
    controller.selectedPath = null;
    controller.selection = null;
  }
}
export function activateWorkspaceDiff(
  controller: StackWorkspaceController,
  diff: WorkspaceOpenDiff,
) {
  controller.selectedPath = diff.path;
  controller.source = diff.source;
  controller.selectedCommit = diff.origin === "working" ? null : diff.source;
  controller.selection =
    workspaceAggregateSource(diff.source) ||
    controller.details.lazy ||
    !["transfer", "split", "create_commit"].some(
      (kind) =>
        controller.eligibility(kind as WorkspaceCommand["kind"]).allowed,
    )
      ? null
      : {
          source: diff.source,
          path: diff.path,
          ...(diff.hunks ? { hunks: [...diff.hunks] } : {}),
        };
}

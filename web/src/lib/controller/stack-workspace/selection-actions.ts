import type { StackWorkspaceController } from "./controller.svelte.js";
import { workspaceBranchSource } from "./files.js";

function clearActiveSelection(controller: StackWorkspaceController) {
  controller.selectedCommit =
    controller.selectedPath =
    controller.selection =
      null;
}

/** User activation toggles; programmatic selection and lazy reads remain idempotent. */
export function toggleStackSource(
  controller: StackWorkspaceController,
  stack: string,
  source: { commit: string } | { branch: string },
) {
  if (controller.locked) return;
  const value =
    "commit" in source
      ? source.commit
      : workspaceBranchSource(stack, source.branch);
  if (controller.stackSelections[stack] === value) {
    const selections = { ...controller.stackSelections };
    delete selections[stack];
    controller.stackSelections = selections;
    clearActiveSelection(controller);
  } else if ("commit" in source) controller.selectCommit(source.commit, stack);
  else controller.selectStackFiles(stack, source.branch);
}

export function toggleStackBranch(
  controller: StackWorkspaceController,
  stack: string,
  branch: string,
) {
  if (controller.locked) return;
  if (
    !controller.targetHistory.active &&
    controller.selectedBranch === branch
  ) {
    controller.selectedStack = controller.selectedBranch = null;
    clearActiveSelection(controller);
  } else controller.selectStack(stack, branch);
}

export function toggleTargetBranch(controller: StackWorkspaceController) {
  if (controller.locked) return;
  if (controller.targetHistory.active) controller.targetHistory.active = false;
  else void controller.selectTargetBranch();
}

export function toggleTargetCommit(
  controller: StackWorkspaceController,
  id: string,
) {
  if (controller.locked) return;
  if (controller.targetHistory.selectedId === id)
    controller.targetHistory.selectedId = null;
  else controller.targetHistory.selectCommit(id);
}

import { activeWorkspace } from "../../../web/src/lib/controller/stack-workspace/presentation.js";
import {
  moveCommitProblem,
  sortCommitProblem,
} from "../../../web/src/lib/controller/stack-workspace/commit-placement.js";
import {
  WorkspaceCommandError,
  type WorkspaceCommand,
  type WorkspaceSnapshot,
} from "../../../web/src/lib/controller/stack-workspace/types.js";
import { validateMenuFixture } from "./menu-validation.js";
import { findCommit, selectedChanges } from "./changes.js";

export function validateFixtureCommand(
  snapshot: WorkspaceSnapshot,
  command: WorkspaceCommand,
) {
  const fail = (message: string): never => {
    throw new WorkspaceCommandError("invalid", message);
  };
  if (
    snapshot.availability !== "ready" ||
    !snapshot.folder ||
    !snapshot.capabilities.includes(command.kind)
  )
    fail("An available local checkout is required.");
  if (
    "stack" in command &&
    !snapshot.stacks.some((stack) => stack.id === command.stack)
  )
    fail("Choose an existing stack.");
  if (
    "workspace" in command &&
    !snapshot.workspaces.some((item) => item.id === command.workspace)
  )
    fail("Choose an existing workspace.");
  validateMenuFixture(snapshot, command, fail);
  if (
    "name" in command &&
    !["delete_branch", "squash_branch"].includes(command.kind)
  ) {
    if (
      !command.name.trim() ||
      command.name.length > 128 ||
      /[\u0000-\u001f]/u.test(command.name)
    )
      fail("Enter a name without control characters, up to 128 characters.");
    if (
      command.kind === "create_workspace" &&
      snapshot.workspaces.some((item) => item.name === command.name.trim())
    )
      fail("This workspace name is already in use.");
    if (
      [
        "create_stack",
        "create_branch",
        "rename_branch",
        "rename_stack",
      ].includes(command.kind) &&
      snapshot.stacks.some(
        (stack) =>
          stack.name === command.name.trim() ||
          stack.heads.some((head) => head.name === command.name.trim()),
      )
    )
      fail("This stack name is already in use.");
  }
  if ("message" in command && !command.message.trim())
    fail("Enter a commit message.");
  if (
    "commit" in command &&
    command.kind !== "create_branch" &&
    !(command.kind === "add_empty_commit" && command.placement === "above") &&
    findCommit(snapshot, command.commit).commit.immutable
  )
    fail("This commit is immutable.");
  if ("target" in command) {
    if (
      command.target !== "unassigned" &&
      command.kind !== "reorder_commit" &&
      findCommit(snapshot, command.target).commit.immutable
    )
      fail("The destination commit is immutable.");
    if ("commit" in command && command.commit === command.target)
      fail("Choose a different destination commit.");
    if (
      command.kind === "transfer" &&
      (command.selections ?? [command.selection]).some(
        (item) => item.source === command.target,
      )
    )
      fail("Choose a different destination commit.");
  }
  if ("selection" in command && command.selection) {
    if (
      command.selection.source !== "unassigned" &&
      findCommit(snapshot, command.selection.source).commit.immutable
    )
      fail("This source commit is immutable.");
    selectedChanges(snapshot, command.selection);
  }
  if (
    (command.kind === "create_commit" ||
      command.kind === "edit_commit" ||
      command.kind === "transfer") &&
    command.selections
  ) {
    if (command.kind === "create_commit" && command.selection)
      fail("Use one exact selection list.");
    for (const selection of command.selections) {
      if (
        selection.source !== "unassigned" &&
        findCommit(snapshot, selection.source).commit.immutable
      )
        fail("This source commit is immutable.");
      selectedChanges(snapshot, selection);
    }
  }
  if (
    command.kind === "delete_workspace" &&
    (snapshot.active === command.workspace || snapshot.workspaces.length === 1)
  )
    fail("Switch away before deleting this workspace.");
  if (
    command.kind === "resolve_conflict" &&
    !snapshot.conflicts.some((item) => item.path === command.path)
  )
    fail("This conflict is no longer present.");
  if (command.kind === "undo" && !snapshot.canUndo)
    fail("There is no command to undo.");
  if (command.kind === "reorder_commit" && "target" in command) {
    const problem = sortCommitProblem(
      snapshot,
      command.commit,
      command.target,
      command.placement,
    );
    if (problem) fail(problem);
  }
  if (command.kind === "reorder_commit" && "direction" in command) {
    const { stack, commit } = findCommit(snapshot, command.commit);
    const index =
      stack.commits.indexOf(commit) + (command.direction === "up" ? 1 : -1);
    if (index < 0 || index >= stack.commits.length)
      fail("The commit is already at this end of its stack.");
    if (stack.commits[index]!.immutable)
      fail("Cannot reorder across an immutable commit.");
  }
  if (
    command.kind === "squash" &&
    findCommit(snapshot, command.commit).stack.id !==
      findCommit(snapshot, command.target).stack.id
  )
    fail("Choose a squash destination in the same stack.");
  if (command.kind === "move_commit") {
    const problem = moveCommitProblem(snapshot, command);
    if (problem) fail(problem);
  }
  if (command.kind === "split" && command.selection.source !== command.commit)
    fail("Choose changes from the commit being split.");
  if (
    command.kind === "delete_stack" &&
    snapshot.stacks
      .find((stack) => stack.id === command.stack)!
      .commits.some((commit) => commit.immutable)
  )
    fail("This stack contains immutable commits.");
  if (
    snapshot.conflicts.length &&
    !["resolve_conflict", "undo", "unapply_stack"].includes(command.kind)
  )
    fail("Resolve the workspace conflicts before changing its history.");
  // Validate the active combination even when a command addresses another object.
  activeWorkspace(snapshot);
}

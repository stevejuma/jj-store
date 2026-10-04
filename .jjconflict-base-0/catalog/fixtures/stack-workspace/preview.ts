import {
  activeWorkspace,
  commandTitle,
} from "../../../web/src/lib/controller/stack-workspace/presentation.js";
import type {
  WorkspaceCommand,
  WorkspaceSnapshot,
} from "../../../web/src/lib/controller/stack-workspace/types.js";
import { findCommit } from "./changes.js";

/** Human-readable fixture preview; describes the simulated transition precisely. */
export function fixtureSummary(
  snapshot: WorkspaceSnapshot,
  command: WorkspaceCommand,
) {
  const workspace = activeWorkspace(snapshot);
  const stackName = (id: string) =>
    snapshot.stacks.find((item) => item.id === id)?.name ?? id;
  const workspaceName = (id: string) =>
    snapshot.workspaces.find((item) => item.id === id)?.name ?? id;
  const commitName = (id: string) =>
    findCommit(snapshot, id).commit.message.split("\n")[0];
  if (command.kind === "create_commit" && command.selections?.length)
    return `Create “${command.message}” in “${stackName(command.stack)}” using only the staged changes in ${command.selections.map((item) => item.path).join(", ")}. Unstaged changes remain in their current workspace.`;
  if (command.kind === "transfer" && command.selections?.length)
    return `Move the selected changes in ${command.selections.map((item) => item.path).join(", ")} to ${command.target === "unassigned" ? "Unstaged" : `commit “${commitName(command.target)}”`}. Other edits remain in their current workspace.`;
  if ("selection" in command && command.selection) {
    const target =
      command.kind === "transfer"
        ? command.target === "unassigned"
          ? "Unstaged"
          : `commit “${commitName(command.target)}”`
        : command.kind === "create_commit"
          ? `a new commit in “${stackName(command.stack)}”`
          : `a new commit after “${commitName(command.commit)}”`;
    return `Only the selected changes in ${command.selection.path} move to ${target}. ${command.selection.hunks ? `${command.selection.hunks.length} selected hunk(s).` : "The entire file is selected."} Other edits remain in their current workspace.`;
  }
  switch (command.kind) {
    case "delete_stack":
      return `Delete “${stackName(command.stack)}” and its commit entries, and remove it from every named combination. Unassigned edits remain. This can be undone.`;
    case "delete_workspace":
      return `Delete “${workspaceName(command.workspace)}”, including its independent unassigned edits. Other named combinations remain. This can be undone.`;
    case "switch_workspace":
      return `Switch the current folder to “${workspaceName(command.workspace)}”. Unfinished edits in “${workspace.name}” are retained independently.`;
    case "create_workspace":
      return `Create and switch to “${command.name.trim()}” in the current folder with no applied stacks or unassigned edits. Existing combinations retain their edits.`;
    case "apply_stack":
      return `Apply “${stackName(command.stack)}” to “${workspace.name}”. Existing unassigned edits remain.`;
    case "unapply_stack":
      return `Remove “${stackName(command.stack)}” from “${workspace.name}”. The stack, other combinations and unassigned edits remain.`;
    case "move_commit":
      return `Move “${commitName(command.commit)}” to ${command.branch ? `branch “${command.branch}” in ` : ""}“${stackName(command.stack)}”. Named combinations using these stacks will show the updated histories.`;
    case "squash":
      return `Combine “${commitName(command.commit)}” into “${commitName(command.target)}”, retaining both sets of changes and messages. The source commit entry is removed.`;
    case "uncommit":
      return `Remove “${commitName(command.commit)}” from its stack and return its changes to the unassigned edits in “${workspace.name}”.`;
    case "reword":
      return `Change the message of “${commitName(command.commit)}” to “${command.message}”. File changes are retained.`;
    case "rebase":
      return `Rebase “${stackName(command.stack)}” onto “${snapshot.base}”. Unassigned edits remain.`;
    case "rename_stack":
      return `Rename “${stackName(command.stack)}” to “${command.name.trim()}” in all combinations.`;
    case "create_stack":
      if (command.parent)
        return `Create “${command.name.trim()}” as a dependent branch above “${stackName(command.parent)}” in the same stack. Existing branch histories and Unstaged edits remain.`;
      return `Create “${command.name.trim()}” at “${snapshot.base}” and apply it to “${workspace.name}”.`;
    case "add_empty_commit":
      return `Insert an empty commit “${command.message}” ${command.placement} “${commitName(command.commit)}”. Existing file changes and Unstaged edits remain.`;
    case "create_branch":
      return `Create branch “${command.name.trim()}” ${command.placement} “${command.commit === snapshot.stacks.find((stack) => stack.id === command.stack)?.base ? command.commit : commitName(command.commit)}” in “${stackName(command.stack)}”. No commits are duplicated.`;
    case "rename_branch":
      return `Rename branch “${command.branch}” to “${command.name.trim()}”. Its commit history remains.`;
    case "delete_branch":
      return `Delete branch “${command.name}” and its own commits. Lower branch segments and Unstaged edits remain. This can be undone.`;
    case "squash_branch":
      return `Squash all commits in branch “${command.name}”, retaining their messages and files. Lower branch segments remain.`;
    case "delete_commit":
      return `Delete “${commitName(command.commit)}” and its file changes. Unstaged edits remain. This can be undone.`;
    case "edit_commit":
      return `Save the draft for “${commitName(command.commit)}” with message “${command.message}” and only explicitly staged additions. Existing files and other Unstaged edits remain.`;
    case "create_commit":
      return `Create an empty commit “${command.message}” in “${stackName(command.stack)}”. Unassigned edits remain.`;
    case "reorder_commit":
      if ("target" in command)
        return `Place “${commitName(command.commit)}” ${command.placement} “${commitName(command.target)}” within its stack. Identities and file changes are retained.`;
      return `Move “${commitName(command.commit)}” ${command.direction} within its stack. File changes are retained.`;
    case "resolve_conflict":
      return `Resolve ${command.path} with the reviewed text and retain it as an unassigned workspace edit.`;
    case "undo":
      return "Restore the state before the last command, including stack histories, applied combinations and unassigned edits.";
    default:
      return commandTitle(command);
  }
}

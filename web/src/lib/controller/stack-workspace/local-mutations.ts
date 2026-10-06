import { localFileSelections, localCommitSource } from "./local-selections.js";
import type { LocalStacksCommand } from "./local-transport.js";
import {
  WorkspaceCommandError,
  type WorkspaceSnapshot,
  type WorkspaceCommand,
} from "./types.js";

export const localMutationKinds = [
  "create_commit",
  "create_stack",
  "create_branch",
  "rename_stack",
  "rename_branch",
  "reword",
  "add_empty_commit",
  "transfer",
  "split",
  "edit_commit",
  "move_commit",
  "reorder_commit",
  "squash",
  "squash_branch",
  "uncommit",
  "rebase",
  "delete_stack",
  "delete_branch",
  "delete_commit",
  "resolve_conflict",
  "undo",
  "redo",
  "discard",
  "reconcile_workspace",
] as const;
export function localMutationCommand(
  snapshot: WorkspaceSnapshot,
  command: WorkspaceCommand,
): LocalStacksCommand | null {
  const stack = (id: string) => {
    const found = snapshot.stacks.find((s) => s.id === id);
    if (!found)
      throw new WorkspaceCommandError(
        "invalid",
        "The selected stack is unavailable.",
      );
    return found;
  };
  const commit = (id: string) => {
    if (!snapshot.stacks.some((s) => s.commits.some((c) => c.id === id)))
      throw new WorkspaceCommandError(
        "invalid",
        "This commit is outside the displayed stack history.",
      );
    return id;
  };
  switch (command.kind) {
    case "create_stack":
      return {
        kind: "create_stack",
        name: command.name.trim(),
        parent: command.parent ? stack(command.parent).name : null,
      };
    case "create_branch":
      if (!stack(command.stack).commits.some((c) => c.id === command.commit))
        throw new WorkspaceCommandError(
          "invalid",
          "This commit is outside the selected stack.",
        );
      return {
        kind: "create_branch",
        commit: commit(command.commit),
        name: command.name.trim(),
        placement: command.placement,
      };
    case "rename_stack":
      return {
        kind: "rename_branch",
        branch: stack(command.stack).name,
        name: command.name.trim(),
      };
    case "rename_branch":
      if (
        !stack(command.stack).heads.some((head) => head.name === command.branch)
      )
        throw new WorkspaceCommandError(
          "invalid",
          "The selected branch is unavailable.",
        );
      return {
        kind: "rename_branch",
        branch: command.branch,
        name: command.name.trim(),
      };
    case "reword":
      return {
        kind: "reword",
        commit: commit(command.commit),
        message: command.message,
      };
    case "add_empty_commit":
      return {
        kind: "empty",
        commit: commit(command.commit),
        message: command.message,
        placement: command.placement,
      };
    case "transfer":
      return {
        kind: "transfer",
        target: localCommitSource(snapshot, command.target),
        selections: localFileSelections(
          snapshot,
          command.selections ?? [command.selection],
        ),
      };
    case "split":
      return {
        kind: "split",
        commit: commit(command.commit),
        message: command.message,
        selections: localFileSelections(snapshot, [command.selection]),
      };
    case "edit_commit":
      return {
        kind: "edit",
        commit: commit(command.commit),
        message: command.message,
        selections: localFileSelections(snapshot, command.selections ?? []),
      };
    case "move_commit": {
      const selected = stack(command.stack);
      const head = selected.heads.find(
        (h) => h.name === (command.branch ?? selected.name),
      );
      if (!head)
        throw new WorkspaceCommandError(
          "invalid",
          "Choose a branch in the destination stack.",
        );
      return {
        kind: "move",
        commit: commit(command.commit),
        target: head.commit,
        placement: "above",
      };
    }
    case "reorder_commit": {
      if ("target" in command)
        return {
          kind: "move",
          commit: commit(command.commit),
          target: commit(command.target),
          placement: command.placement,
        };
      const owner = snapshot.stacks.find((s) =>
        s.commits.some((c) => c.id === command.commit),
      );
      const index =
        owner?.commits.findIndex((c) => c.id === command.commit) ?? -1;
      const target =
        owner?.commits[index + (command.direction === "up" ? 1 : -1)];
      if (!target)
        throw new WorkspaceCommandError(
          "invalid",
          "This commit is already at the branch boundary.",
        );
      return {
        kind: "move",
        commit: commit(command.commit),
        target: commit(target.id),
        placement: command.direction === "up" ? "above" : "below",
      };
    }
    case "squash":
      return {
        kind: "squash",
        commit: commit(command.commit),
        target: commit(command.target),
      };
    case "uncommit":
      return { kind: "uncommit", commit: commit(command.commit) };
    case "rebase":
      return {
        kind: "rebase",
        branch: command.branch ?? stack(command.stack).name,
      };
    case "squash_branch":
      if (!stack(command.stack).heads.some((h) => h.name === command.name))
        throw new WorkspaceCommandError(
          "invalid",
          "Choose an available branch.",
        );
      return { kind: "squash_branch", branch: command.name };
    case "delete_stack":
      return { kind: "delete_branch", branch: stack(command.stack).name };
    case "delete_branch":
      if (!stack(command.stack).heads.some((h) => h.name === command.name))
        throw new WorkspaceCommandError(
          "invalid",
          "Choose an available branch.",
        );
      return { kind: "delete_branch", branch: command.name };
    case "delete_commit":
      return { kind: "abandon", commit: commit(command.commit) };
    case "resolve_conflict":
      return { kind: "resolve", path: command.path, content: command.content };
    case "undo":
      return { kind: "undo" };
    case "redo":
      return { kind: "redo" };
    case "reconcile_workspace":
      return { kind: "reconcile" };
    case "discard":
      return {
        kind: "discard",
        selections: localFileSelections(snapshot, command.selections),
      };
    default:
      return null;
  }
}

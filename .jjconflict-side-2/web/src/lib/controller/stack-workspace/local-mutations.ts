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
    default:
      return null;
  }
}

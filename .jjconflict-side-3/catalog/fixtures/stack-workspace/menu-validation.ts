import { stackGroups } from "../../../web/src/lib/controller/stack-workspace/presentation.js";
import type {
  WorkspaceCommand,
  WorkspaceSnapshot,
} from "../../../web/src/lib/controller/stack-workspace/types.js";
import { findCommit } from "./changes.js";

export function validateMenuFixture(
  snapshot: WorkspaceSnapshot,
  command: WorkspaceCommand,
  fail: (message: string) => never,
) {
  if (
    command.kind === "create_stack" &&
    command.parent &&
    !snapshot.stacks.some((stack) => stack.id === command.parent)
  )
    fail("Choose an existing stack.");
  if (command.kind === "create_branch") {
    const stack = snapshot.stacks.find((stack) => stack.id === command.stack)!;
    if (
      command.commit !== stack.base &&
      findCommit(snapshot, command.commit).stack.id !== stack.id
    )
      fail("Choose a commit in this stack.");
    if (!["above", "below"].includes(command.placement))
      fail("Choose above or below.");
  }
  if (
    command.kind === "add_empty_commit" &&
    !["above", "below"].includes(command.placement)
  )
    fail("Choose above or below.");
  if (
    ["rename_branch", "delete_branch", "squash_branch"].includes(
      command.kind,
    ) &&
    "stack" in command &&
    "name" in command
  ) {
    const stack = snapshot.stacks.find((stack) => stack.id === command.stack)!;
    const name =
      command.kind === "rename_branch" ? command.branch : command.name;
    const group = stackGroups(stack).find((group) => group.head.name === name);
    if (!group) fail("Choose an existing branch.");
    if (
      command.kind !== "rename_branch" &&
      group.commits.some((commit) => commit.immutable)
    )
      fail("This branch contains immutable commits.");
    if (command.kind === "squash_branch" && group.commits.length < 2)
      fail("Choose a branch with multiple commits.");
  }
}

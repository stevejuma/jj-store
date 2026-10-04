import { stackGroups } from "../../../web/src/lib/controller/stack-workspace/presentation.js";
import type {
  WorkspaceCommand,
  WorkspaceSnapshot,
} from "../../../web/src/lib/controller/stack-workspace/types.js";
import {
  findCommit,
  mergeChanges,
  removeCommit,
  selectedChanges,
} from "./changes.js";
import { fixtureCommit } from "./data.js";

/** Local simulation of Changeyard positional and branch-segment menu actions. */
export function reduceMenuFixture(
  snapshot: WorkspaceSnapshot,
  command: WorkspaceCommand,
  nextId: () => string,
) {
  switch (command.kind) {
    case "add_empty_commit": {
      const { stack, commit } = findCommit(snapshot, command.commit);
      const next = fixtureCommit(nextId(), command.message, []);
      stack.commits.splice(
        stack.commits.indexOf(commit) + (command.placement === "above" ? 1 : 0),
        0,
        next,
      );
      if (command.placement === "above")
        for (const head of stack.heads)
          if (head.commit === commit.id) head.commit = next.id;
      break;
    }
    case "create_branch": {
      const stack = snapshot.stacks.find(
        (stack) => stack.id === command.stack,
      )!;
      const index = stack.commits.findIndex(
        (commit) => commit.id === command.commit,
      );
      const target =
        command.placement === "above"
          ? command.commit
          : (stack.commits[index - 1]?.id ?? stack.base);
      stack.heads.unshift({ name: command.name.trim(), commit: target });
      if (target === (stack.commits.at(-1)?.id ?? stack.base))
        stack.name = command.name.trim();
      break;
    }
    case "rename_branch": {
      const stack = snapshot.stacks.find(
        (stack) => stack.id === command.stack,
      )!;
      stack.heads.find((head) => head.name === command.branch)!.name =
        command.name.trim();
      if (stack.name === command.branch) stack.name = command.name.trim();
      break;
    }
    case "delete_branch":
    case "squash_branch": {
      const stack = snapshot.stacks.find(
        (stack) => stack.id === command.stack,
      )!;
      const group = stackGroups(stack).find(
        (group) => group.head.name === command.name,
      )!;
      if (command.kind === "squash_branch") {
        const target = group.commits.at(-1)!;
        for (const commit of group.commits.slice(0, -1).reverse()) {
          mergeChanges(target.files, commit.files);
          target.message += `\n\n${commit.message}`;
          removeCommit(snapshot, commit.id);
        }
      } else {
        for (const commit of group.commits) removeCommit(snapshot, commit.id);
        stack.heads = stack.heads.filter((head) => head.name !== command.name);
        stack.name = stack.heads[0]?.name ?? stack.name;
        if (!stack.heads.length) {
          snapshot.stacks = snapshot.stacks.filter(
            (item) => item.id !== stack.id,
          );
          for (const workspace of snapshot.workspaces)
            workspace.applied = workspace.applied.filter(
              (id) => id !== stack.id,
            );
        }
      }
      break;
    }
    case "delete_commit":
      removeCommit(snapshot, command.commit);
      break;
    case "edit_commit": {
      const { commit } = findCommit(snapshot, command.commit);
      for (const selection of command.selections ?? [])
        if (selection.source !== commit.id)
          mergeChanges(commit.files, [
            selectedChanges(snapshot, selection, true),
          ]);
      commit.message = command.message;
      break;
    }
  }
}

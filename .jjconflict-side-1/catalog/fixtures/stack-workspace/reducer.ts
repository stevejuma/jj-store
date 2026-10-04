import {
  activeWorkspace,
  normalizeAppliedStacks,
} from "../../../web/src/lib/controller/stack-workspace/presentation.js";
import type {
  WorkspaceCommand,
  WorkspaceFile,
  WorkspaceSnapshot,
} from "../../../web/src/lib/controller/stack-workspace/types.js";
import { reduceMenuFixture } from "./menu-reducer.js";
import { fixtureCommit } from "./data.js";
import { placedCommits } from "../../../web/src/lib/controller/stack-workspace/commit-placement.js";
import {
  findCommit,
  removeCommit,
  mergeChanges,
  selectedChanges,
  synchronizeHeads,
} from "./changes.js";

/** Simulation of logical selections; never writes or executes a repository. */
export function reduceFixture(
  snapshot: WorkspaceSnapshot,
  command: WorkspaceCommand,
  nextId: () => string,
) {
  const workspace = activeWorkspace(snapshot);
  const stack =
    "stack" in command
      ? snapshot.stacks.find((item) => item.id === command.stack)!
      : undefined;
  switch (command.kind) {
    case "apply_stack":
      workspace.applied = normalizeAppliedStacks([
        ...workspace.applied,
        command.stack,
      ]);
      break;
    case "unapply_stack":
      workspace.applied = workspace.applied.filter(
        (id) => id !== command.stack,
      );
      break;
    case "create_workspace": {
      const id = nextId();
      snapshot.workspaces.push({
        id,
        name: command.name.trim(),
        applied: [],
        unassigned: [],
      });
      snapshot.active = id;
      break;
    }
    case "switch_workspace":
      snapshot.active = command.workspace;
      break;
    case "delete_workspace":
      snapshot.workspaces = snapshot.workspaces.filter(
        (item) => item.id !== command.workspace,
      );
      break;
    case "create_stack": {
      if (command.parent) {
        const parent = snapshot.stacks.find(
          (stack) => stack.id === command.parent,
        )!;
        parent.heads.unshift({
          name: command.name.trim(),
          commit: parent.commits.at(-1)?.id ?? parent.base,
        });
        parent.name = command.name.trim();
        workspace.applied = normalizeAppliedStacks([
          ...workspace.applied,
          parent.id,
        ]);
        break;
      }
      const id = nextId();
      snapshot.stacks.push({
        id,
        name: command.name.trim(),
        base: snapshot.base,
        heads: [{ name: command.name.trim(), commit: snapshot.base }],
        commits: [],
        conflicts: false,
      });
      workspace.applied.push(id);
      break;
    }
    case "rename_stack":
      for (const head of stack!.heads)
        if (head.name === stack!.name) head.name = command.name.trim();
      stack!.name = command.name.trim();
      break;
    case "delete_stack":
      snapshot.stacks = snapshot.stacks.filter(
        (item) => item.id !== command.stack,
      );
      for (const item of snapshot.workspaces)
        item.applied = item.applied.filter((id) => id !== command.stack);
      break;
    case "create_commit": {
      const files: WorkspaceFile[] = [];
      for (const selection of command.selections ??
        (command.selection ? [command.selection] : []))
        mergeChanges(files, [selectedChanges(snapshot, selection, true)]);
      stack!.commits.push(fixtureCommit(nextId(), command.message, files));
      break;
    }
    case "reword":
      findCommit(snapshot, command.commit).commit.message = command.message;
      break;
    case "move_commit": {
      const source = findCommit(snapshot, command.commit);
      removeCommit(snapshot, command.commit);
      const head = stack!.heads.find((head) => head.name === command.branch);
      const index = head
        ? stack!.commits.findIndex((commit) => commit.id === head.commit) + 1
        : stack!.commits.length;
      stack!.commits.splice(index, 0, source.commit);
      if (head) head.commit = source.commit.id;
      break;
    }
    case "reorder_commit": {
      const { stack: owner, commit } = findCommit(snapshot, command.commit);
      if ("target" in command) {
        const sorted = placedCommits(
          owner,
          command.commit,
          command.target,
          command.placement,
        );
        removeCommit(snapshot, command.commit);
        owner.commits = sorted;
        if (command.placement === "above")
          for (const head of owner.heads)
            if (head.commit === command.target) head.commit = command.commit;
        break;
      }
      const index = owner.commits.indexOf(commit),
        other = index + (command.direction === "up" ? 1 : -1);
      [owner.commits[index], owner.commits[other]] = [
        owner.commits[other]!,
        owner.commits[index]!,
      ];
      break;
    }
    case "squash": {
      const source = findCommit(snapshot, command.commit),
        target = findCommit(snapshot, command.target);
      mergeChanges(target.commit.files, source.commit.files);
      target.commit.message += `\n\n${source.commit.message}`;
      removeCommit(snapshot, command.commit);
      break;
    }
    case "split": {
      const { stack: owner, commit } = findCommit(snapshot, command.commit);
      const file = selectedChanges(snapshot, command.selection, true);
      owner.commits.splice(
        owner.commits.indexOf(commit) + 1,
        0,
        fixtureCommit(nextId(), command.message, [file]),
      );
      break;
    }
    case "transfer":
      mergeChanges(
        command.target === "unassigned"
          ? workspace.unassigned
          : findCommit(snapshot, command.target).commit.files,
        (command.selections ?? [command.selection]).map((selection) =>
          selectedChanges(snapshot, selection, true),
        ),
      );
      break;
    case "uncommit": {
      const source = findCommit(snapshot, command.commit);
      mergeChanges(workspace.unassigned, source.commit.files);
      removeCommit(snapshot, command.commit);
      break;
    }
    case "rebase":
      stack!.base = snapshot.base;
      break;
    case "resolve_conflict":
      snapshot.conflicts = snapshot.conflicts.filter(
        (item) => item.path !== command.path,
      );
      mergeChanges(workspace.unassigned, [
        {
          path: command.path,
          hunks: [
            {
              id: nextId(),
              label: "workspace conflict resolution",
              patch: `@@ -1,1 +1,1 @@\n-conflicted\n+${command.content.trim()}\n`,
            },
          ],
        },
      ]);
      for (const item of snapshot.stacks) item.conflicts = false;
      break;
    case "undo":
      break;
    default:
      reduceMenuFixture(snapshot, command, nextId);
  }
  if (
    command.kind === "apply_stack" &&
    stack!.conflicts &&
    !snapshot.conflicts.length
  )
    snapshot.conflicts.push({
      path: "src/settings.ts",
      left: "export const search = true;\n",
      base: "export const search = false;\n",
      right: "export const search = 'workspace';\n",
    });
  if (command.kind === "unapply_stack" && stack!.conflicts)
    snapshot.conflicts = [];
  synchronizeHeads(snapshot);
}

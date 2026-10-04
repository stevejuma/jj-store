import type { Bookmark } from "../../projects/presentation.js";
import type { RepositoryLogRevision } from "../../projects/repository-log.js";
import type {
  WorkspaceSnapshot,
  WorkspaceStack,
  WorkspaceCommand,
} from "./types.js";

/** Adapted from Changeyard branches-stack-model.ts; source is recorded in provenance. */
export function normalizeAppliedStacks(values: readonly string[]) {
  return [...new Set(values.map((value) => value.trim()).filter(Boolean))];
}
export function activeWorkspace(snapshot: WorkspaceSnapshot) {
  const workspace = snapshot.workspaces.find(
    (item) => item.id === snapshot.active,
  );
  if (!workspace) throw new Error("The selected workspace is unavailable.");
  return workspace;
}
export function appliedStacks(snapshot: WorkspaceSnapshot) {
  const byId = new Map(snapshot.stacks.map((stack) => [stack.id, stack]));
  return normalizeAppliedStacks(activeWorkspace(snapshot).applied)
    .map((id) => byId.get(id))
    .filter((stack): stack is WorkspaceStack => Boolean(stack));
}
/** Group each bookmark's own commits, newest first, without duplicating lower segments. */
export function stackGroups(stack: WorkspaceStack) {
  return stack.heads.map((head, index) => {
    const tip = stack.commits.findIndex((commit) => commit.id === head.commit);
    const lower = stack.heads[index + 1];
    const boundary = lower
      ? stack.commits.findIndex((commit) => commit.id === lower.commit)
      : -1;
    return {
      head,
      commits:
        tip < 0
          ? []
          : stack.commits
              .slice(
                boundary >= 0 && boundary <= tip ? boundary + 1 : 0,
                tip + 1,
              )
              .reverse(),
    };
  });
}
/** Feed the existing repository branch picker; bookmark target conflicts remain distinct from workspace conflicts. */
export function stackBookmarks(snapshot: WorkspaceSnapshot): Bookmark[] {
  return snapshot.stacks.flatMap((stack) =>
    stack.heads.map((head) => {
      const tip = stack.commits.find((commit) => commit.id === head.commit);
      return {
        name: head.name,
        targets: [head.commit],
        workspace: activeWorkspace(snapshot).applied.includes(stack.id),
        description: tip?.message,
        author: tip?.author,
        authorEmail: tip?.email,
        updatedAt: tip?.createdAt,
      };
    }),
  );
}
/** Present native stack identities using the existing application graph. */
export function stackRevisions(
  stack: WorkspaceStack,
  commits = stack.commits,
): RepositoryLogRevision[] {
  return commits.map((commit) => ({
    eventId: commit.id,
    changeId: commit.changeId,
    commitId: commit.id,
    parents:
      stack.commits.findIndex((item) => item.id === commit.id) > 0
        ? [
            stack.commits[
              stack.commits.findIndex((item) => item.id === commit.id) - 1
            ]!.id,
          ]
        : [],
    message: commit.message,
    reason: "checkpoint",
    createdAt: commit.createdAt,
    root: stack.base,
    author: commit.author,
    authorEmail: commit.email,
    empty: !commit.files.length,
    bookmarks: [],
    workspaces: [],
    conflicted: false,
    divergent: false,
  }));
}
export function commandTitle(command: WorkspaceCommand) {
  return (
    {
      apply_stack: "Apply stack",
      unapply_stack: "Unapply stack",
      create_stack: "Create stack",
      rename_stack: "Rename stack",
      delete_stack: "Delete stack",
      create_workspace: "Create workspace",
      switch_workspace: "Switch workspace",
      delete_workspace: "Delete workspace",
      create_commit: "Create commit",
      reword: "Reword commit",
      create_branch: "Create branch",
      rename_branch: "Rename branch",
      delete_branch: "Delete branch",
      squash_branch: "Squash all commits",
      add_empty_commit: "Add empty commit",
      delete_commit: "Delete commit",
      edit_commit: "Edit commit",
      move_commit: "Move commit",
      reorder_commit: "Reorder commit",
      squash: "Squash commits",
      split: "Split commit",
      transfer: "Assign selected changes",
      uncommit: "Uncommit",
      rebase: "Rebase stack",
      resolve_conflict: "Resolve conflict",
      undo: "Undo last command",
    } satisfies Record<WorkspaceCommand["kind"], string>
  )[command.kind];
}

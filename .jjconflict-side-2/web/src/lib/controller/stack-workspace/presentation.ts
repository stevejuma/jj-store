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
export function selectedWorkspace(snapshot: WorkspaceSnapshot) {
  return snapshot.workspaces.find((item) => item.id === snapshot.active);
}
export function activeWorkspace(snapshot: WorkspaceSnapshot) {
  const workspace = selectedWorkspace(snapshot);
  if (!workspace) throw new Error("The selected workspace is unavailable.");
  return workspace;
}
export function appliedStacks(snapshot: WorkspaceSnapshot) {
  const byId = new Map(snapshot.stacks.map((stack) => [stack.id, stack]));
  return normalizeAppliedStacks(selectedWorkspace(snapshot)?.applied ?? [])
    .map((id) => byId.get(id))
    .filter((stack): stack is WorkspaceStack => Boolean(stack));
}
/** Group each bookmark's own commits, newest first, without duplicating lower segments. */
export function stackGroups(stack: WorkspaceStack) {
  if (stack.segments)
    return stack.segments.map((segment) => ({
      head: segment.head,
      commits: segment.commits,
    }));
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
  if (snapshot.bookmarks)
    return snapshot.bookmarks.map((bookmark) => ({
      ...bookmark,
      workspace: snapshot.stacks.some(
        (stack) =>
          stack.heads.some((head) => head.name === bookmark.name) &&
          Boolean(selectedWorkspace(snapshot)?.applied.includes(stack.id)),
      ),
    }));
  return snapshot.stacks.flatMap((stack) =>
    stack.heads.map((head) => {
      const tip = stack.commits.find((commit) => commit.id === head.commit);
      return {
        name: head.name,
        targets: [head.commit],
        workspace:
          selectedWorkspace(snapshot)?.applied.includes(stack.id) ?? false,
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
  const parents = new Map(
    stack.commits.map((commit, index) => [
      commit.id,
      index > 0 ? [stack.commits[index - 1]!.id] : [],
    ]),
  );
  return commits.map((commit) => ({
    eventId: commit.id,
    changeId: commit.changeId,
    commitId: commit.id,
    parents: commit.graph?.parents ?? parents.get(commit.id) ?? [],
    message: commit.message,
    reason: "checkpoint",
    createdAt: commit.createdAt,
    root: stack.base,
    author: commit.author,
    authorEmail: commit.email,
    empty: commit.graph ? false : !commit.files.length,
    bookmarks: commit.graph?.bookmarks ?? [],
    workspaces: commit.graph?.workspaces ?? [],
    conflicted: commit.graph?.conflicted ?? false,
    divergent: commit.graph?.divergent ?? false,
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

/** Preserve loaded selections when accepted commands remove or replace heads. */
export function retainedWorkspaceSelection(
  snapshot: WorkspaceSnapshot,
  selection: {
    selectedStack: string | null;
    selectedBranch: string | null;
    selectedCommit: string | null;
  },
) {
  const selectedStack = snapshot.stacks.some(
    (stack) => stack.id === selection.selectedStack,
  )
    ? selection.selectedStack
    : (selectedWorkspace(snapshot)?.applied[0] ?? null);
  const stack = snapshot.stacks.find((stack) => stack.id === selectedStack);
  const selectedBranch = stack?.heads.some(
    (head) => head.name === selection.selectedBranch,
  )
    ? selection.selectedBranch
    : (stack?.name ?? null);
  const selectedCommit = snapshot.stacks.some((stack) =>
    stack.commits.some((commit) => commit.id === selection.selectedCommit),
  )
    ? selection.selectedCommit
    : null;
  return { selectedStack, selectedBranch, selectedCommit };
}

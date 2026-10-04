import {
  WorkspaceCommandError,
  cloneWorkspace,
  type WorkspaceFile,
  type WorkspaceSelection,
  type WorkspaceSnapshot,
} from "../../../web/src/lib/controller/stack-workspace/types.js";
import { activeWorkspace } from "../../../web/src/lib/controller/stack-workspace/presentation.js";

export function findCommit(snapshot: WorkspaceSnapshot, id: string) {
  for (const stack of snapshot.stacks) {
    const commit = stack.commits.find((item) => item.id === id);
    if (commit) return { stack, commit };
  }
  throw new WorkspaceCommandError(
    "invalid",
    "The selected commit no longer exists.",
  );
}
export function selectedChanges(
  snapshot: WorkspaceSnapshot,
  selection: WorkspaceSelection,
  remove = false,
) {
  const files =
    selection.source === "unassigned"
      ? activeWorkspace(snapshot).unassigned
      : findCommit(snapshot, selection.source).commit.files;
  const file = files.find((item) => item.path === selection.path);
  if (!file)
    throw new WorkspaceCommandError(
      "invalid",
      "The selected file is unavailable.",
    );
  const hunks = selection.hunks;
  if (
    hunks &&
    (!hunks.length ||
      hunks.some((id) => !file.hunks.some((hunk) => hunk.id === id)))
  )
    throw new WorkspaceCommandError("invalid", "The selected hunks are stale.");
  const selected = {
    path: file.path,
    hunks: cloneWorkspace(
      file.hunks.filter((hunk) => !hunks || hunks.includes(hunk.id)),
    ),
  };
  if (remove) {
    file.hunks = file.hunks.filter(
      (hunk) => !selected.hunks.some((item) => item.id === hunk.id),
    );
    if (!file.hunks.length) files.splice(files.indexOf(file), 1);
  }
  return selected;
}
export function mergeChanges(
  destination: WorkspaceFile[],
  files: WorkspaceFile[],
) {
  for (const source of files) {
    let file = destination.find((item) => item.path === source.path);
    if (!file) {
      file = { path: source.path, hunks: [] };
      destination.push(file);
    }
    for (const hunk of source.hunks)
      if (!file.hunks.some((item) => item.id === hunk.id))
        file.hunks.push(cloneWorkspace(hunk));
  }
}
/** Retarget removed identities to their previous surviving parent. */
export function removeCommit(snapshot: WorkspaceSnapshot, id: string) {
  const { stack, commit } = findCommit(snapshot, id);
  const index = stack.commits.indexOf(commit);
  const parent = stack.commits[index - 1]?.id ?? stack.base;
  for (const head of stack.heads) if (head.commit === id) head.commit = parent;
  stack.commits.splice(index, 1);
}
export function synchronizeHeads(snapshot: WorkspaceSnapshot) {
  for (const stack of snapshot.stacks) {
    const tip = stack.commits.at(-1);
    stack.heads = stack.heads.filter(
      (head) =>
        head.commit === stack.base ||
        stack.commits.some((commit) => commit.id === head.commit),
    );
    const head = stack.heads.find((item) => item.name === stack.name);
    if (tip && head) head.commit = tip.id;
    else if (tip) stack.heads.unshift({ name: stack.name, commit: tip.id });
    stack.heads.sort(
      (a, b) =>
        stack.commits.findIndex((c) => c.id === b.commit) -
        stack.commits.findIndex((c) => c.id === a.commit),
    );
  }
}

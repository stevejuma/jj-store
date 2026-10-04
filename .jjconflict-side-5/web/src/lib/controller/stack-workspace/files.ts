import { activeWorkspace, appliedStacks, stackGroups } from "./presentation.js";
import {
  cloneWorkspace,
  type WorkspaceFile,
  type WorkspaceSnapshot,
  type WorkspaceSelection,
} from "./types.js";

export const workspaceBranchSource = (stack: string, branch: string) =>
  `branch:${JSON.stringify([stack, branch])}`;
export const workspaceAggregateSource = (source: string) =>
  source === "combined" ||
  source.startsWith("stack:") ||
  source.startsWith("branch:");
export function workspaceSourceStack(
  snapshot: WorkspaceSnapshot,
  source: string,
) {
  if (source.startsWith("branch:")) {
    try {
      const [id, name] = JSON.parse(source.slice(7));
      return snapshot.stacks.find(
        (stack) =>
          stack.id === id && stack.heads.some((head) => head.name === name),
      );
    } catch {
      return undefined;
    }
  }
  return snapshot.stacks.find(
    (stack) =>
      source === `stack:${stack.id}` ||
      stack.commits.some((commit) => commit.id === source),
  );
}
function sourceCommits(snapshot: WorkspaceSnapshot, source: string) {
  const stack = workspaceSourceStack(snapshot, source);
  if (source.startsWith("branch:")) {
    if (!stack) return [];
    const [, name] = JSON.parse(source.slice(7));
    return (
      stackGroups(stack).find((group) => group.head.name === name)?.commits ??
      []
    );
  }
  return source.startsWith("stack:")
    ? (stack?.commits ?? [])
    : snapshot.stacks
        .flatMap((stack) => stack.commits)
        .filter((commit) => commit.id === source);
}
/** Presentation union, not a repository merge or an executable patch. */
export function combinedWorkspaceFiles(snapshot: WorkspaceSnapshot) {
  return unionFiles([
    ...appliedStacks(snapshot).flatMap((stack) =>
      stack.commits.flatMap((commit) => commit.files),
    ),
    ...activeWorkspace(snapshot).unassigned,
  ]);
}
function unionFiles(source: WorkspaceFile[]) {
  const files = new Map<string, WorkspaceFile>();
  for (const file of source) {
    const existing = files.get(file.path);
    if (!existing) files.set(file.path, cloneWorkspace(file));
    else
      for (const hunk of file.hunks)
        if (!existing.hunks.some((item) => item.id === hunk.id))
          existing.hunks.push(cloneWorkspace(hunk));
  }
  return [...files.values()].sort((a, b) => a.path.localeCompare(b.path));
}
export function workspaceSourceFiles(
  snapshot: WorkspaceSnapshot,
  source: string,
) {
  if (source.startsWith("stack:") || source.startsWith("branch:"))
    return unionFiles(
      sourceCommits(snapshot, source).flatMap((commit) => commit.files),
    );
  if (source === "combined") return combinedWorkspaceFiles(snapshot);
  if (source === "unassigned") return activeWorkspace(snapshot).unassigned;
  return (
    snapshot.stacks
      .flatMap((stack) => stack.commits)
      .find((commit) => commit.id === source)?.files ?? []
  );
}

/** Resolve a displayed union hunk only when one mutable origin owns it. */
export function workspaceHunkSelection(
  snapshot: WorkspaceSnapshot,
  source: string,
  path: string,
  hunk: string,
): WorkspaceSelection | null {
  const commits =
    source === "combined"
      ? appliedStacks(snapshot).flatMap((stack) => stack.commits)
      : sourceCommits(snapshot, source);
  const owns = (files: WorkspaceFile[]) =>
    files.some(
      (file) =>
        file.path === path && file.hunks.some((item) => item.id === hunk),
    );
  const owners = commits.filter((commit) => owns(commit.files));
  const unassigned =
    (source === "combined" || source === "unassigned") &&
    owns(activeWorkspace(snapshot).unassigned);
  if (owners.length + Number(unassigned) !== 1 || owners[0]?.immutable)
    return null;
  return {
    source: unassigned ? "unassigned" : owners[0]!.id,
    path,
    hunks: [hunk],
  };
}

/** A whole displayed file is draggable only when all hunks share one mutable origin. */
export function workspaceFileSelection(
  snapshot: WorkspaceSnapshot,
  source: string,
  path: string,
): WorkspaceSelection | null {
  const file = workspaceSourceFiles(snapshot, source).find(
    (item) => item.path === path,
  );
  const selections = file?.hunks.map((hunk) =>
    workspaceHunkSelection(snapshot, source, path, hunk.id),
  );
  const owner = selections?.[0]?.source;
  if (!owner || selections?.some((item) => !item || item.source !== owner))
    return null;
  return { source: owner, path, hunks: file!.hunks.map((hunk) => hunk.id) };
}
/** Preserve exact branch/commit selections only while their originating stack owns them. */
export function retainedStackSelections(
  snapshot: WorkspaceSnapshot,
  selections: Record<string, string>,
) {
  return Object.fromEntries(
    Object.entries(selections).filter(([id, selection]) =>
      snapshot.stacks.some(
        (stack) =>
          stack.id === id &&
          (workspaceSourceStack(snapshot, selection)?.id === id ||
            stack.commits.some((commit) => commit.id === selection)),
      ),
    ),
  );
}

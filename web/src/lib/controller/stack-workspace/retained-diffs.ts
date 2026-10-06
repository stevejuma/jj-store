import { workspaceSourceFiles, workspaceSourceStack } from "./files.js";
import { cloneWorkspace } from "./types.js";
import type {
  WorkspaceSnapshot,
  WorkspaceOpenDiff,
  WorkspaceFile,
} from "./types.js";
export function retainedWorkspaceDiffs(
  snapshot: WorkspaceSnapshot,
  previousActive: string | null | undefined,
  diffs: WorkspaceOpenDiff[],
  filesFor?: (source: string) => WorkspaceFile[],
) {
  const retained = new Map<string, WorkspaceOpenDiff>();
  for (const diff of diffs) {
    if (previousActive !== snapshot.active && diff.origin === "working")
      continue;
    const file = (
      filesFor
        ? filesFor(diff.source)
        : workspaceSourceFiles(snapshot, diff.source)
    ).find((file) => file.path === diff.path);
    if (!file && !filesFor) continue;
    const stack = workspaceSourceStack(
      snapshot,
      diff.source,
      diff.origin.startsWith("stack-") ? diff.origin.slice(6) : undefined,
    );
    const origin =
      diff.origin === "working"
        ? "working"
        : stack
          ? `stack-${stack.id}`
          : null;
    if (!origin) continue;
    retained.set(origin, {
      ...diff,
      origin,
      ...(diff.hunks && file && file.content?.status !== "unloaded"
        ? {
            hunks: diff.hunks.filter((id) =>
              file.hunks.some((hunk) => hunk.id === id),
            ),
          }
        : {}),
    });
  }
  return [...retained.values()];
}
export function reconcileSourceDiffs(
  diffs: WorkspaceOpenDiff[],
  source: string,
  files: WorkspaceFile[],
  complete: boolean,
) {
  return diffs.flatMap((diff) => {
    if (diff.source !== source) return [diff];
    const file = files.find((item) => item.path === diff.path);
    if (!file && complete) return [];
    return [
      {
        ...diff,
        ...(diff.hunks && file && file.content?.status !== "unloaded"
          ? {
              hunks: diff.hunks.filter((id) =>
                file.hunks.some((hunk) => hunk.id === id),
              ),
            }
          : {}),
      },
    ];
  });
}

/** Retain exact immutable sources while the new operation's graph is preparing.
 * Presentation segments and cursors are reissued, so the lane shows a skeleton. */
export function retainStackContents(
  previous: WorkspaceSnapshot | null,
  next: WorkspaceSnapshot,
) {
  if (previous?.target?.commit !== next.target?.commit) return [];
  return next.stacks.flatMap((stack) => {
    const prior = previous?.stacks.find((item) => item.id === stack.id);
    if (
      stack.segments ||
      !stack.heads[0]?.commit ||
      prior?.heads[0]?.commit !== stack.heads[0].commit
    )
      return [];
    stack.commits = cloneWorkspace(prior.commits);
    return stack.commits;
  });
}

import type { StackCommit, WorkspaceStack } from "./types.js";
/** Derive branch boundaries only from issued native ancestry, never row order. */
export function projectStackRange(
  stack: WorkspaceStack,
  base: string,
  commits: StackCommit[],
  cursor?: string,
): WorkspaceStack {
  const head =
    stack.topologyHeads?.[0] ??
    stack.heads.find((head) => head.name === stack.name)!;
  const byId = new Map(commits.map((commit) => [commit.id, commit]));
  const retained = stack.commits.filter((commit) => !byId.has(commit.id));
  const path: StackCommit[] = [],
    seen = new Set<string>();
  let next = head.commit,
    linear = true;
  while (byId.has(next) && !seen.has(next)) {
    const commit = byId.get(next)!;
    seen.add(next);
    path.push(commit);
    if (
      commit.graph?.parents.length !== 1 ||
      commit.graph.edges.some((edge) => edge.kind !== "direct")
    ) {
      linear = false;
      break;
    }
    next = commit.graph.parents[0]!;
  }
  if (!linear || path.length !== commits.length)
    return {
      ...stack,
      heads: [head],
      commits: [...retained, ...[...commits].reverse()],
      segments: [
        { head, baseCommit: cursor ? null : base, commits, nextCursor: cursor },
      ],
    };
  const boundaries = [{ head, index: 0 }];
  for (const [index, commit] of path.entries()) {
    if (!index) continue;
    const names = commit
      .graph!.bookmarks.filter(
        (bookmark) =>
          bookmark.added &&
          !bookmark.removed &&
          !bookmark.conflicted &&
          !bookmark.remote &&
          !bookmark.name.startsWith("jj-store/workspaces/") &&
          bookmark.name !== stack.name &&
          bookmark.name !== stack.base,
      )
      .map((bookmark) => bookmark.name)
      .sort();
    const known = stack.topologyHeads?.find(
      (head) => head.commit === commit.id,
    );
    if (known || names[0])
      boundaries.push({
        head: known ?? {
          name: names[0]!,
          commit: commit.id,
          ...(names.length > 1 ? { aliases: names.slice(1) } : {}),
        },
        index,
      });
  }
  return {
    ...stack,
    heads: boundaries.map((boundary) => boundary.head),
    commits: [...retained, ...[...path].reverse()],
    segments: boundaries.map((boundary, index) => ({
      head: boundary.head,
      baseCommit: boundaries[index + 1]?.head.commit ?? (cursor ? null : base),
      commits: path.slice(boundary.index, boundaries[index + 1]?.index),
      ...(index === boundaries.length - 1 ? { nextCursor: cursor } : {}),
    })),
  };
}

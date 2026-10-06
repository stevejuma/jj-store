import type {
  StackCommit,
  StackHead,
  WorkspaceStack,
  WorkspaceSnapshot,
} from "./types.js";

/** Native parents and topology own boundaries. Drawing edges may end at a range/page. */
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
  let heads = stack.topologyHeads;
  // Compatibility for graph providers without the separate topology query. Preserve
  // each proven card above a merge; put its remaining DAG in the lowest card.
  if (!heads) {
    heads = [head];
    let next = head.commit;
    const seen = new Set<string>();
    while (byId.has(next) && !seen.has(next)) {
      seen.add(next);
      const commit = byId.get(next)!;
      const names = (commit.graph?.bookmarks ?? [])
        .filter(
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
      if (next !== head.commit && names.length)
        heads.push({
          name: names[0]!,
          commit: next,
          ...(names.length > 1 ? { aliases: names.slice(1) } : {}),
        });
      if (commit.graph?.parents.length !== 1) break;
      next = commit.graph.parents[0]!;
    }
  }
  const used = new Set<string>();
  const segments = heads.map((head: StackHead, index) => {
    const lower = heads[index + 1]?.commit;
    const boundary = head.baseCommit !== undefined ? head.baseCommit : lower;
    const rows: StackCommit[] = [];
    let next = head.commit;
    while (byId.has(next) && next !== boundary && !used.has(next)) {
      const commit = byId.get(next)!;
      used.add(next);
      rows.push(commit);
      if (commit.graph?.parents.length !== 1) break;
      next = commit.graph.parents[0]!;
    }
    if (index === heads.length - 1) {
      // This includes both sides of ambiguous merge ancestry without linearizing it.
      rows.push(...commits.filter((commit) => !used.has(commit.id)));
      for (const row of rows) used.add(row.id);
    }
    const last = rows.at(-1);
    const inferred =
      !cursor && last?.graph?.parents.length === 1
        ? last.graph.parents[0]!
        : null;
    return {
      head,
      baseCommit:
        boundary ??
        (head.baseCommit === null
          ? null
          : (inferred ?? (!cursor && !last?.graph ? base : null))),
      commits: rows,
      unloaded:
        Boolean(cursor) && !byId.has(head.commit) && head.commit !== boundary,
      ...(index === heads.length - 1 ? { nextCursor: cursor } : {}),
    };
  });
  return {
    ...stack,
    heads,
    commits: segments.flatMap((segment) => segment.commits).reverse(),
    segments,
  };
}

/** Share issued rows with raw bookmark owners without rekeying drafts or selections. */
export function shareStackSegments(
  snapshot: WorkspaceSnapshot,
  stack: WorkspaceStack,
) {
  if (!stack.segments) return;
  for (const [index, segment] of stack.segments.entries()) {
    for (const name of [segment.head.name, ...(segment.head.aliases ?? [])]) {
      const owner = snapshot.stacks.find((item) => item.name === name);
      if (!owner || owner === stack) continue;
      const ownHead = owner.topologyHeads?.[0];
      if (!ownHead || ownHead.commit !== segment.head.commit) continue;
      const segments = stack.segments
        .slice(index)
        .map((part, i) => (i === 0 ? { ...part, head: ownHead } : part));
      if (
        segments.at(-1)?.nextCursor &&
        owner.segments &&
        owner.commits.length >
          segments.reduce((count, part) => count + part.commits.length, 0)
      )
        continue;
      owner.segments = segments;
      owner.heads = segments.map((part) => part.head);
      owner.commits = segments.flatMap((part) => part.commits).reverse();
    }
  }
}

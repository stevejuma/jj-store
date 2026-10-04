/** Loaded fragments retain missing parents; traversal avoids recursive depth. */
export function workHierarchy<T extends { id: string }>(
  rows: readonly T[],
  parent: (row: T) => string | null,
): { row: T; depth: number }[] {
  const ids = new Set(rows.map((row) => row.id));
  const children = new Map<string | null, T[]>();
  for (const row of rows) {
    const key = parent(row),
      root = key && ids.has(key) ? key : null;
    const group = children.get(root) ?? [];
    group.push(row);
    children.set(root, group);
  }
  const result: { row: T; depth: number }[] = [],
    seen = new Set<string>();
  const stack = (children.get(null) ?? [])
    .slice()
    .reverse()
    .map((row) => ({ row, depth: 0 }));
  while (stack.length) {
    const entry = stack.pop()!;
    if (seen.has(entry.row.id)) continue;
    seen.add(entry.row.id);
    result.push(entry);
    for (const row of (children.get(entry.row.id) ?? []).slice().reverse())
      stack.push({ row, depth: entry.depth + 1 });
  }
  // Even malformed transport fragments remain visible rather than hanging.
  for (const row of rows) if (!seen.has(row.id)) result.push({ row, depth: 0 });
  return result;
}

import {
  parsePatchToRows,
  type UnifiedDiffRow,
} from "@lapismd/design-core/diff";
import type { WorkspaceFile } from "./types.js";

/** Display-only context partitions; original patches and native tokens stay intact. */
export function workspaceHunkContext(file: WorkspaceFile): Map<string, string> {
  const result = new Map(file.hunks.map((hunk) => [hunk.id, hunk.patch]));
  if (!file.patch || !file.hunks.length) return result;
  const rows = parsePatchToRows(file.patch);
  const indices = new Map(rows.map((row, index) => [row.key, index]));
  const ranges = file.hunks.map((hunk) => {
    const part = parsePatchToRows(hunk.patch);
    const start = indices.get(part[0]?.key ?? ""),
      end = indices.get(part.at(-1)?.key ?? "");
    if (
      start === undefined ||
      end === undefined ||
      end < start ||
      end - start + 1 !== part.length ||
      part.some(
        (row, index) =>
          row.key !== rows[start + index]?.key ||
          row.text !== rows[start + index]?.text,
      )
    )
      return null;
    return { start, end: end + 1 };
  });
  if (
    ranges.some(
      (range, index) =>
        !range || (index > 0 && range.start < ranges[index - 1]!.end),
    )
  )
    return result;
  for (let index = 0; index < ranges.length; index++) {
    const range = ranges[index]!;
    const start =
      index === 0 ? 0 : Math.floor((ranges[index - 1]!.end + range.start) / 2);
    const end =
      index === ranges.length - 1
        ? rows.length
        : Math.floor((range.end + ranges[index + 1]!.start) / 2);
    const part = rows.slice(start, end);
    // Never let omitted/foreign changes become part of an authorized hunk card.
    if (
      rows
        .slice(start, range.start)
        .concat(rows.slice(range.end, end))
        .some((row) => row.variant !== "context")
    )
      continue;
    result.set(file.hunks[index]!.id, patch(file.path, part));
  }
  return result;
}
function patch(path: string, rows: UnifiedDiffRow[]) {
  const old = rows.filter((row) => row.variant !== "added"),
    next = rows.filter((row) => row.variant !== "removed");
  return (
    `diff --git a/${path} b/${path}\n--- a/${path}\n+++ b/${path}\n@@ -${old[0]?.oldLineNumber ?? 0},${old.length} +${next[0]?.lineNumber ?? 0},${next.length} @@\n` +
    rows
      .map(
        (row) =>
          `${row.variant === "context" ? " " : row.variant === "added" ? "+" : "-"}${row.text}\n`,
      )
      .join("")
  );
}

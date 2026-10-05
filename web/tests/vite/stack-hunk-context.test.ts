import { expect, test } from "vitest";
import { parsePatchToRows } from "@lapismd/design-core/diff";
import { workspaceHunkContext } from "../../src/lib/controller/stack-workspace/hunk-context.js";
import type { WorkspaceFile } from "../../src/lib/controller/stack-workspace/types.js";

function patch(start: number, end: number, changes: number[]) {
  const lines = Array.from({ length: end - start + 1 }, (_, index) => {
    const line = start + index;
    return changes.includes(line)
      ? `-old ${line}\n+new ${line}\n`
      : ` line ${line}\n`;
  }).join("");
  return `diff --git a/context.ts b/context.ts\n--- a/context.ts\n+++ b/context.ts\n@@ -${start},${end - start + 1} +${start},${end - start + 1} @@\n${lines}`;
}
function file(): WorkspaceFile {
  return {
    path: "context.ts",
    patch: patch(1, 35, [8, 25]),
    hunks: [
      { id: "native:first", label: "First", patch: patch(5, 11, [8]) },
      { id: "native:second", label: "Second", patch: patch(22, 28, [25]) },
    ],
  };
}
test("hunk context partitions all prefix, intervening and suffix lines without changing native descriptors", () => {
  const input = file(),
    original = structuredClone(input);
  const result = workspaceHunkContext(input);
  expect([...result.keys()]).toEqual(input.hunks.map((h) => h.id));
  const rows = [...result.values()].flatMap(parsePatchToRows);
  expect(rows).toEqual(parsePatchToRows(input.patch!));
  expect(new Set(rows.map((row) => row.key)).size).toBe(rows.length);
  expect(input).toEqual(original);
});
test("unmapped, overlapping and reordered hunks retain their original patches", () => {
  for (const transform of [
    (input: WorkspaceFile) => {
      input.patch = undefined;
    },
    (input: WorkspaceFile) => {
      input.hunks.reverse();
    },
    (input: WorkspaceFile) => {
      input.hunks[1]!.patch = input.hunks[0]!.patch;
    },
    (input: WorkspaceFile) => {
      input.hunks[0]!.patch = input.hunks[0]!.patch.replace("old 8", "foreign");
    },
  ]) {
    const input = file();
    transform(input);
    expect([...workspaceHunkContext(input).values()]).toEqual(
      input.hunks.map((h) => h.patch),
    );
  }
});
test("context partitions never include a foreign change absent from an issued hunk", () => {
  const input = file();
  input.patch = patch(1, 35, [2, 8, 25, 34]);
  expect([...workspaceHunkContext(input).values()]).toEqual(
    input.hunks.map((h) => h.patch),
  );
});

import { expect, test } from "vitest";
import { navigationTurns } from "../../../web/src/lib/ui/store-agent-chat-body/navigation-turns.js";
import { patchLines } from "../../../web/src/lib/ui/store-agent-chat-body/patch-lines.js";

test("turn totals distinguish file headers from triple-prefix content inside hunks", () => {
  const text = [
    "diff --git a/example.md b/example.md",
    "--- a/example.md",
    "+++ b/example.md",
    "@@ -1,2 +1,2 @@",
    "--- old heading 👩🏽‍💻",
    "+++ new heading café",
    "-old paragraph",
    "+new paragraph",
  ].join("\n");
  const [turn] = navigationTurns([
    {
      id: "tools",
      kind: "tools",
      label: "Apply patch",
      blocks: [],
      tools: [
        {
          id: "patch",
          revision: "v1",
          title: "Apply patch example.md",
          status: "complete",
          raw: {},
          blocks: [
            { id: "file", revision: "v1", label: "Patch", kind: "patch", text },
          ],
        },
      ],
    },
  ]);
  expect(turn.additions).toBe(2);
  expect(turn.deletions).toBe(2);
});

test("captured hunks keep triple prefixes as content and preserve Unicode offsets", () => {
  const text = "@@ captured hunk\n--- 👩🏽‍💻\n+++ café\n unchanged\n";
  const lines = patchLines(text);
  expect(lines.map((line) => line.kind)).toEqual([
    "hunk",
    "deletion",
    "addition",
    "context",
    "context",
  ]);
  for (const line of lines)
    expect(text.slice(line.from, line.from + line.text.length)).toBe(line.text);
});

test("a new diff resets header classification without changing captured source", () => {
  const lines = patchLines(
    "@@ hunk\n+first\ndiff --git a/b b/b\n--- a/b\n+++ b/b\n@@ hunk\n+second",
  );
  expect(lines.map((line) => line.kind)).toEqual([
    "hunk",
    "addition",
    "context",
    "context",
    "context",
    "hunk",
    "addition",
  ]);
});

import { expect, test } from "vitest";
import {
  middleRange,
  middleDecorations,
} from "../../../web/src/lib/ui/store-agent-chat-body/middle-fold.js";
import { commentAnchor } from "../../../web/src/lib/ui/store-agent-chat-body/anchors.js";
const block = {
  id: "large",
  revision: "v1",
  kind: "code" as const,
  label: "Captured code",
  text: Array.from(
    { length: 60 },
    (_, i) => `const café = '👩🏽‍💻'; // line ${i + 1}`,
  ).join("\n"),
};
test("folds only long code and patches at complete line boundaries", () => {
  expect(
    middleRange({ ...block, text: Array(32).fill("line").join("\n") }),
  ).toBeUndefined();
  expect(
    middleRange({ ...block, text: Array(33).fill("line").join("\n") }),
  ).toMatchObject({ lines: 13 });
  expect(middleRange({ ...block, kind: "markdown" })).toBeUndefined();
  expect(middleRange({ ...block, kind: "patch" })).toEqual(middleRange(block));
});
test("retains the exact Unicode source and offsets across a folded middle", () => {
  const original = block.text,
    range = middleRange(block)!;
  expect(range.lines).toBe(40);
  expect(block.text.slice(0, range.from)).toMatch(/line 12\n$/);
  expect(block.text.slice(range.to)).toMatch(
    /^const café = '👩🏽‍💻'; \/\/ line 53/,
  );
  const anchor = commentAnchor(block, { from: range.from, to: range.from });
  expect(anchor.quote).toContain("line 13");
  expect(block.text).toBe(original);
});
test("reveals a folded discussion while stale anchors cannot reveal a new revision", () => {
  const anchor = commentAnchor(block, {
    from: middleRange(block)!.from,
    to: middleRange(block)!.from,
  });
  const comments = {
    disclosures: {},
    threads: [{ id: "thread", label: "Code", anchor, comments: [] }],
    active: undefined,
  };
  expect(middleDecorations(block, comments).size).toBe(0);
  expect(middleDecorations({ ...block, revision: "v2" }, comments).size).toBe(
    1,
  );
});

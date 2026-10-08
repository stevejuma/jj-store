import { expect, test } from "vitest";
import fixture from "./hello-world.json";
import { deltaChatItems } from "./adapter.js";
import { deltaChatThreads } from "./discussions.js";
import { navigationTurns } from "../../../web/src/lib/ui/store-agent-chat-body/navigation-turns.js";
import { validAnchor } from "../../../web/src/lib/ui/store-agent-chat-body/anchors.js";
import { markdownBlocks } from "../../../web/src/lib/ui/store-agent-chat-body/markdown-blocks.js";
test("Hello World preserves three turns, two comments-only batches and three linked replies", () => {
  const items = deltaChatItems(fixture),
    threads = deltaChatThreads(fixture, items);
  expect(navigationTurns(items)).toHaveLength(3);
  expect(
    items.filter((i) => i.role === "user" && !i.blocks.length),
  ).toHaveLength(2);
  expect(items.filter((i) => i.replyContext)).toHaveLength(3);
  expect(threads).toHaveLength(4);
  expect(
    threads.flatMap((t) => t.comments).filter((c) => c.status === "replied"),
  ).toHaveLength(3);
  expect(threads.at(-1)?.comments[0]).toMatchObject({
    status: "draft",
    text: "is this correct?",
  });
  expect(
    items
      .flatMap((i) => i.blocks)
      .some((b) => b.text.includes("<comment-reply>")),
  ).toBe(false);
  expect(JSON.stringify(fixture)).not.toMatch(
    /stevejuma|846dde457303|1dd1461ce13c/,
  );
  for (const thread of threads) {
    const blocks = items.flatMap((i) => [
      ...i.blocks,
      ...(i.tools ?? []).flatMap((t) => t.blocks),
    ]);
    expect(blocks.some((b) => validAnchor(b, thread.anchor))).toBe(true);
  }
});
test("source spans retain trimming, Unicode and fenced-code offsets", () => {
  const source =
    "  Café 👩🏽‍💻.\n\n```rust\nlet x = 1;\n```\n\n  Last paragraph.  ";
  for (const block of markdownBlocks("b", "r", "Source", source))
    expect(
      source.slice(
        block.sourceFrom ?? 0,
        (block.sourceFrom ?? 0) + block.text.length,
      ),
    ).toBe(block.text);
  const altered = structuredClone(fixture);
  altered.discussions[0].target.quote = "Different text";
  expect(() => deltaChatThreads(altered, deltaChatItems(altered))).toThrow(
    /quote/,
  );
});
test("navigation previews delivered and pending comment batches without duplicating turns", () => {
  const items = deltaChatItems(fixture);
  const threads = deltaChatThreads(fixture, items);
  const turns = navigationTurns(items, threads);
  expect(turns).toHaveLength(4);
  expect(turns[1]).toMatchObject({
    prompt: "Comment: show me contents of this file",
    commentsOnly: true,
  });
  expect(turns[2].comments).toHaveLength(2);
  expect(turns[3]).toMatchObject({
    id: "pending-comments",
    prompt: "Comment: is this correct?",
    pending: true,
  });
});

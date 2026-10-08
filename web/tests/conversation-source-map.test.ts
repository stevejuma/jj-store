import assert from "node:assert/strict";
import test from "node:test";
import {
  exactSource,
  sourceAnchor,
  type ConversationTextSource,
} from "../src/lib/conversations/source-map.js";
import { markdownBlocks } from "../src/lib/ui/store-agent-chat-body/markdown-blocks.js";
import type { ChatBodyBlock } from "../src/lib/ui/store-agent-chat-body/model.js";
const target = {
  type: "human_message" as const,
  thread_id: "1".repeat(32),
  message_id: "2".repeat(32),
  revision: { kind: "human_message_revision" as const, id: "3".repeat(64) },
};
test("exact source maps trimmed Markdown, fences and Unicode in both directions", () => {
  const text = "  Hello 🦀 é.\n\n```ts\nconst value = '🦀';\n```\n\nEnd. ";
  for (const block of markdownBlocks("m", "rev", "Message", text)) {
    const source: ConversationTextSource = {
      target,
      part: null,
      text,
      from: block.sourceFrom ?? 0,
      revision: "rev",
    };
    const anchor = {
      blockId: block.id,
      revision: "rev",
      from: 0,
      to: block.text.length,
      quote: block.text,
    };
    const selector = exactSource(block, anchor, source);
    assert.deepEqual(sourceAnchor(block, source, selector), anchor);
    assert.equal(
      new TextDecoder().decode(
        new TextEncoder()
          .encode(text)
          .slice(
            Number(selector.selection!.start_byte),
            Number(selector.selection!.end_byte),
          ),
      ),
      block.text,
    );
  }
});
test("source maps refuse stale revisions, invented text, split surrogate pairs and changed hashes", () => {
  const block: ChatBodyBlock = {
    id: "b",
    revision: "r",
    kind: "code",
    label: "Code",
    text: "é🦀",
  };
  const source: ConversationTextSource = {
    target,
    part: null,
    text: block.text,
    from: 0,
    revision: "r",
  };
  const anchor = {
    blockId: "b",
    revision: "r",
    from: 0,
    to: 3,
    quote: block.text,
  };
  assert.throws(() =>
    exactSource(
      block,
      { ...anchor, to: 2, quote: block.text.slice(0, 2) },
      source,
    ),
  );
  assert.throws(() =>
    exactSource(block, anchor, { ...source, revision: "new" }),
  );
  assert.throws(() =>
    exactSource(block, anchor, { ...source, text: "changed" }),
  );
  const selector = exactSource(block, anchor, source);
  assert.equal(
    sourceAnchor(block, source, {
      ...selector,
      selection: { ...selector.selection!, selected_sha256: "0".repeat(64) },
    }),
    undefined,
  );
});

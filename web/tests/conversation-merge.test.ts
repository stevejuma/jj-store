import { test } from "node:test";
import assert from "node:assert/strict";
import { mergeOrderedItems } from "../src/lib/conversations/merge-projections.js";
import { sameChatOrigin } from "../src/lib/controller/agent-chat-context.js";
const item = (id: string, observedAt?: number) => ({
  id,
  observedAt,
  kind: "message" as const,
  label: id,
  blocks: [],
});
test("merge preserves within-capture ordering and uses only recorded times across streams", () => {
  assert.deepEqual(
    mergeOrderedItems(
      [item("human", 1), item("later", 4)],
      [item("agent", 2), item("tool", 3)],
    ).map((i) => i.id),
    ["human", "agent", "tool", "later"],
  );
  assert.deepEqual(
    mergeOrderedItems(
      [item("human")],
      [item("old agent"), item("old tool")],
    ).map((i) => i.id),
    ["human", "old agent", "old tool"],
  );
});
test("main thread and individual reply conversations cannot reuse each other", () => {
  const base = {
    scope: "thread:repo:id",
    home: "repo",
    thread: "id",
    agent_target: null,
    context: "{}",
  };
  const reply = {
    ...base,
    context: JSON.stringify({ reply: { message: "one" } }),
  };
  assert.equal(sameChatOrigin(base, reply), false);
  assert.equal(
    sameChatOrigin(reply, {
      ...reply,
      context: JSON.stringify({ reply: { message: "two" } }),
    }),
    false,
  );
  assert.equal(
    sameChatOrigin(reply, {
      ...reply,
      context: JSON.stringify({ reply: { message: "one", content: "edited" } }),
    }),
    true,
  );
});

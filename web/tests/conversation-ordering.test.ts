import assert from "node:assert/strict";
import test from "node:test";
import { activityRows } from "../src/lib/ui/store-agent-execution/presentation.js";
import { orderedAssistantRows } from "../src/lib/conversations/ordered-activity.js";
import type { AgentEvent } from "../src/lib/controller/agent-execution-types.js";
const events = (values: object[]) =>
  values.map(
    (payload, i) =>
      ({ sequence: String(i + 1), payload, committed: true }) as AgentEvent,
  );
const chunk = (text: string) => ({
  type: "activity",
  requestId: "r",
  update: { sessionUpdate: "agent_message_chunk", content: { text } },
});
test("assistant chunks preserve tool boundaries and exact whole-message offsets", () => {
  const input = events([
    chunk("First 🦀"),
    {
      type: "activity",
      requestId: "r",
      toolSnapshot: { toolCallId: "t", title: "Read" },
    },
    chunk("Second"),
    chunk(" answer"),
    {
      type: "activity",
      requestId: "r",
      toolSnapshot: { toolCallId: "t", title: "Read complete" },
    },
    {
      type: "record",
      requestId: "r",
      assistantText: "First 🦀Second answer",
      record: { status: "completed" },
    },
  ]);
  const rows = activityRows(input);
  assert.deepEqual(
    rows.map((r) => r.kind),
    ["assistant", "tool", "assistant", "record"],
  );
  assert.deepEqual(
    rows
      .filter((r) => r.kind === "assistant")
      .map((r) => [r.sourceFrom, r.sourceTo]),
    [
      [0, 8],
      [8, 21],
    ],
  );
  assert.equal(rows[2]!.sourceRowId, "r/assistant");
});
test("legacy answers retain explicit unknown ordering, inconsistent streams have no exact selection", () => {
  const legacy = events([
    {
      type: "record",
      requestId: "r",
      assistantText: "Old answer",
      record: { status: "completed" },
    },
  ]);
  assert.equal(
    activityRows(legacy).find((r) => r.kind === "assistant")?.orderingKnown,
    false,
  );
  const mismatch = events([
    chunk("stream"),
    {
      type: "record",
      requestId: "r",
      assistantText: "different",
      record: { status: "completed" },
    },
  ]);
  assert.equal(
    activityRows(mismatch).find((r) => r.kind === "assistant")?.sourceFrom,
    undefined,
  );
});

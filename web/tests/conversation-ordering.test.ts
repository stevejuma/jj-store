import assert from "node:assert/strict";
import test from "node:test";
import { activityRows } from "../src/lib/ui/store-agent-execution/presentation.js";
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

test("each assistant segment retains its first valid time without borrowing neighboring times", () => {
  const input = events([
    chunk("unknown"),
    chunk(" then known"),
    chunk(" then later"),
    { type: "activity", requestId: "r", toolSnapshot: { toolCallId: "t" } },
    chunk("invalid"),
    chunk(" still invalid"),
    chunk(" known"),
    { type: "permission", requestId: "r" },
    chunk("untimed"),
  ]);
  for (const [index, millis] of [
    [1, 3000],
    [2, 4000],
    [4, NaN],
    [5, -1],
    [6, 5000],
  ] as const)
    input[index]!.observed_at_millis = millis;
  const rows = activityRows(input).filter((row) => row.kind === "assistant");
  assert.deepEqual(
    rows.map((row) => row.observedAtMillis),
    [3000, 5000, undefined],
  );
  assert.deepEqual(
    rows.map((row) => [row.sourceFrom, row.sourceTo]),
    [
      [0, 29],
      [29, 56],
      [56, 63],
    ],
  );
});

test("legacy final answers use only a valid observation time from their own record", () => {
  for (const millis of [
    undefined,
    NaN,
    -1,
    1.5,
    Number.MAX_SAFE_INTEGER,
    0,
    6000,
  ]) {
    const input = events([
      {
        type: "record",
        requestId: "r",
        assistantText: "Legacy",
        record: { status: "completed" },
      },
    ]);
    input[0]!.observed_at_millis = millis;
    const row = activityRows(input).find((row) => row.kind === "assistant")!;
    assert.equal(
      row.observedAtMillis,
      millis === 0 || millis === 6000 ? millis : undefined,
    );
    assert.equal(row.orderingKnown, false);
  }
});

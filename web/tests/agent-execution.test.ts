import { test } from "node:test";
import assert from "node:assert/strict";
import {
  activityRows,
  runtimeChoices,
  queuedRequests,
} from "../src/lib/ui/store-agent-execution/presentation.js";
import type { AgentEvent } from "../src/lib/controller/agent-execution-types.js";
const event = (sequence: number, payload: unknown): AgentEvent => ({
  sequence,
  payload,
  committed: false,
});
test("queue cancellation targets only prompts that have not started or settled", () => {
  assert.deepEqual(
    [
      ...queuedRequests([
        event(1, { type: "queued", requestId: "running" }),
        event(2, { type: "started", requestId: "running" }),
        event(3, { type: "queued", requestId: "settled" }),
        event(4, {
          type: "record",
          requestId: "settled",
          record: { status: "cancelled" },
        }),
        event(5, { type: "queued", requestId: "retained" }),
      ]),
    ],
    ["retained"],
  );
});
test("runtime choices prefer advertised configuration values and retain groups", () => {
  const session = {
    modes: { availableModes: [{ id: "old", name: "Old" }] },
    configOptions: [
      { category: "mode", options: [{ value: "ask", name: "Ask" }] },
      {
        category: "model",
        options: [
          {
            name: "Vendor",
            options: [{ value: "vendor-model", name: "Model" }],
          },
        ],
      },
      { category: "thought_level", options: [{ value: "high", name: "High" }] },
    ],
  };
  assert.deepEqual(runtimeChoices(session, "mode"), [
    { id: "ask", name: "Ask" },
  ]);
  assert.deepEqual(runtimeChoices(session, "model"), [
    { id: "vendor-model", name: "Model" },
  ]);
  assert.deepEqual(runtimeChoices(session, "thought_level"), [
    { id: "high", name: "High" },
  ]);
});
test("live text and tool deltas retain turn identity and exact permission lifetimes", () => {
  const rows = activityRows([
    event(1, { type: "prompt", requestId: "one", text: "Fix it" }),
    event(2, {
      type: "activity",
      requestId: "one",
      update: {
        sessionUpdate: "agent_message_chunk",
        content: { text: "Hello " },
      },
    }),
    event(3, {
      type: "activity",
      requestId: "one",
      update: {
        sessionUpdate: "agent_message_chunk",
        content: { text: "world" },
      },
    }),
    event(4, {
      type: "activity",
      requestId: "one",
      toolSnapshot: {
        toolCallId: "same",
        title: "Read",
        status: "in_progress",
      },
    }),
    event(5, {
      type: "activity",
      requestId: "one",
      toolSnapshot: { toolCallId: "same", title: "Read", status: "completed" },
    }),
    event(6, {
      type: "permission",
      requestId: "one",
      permissionId: "permission",
      request: {},
    }),
    event(7, {
      type: "permission_resolved",
      requestId: "one",
      permissionId: "permission",
      optionId: "reject",
    }),
    event(8, {
      type: "activity",
      requestId: "two",
      toolSnapshot: { toolCallId: "same", title: "Write" },
    }),
  ]);
  assert.equal(rows.find((r) => r.kind === "assistant")?.text, "Hello world");
  assert.equal(rows.filter((r) => r.kind === "tool").length, 2);
  assert.equal(
    rows.find((r) => r.id === "one/tool/same")?.value.status,
    "completed",
  );
  assert.equal(rows.filter((r) => r.kind === "permission").length, 0);
});

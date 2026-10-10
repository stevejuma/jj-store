import { test } from "node:test";
import assert from "node:assert/strict";
import {
  sessionActivity,
  sessionStopAction,
} from "../src/lib/controller/agent-session-activity.js";
import type {
  AgentSession,
  AgentEvent,
  AgentOutcome,
} from "../src/lib/controller/agent-execution-types.js";
const session = (state = "running", connected = true) =>
  ({ id: "session", state, connected }) as AgentSession;
const event = (type: string, requestId = "11".repeat(16)) =>
  ({ payload: { type, requestId } }) as AgentEvent;
const pending = { body: { session_id: "session", request_id: "request" } };
const outcome = (state: string) =>
  ({ request_id: "request", state }) as AgentOutcome;

test("an opening handshake can be stopped before the session becomes connected", () => {
  assert.deepEqual(sessionStopAction(session("opening", false), []), {
    action: "end",
  });
});

test("interruption outranks a stale dispatch receipt and does not animate Starting", () => {
  assert.deepEqual(
    sessionActivity(
      session("interrupted", false),
      pending,
      outcome("dispatching"),
    ),
    { state: "interrupted", label: "Interrupted", streaming: false },
  );
  assert.equal(
    sessionActivity(
      session("interrupted", false),
      pending,
      outcome("outcome_unknown"),
    )?.label,
    "Outcome unknown",
  );
  assert.equal(
    sessionActivity(
      session("idle"),
      { body: { session_id: "other" } },
      outcome("dispatching"),
    ),
    undefined,
  );
  assert.equal(sessionActivity(session("opening"))?.label, "Starting");
  assert.equal(sessionActivity(session())?.label, "Working");
  assert.equal(sessionActivity(session("running", false)), undefined);
});

test("Stop uses the exact uncompleted turn, ignores handshake and terminal history, and never targets disconnected sessions", () => {
  const events = [
    event("started"),
    event("record"),
    event("queued", "22".repeat(16)),
    event("queued", "open-session"),
  ];
  assert.deepEqual(sessionStopAction(session(), events), {
    action: "cancel",
    target: "22".repeat(16),
  });
  assert.equal(
    sessionStopAction(session("idle"), [event("started"), event("record")]),
    undefined,
  );
  assert.equal(
    sessionStopAction(session("interrupted", false), events),
    undefined,
  );
  assert.deepEqual(sessionStopAction(session("opening"), []), {
    action: "end",
  });
  assert.deepEqual(
    sessionStopAction(session(), [
      event("started"),
      event("queued", "22".repeat(16)),
    ]),
    { action: "cancel", target: "11".repeat(16) },
  );
  assert.deepEqual(sessionStopAction(session("opening"), [event("prompt")]), {
    action: "end",
  });
  assert.equal(
    sessionStopAction(session("idle"), [
      event("prompt"),
      event("opened", "open-current"),
    ]),
    undefined,
  );
});

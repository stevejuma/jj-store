import assert from "node:assert/strict";
import test from "node:test";
import type { Model } from "../src/lib/api/client.js";
import {
  agentSessionTiming,
  formatAgentSessionDuration,
} from "../src/lib/collaboration/presentation.js";

function summary(
  first_activity_at_millis: number | null,
  last_activity_at_millis: number | null,
): Model<"AgentSessionSummary"> {
  return {
    session_id: "11".repeat(16),
    snapshot_id: "22".repeat(32),
    integration: "codex",
    state: "ended",
    parent_session_id: null,
    first_activity_at_millis,
    last_activity_at_millis,
    coverage: {
      completeness: "complete",
      gaps: 0,
      redacted_fields: 0,
      truncated: false,
    },
    evidence_count: 1,
    event_batch_count: 1,
    transcript_count: 1,
    tool_count: 1,
  };
}

test("agent session timing uses captured first and last activity", () => {
  assert.deepEqual(
    agentSessionTiming(summary(1_700_000_000_000, 1_700_000_125_000)),
    {
      startedAt: 1_700_000_000,
      durationMs: 125_000,
      durationLabel: "2m 5s",
    },
  );
  assert.equal(agentSessionTiming(summary(null, null)), undefined);
  assert.equal(
    agentSessionTiming(summary(1_700_000_125_000, 1_700_000_000_000)),
    undefined,
  );
});

test("agent session durations stay compact", () => {
  assert.equal(formatAgentSessionDuration(0), "0s");
  assert.equal(formatAgentSessionDuration(2_000), "2s");
  assert.equal(formatAgentSessionDuration(3_600_000), "1h");
  assert.equal(formatAgentSessionDuration(90_000_000), "1d 1h");
});

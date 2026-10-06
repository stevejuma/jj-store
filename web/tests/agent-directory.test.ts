import { test } from "node:test";
import assert from "node:assert/strict";
import { agentAvailability } from "../src/lib/ui/community-agents/model.js";
import type { AgentSession } from "../src/lib/controller/agent-execution-types.js";
const session = (
  state: string,
  connected: boolean,
  setup_id: string | null = "helper",
) => ({ state, connected, setup_id }) as AgentSession;
test("presence is associated with stable setup identity, not runner or historical titles", () => {
  assert.equal(
    agentAvailability("helper", [session("running", true, "other")], false)
      .label,
    "Offline",
  );
  assert.equal(
    agentAvailability("helper", [session("idle", false)], false).label,
    "Offline",
  );
  assert.equal(
    agentAvailability("helper", [session("running", true, null)], false).label,
    "Offline",
  );
});
test("presence prioritizes active sessions and never treats failed refresh as offline", () => {
  const rows = [session("ended", false), session("opening", false)];
  assert.equal(agentAvailability("helper", rows, false).label, "Starting");
  rows.push(session("idle", true));
  assert.equal(agentAvailability("helper", rows, false).label, "Online");
  rows.push(session("running", true));
  assert.equal(agentAvailability("helper", rows, false).label, "Working");
  assert.equal(agentAvailability("helper", rows, true).label, "Unknown");
});

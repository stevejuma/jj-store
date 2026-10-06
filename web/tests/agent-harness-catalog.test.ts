import test from "node:test";
import assert from "node:assert/strict";
import { harnessCatalog } from "../src/lib/ui/store-agent-execution/harness-catalog.js";
import { runtimeChoices } from "../src/lib/ui/store-agent-execution/presentation.js";
import type { AgentSession } from "../src/lib/controller/agent-execution-types.js";
const session = (id: string, runner: string, connected: boolean, at: number) =>
  ({
    id,
    runner,
    connected,
    last_activity_at_millis: at,
    adapter_session_id: id,
    capabilities: {
      session: { models: { availableModels: [{ modelId: id, name: id }] } },
    },
  }) as AgentSession;

test("offline editor retains its latest negotiated harness catalog without mixing harnesses", () => {
  const catalog = harnessCatalog(
    [
      session("old", "codex", false, 1),
      session("new", "codex", false, 2),
      session("foreign", "cursor", true, 3),
    ],
    "codex",
  );
  assert.equal(catalog.remembered, true);
  assert.deepEqual(runtimeChoices(catalog.facts, "model"), [
    { id: "new", name: "new" },
  ]);
});
test("current negotiation supersedes remembered models even when it advertises no models", () => {
  const live = session("live", "codex", true, 1);
  live.capabilities = { session: {} };
  const catalog = harnessCatalog(
    [session("remembered", "codex", false, 2), live],
    "codex",
  );
  assert.equal(catalog.remembered, false);
  assert.deepEqual(runtimeChoices(catalog.facts, "model"), []);
  assert.equal(harnessCatalog([], "codex").session, undefined);
});

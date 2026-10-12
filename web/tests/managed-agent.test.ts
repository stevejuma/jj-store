import assert from "node:assert/strict";
import test from "node:test";
import type { Model } from "../src/lib/api/client.js";
import type {
  AgentSession,
  AgentStatus,
} from "../src/lib/controller/agent-execution-types.js";
import {
  managedRecordedAgent,
  managedWorkspaceAgent,
} from "../src/lib/collaboration/managed-agent.js";
import {
  agentLastActivityAt,
  agentMember,
} from "../src/lib/collaboration/presentation.js";
import { sessionItems } from "../src/lib/ui/store-agent-execution/session-presentation.js";
import { storeRouteFromHash } from "../src/lib/controller/store-route-codec.js";
const repository = "11".repeat(16),
  operation = "22".repeat(64),
  recordedId = "33".repeat(16);
const recorded = {
  session_id: recordedId,
  integration: "owned_controller",
  state: "ended",
  coverage: {
    completeness: "complete",
    gaps: 0,
    redacted_fields: 0,
    truncated: false,
  },
} as Model<"AgentSessionSummary">;
function captured(overrides: Partial<AgentSession> = {}): AgentSession {
  return {
    id: "44".repeat(16),
    repository,
    operation,
    workspace_operation: "77".repeat(64),
    setup_id: "deleted-agent",
    title: "Nyra",
    avatar: "🐝",
    runner: "codex",
    connected: false,
    state: "ended",
    recorded_session_ids: [recordedId],
    repository_bindings: [],
    workspace_name: "agent/workspace 日本語",
    ...overrides,
  } as AgentSession;
}
test("recorded identity requires exact repository, integration and a unique admitted alias", () => {
  const session = captured();
  assert.equal(managedRecordedAgent(repository, recorded, [session]), session);
  assert.equal(agentMember(recorded, session).name, "Nyra");
  assert.equal(agentMember(recorded, session).id, recordedId);
  assert.equal(
    managedRecordedAgent("55".repeat(16), recorded, [session]),
    undefined,
  );
  assert.equal(
    managedRecordedAgent(repository, { ...recorded, integration: "codex" }, [
      session,
    ]),
    undefined,
  );
  assert.equal(
    managedRecordedAgent(repository, recorded, [
      captured({ recorded_session_ids: [] }),
    ]),
    undefined,
  );
  assert.equal(
    managedRecordedAgent(repository, recorded, [
      session,
      captured({ id: "66".repeat(16) }),
    ]),
    undefined,
  );
  // No name or setup lookup is needed; historical identity survives setup deletion and cleanup.
  assert.equal(
    managedRecordedAgent(repository, recorded, [captured({ state: "cleaned" })])
      ?.title,
    "Nyra",
  );
});
test("workspace avatars require exact repository and ready retained workspace names", () => {
  const session = captured();
  assert.equal(
    managedWorkspaceAgent(repository, session.workspace_name!, [session]),
    session,
  );
  assert.equal(
    managedWorkspaceAgent(repository, "agent-" + session.id, [session]),
    undefined,
  );
  assert.equal(
    managedWorkspaceAgent("55".repeat(16), session.workspace_name!, [session]),
    undefined,
  );
  assert.equal(
    managedWorkspaceAgent(repository, session.workspace_name!, [
      captured({ state: "cleaned" }),
    ]),
    undefined,
  );
  assert.equal(
    managedWorkspaceAgent(repository, session.workspace_name!, [
      session,
      session,
    ]),
    undefined,
  );
});
test("last activity remains available without a first activity timestamp", () => {
  assert.equal(
    agentLastActivityAt({
      ...recorded,
      first_activity_at_millis: null,
      last_activity_at_millis: 1700000000000,
    }),
    1700000000,
  );
  for (const value of [null, undefined, NaN, Infinity, -1, 1.2, 9e15])
    assert.equal(
      agentLastActivityAt({
        ...recorded,
        last_activity_at_millis: value,
      } as Model<"AgentSessionSummary">),
      undefined,
    );
});
test("inbox repository links name the repository and round trip exact workspace and operation", () => {
  const status = {
    sessions: [captured()],
    setups: [],
  } as unknown as AgentStatus;
  const item = sessionItems(status, null, false, [
    { id: repository, name: "Local repository" },
  ])[0]!;
  assert.equal(item.repository.label, "Open Local repository");
  assert.equal(item.repository.id, repository);
  assert.equal(item.repository.name, "Local repository");
  const result = storeRouteFromHash(item.repository.href);
  assert.equal(result.kind, "route");
  if (result.kind === "route") {
    assert.equal(result.route.repository, repository);
    assert.equal(result.route.workspace, status.sessions[0]!.workspace_name);
    assert.equal(
      result.route.snapshotOperationId,
      status.sessions[0]!.workspace_operation,
    );
  }
  status.sessions[0]!.workspace_name = null;
  assert.equal(
    new URLSearchParams(
      sessionItems(status)[0]!.repository.href.split("?")[1],
    ).has("workspace"),
    false,
  );
});

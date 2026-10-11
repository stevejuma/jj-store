import assert from "node:assert/strict";
import test from "node:test";
import { repositoryExecutionSessions } from "../src/lib/ui/store-agent-execution/repository-sessions.js";
import {
  fixtureExecutionStatus,
  fixtureExecutionSetup,
} from "../../catalog/fixtures/agent-execution-status.js";
import { agentSessions } from "../../catalog/fixtures/collaboration.js";
import type { AgentSession } from "../src/lib/controller/agent-execution-types.js";

function status() {
  return fixtureExecutionStatus({
    repository: "aa".repeat(16),
    operation: "22".repeat(64),
    commit: "33".repeat(20),
    setup: fixtureExecutionSetup(null),
    status: "Online",
    preparing: false,
    alternativeAgent: false,
    permission: false,
    workspacePath: "/fixture/workspace",
    lastActivityAt: 1,
    sessionPreferences: undefined,
    agentModeReady: true,
    advertisedModels: true,
  });
}

test("explicit_code_bindings_replace_the_metadata_home_for_session_rows", () => {
  const value = status(),
    session = value.sessions[0];
  session.repository_bindings = ["bb", "cc"].map(
    (byte) =>
      ({
        repository: byte.repeat(16),
        target_id: "fixture",
        profile: "local_jj_git",
        workspace: "/fixture/workspace",
        selected_operation: session.operation,
        base_commit: session.base_commit,
        operation: session.operation,
        commit: session.base_commit,
      }) satisfies AgentSession["repository_bindings"][number],
  );
  assert.deepEqual(repositoryExecutionSessions(value, session.repository), []);
  for (const byte of ["bb", "cc"])
    assert.deepEqual(repositoryExecutionSessions(value, byte.repeat(16)), [
      session,
    ]);
  assert.deepEqual(repositoryExecutionSessions(value, "dd".repeat(16)), []);
});

test("legacy_unbound_sessions_use_the_exact_primary_repository", () => {
  const value = status(),
    session = value.sessions[0];
  session.repository_bindings = [];
  assert.deepEqual(repositoryExecutionSessions(value, session.repository), [
    session,
  ]);
  assert.deepEqual(repositoryExecutionSessions(value, "bb".repeat(16)), []);
});

test("only_exact_admitted_recorded_aliases_suppress_a_duplicate_live_row", () => {
  const value = status(),
    session = value.sessions[0];
  session.repository_bindings = [];
  session.recorded_session_ids = [agentSessions[0].session_id];
  assert.equal(
    repositoryExecutionSessions(value, session.repository, agentSessions)
      .length,
    1,
  );
  const recorded = { ...agentSessions[0], integration: "owned_controller" };
  assert.deepEqual(
    repositoryExecutionSessions(value, session.repository, [recorded]),
    [],
  );
});

test("ambiguous_recorded_aliases_do_not_hide_distinct_interactive_sessions", () => {
  const value = status(),
    session = value.sessions[0];
  session.repository_bindings = [];
  session.recorded_session_ids = [agentSessions[0].session_id];
  value.sessions.push({ ...session, id: "bb".repeat(16) });
  assert.equal(
    repositoryExecutionSessions(value, session.repository, [
      { ...agentSessions[0], integration: "owned_controller" },
    ]).length,
    2,
  );
});

test("interactive_row_search_preserves_the_repository_boundary", () => {
  const value = status(),
    session = value.sessions[0];
  session.repository_bindings = [];
  session.conversation_title = "Review UI layouts";
  assert.deepEqual(
    repositoryExecutionSessions(value, session.repository, [], "  UI LAYOUTS "),
    [session],
  );
  assert.deepEqual(
    repositoryExecutionSessions(value, "bb".repeat(16), [], "UI layouts"),
    [],
  );
  assert.deepEqual(
    repositoryExecutionSessions(value, session.repository, [], "missing title"),
    [],
  );
});

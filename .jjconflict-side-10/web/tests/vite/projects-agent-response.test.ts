import { expect, test } from "vitest";
import { ApiError } from "../../src/lib/api/client.js";
import type { StoreController } from "../../src/lib/controller/contracts.js";
import type { StoreDataSource } from "../../src/lib/controller/native-data-source.js";
import { createProjectAgentResponse } from "../../src/lib/controller/project-agent-response.svelte.js";
import { agentDetail } from "../../../catalog/fixtures/collaboration.js";

function fixture() {
  const repository = agentDetail.snapshot.repo_id;
  let accessible = true;
  const base = {
    state: {
      repositories: [{ id: repository, name: "Code", permission: "read" }],
      accessMode: "local_trusted",
      storageReady: true,
      now: 0,
      member: { id: "local", name: "Local" },
    },
    actions: {},
  } as unknown as StoreController;
  const source = {
    repository,
    profile: "native",
    snapshot: async () => ({ heads: ["22".repeat(64)] }),
    agentSession: async () => ({ ...agentDetail, transcripts: [] }),
    reviewThreads: async () => ({ threads: [], next_cursor: null }),
  } as unknown as StoreDataSource;
  const response = createProjectAgentResponse(source, () => ({
    ...base,
    state: {
      ...base.state,
      repositories: accessible ? base.state.repositories : [],
    },
  }));
  return {
    response,
    source,
    revoke: () => {
      accessible = false;
    },
  };
}

test("Agent response rejects late selection and clears protected data on revocation", async () => {
  const { response, source, revoke } = fixture();
  const first = agentDetail.session.session_id,
    second = "b1".repeat(16);
  let resolve!: (value: typeof agentDetail) => void;
  source.agentSession = async (id) =>
    id === first
      ? new Promise((done) => {
          resolve = done;
        })
      : {
          ...agentDetail,
          session: { ...agentDetail.session, session_id: second },
          transcripts: [],
        };
  const late = response.load(first);
  await Promise.resolve();
  await response.load(second);
  resolve({ ...agentDetail, transcripts: [] });
  await late;
  expect(response.controller.state.agentSession?.session.session_id).toBe(
    second,
  );
  revoke();
  expect(response.controller.state.agentSession).toBeUndefined();
  await response.load(first);
  expect(response.controller.state.status.error).toContain("access");
});

test("Agent response verifies session identity and reports denied reads", async () => {
  const { response, source } = fixture();
  await response.load("bb".repeat(16));
  expect(response.controller.state.agentSession).toBeUndefined();
  expect(response.controller.state.status.error).toContain("snapshot");
  source.agentSession = async () => {
    throw new ApiError("access_denied");
  };
  await response.load(agentDetail.session.session_id);
  expect(response.controller.state.agentSession).toBeUndefined();
  expect(response.controller.state.status.error).toContain("access");
});

test("Cached agent response uses the current source after access is restored", async () => {
  const { source } = fixture();
  const repository = agentDetail.snapshot.repo_id;
  let current = source;
  const base = {
    state: { repositories: [{ id: repository, permission: "read" }], now: 0 },
    actions: {},
  } as unknown as StoreController;
  const response = createProjectAgentResponse(
    () => current,
    () => base,
  );
  await response.load(agentDetail.session.session_id);
  expect(response.controller.state.agentSession?.session.session_id).toBe(
    agentDetail.session.session_id,
  );
  response.reset();
  source.agentSession = async () => {
    throw new Error("Closed source used");
  };
  current = {
    ...source,
    agentSession: async () => ({ ...agentDetail, transcripts: [] }),
  };
  await response.load(agentDetail.session.session_id);
  expect(response.controller.state.status.error).toBe("");
  expect(response.controller.state.agentSession?.session.session_id).toBe(
    agentDetail.session.session_id,
  );
});

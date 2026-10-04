import { expect, test } from "vitest";
import { ApiError } from "../../src/lib/api/client.js";
import { createProjectsHub } from "../../src/lib/controller/production-projects-hub.svelte.js";
import { initialStoreRoute } from "../../src/lib/controller/store-route-history.svelte.js";
import type { WorkBrowseTransport } from "../../src/lib/controller/projects-hub-types.js";
import type { StoreRoute } from "../../src/lib/controller/contracts.js";
import {
  fixtureTransport,
  homeA,
  homeB,
  project,
  projectId,
} from "../../../catalog/fixtures/projects-hub-fixture.js";
import { agentSessions } from "../../../catalog/fixtures/collaboration.js";
import type { StoreController } from "../../src/lib/controller/contracts.js";
import {
  projectsRouteToHash,
  projectsRouteFromHash,
} from "../../src/lib/controller/projects-route-codec.js";
function fixture() {
  let route: StoreRoute = {
    ...initialStoreRoute(),
    page: "projects",
    projectsHub: true,
    workTab: "agents",
  };
  let repositories = [homeA, homeB, "43".repeat(16)].map((id) => ({
    id,
    name: id,
    permission: "read" as const,
  }));
  const calls: string[] = [],
    sources = new Map<string, WorkBrowseTransport>();
  const hub = createProjectsHub({
    agentResponse: () => ({
      controller: {} as StoreController,
      async load() {},
      reset() {},
    }),
    repositories: () => repositories,
    route: () => route,
    push: (next) => {
      route = next;
    },
    changed() {},
    source(home) {
      const source: WorkBrowseTransport = {
        ...fixtureTransport(home),
        async agents(cursor) {
          calls.push(home);
          return {
            version: 1,
            snapshot: {
              repo_id: home,
              epoch: "33".repeat(16),
              published_seq: 1,
              manifest_revision: 1,
              agent_root: "55".repeat(32),
              agent_state_version: 1,
            },
            sessions: [agentSessions[cursor ? 1 : 0]!],
            next_cursor: cursor ? null : "next",
          };
        },
      };
      sources.set(home, source);
      return source;
    },
  });
  return {
    hub,
    calls,
    sources,
    removeAccess: (id: string) => {
      repositories = repositories.filter((r) => r.id !== id);
    },
  };
}
test("Agents reads only deduplicated project repositories and retains pinned continuations", async () => {
  const { hub, calls } = fixture();
  await hub.openHub("agents");
  expect(calls.sort()).toEqual([homeA, homeB].sort());
  expect(hub.agents.homes.map((h) => h.repository.id).sort()).toEqual(
    [homeA, homeB].sort(),
  );
  await hub.agents.more(homeA);
  expect(
    hub.agents.homes.find((h) => h.repository.id === homeA)?.sessions,
  ).toHaveLength(2);
  await hub.refresh();
  expect(
    hub.agents.homes.find((h) => h.repository.id === homeA)?.sessions,
  ).toHaveLength(2);
  hub.reset();
  expect(hub.agents.homes).toHaveLength(0);
});

test("Agent pane selection retains the scoped directory and exact route identities", async () => {
  const { hub, calls } = fixture();
  await hub.openHub("agents");
  const rows = hub.agents.homes[0]!.sessions;
  const count = calls.length;
  await hub.agents.select(homeA, agentSessions[0]!.session_id);
  expect(hub.route).toMatchObject({
    page: "projects",
    workTab: "agents",
    workAgentRepository: homeA,
    agentSessionId: agentSessions[0]!.session_id,
  });
  expect(hub.agents.homes[0]!.sessions).toBe(rows);
  expect(calls).toHaveLength(count);
  expect(hub.agents.response).toBeDefined();
  const hash = projectsRouteToHash(hub.route)!;
  expect(projectsRouteFromHash(hash)).toMatchObject({
    workAgentRepository: homeA,
    agentSessionId: agentSessions[0]!.session_id,
  });
  expect(
    projectsRouteFromHash(hash.replace("tab=agents", "tab=tasks")),
  ).toBeNull();
  await hub.agents.select(null, null);
  expect(hub.agents.response).toBeUndefined();
  expect(hub.agents.homes[0]!.sessions).toBe(rows);
});
test("Selected project scope never falls back to all repositories", async () => {
  const { hub, calls, sources } = fixture();
  await hub.openProject(homeA, projectId);
  sources.get(homeA)!.page = async () => ({
    version: 1,
    snapshot: {
      repo_id: homeA,
      epoch: "33".repeat(16),
      published_seq: 1,
      manifest_revision: 1,
      work_root: null,
      work_state_version: 1,
    },
    projects: [{ ...project, repositories: { [homeB]: true } }],
    tasks: [],
    comments: [],
    next_cursor: null,
  });
  hub.work(homeA).invalidate?.();
  await hub.filter({ workTab: "agents" });
  expect(calls).toEqual([homeB]);
  sources.get(homeA)!.page = async () => {
    throw new Error("Project unavailable");
  };
  await hub.refresh();
  expect(hub.agents.homes).toHaveLength(0);
  expect(hub.error).toContain("Project unavailable");
});
test("Denied reads and rediscovery clear retained agent sessions", async () => {
  const { hub, sources, removeAccess } = fixture();
  await hub.openHub("agents");
  sources.get(homeA)!.agents = async () => {
    throw new ApiError("access_denied");
  };
  await hub.agents.more(homeA);
  expect(
    hub.agents.homes.find((h) => h.repository.id === homeA)?.sessions,
  ).toHaveLength(0);
  removeAccess(homeB);
  await hub.refresh();
  expect(hub.agents.homes.some((h) => h.repository.id === homeB)).toBe(false);
});

test("Agent scope includes repositories linked on later project pages", async () => {
  const { hub, sources, calls } = fixture();
  const source = hub.transport(homeA),
    browse = source.browse;
  source.browse = async (query, signal) => {
    const page = await browse(query, signal);
    return {
      ...page,
      projects: query.cursor
        ? [{ ...project, repositories: { [homeB]: true } }]
        : [{ ...project, repositories: { [homeA]: true } }],
      next_cursor: query.cursor ? null : "later-projects",
    };
  };
  await hub.openHub("agents");
  expect(calls.sort()).toEqual([homeA, homeB].sort());
  expect(
    hub.navigation.homes.find((home) => home.repository.id === homeA)?.projects,
  ).toHaveLength(2);
  const agentRead = sources.get(homeA)!.agents!;
  sources.get(homeA)!.agents = async (cursor) => {
    const page = await agentRead(cursor);
    return { ...page, snapshot: { ...page.snapshot, published_seq: 2 } };
  };
  await hub.agents.more(homeA);
  expect(
    hub.agents.homes.find((home) => home.repository.id === homeA)?.error,
  ).toContain("snapshot mismatch");
  expect(
    hub.agents.homes.find((home) => home.repository.id === homeA)?.sessions,
  ).toHaveLength(1);
});

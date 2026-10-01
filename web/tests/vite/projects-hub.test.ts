import { expect, test } from "vitest";
import { createProjectsHub } from "../../src/lib/controller/production-projects-hub.svelte.js";
import { initialStoreRoute } from "../../src/lib/controller/store-route-history.svelte.js";
import type { WorkBrowseTransport } from "../../src/lib/controller/projects-hub-types.js";
import type { Model } from "../../src/lib/api/client.js";
const a = "11".repeat(16),
  b = "22".repeat(16),
  id = "33".repeat(16);
const snapshot = (home: string): Model<"WorkReadSnapshot"> => ({
  repo_id: home,
  epoch: "44".repeat(16),
  published_seq: 1,
  manifest_revision: 1,
  work_root: null,
  work_state_version: 1,
});
function fixture(fail = false, discover?: () => Promise<void>) {
  let route: import("../../src/lib/controller/contracts.js").StoreRoute = {
    ...initialStoreRoute(),
    page: "projects" as const,
    projectsHub: true,
  };
  const calls: { home: string; query: unknown }[] = [],
    requests: unknown[] = [];
  const sources = new Map<string, WorkBrowseTransport>();
  const hub = createProjectsHub({
    discover,
    repositories: () => [
      { id: a, name: "Home", permission: "write" },
      { id: b, name: "Home", permission: "read" },
    ],
    route: () => route,
    push: (next) => {
      route = next as typeof route;
    },
    changed: () => {},
    source: (home) => {
      const source: WorkBrowseTransport = {
        async browse(query) {
          calls.push({ home, query });
          if (home === b && fail) throw new Error("Unavailable");
          return {
            version: 1,
            snapshot: snapshot(home),
            projects: [],
            tasks: [],
            next_cursor: query.cursor ? null : "cursor",
            total_matches: 1,
          };
        },
        async page() {
          return {
            version: 1,
            snapshot: snapshot(home),
            projects: [],
            tasks: [],
            comments: [],
            next_cursor: null,
          };
        },
        async apply(request) {
          requests.push(request);
          throw new Error("Lost acknowledgement");
        },
        async status(request_id) {
          return { status: "not_known", request_id };
        },
      };
      sources.set(home, source);
      return source;
    },
  });
  return {
    hub,
    calls,
    requests,
    sources,
    route: () => route,
    setRoute: (next: typeof route) => {
      route = next;
    },
  };
}
test("hub reads homes independently and keeps failed homes visible", async () => {
  const { hub, calls } = fixture(true);
  await hub.openHub("tasks");
  expect(calls).toHaveLength(2);
  expect(hub.homes[0]!.snapshot?.repo_id).toBe(a);
  expect(hub.homes[1]!.error).toBe("Unavailable");
  await hub.filter({
    workQuery: "needle",
    workState: "blocked",
    workRepository: b,
  });
  expect(calls.at(-2)!.query).toMatchObject({
    kind: "tasks",
    q: "needle",
    state: "blocked",
    repository: b,
  });
  await hub.more(a);
  expect(calls.at(-1)!.query).toMatchObject({
    cursor: "cursor",
    q: "needle",
    state: "blocked",
  });
});
test("pending requests and drafts survive hub navigation but clear on session reset", async () => {
  const { hub, requests } = fixture();
  const work = hub.work(a);
  await work.submit({
    command: "create_project",
    id,
    title: "Native",
    body: "",
  });
  const pending = work.state.pending;
  work.state.drafts = {
    comment: {
      title: "",
      body: "retained",
      repository: "",
      parent: "",
      status: "open",
      reply: null,
      edit: null,
    },
  };
  await hub.openHub();
  await hub.openProject(a, id);
  await hub.openHub("repositories");
  expect(hub.work(a).state.pending).toEqual(pending);
  expect(work.state.drafts.comment!.body).toBe("retained");
  await work.retry();
  expect(requests[1]).toEqual(requests[0]);
  hub.reset();
  expect(work.state.pending).toBeNull();
  expect(work.state.drafts).toBeUndefined();
});
test("late browse results cannot replace a newer filter", async () => {
  const { hub, sources } = fixture();
  await hub.openHub();
  let resolve!: (page: Model<"WorkBrowsePage">) => void;
  sources.get(a)!.browse = () =>
    new Promise((done) => {
      resolve = done;
    });
  const old = hub.filter({ workHome: a, workQuery: "old" });
  sources.get(a)!.browse = async () => ({
    version: 1,
    snapshot: snapshot(a),
    projects: [],
    tasks: [],
    next_cursor: null,
    total_matches: 0,
  });
  await hub.filter({ workQuery: "new" });
  resolve({
    version: 1,
    snapshot: snapshot(a),
    projects: [],
    tasks: [],
    next_cursor: "late",
    total_matches: 99,
  });
  await old;
  expect(hub.homes[0]!.total).toBe(0);
  expect(hub.homes[0]!.cursor).toBeNull();
});

test("navigation restores pinned pages and continuation filters until explicit refresh", async () => {
  const { hub, calls, route, setRoute } = fixture();
  await hub.openHub("tasks");
  await hub.filter({ workQuery: "older", workState: "blocked", workHome: a });
  const retained = route();
  await hub.more(a);
  const reads = calls.length;
  await hub.filter({ workQuery: "newer" });
  setRoute(retained);
  await hub.restore();
  expect(calls).toHaveLength(reads + 1);
  expect(hub.homes[0]!.cursor).toBeNull();
  await hub.refresh();
  expect(calls.at(-1)!.query).toMatchObject({ q: "older", state: "blocked" });
  const refreshed = calls.length;
  await hub.filter({ workQuery: "newer" });
  setRoute(retained);
  await hub.restore();
  await hub.more(a);
  expect(calls.at(-1)!.query).toMatchObject({
    q: "older",
    state: "blocked",
    cursor: "cursor",
  });
  expect(calls.length).toBeGreaterThan(refreshed);
});
test("browse rejects a continuation from a different snapshot", async () => {
  const { hub, sources } = fixture();
  await hub.openHub();
  sources.get(a)!.browse = async () => ({
    version: 1,
    snapshot: { ...snapshot(a), manifest_revision: 2 },
    projects: [],
    tasks: [],
    next_cursor: null,
    total_matches: 99,
  });
  await hub.more(a);
  expect(hub.homes[0]!.error).toContain("snapshot mismatch");
  expect(hub.homes[0]!.snapshot!.manifest_revision).toBe(1);
});

test("denied browse clears home content and its unresolved command", async () => {
  const { ApiError } = await import("../../src/lib/api/client.js");
  const { hub, sources } = fixture();
  await hub.openHub();
  await hub
    .work(a)
    .submit({ command: "create_project", id, title: "Retained", body: "" });
  expect(hub.commands).toHaveLength(1);
  sources.get(a)!.browse = async () => {
    throw new ApiError("access_denied");
  };
  await hub.refresh();
  expect(hub.homes[0]!.snapshot).toBeNull();
  expect(hub.work(a).state.pending).toBeNull();
  expect(hub.commands).toHaveLength(0);
});

test("refresh becomes busy before discovery and remains busy through reads", async () => {
  let discover!: () => void;
  const { hub } = fixture(
    false,
    () =>
      new Promise<void>((resolve) => {
        discover = resolve;
      }),
  );
  await hub.openHub();
  const refresh = hub.refresh();
  expect(hub.loading).toBe(true);
  discover();
  await refresh;
  expect(hub.loading).toBe(false);
});
test("missing project never broadens reviews to unrelated repositories", async () => {
  const { hub, sources } = fixture();
  await hub.openProject(a, id);
  let reads = 0;
  for (const source of sources.values())
    source.reviews = async () => {
      reads++;
      throw new Error("Must not read");
    };
  await hub.filter({ workTab: "reviews" });
  expect(reads).toBe(0);
  expect(hub.reviews.homes).toEqual([]);
  expect(hub.error).toContain("Project unavailable");
});

test("repository Projects browses linked projects across homes and keeps a separate cache", async () => {
  const { hub, calls, route, setRoute } = fixture();
  await hub.openHub();
  const landing = route();
  const local = {
    ...initialStoreRoute(),
    page: "projects" as const,
    repository: a,
    projectsHub: false,
  };
  setRoute(local);
  await hub.restore();
  expect(route()).toEqual(local);
  expect(calls.slice(-2)).toEqual([
    { home: a, query: { kind: "projects", repository: a, limit: 50 } },
    { home: b, query: { kind: "projects", repository: a, limit: 50 } },
  ]);
  const reads = calls.length;
  setRoute({ ...local, repository: b });
  await hub.restore();
  expect(calls.length).toBe(reads + 2);
  expect(calls.at(-1)!.query).toMatchObject({ repository: b });
  setRoute(landing);
  await hub.restore();
  expect(calls.length).toBe(reads + 2);
  setRoute(local);
  await hub.restore();
  expect(calls.length).toBe(reads + 2);
});

test("missing project never broadens Activity to unrelated repositories", async () => {
  const { hub, sources } = fixture();
  await hub.openProject(a, id);
  let reads = 0;
  for (const source of sources.values())
    source.activity = async () => {
      reads++;
      throw new Error("Must not read");
    };
  await hub.filter({ workTab: "activity", workActivity: "agent" });
  expect(reads).toBe(0);
  expect(hub.activity.homes).toEqual([]);
  expect(hub.error).toContain("Project unavailable");
});

import { expect, test, vi } from "vitest";
import { createWorkController } from "../../src/lib/controller/production-work.svelte.js";
import { createProjectsNavigation } from "../../src/lib/controller/projects-navigation.svelte.js";
import type { Model } from "../../src/lib/api/client.js";
import type { WorkTransport } from "../../src/lib/controller/work-types.js";
const snapshot: Model<"WorkReadSnapshot"> = {
  repo_id: "11".repeat(16),
  epoch: "22".repeat(16),
  published_seq: 1,
  manifest_revision: 1,
  work_root: null,
  work_state_version: 1,
};
const page: Model<"WorkPage"> = {
  version: 1,
  snapshot,
  projects: [],
  tasks: [],
  comments: [],
  next_cursor: null,
};
function fixture() {
  let saved = false;
  const transport: WorkTransport = {
    refresh: vi.fn(async () => {}),
    page: vi.fn(async () => ({ ...page, next_cursor: saved ? "new" : null })),
    apply: async () => {
      saved = true;
      return { status: "committed" } as never;
    },
    status: async () => {
      saved = true;
      return { status: "committed" } as never;
    },
  };
  const changed = vi.fn();
  const controller = createWorkController({
    source: () => transport,
    route: () => ({}),
    select: () => {},
    changed,
  });
  return { controller, transport, changed };
}
const command = {
  command: "create_project" as const,
  id: "33".repeat(16),
  title: "Saved",
  body: "",
};
test("a committed save reloads WorkStore and leaves no refresh banner", async () => {
  const { controller, transport, changed } = fixture();
  await controller.restore();
  await controller.submit(command);
  expect(controller.state.projectCursor).toBe("new");
  expect(transport.refresh).toHaveBeenCalledTimes(1);
  expect(changed).toHaveBeenCalledTimes(1);
  expect(controller.state.pending).toBeNull();
  expect(controller.state.message).toBe("");
});
test("a confirmed unknown command reloads and preserves unrelated drafts", async () => {
  const { controller, transport } = fixture();
  transport.apply = async (r) => ({
    status: "outcome_unknown",
    request_id: r.request_id,
  });
  await controller.submit(command);
  const draft = {
    title: "",
    body: "Unsent text",
    repository: "",
    parent: "",
    status: "open" as const,
    reply: null,
    edit: null,
  };
  controller.state.drafts = { comment: draft };
  await controller.check();
  expect(controller.state.projectCursor).toBe("new");
  expect(controller.state.pending).toBeNull();
  expect(controller.state.message).toBe("");
  expect(controller.state.drafts.comment).toEqual(draft);
});
test("read failures after commitment never turn a saved command into an unknown write", async () => {
  const { controller, transport } = fixture();
  transport.refresh = async () => {
    throw new Error("offline");
  };
  await controller.submit(command);
  expect(controller.state.pending).toBeNull();
  expect(controller.state.message).toBe("");
  expect(controller.state.error).toBe("offline");
});

test("automatic refresh rebuilds the loaded window using new-root cursors", async () => {
  const { controller, transport } = fixture();
  controller.state.projects = [{ id: "old-a" }, { id: "old-b" }] as never;
  controller.state.projectCursor = "old-cursor";
  const queries: unknown[] = [];
  transport.page = async (query) => {
    queries.push(query);
    return {
      ...page,
      snapshot: { ...snapshot, published_seq: 2 },
      projects: [{ id: query.cursor ? "new-b" : "new-a" }] as never,
      next_cursor: query.cursor ? null : "fresh-cursor",
    };
  };
  await controller.refresh();
  expect(controller.state.projects.map((p) => p.id)).toEqual([
    "new-a",
    "new-b",
  ]);
  expect(queries).toEqual([
    { limit: 50 },
    { cursor: "fresh-cursor", limit: 50 },
  ]);
});

test("sidebar refresh retains loaded projects using fresh-root cursors", async () => {
  let revision = 1;
  const queries: unknown[] = [];
  const transport = {
    browse: async (query: { cursor?: string }) => {
      queries.push(query);
      return {
        version: 1,
        snapshot: { ...snapshot, published_seq: revision },
        projects: [{ id: `${revision}-${query.cursor ? "b" : "a"}` }],
        tasks: [],
        total_matches: 2,
        next_cursor: query.cursor ? null : `cursor-${revision}`,
      };
    },
  };
  const navigation = createProjectsNavigation({
    repositories: () => [
      { id: snapshot.repo_id, name: "Home", permission: "read" },
    ],
    transport: () => transport as never,
    denied: () => {},
  });
  await navigation.load();
  await navigation.more(snapshot.repo_id);
  revision++;
  queries.length = 0;
  await navigation.load(true);
  expect(navigation.homes[0]!.projects.map((p) => p.id)).toEqual([
    "2-a",
    "2-b",
  ]);
  expect(queries).toEqual([
    { kind: "projects", limit: 50 },
    { kind: "projects", limit: 50, cursor: "cursor-2" },
  ]);
  navigation.reset();
});

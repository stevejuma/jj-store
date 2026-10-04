import { expect, test } from "vitest";
import { createWorkController } from "../../src/lib/controller/production-work.svelte.js";
import type { Model } from "../../src/lib/api/client.js";
import type {
  WorkRequest,
  WorkTransport,
} from "../../src/lib/controller/work-types.js";
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
test("work keeps unknown request intent across refresh and retries exactly", async () => {
  const requests: WorkRequest[] = [];
  const transport: WorkTransport = {
    page: async () => page,
    apply: async (request) => {
      requests.push(request);
      throw new TypeError("lost acknowledgement");
    },
    status: async (id) => ({ status: "not_known", request_id: id }),
  };
  const controller = createWorkController({
    source: () => transport,
    route: () => ({}),
    select: () => {},
    changed: () => {},
  });
  await controller.submit({
    command: "create_project",
    id: "33".repeat(16),
    title: "Release",
    body: "",
  });
  const retained = controller.state.pending;
  await controller.refresh();
  await controller.check();
  expect(controller.state.pending).toEqual(retained);
  await controller.retry();
  expect(requests[1]).toEqual(requests[0]);
});
test("late reads cannot replace a new selection or another session", async () => {
  let resolve!: (value: Model<"WorkPage">) => void;
  const transport: WorkTransport = {
    page: () =>
      new Promise((done) => {
        resolve = done;
      }),
    apply: async (request) => ({
      status: "outcome_unknown",
      request_id: request.request_id,
    }),
    status: async (id) => ({ status: "not_known", request_id: id }),
  };
  const controller = createWorkController({
    source: () => transport,
    route: () => ({}),
    select: () => {},
    changed: () => {},
  });
  const reading = controller.restore();
  controller.reset();
  resolve({ ...page, next_cursor: "old" });
  await reading;
  expect(controller.state.projectCursor).toBeNull();
});

test("explicit model rejection releases the draft for correction", async () => {
  const { ApiError } = await import("../../src/lib/api/client.js");
  const transport: WorkTransport = {
    page: async () => page,
    apply: async () => {
      throw new ApiError("invalid_request");
    },
    status: async (id) => ({ status: "not_known", request_id: id }),
  };
  const controller = createWorkController({
    source: () => transport,
    route: () => ({}),
    select: () => {},
    changed: () => {},
  });
  await controller.submit({
    command: "create_project",
    id: "33".repeat(16),
    title: "Rejected",
    body: "",
  });
  expect(controller.state.pending).toBeNull();
  expect(controller.state.error).toBe("invalid request");
});

test("loaded hierarchy orders children after parents and retains page fragments", async () => {
  const { workHierarchy } =
    await import("../../src/lib/ui/store-work/work-hierarchy.js");
  const rows = [
    { id: "child", parent: "parent" },
    { id: "orphan", parent: "outside" },
    { id: "parent", parent: null },
  ];
  expect(
    workHierarchy(rows, (row) => row.parent).map(({ row, depth }) => [
      row.id,
      depth,
    ]),
  ).toEqual([
    ["orphan", 0],
    ["parent", 0],
    ["child", 1],
  ]);
});

test("revoked WorkStore reads clear protected content and retained requests", async () => {
  const { ApiError } = await import("../../src/lib/api/client.js");
  const transport: WorkTransport = {
    page: async () => {
      throw new ApiError("access_denied");
    },
    apply: async (r) => ({
      status: "outcome_unknown",
      request_id: r.request_id,
    }),
    status: async (id) => ({ status: "not_known", request_id: id }),
  };
  const work = createWorkController({
    source: () => transport,
    route: () => ({}),
    select: () => {},
    changed: () => {},
  });
  work.state.comments = [
    {
      schema_version: 1,
      id: "33".repeat(16),
      task: "44".repeat(16),
      reply_to: null,
      body: "Protected discussion",
      version: 1,
      actor: {
        created_by: "55".repeat(16),
        updated_by: "55".repeat(16),
        created_at_millis: 1,
        updated_at_millis: 1,
      },
    },
  ];
  await work.submit({
    command: "create_project",
    id: "66".repeat(16),
    title: "Pending",
    body: "",
  });
  expect(work.state.pending).not.toBeNull();
  await work.refresh();
  expect(work.state.comments).toEqual([]);
  expect(work.state.pending).toBeNull();
  expect(work.state.loading).toBe(false);
  expect(work.state.error).toContain("access denied");
});

import { followProductionExternalRoutes } from "../../src/lib/controller/production-store-lifecycle.svelte.js";
import { restoreProductionStoreRoute } from "../../src/lib/controller/production-route-restorer.js";
import { expect, test } from "vitest";
import { createProductionStoreActions } from "../../src/lib/controller/production-store-actions.js";
import {
  createStoreRouteHistory,
  initialStoreRoute,
} from "../../src/lib/controller/store-route-history.svelte.js";
import { createProductionStoreState } from "../../src/lib/controller/production-store-state.svelte.js";
const home = "11".repeat(16),
  code = "22".repeat(16),
  thread = "33".repeat(16);
function fixture(active = home) {
  const history = createStoreRouteHistory(),
    data = createProductionStoreState();
  const landing = {
    ...initialStoreRoute(),
    page: "projects" as const,
    projectsHub: true,
    repository: home,
  };
  history.push(landing);
  data.repositories = [{ id: code, name: "Linked code", permission: "read" }];
  data.heads = ["44".repeat(64)];
  let connected!: () => void,
    restores = 0;
  const actions = createProductionStoreActions({
    data,
    history,
    reader: {},
    search: {},
    mutations: { actions: {} },
    conflicts: {},
    branches: {},
    settings: {},
    activeRepository: () => active,
    connectRepository: async () => {
      history.push({ ...initialStoreRoute(), repository: code, page: "files" });
      await new Promise<void>((resolve) => {
        connected = resolve;
      });
    },
    restoreRoute: async () => {
      restores++;
    },
  } as Parameters<typeof createProductionStoreActions>[0]);
  return {
    actions,
    history,
    landing,
    connected: () => connected(),
    restores: () => restores,
  };
}
test.each(["repository", "review"])(
  "late %s connection cannot overwrite a restored Projects route",
  async (kind) => {
    const f = fixture();
    const pending =
      kind === "repository"
        ? f.actions.openRepository(code)
        : f.actions.openReview!(code, thread);
    f.history.push(f.landing);
    f.connected();
    await pending;
    expect(f.history.route).toEqual(f.landing);
    expect(f.restores()).toBe(0);
  },
);
test("completed authorized review connection opens the full native thread", async () => {
  const f = fixture();
  const pending = f.actions.openReview!(code, thread);
  f.connected();
  await pending;
  expect(f.history.route).toMatchObject({
    repository: code,
    page: "reviews",
    reviewThreadId: thread,
    projectsHub: false,
  });
  expect(f.restores()).toBe(1);
  f.history.navigate(-1);
  expect(f.history.route).toEqual(f.landing);
});

test("late code reference reads cannot replace Projects restored with Back", async () => {
  const history = createStoreRouteHistory(),
    data = createProductionStoreState();
  const codeRoute = {
    ...initialStoreRoute(),
    page: "files" as const,
    repository: code,
    commitId: "44".repeat(64),
  };
  history.push(codeRoute);
  let references!: () => void,
    fileReads = 0;
  const pending = restoreProductionStoreRoute(codeRoute, data, history, {
    loadReferences: () =>
      new Promise<void>((resolve) => {
        references = resolve;
      }),
    loadFiles: async () => {
      fileReads++;
      return true;
    },
  } as Parameters<typeof restoreProductionStoreRoute>[3]);
  history.push({ ...initialStoreRoute(), page: "projects", projectsHub: true });
  references();
  await pending;
  expect(history.route.page).toBe("projects");
  expect(fileReads).toBe(0);
});

test("review navigation uses the target code operation rather than a home code pin", async () => {
  const f = fixture(code);
  f.history.push({ ...f.landing, snapshotOperationId: "55".repeat(64) });
  await f.actions.openReview!(code, thread);
  expect(f.history.route).toMatchObject({
    repository: code,
    snapshotOperationId: "44".repeat(64),
    reviewThreadId: thread,
  });
});

test("native activity links authorize a code connection when Projects has no active source", async () => {
  const data = createProductionStoreState();
  data.repositories = [{ id: code, name: "Linked code", permission: "read" }];
  const connected: string[] = [],
    denied: string[] = [];
  let external!: Parameters<
    ReturnType<typeof createStoreRouteHistory>["onExternalRoute"]
  >[0];
  followProductionExternalRoutes({
    data,
    history: {
      onExternalRoute: (listener: typeof external) => {
        external = listener;
      },
    } as ReturnType<typeof createStoreRouteHistory>,
    source: () => null,
    connect: async (repository, token) => {
      expect(token).toBeNull();
      connected.push(repository);
    },
    disconnect: (message) => {
      denied.push(message!);
    },
    restore: async () => {
      throw new Error("No active code source to restore");
    },
  });
  external({
    kind: "route",
    route: {
      ...initialStoreRoute(),
      repository: code,
      page: "log",
      snapshotOperationId: "44".repeat(64),
      commitId: "55".repeat(64),
    },
  });
  expect(connected).toEqual([code]);
  external({
    kind: "route",
    route: { ...initialStoreRoute(), repository: home, page: "reviews" },
  });
  expect(connected).toEqual([code]);
  expect(denied).toEqual(["Reconnect to open the requested repository."]);
});

test("repository Projects external routes authorize their separate code connection", () => {
  const data = createProductionStoreState();
  data.repositories = [{ id: code, name: "Linked code", permission: "read" }];
  const connected: string[] = [];
  let external!: Parameters<
    ReturnType<typeof createStoreRouteHistory>["onExternalRoute"]
  >[0];
  followProductionExternalRoutes({
    data,
    history: {
      onExternalRoute: (listener: typeof external) => {
        external = listener;
      },
    } as ReturnType<typeof createStoreRouteHistory>,
    source: () => null,
    connect: async (repository) => {
      connected.push(repository);
    },
    disconnect: () => {},
    restore: async () => {
      throw new Error("No active code source");
    },
  });
  external({
    kind: "route",
    route: {
      ...initialStoreRoute(),
      repository: code,
      page: "projects",
      snapshotOperationId: "44".repeat(64),
    },
  });
  expect(connected).toEqual([code]);
});

test("departing a delayed graph cannot start old commit reads or overwrite navigation status", async () => {
  const data = createProductionStoreState();
  const codeRoute = {
    ...initialStoreRoute(),
    repository: "11".repeat(16),
    page: "log" as const,
    commitId: "aa".repeat(64),
    snapshotOperationId: "bb".repeat(64),
  };
  const history = { route: codeRoute } as Parameters<
    typeof restoreProductionStoreRoute
  >[2];
  let release!: () => void;
  const calls: string[] = [];
  const pending = restoreProductionStoreRoute(codeRoute, data, history, {
    prepareReferences: async () => true,
    loadGraph: () =>
      new Promise<void>((done) => {
        release = done;
      }),
    loadCommit: async () => {
      calls.push("obsolete commit");
    },
    loadReferencePickers: async () => {
      calls.push("obsolete pickers");
    },
  } as Parameters<typeof restoreProductionStoreRoute>[3]);
  await Promise.resolve();
  history.route = {
    ...codeRoute,
    projectsHub: true,
    page: "projects",
    commitId: null,
    snapshotOperationId: null,
  };
  data.status.navigation = "failed";
  release();
  await pending;
  expect(calls).toEqual([]);
  expect(data.status.navigation).toBe("failed");
});

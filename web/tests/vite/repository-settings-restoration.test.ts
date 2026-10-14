import { expect, test, vi } from "vitest";
import { connectRepositorySource } from "../../src/lib/controller/production-repository-connect.js";
import { createProfileRouteRestorer } from "../../src/lib/controller/production-profile-route.js";
import { createProductionStoreState } from "../../src/lib/controller/production-store-state.svelte.js";
import { storeRouteFromHash } from "../../src/lib/controller/store-route-codec.js";
import type { StoreDataSource } from "../../src/lib/controller/native-data-source.js";

test("settings URL restoration preserves drafts and code views without ordinary reads", async () => {
  const data = createProductionStoreState();
  data.workspacePath = "/draft";
  data.searchSettings.dirty = true;
  const graph = data.graph,
    draft = data.searchSettings;
  const parsed = storeRouteFromHash(
    `#/repos/${"ab".repeat(16)}/settings?section=search&tab=scopes&at_op=${"cd".repeat(64)}`,
  );
  if (parsed.kind !== "route") throw Error("invalid fixture");
  const route = parsed.route;
  const read = vi.fn(async () => {}),
    connect = vi.fn(async () => {});
  const restore = createProfileRouteRestorer({
    data,
    route: () => route,
    source: () =>
      ({ repository: route.repository, profile: "native" }) as StoreDataSource,
    projects: { restore: read },
    connect,
    token: () => null,
    invalidate: vi.fn(),
    restore: read,
  });
  await restore();
  expect(read).not.toHaveBeenCalled();
  expect(connect).not.toHaveBeenCalled();
  expect(data.graph).toBe(graph);
  expect(data.searchSettings).toBe(draft);
  expect(data.workspacePath).toBe("/draft");
  expect(data.searchSettings.dirty).toBe(true);
  expect(data.status.navigation).toBe("ready");
  expect(route.snapshotOperationId).toBe("cd".repeat(64));
});

test("guarded repository changes leave the active owner and drafts intact before connection teardown", async () => {
  const data = createProductionStoreState();
  data.workspacePath = "/retained/draft";
  const parsed = storeRouteFromHash(
    `#/repos/${"ab".repeat(16)}/settings?section=folders`,
  );
  if (parsed.kind !== "route") throw Error("invalid fixture");
  const disconnect = vi.fn(),
    source = vi.fn();
  const canNavigateTo = vi.fn(() => false);
  await connectRepositorySource(
    {
      data,
      history: { route: parsed.route, canNavigateTo } as never,
      disconnect,
      source,
    } as never,
    "ef".repeat(16),
    null,
  );
  expect(canNavigateTo).toHaveBeenCalledOnce();
  expect(disconnect).not.toHaveBeenCalled();
  expect(source).not.toHaveBeenCalled();
  expect(data.workspacePath).toBe("/retained/draft");
  expect(data.status.connection).toBe("disconnected");
});

test("index recovery retains the captured graph time and identity while ordinary navigation clears them", async () => {
  const data = createProductionStoreState();
  const parsed = storeRouteFromHash(
    `#/repos/${"ab".repeat(16)}/log?at_op=${"cd".repeat(64)}`,
  );
  if (parsed.kind !== "route") throw Error("invalid fixture");
  const observed: unknown[] = [];
  const restore = createProfileRouteRestorer({
    data,
    route: () => parsed.route,
    source: () =>
      ({
        repository: parsed.route.repository,
        profile: "native",
      }) as StoreDataSource,
    projects: { restore: async () => {} },
    connect: async () => {},
    token: () => null,
    invalidate: () => {},
    restore: async () => {
      observed.push({ time: data.graphAtTimeMillis, email: data.historyEmail });
    },
  });
  const retained = {
    graphAtTimeMillis: 123456,
    historyEmail: "retained@example.test",
  };
  await (restore as (query: typeof retained) => Promise<void>)(retained);
  expect(observed).toEqual([{ time: 123456, email: "retained@example.test" }]);
  await restore();
  expect(observed[1]).toEqual({ time: null, email: "" });
});

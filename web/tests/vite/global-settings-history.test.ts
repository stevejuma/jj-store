import { expect, test, vi } from "vitest";
import { createProductionConnect } from "../../src/lib/controller/production-store-connect.js";
import {
  createStoreRouteHistory,
  type StoreRouteBrowser,
} from "../../src/lib/controller/store-route-history.svelte.js";
import { createProfileRouteRestorer } from "../../src/lib/controller/production-profile-route.js";
import { createProductionStoreState } from "../../src/lib/controller/production-store-state.svelte.js";
import { storeRouteFromHash } from "../../src/lib/controller/store-route-codec.js";

function browser(initial: string) {
  const entries = [{ hash: initial, id: undefined as string | undefined }];
  let index = 0,
    listen = () => {};
  const adapter: StoreRouteBrowser = {
    hash: () => entries[index].hash,
    entryId: () => entries[index].id,
    push(hash, id) {
      entries.splice(++index);
      entries.push({ hash, id });
    },
    replace(hash, id) {
      entries[index] = { hash, id };
    },
    go(delta) {
      index += delta;
      listen();
    },
    listen(fn) {
      listen = fn;
      return () => {};
    },
  };
  return {
    adapter,
    entries,
    visit(at: number) {
      index = at;
      listen();
    },
  };
}
test("global settings history restores tabs and exact returns across repositories", () => {
  const a = "ab".repeat(16),
    b = "ef".repeat(16),
    operation = "cd".repeat(64);
  const f = browser(`#/repos/${a}/files?at_op=${operation}`),
    h = createStoreRouteHistory(f.adapter);
  const entryA = { ...h.route };
  h.globalSettings.open("agents", "capture");
  h.globalSettings.select("agents", "configuration");
  h.navigate(-1);
  expect(h.route.globalSettings?.tab).toBe("capture");
  h.navigate(1);
  expect(h.route.globalSettings?.tab).toBe("configuration");
  h.globalSettings.leave();
  expect(h.route).toEqual(entryA);
  h.push({ ...entryA, repository: b });
  h.globalSettings.open("agents", "capture");
  h.globalSettings.leave();
  h.navigate(-1);
  expect(h.route.repository).toBe(b);
  expect(h.route.globalSettings?.tab).toBe("capture");
  h.navigate(-1);
  h.navigate(-1);
  expect(h.route.repository).toBe(a);
  expect(h.route.globalSettings).toBeUndefined();
  h.navigate(-1);
  expect(h.route.repository).toBe(a);
  expect(h.route.globalSettings?.tab).toBe("configuration");
  h.destroy();
});
test("global settings keep navigation guards and fresh links return to Projects", () => {
  const f = browser("#/settings?section=compute&tab=resources"),
    h = createStoreRouteHistory(f.adapter);
  expect(h.route.globalSettings?.tab).toBe("resources");
  h.setNavigationGuard("pending", () => "Wait for pending work.");
  h.globalSettings.select("agents");
  expect(h.route.globalSettings?.section).toBe("compute");
  h.setNavigationGuard("pending");
  h.globalSettings.leave();
  expect(h.route.projectsHub).toBe(true);
  expect(h.route.workTab).toBe("projects");
  expect(h.route.globalSettings).toBeUndefined();
  h.destroy();
});
test("restoring global settings does not invalidate reads owners or drafts", async () => {
  const parsed = storeRouteFromHash("#/settings?section=service&tab=limits");
  if (parsed.kind !== "route") throw Error("fixture");
  const data = createProductionStoreState();
  data.workspacePath = "/retained";
  data.searchSettings.dirty = true;
  const read = vi.fn(async () => {}),
    invalidate = vi.fn();
  const restore = createProfileRouteRestorer({
    data,
    route: () => parsed.route,
    source: () => null,
    projects: { restore: read },
    connect: read,
    token: () => null,
    invalidate,
    restore: read,
  });
  await restore();
  expect(read).not.toHaveBeenCalled();
  expect(invalidate).not.toHaveBeenCalled();
  expect(data.workspacePath).toBe("/retained");
  expect(data.searchSettings.dirty).toBe(true);
});

test("global settings startup skips repository and project reads", async () => {
  const f = browser("#/settings?section=agents&tab=configuration"),
    history = createStoreRouteHistory(f.adapter);
  const data = createProductionStoreState();
  data.accessMode = "local_trusted";
  const read = vi.fn(async () => {}),
    fetcher = vi.fn();
  const connect = createProductionConnect({
    data,
    route: () => history.route,
    projects: { restore: read, openHub: read } as never,
    connectRepository: read,
    disconnect: vi.fn(),
    version: () => 0,
    discovery: vi.fn(),
    token: vi.fn(),
    fetcher,
  });
  await connect("", null);
  expect(read).not.toHaveBeenCalled();
  expect(fetcher).not.toHaveBeenCalled();
  expect(data.status.navigation).toBe("ready");
  history.destroy();
});
test.each(["#/agents", "#/threads"])(
  "reauthentication retains installation destination %s after discovery",
  async (hash) => {
    const f = browser(hash),
      history = createStoreRouteHistory(f.adapter);
    const data = createProductionStoreState();
    data.accessMode = "authenticated";
    const read = vi.fn(async () => {}),
      token = vi.fn();
    const connect = createProductionConnect({
      data,
      route: () => history.route,
      projects: { restore: read, openHub: read } as never,
      connectRepository: read,
      disconnect: vi.fn(),
      version: () => 0,
      discovery: vi.fn(),
      token,
      fetcher: async () =>
        Response.json({ version: 1, generation: 0, repositories: [] }),
    });
    await connect("", "enrolled");
    expect(token).toHaveBeenCalledWith("enrolled");
    expect(read).not.toHaveBeenCalled();
    expect(f.adapter.hash()).toBe(hash);
    expect(data.status.navigation).toBe("ready");
    history.destroy();
  },
);
test("identical global URLs retain repository navigation guards", () => {
  const a = "ab".repeat(16),
    b = "ef".repeat(16),
    operation = "cd".repeat(64);
  const f = browser(`#/repos/${a}/files?at_op=${operation}`),
    h = createStoreRouteHistory(f.adapter);
  const entryA = { ...h.route };
  h.globalSettings.open("agents", "capture");
  h.globalSettings.leave();
  h.push({ ...entryA, repository: b });
  h.globalSettings.open("agents", "capture");
  h.setNavigationGuard("owner", (current, next) =>
    current.repository !== next.repository ? "Retain pending command." : "",
  );
  // Browser traversal may jump over intervening entries, including another settings URL.
  f.visit(1);
  expect(h.route.repository).toBe(b);
  expect(h.error).toBe("Retain pending command.");
  h.destroy();
});

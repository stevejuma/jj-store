import { expect, test } from "vitest";
import {
  createStoreStacksNavigation,
  isStacksRoute,
  stacksRouteView,
} from "../../src/lib/controller/store-stacks-navigation.js";
import { initialStoreRoute } from "../../src/lib/controller/store-route-history.svelte.js";
import {
  storeRouteFromHash,
  storeRouteToHash,
} from "../../src/lib/controller/store-route-codec.js";
import type { StoreRoute } from "../../src/lib/controller/store-route-types.js";

const repository = "ab".repeat(16),
  operation = "12".repeat(64);
function fixture(page: "files" | "stacks" = "files") {
  let route: StoreRoute = {
    ...initialStoreRoute(),
    page,
    repository,
    snapshotOperationId: operation,
  };
  const navigation = createStoreStacksNavigation({
    route: () => route,
    push: (next) => {
      route = next;
    },
  });
  return {
    navigation,
    get route() {
      return route;
    },
    select: (patch: Partial<typeof route>) => {
      route = { ...route, ...patch };
    },
  };
}

test("focused tabs retain independent selections and return to the exact repository route", () => {
  const f = fixture();
  f.select({
    bookmark: "main",
    commitId: "34".repeat(20),
    path: "README.md",
    workspace: "default",
  });
  const before = f.route;
  f.navigation.select("branches");
  expect(f.route).toMatchObject({
    page: "stacks",
    stackTab: "branches",
    workspace: null,
  });
  f.navigation.select("log");
  expect(f.route).toMatchObject({
    page: "log",
    context: "stacks",
    bookmark: "",
    workspace: null,
  });
  f.select({ commitId: "56".repeat(20), path: "src/app.ts" });
  const commits = f.route;
  f.navigation.select("oplog");
  f.select({ inspectedOperationId: "78".repeat(64) });
  const operations = f.route;
  f.navigation.select("workspace");
  f.navigation.select("log");
  expect(f.route).toEqual(commits);
  f.navigation.select("oplog");
  expect(f.route).toEqual(operations);
  f.navigation.leave();
  expect(f.route).toEqual(before);
});

test("direct Stacks links fall back to Overview and never reuse another repository return route", () => {
  const f = fixture("stacks");
  expect(stacksRouteView(f.route)).toBe("workspace");
  f.navigation.leave();
  expect(f.route).toMatchObject({
    page: "files",
    repository,
    snapshotOperationId: operation,
  });
  f.navigation.select("workspace");
  f.select({ repository: "cd".repeat(16) });
  f.navigation.leave();
  expect(f.route).toMatchObject({ page: "files", repository: "cd".repeat(16) });
});

test.each(["branches", "workspace", "log", "oplog"] as const)(
  "%s focused route round-trips without changing the history page",
  (view) => {
    const f = fixture();
    f.navigation.select(view);
    const result = storeRouteFromHash(storeRouteToHash(f.route)!);
    expect(result.kind).toBe("route");
    if (result.kind === "route") {
      expect(storeRouteToHash(result.route)).toBe(storeRouteToHash(f.route));
      expect(result.route).toMatchObject({
        page: f.route.page,
        repository,
        snapshotOperationId: operation,
        workspace: null,
      });
      expect(isStacksRoute(result.route)).toBe(true);
      expect(stacksRouteView(result.route)).toBe(view);
    }
  },
);

test.each([
  "files?context=stacks",
  "log?context=other",
  "log?context=stacks&context=stacks",
  "log?stack_tab=branches",
  "stacks?stack_tab=other",
  "stacks?stack_tab=branches&stack_tab=workspace",
])("rejects incompatible or duplicate focused route fields: %s", (path) => {
  expect(storeRouteFromHash(`#/repos/${repository}/${path}`).kind).toBe(
    "invalid",
  );
});

test("accepted native operations follow both histories while explicit pins survive tab changes", () => {
  const f = fixture("stacks");
  const current = "98".repeat(64);
  f.navigation.acceptOperation(repository, current);
  for (const view of ["log", "oplog"] as const) {
    f.navigation.select(view);
    expect(f.route).toMatchObject({
      snapshotOperationId: current,
      stackHistory: "live",
    });
  }
  f.select({ stackHistory: "pinned", snapshotOperationId: operation });
  f.navigation.select("workspace");
  f.navigation.acceptOperation(repository, "76".repeat(64));
  f.navigation.select("oplog");
  expect(f.route).toMatchObject({
    snapshotOperationId: operation,
    stackHistory: "pinned",
  });
  const reloaded = storeRouteFromHash(storeRouteToHash(f.route)!);
  expect(reloaded.kind === "route" && reloaded.route.stackHistory).toBe(
    "pinned",
  );
  f.navigation.select("log");
  expect(f.route).toMatchObject({
    snapshotOperationId: "76".repeat(64),
    stackHistory: "live",
  });
});

test.each([
  "log?stack_history=live",
  "stacks?stack_history=pinned",
  "oplog?context=stacks&stack_history=wrong",
  "log?context=stacks&stack_history=live&stack_history=pinned",
])("rejects invalid history modes: %s", (path) => {
  expect(storeRouteFromHash(`#/repos/${repository}/${path}`).kind).toBe(
    "invalid",
  );
});

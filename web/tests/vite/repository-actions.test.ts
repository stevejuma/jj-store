import { expect, test, vi } from "vitest";
import {
  storeRouteFromHash,
  storeRouteToHash,
} from "../../src/lib/controller/store-route-codec.js";
import { storePageRoute } from "../../src/lib/controller/store-stacks-navigation.js";
import { workflowViewRoute } from "../../src/lib/ui/store-workflows/workflow-navigation.js";
import { createProfileRouteRestorer } from "../../src/lib/controller/production-profile-route.js";
import { createProductionStoreState } from "../../src/lib/controller/production-store-state.svelte.js";
import type { StoreDataSource } from "../../src/lib/controller/native-data-source.js";
const repository = "42".repeat(16),
  home = "41".repeat(16),
  run = "51".repeat(16);
function decode(hash: string) {
  const value = storeRouteFromHash(hash);
  if (value.kind !== "route") throw Error("Invalid fixture");
  return value.route;
}
test("repository Actions restores filters and exact historical pins with a fixed source scope", () => {
  const hash = `#/repos/${repository}/actions?q=build&state=skipped&trigger=event&author=${"22".repeat(16)}&bookmark=feature%2Fparser&scope=${home}&workflow=${"11".repeat(16)}&home=${home}&run=${run}&at=18446744073709551615&attempt=${"52".repeat(16)}`;
  const route = decode(hash);
  expect(route).toMatchObject({
    page: "actions",
    repository,
    workflow: {
      tab: "history",
      source: repository,
      at: "18446744073709551615",
      run,
    },
  });
  expect(storeRouteToHash(route)).toBe(hash);
  const next = workflowViewRoute(
    route,
    route.workflow!,
    {
      source: home,
      scope: undefined,
      workflow: undefined,
      home: undefined,
      run: undefined,
      at: undefined,
      attempt: undefined,
    },
    repository,
  );
  expect(next.workflow?.source).toBe(repository);
  expect(storeRouteToHash(next)).toContain(`#/repos/${repository}/actions?`);
  expect(
    storeRouteToHash({
      ...route,
      workflow: { ...route.workflow, source: home },
    }),
  ).toBeNull();
});
test("repository Actions refuses malformed, foreign-scope and incompatible code selections", () => {
  for (const suffix of [
    `source=${home}`,
    "tab=history",
    `home=${home}`,
    `run=${run}`,
    "state=invalid",
    `author=${home}&author=${home}`,
    "at=1",
    "q=%00",
    "workspace=main",
    "at_op=" + "12".repeat(64),
  ])
    expect(
      storeRouteFromHash(`#/repos/${repository}/actions?${suffix}`).kind,
    ).toBe("invalid");
  expect(storeRouteFromHash(`#/repos/invalid/actions`).kind).toBe("invalid");
});
test("Actions navigation clears workflow filters on other repository tabs without dropping code pins", () => {
  const files = decode(`#/repos/${repository}/files?at_op=${"12".repeat(64)}`);
  const actions = storePageRoute(files, "actions");
  expect(actions.workflow).toEqual({ tab: "history", source: repository });
  expect(actions.snapshotOperationId).toBe(files.snapshotOperationId);
  expect(storeRouteToHash(actions)).toBe(`#/repos/${repository}/actions`);
  expect(storePageRoute(actions, "files").workflow).toBeUndefined();
});
test("Actions restores without code operation selection, graph reads or clearing retained drafts", async () => {
  const data = createProductionStoreState(),
    route = decode(`#/repos/${repository}/actions`);
  data.workspacePath = "/retained/draft";
  data.searchSettings.dirty = true;
  const graph = data.graph,
    draft = data.searchSettings,
    read = vi.fn(async () => {}),
    connect = vi.fn(async () => {});
  const restore = createProfileRouteRestorer({
    data,
    route: () => route,
    source: () => ({ repository, profile: "native" }) as StoreDataSource,
    projects: { restore: read },
    connect,
    token: () => null,
    invalidate: vi.fn(),
    restore: read,
  });
  await restore();
  expect(read).not.toHaveBeenCalled();
  expect(connect).not.toHaveBeenCalled();
  expect(data.status.navigation).toBe("ready");
  expect(data.graph).toBe(graph);
  expect(data.searchSettings).toBe(draft);
  expect(data.workspacePath).toBe("/retained/draft");
});

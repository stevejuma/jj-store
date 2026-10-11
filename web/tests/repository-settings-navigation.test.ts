import test from "node:test";
import assert from "node:assert/strict";
import { createRepositorySettingsNavigation } from "../src/lib/controller/repository-settings-navigation.js";
import {
  storeRouteFromHash,
  storeRouteToHash,
} from "../src/lib/controller/store-route-codec.js";
import {
  repositorySettingsSections,
  supportedSettingsDestination,
} from "../src/lib/ui/store-settings/repository-settings-model.js";
import type {
  StoreController,
  StoreRoute,
} from "../src/lib/controller/contracts.js";

const repository = "ab".repeat(16),
  operation = "cd".repeat(64);
function fixture(
  initial = "files?commit=" +
    "ef".repeat(20) +
    "&file=src%2Fapp.ts&at_op=" +
    operation,
) {
  const parsed = storeRouteFromHash(`#/repos/${repository}/${initial}`);
  assert.equal(parsed.kind, "route");
  let route = parsed.kind === "route" ? parsed.route : undefined!;
  const navigation = createRepositorySettingsNavigation({
    route: () => route,
    push: (next) => (route = next),
    replace: (next) => (route = next),
  });
  return {
    navigation,
    get route() {
      return route;
    },
    set route(next: StoreRoute) {
      route = next;
    },
  };
}
test("category/subtab navigation preserves the exact file entry and operation", () => {
  const f = fixture(),
    entry = f.route;
  f.navigation.open();
  assert.equal(f.route.page, "settings");
  assert.equal(f.route.snapshotOperationId, operation);
  assert.equal(f.route.commitId, null);
  f.navigation.select("search", "scopes");
  assert.match(storeRouteToHash(f.route)!, /section=search&tab=scopes/);
  f.navigation.select("storage", "maintenance");
  f.navigation.leave();
  assert.deepEqual(f.route, entry);
});
test("Back restores focused Stacks history without changing its mode or comparison", () => {
  const f = fixture(
      `oplog?context=stacks&stack_history=pinned&operation=${operation}&at_op=${operation}`,
    ),
    entry = f.route;
  f.navigation.open();
  f.navigation.select("advanced");
  f.navigation.leave();
  assert.deepEqual(f.route, entry);
});
test("direct and reloaded settings links fall back to this repository Overview", () => {
  const f = fixture(`settings?section=search&tab=health&at_op=${operation}`);
  f.navigation.leave();
  assert.equal(f.route.page, "files");
  assert.equal(f.route.repository, repository);
  assert.equal(f.route.snapshotOperationId, operation);
});
test("return routes cannot leak into a different repository", () => {
  const f = fixture();
  f.navigation.open();
  f.route = { ...f.route, repository: "12".repeat(16) };
  f.navigation.leave();
  assert.equal(f.route.repository, "12".repeat(16));
  assert.equal(f.route.page, "files");
  assert.equal(f.route.path, null);
});
test("malformed, duplicate and incompatible settings fields fail closed", () => {
  for (const suffix of [
    "settings?section=unknown",
    "settings?section=search&section=search",
    "settings?section=general&tab=health",
    "settings?section=storage&tab=configuration",
    "settings?section=search&tab=health&tab=scopes",
    "settings?section=",
    "settings?tab=",
    "settings?section=general&commit=" + "ef".repeat(20),
    "files?section=general",
    "log?tab=health",
    "settings?context=stacks",
    "settings?at_op=short",
  ]) {
    assert.equal(
      storeRouteFromHash(`#/repos/${repository}/${suffix}`).kind,
      "invalid",
      suffix,
    );
  }
  const defaultRoute = storeRouteFromHash(`#/repos/${repository}/settings`);
  assert.equal(defaultRoute.kind, "route");
  if (defaultRoute.kind === "route")
    assert.equal(defaultRoute.route.repositorySettingsSection, "general");
});
function view(
  profile: "native" | "git_sha1_jj045",
  admin: boolean,
  flags: Record<string, boolean> = {},
) {
  return {
    repository: { profile, permission: admin ? "admin" : "read" },
    accessMode: admin ? "local_trusted" : "authenticated",
    route: {
      repositorySettingsSection: "search",
      repositorySettingsTab: "scopes",
    },
    settings: { capabilities: flags, syncSettings: { value: null } },
  } as unknown as StoreController["state"];
}
test("local and authenticated profiles expose only supported categories", () => {
  assert.deepEqual(
    repositorySettingsSections(view("git_sha1_jj045", true)).map(
      (s) => s.value,
    ),
    ["general", "folders", "advanced"],
  );
  assert.deepEqual(
    repositorySettingsSections(
      view("native", false, {
        search: true,
        local_storage_management: true,
        local_workspace_links: true,
        local_search_management: true,
      }),
    ).map((s) => s.value),
    ["general", "search", "advanced"],
  );
});
test("capability-aware defaults select Health for readers and General for unsupported categories", () => {
  assert.deepEqual(
    supportedSettingsDestination(view("native", false, { search: true })),
    { section: "search", tab: "health", unsupported: true },
  );
  assert.deepEqual(supportedSettingsDestination(view("git_sha1_jj045", true)), {
    section: "general",
    tab: undefined,
    unsupported: true,
  });
  assert.deepEqual(
    supportedSettingsDestination(
      view("native", true, { local_search_management: true }),
    ),
    { section: "search", tab: "scopes", unsupported: false },
  );
});

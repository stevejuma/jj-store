import test from "node:test";
import assert from "node:assert/strict";
import {
  storeRouteFromHash,
  storeRouteToHash,
} from "../src/lib/controller/store-route-codec.js";
import {
  globalSettingsTabs,
  createGlobalSettingsNavigation,
} from "../src/lib/controller/global-settings-navigation.js";
import {
  resumeGlobalSettingsHash,
  rememberGlobalSettingsHash,
} from "../src/lib/controller/global-settings-route-codec.js";
import { globalSettingsSkeletonCards } from "../src/lib/ui/store-settings/global-settings-skeleton.js";

test("global settings sections and tabs round trip with canonical defaults", () => {
  for (const [section, tabs] of Object.entries(globalSettingsTabs))
    for (const tab of tabs) {
      const hash = `#/settings?section=${section}&tab=${tab}`;
      const result = storeRouteFromHash(hash);
      assert.equal(result.kind, "route");
      if (result.kind === "route")
        assert.equal(storeRouteToHash(result.route), hash);
    }
  const result = storeRouteFromHash("#/settings");
  assert.equal(result.kind, "route");
  if (result.kind === "route")
    assert.deepEqual(result.route.globalSettings, {
      section: "service",
      tab: "overview",
    });
});
test("global settings rejects malformed duplicate and incompatible fields", () => {
  for (const hash of [
    "#/settingsx",
    "#/settings/agents",
    "#/settings?section=agents&section=service",
    "#/settings?tab=capture&section=compute",
    "#/settings?section=unknown",
    "#/settings?section=agents&tab=",
    "#/settings?section=agents&token=secret",
    "#/settings?section=agents&tab=capture&tab=capture",
  ])
    assert.equal(storeRouteFromHash(hash).kind, "invalid", hash);
});
test("global settings return retains the exact repository and Stacks entry", () => {
  const parsed = storeRouteFromHash(
    `#/repos/${"ab".repeat(16)}/oplog?context=stacks&stack_history=pinned&at_op=${"cd".repeat(64)}&operation=${"ef".repeat(64)}`,
  );
  if (parsed.kind !== "route") throw Error("fixture");
  let route = parsed.route;
  const original = structuredClone(route);
  const nav = createGlobalSettingsNavigation({
    route: () => route,
    push: (r) => {
      route = r;
    },
    replace: (r) => {
      route = r;
    },
  });
  nav.open("agents", "runtimes");
  nav.select("compute", "sharing");
  nav.leave();
  assert.deepEqual(route, original);
});
test("root startup resumes only validated section and tab while explicit routes win", () => {
  const values = new Map<string, string>();
  const storage = {
    getItem: (k: string) => values.get(k) ?? null,
    setItem: (k: string, v: string) => {
      values.set(k, v);
    },
    removeItem: (k: string) => {
      values.delete(k);
    },
  };
  rememberGlobalSettingsHash(
    "#/settings?section=agents&tab=configuration",
    storage,
  );
  assert.equal(
    resumeGlobalSettingsHash("", storage),
    "#/settings?section=agents&tab=configuration",
  );
  assert.equal(
    resumeGlobalSettingsHash("#/projects?tab=projects", storage),
    "#/projects?tab=projects",
  );
  assert.deepEqual(JSON.parse([...values.values()][0]), {
    section: "agents",
    tab: "configuration",
  });
  rememberGlobalSettingsHash("#/projects?tab=projects", storage);
  assert.equal(values.size, 0);
  values.set(
    "jj-store/settings-destination/v1",
    '{"section":"agents","tab":"bad"}',
  );
  assert.equal(resumeGlobalSettingsHash("", storage), "");
  const blocked = {
    getItem: () => {
      throw Error("denied");
    },
    setItem: () => {
      throw Error("denied");
    },
    removeItem: () => {
      throw Error("denied");
    },
  };
  assert.equal(resumeGlobalSettingsHash("", blocked), "");
  assert.doesNotThrow(() => rememberGlobalSettingsHash("#/settings", blocked));
});
test("global loading geometry follows every selected subtab", () => {
  for (const [section, tabs] of Object.entries(globalSettingsTabs))
    for (const tab of tabs)
      assert.ok(
        globalSettingsSkeletonCards(
          section as keyof typeof globalSettingsTabs,
          tab,
        ).length,
      );
  assert.deepEqual(globalSettingsSkeletonCards("agents", "runtimes")[0].rows, [
    "toggle",
    "toggle",
    "disclosure",
  ]);
  assert.deepEqual(
    globalSettingsSkeletonCards("compute", "resources")[1].rows,
    ["numeric"],
  );
  assert.equal(globalSettingsSkeletonCards("service", "launch").length, 3);
  assert.equal(globalSettingsSkeletonCards("service", "storage").length, 4);
});

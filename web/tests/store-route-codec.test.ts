import assert from "node:assert/strict";
import test from "node:test";
import type { StoreRoute } from "../src/lib/controller/contracts.js";
import {
  storeRouteFromHash,
  storeRouteToHash,
} from "../src/lib/controller/store-route-codec.js";

const repository = "ab".repeat(16);
const operation = "12".repeat(64);
const commit = "34".repeat(64);
const inspected = "56".repeat(64);
const parent = "78".repeat(64);
const change = "9a".repeat(16);
const group = "bc".repeat(16);
const session = "de".repeat(16);
const thread = "f0".repeat(16);

const route: StoreRoute = {
  page: "oplog",
  repository,
  bookmark: "review/native objects",
  workspace: null,
  snapshotOperationId: operation,
  inspectedOperationId: inspected,
  commitId: null,
  evolutionCommitId: null,
  changeId: change,
  comparisonParentId: parent,
  comparisonGroupId: group,
  path: "src/native objects.rs",
  directory: "src",
  agentSessionId: null,
  reviewThreadId: null,
};

test("native operation route hash round-trips every stable selection", () => {
  const hash = storeRouteToHash(route);
  assert.ok(hash?.startsWith(`#/repos/${repository}/oplog?`));
  assert.ok(hash.includes("file=src%2Fnative+objects.rs"));
  assert.deepEqual(storeRouteFromHash(hash), { kind: "route", route });
  assert.doesNotMatch(hash, /token|cursor/u);
});

test("evolution column preserves its full anchor beside the main selected change", () => {
  const evolutionRoute: StoreRoute = {
    ...route,
    page: "log",
    commitId: commit,
    evolutionCommitId: inspected,
    inspectedOperationId: null,
    comparisonParentId: null,
    comparisonGroupId: null,
  };
  const hash = storeRouteToHash(evolutionRoute)!;
  assert.ok(hash.includes(`evolution=${inspected}`));
  assert.deepEqual(storeRouteFromHash(hash), {
    kind: "route",
    route: evolutionRoute,
  });
  for (const invalid of [
    hash.replace(`/log?`, `/files?`),
    hash.replace(`commit=${commit}&`, ""),
    hash.replace(`evolution=${inspected}`, "evolution=short"),
    `${hash}&evolution=${inspected}`,
  ])
    assert.equal(storeRouteFromHash(invalid).kind, "invalid");
});

test("collaboration routes preserve stable session and thread selections", () => {
  const agentRoute: StoreRoute = {
    ...route,
    page: "agents",
    bookmark: "",
    inspectedOperationId: null,
    changeId: null,
    comparisonParentId: null,
    comparisonGroupId: null,
    path: null,
    directory: "",
    agentSessionId: session,
  };
  const reviewRoute: StoreRoute = {
    ...agentRoute,
    page: "reviews",
    agentSessionId: null,
    reviewThreadId: thread,
  };
  assert.deepEqual(storeRouteFromHash(storeRouteToHash(agentRoute)!), {
    kind: "route",
    route: agentRoute,
  });
  const agentReviewRoute = { ...agentRoute, reviewThreadId: thread };
  assert.deepEqual(storeRouteFromHash(storeRouteToHash(agentReviewRoute)!), {
    kind: "route",
    route: agentReviewRoute,
  });
  assert.deepEqual(storeRouteFromHash(storeRouteToHash(reviewRoute)!), {
    kind: "route",
    route: reviewRoute,
  });
  for (const page of ["log", "branches"] as const) {
    const contextRoute: StoreRoute = {
      ...route,
      page,
      inspectedOperationId: null,
      commitId: commit,
      changeId: change,
      comparisonParentId: null,
      comparisonGroupId: null,
      path: null,
      directory: "",
      agentSessionId: session,
      reviewThreadId: thread,
    };
    assert.deepEqual(storeRouteFromHash(storeRouteToHash(contextRoute)!), {
      kind: "route",
      route: contextRoute,
    });
  }
});

test("repository paths and bookmark names remain safely encoded", () => {
  const unicode = {
    ...route,
    page: "files" as const,
    bookmark: "review/mañana & ready",
    inspectedOperationId: null,
    commitId: commit,
    comparisonParentId: null,
    comparisonGroupId: null,
    path: "docs/日本 語.md",
    directory: "docs",
  };
  const hash = storeRouteToHash(unicode)!;
  assert.equal(storeRouteFromHash(hash).kind, "route");
  assert.deepEqual(storeRouteFromHash(hash), { kind: "route", route: unicode });
});

test("malformed, duplicate, foreign and unsafe route data fails closed", () => {
  assert.deepEqual(storeRouteFromHash("#/inbox"), { kind: "none" });
  for (const hash of [
    "#/repos/not-a-repository/files",
    `#/repos/${repository}/unknown`,
    `#/repos/${repository}/files?at_op=${operation}&at_op=${operation}`,
    `#/repos/${repository}/files?at_op=${operation.slice(2)}`,
    `#/repos/${repository}/files?file=../secret`,
    `#/repos/${repository}/files?bookmark=main`,
    `#/repos/${repository}/files?at_op=${operation}&operation=${inspected}`,
    `#/repos/${repository}/files?future=value`,
    `#/repos/${repository}/files?at_op=${operation}&session=${session}`,
    "#/repos/%zz/files",
  ]) {
    assert.equal(storeRouteFromHash(hash).kind, "invalid", hash);
  }
});

test("workspace names round-trip independently of shared commit identities", () => {
  for (const workspace of ["default", "feature/日本 語 & review"]) {
    const selected = { ...route, workspace };
    assert.deepEqual(storeRouteFromHash(storeRouteToHash(selected)!), {
      kind: "route",
      route: selected,
    });
  }
  for (const suffix of [
    "workspace=",
    "workspace=a&workspace=b",
    "workspace=%00",
    "workspace=default",
  ]) {
    const snapshot =
      suffix === "workspace=default" ? "" : `at_op=${operation}&`;
    assert.equal(
      storeRouteFromHash(`#/repos/${repository}/files?${snapshot}${suffix}`)
        .kind,
      "invalid",
    );
  }
});

test("WorkStore task routes need a project and preserve independent code pin", () => {
  const workRoute: StoreRoute = {
    ...route,
    page: "projects",
    bookmark: "",
    workspace: null,
    snapshotOperationId: null,
    inspectedOperationId: null,
    commitId: null,
    changeId: null,
    comparisonParentId: null,
    comparisonGroupId: null,
    path: null,
    directory: "",
    agentSessionId: null,
    reviewThreadId: null,
    projectId: "11".repeat(16),
    taskId: "22".repeat(16),
  };
  assert.deepEqual(storeRouteFromHash(storeRouteToHash(workRoute)!), {
    kind: "route",
    route: workRoute,
  });
  assert.equal(
    storeRouteFromHash(`#/repos/${repository}/projects?task=${"22".repeat(16)}`)
      .kind,
    "invalid",
  );
  assert.equal(
    storeRouteFromHash(`#/repos/${repository}/files?project=${"11".repeat(16)}`)
      .kind,
    "invalid",
  );
});

test("Projects hub routes round-trip full identities and filters without a code pin", () => {
  const base = storeRouteFromHash("#/projects");
  assert.equal(base.kind, "route");
  if (base.kind !== "route") return;
  const selected = {
    ...base.route,
    page: "projects" as const,
    projectsHub: true,
    repository: "11".repeat(16),
    projectId: "22".repeat(16),
    taskId: "33".repeat(16),
    workTab: "tasks" as const,
    workQuery: "native tasks",
    workState: "blocked" as const,
    workRepository: "44".repeat(16),
    workHome: "",
  };
  const hash = storeRouteToHash(selected)!;
  const result = storeRouteFromHash(hash);
  assert.equal(result.kind, "route");
  if (result.kind === "route") assert.deepEqual(result.route, selected);
  for (const invalid of [
    "#/projects?task=" + "33".repeat(16),
    "#/projects?home=bad",
    "#/projects?tab=board",
    "#/projects?tab=tasks&tab=projects",
  ])
    assert.equal(storeRouteFromHash(invalid).kind, "invalid");
});

test("Projects hub carries a legacy home code operation independently of WorkStore", () => {
  const parsed = storeRouteFromHash(
    `#/projects?home=${repository}&project=${"11".repeat(16)}&at_op=${operation}`,
  );
  assert.equal(parsed.kind, "route");
  if (parsed.kind !== "route") return;
  assert.equal(parsed.route.snapshotOperationId, operation);
  assert.ok(storeRouteToHash(parsed.route)?.includes(`at_op=${operation}`));
  assert.equal(
    storeRouteFromHash(`#/projects?at_op=${operation}`).kind,
    "invalid",
  );
});

test("activity filters round trip and reject invalid or repeated classifications", () => {
  for (const activity of ["all", "agent", "unassisted"]) {
    const hash = `#/projects?tab=activity&activity=${activity}`;
    const parsed = storeRouteFromHash(hash);
    assert.equal(parsed.kind, "route");
    if (parsed.kind === "route") {
      assert.equal(parsed.route.workActivity, activity);
      const restored = storeRouteFromHash(storeRouteToHash(parsed.route)!);
      assert.equal(restored.kind, "route");
      if (restored.kind === "route")
        assert.equal(restored.route.workActivity, activity);
    }
  }
  for (const query of [
    "activity=missing",
    "activity=agent&activity=unassisted",
    "activity=",
  ]) {
    assert.equal(
      storeRouteFromHash(`#/projects?tab=activity&${query}`).kind,
      "invalid",
    );
  }
});

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
  snapshotOperationId: operation,
  inspectedOperationId: inspected,
  commitId: null,
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
    `#/repos/${repository}/agents?at_op=${operation}&thread=${thread}`,
    "#/repos/%zz/files",
  ]) {
    assert.equal(storeRouteFromHash(hash).kind, "invalid", hash);
  }
});

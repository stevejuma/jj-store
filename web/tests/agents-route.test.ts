import { test } from "node:test";
import assert from "node:assert/strict";
import {
  storeRouteFromHash,
  storeRouteToHash,
} from "../src/lib/controller/store-route-codec.js";
const session = "44".repeat(16);
test("installation Agents routes round trip without a repository", () => {
  for (const hash of [
    "#/agents",
    "#/agents/new?agent=helper",
    "#/agents/sessions",
    "#/agents/sessions?agent=helper",
    `#/agents/sessions/${session}`,
    `#/agents/sessions/${session}?agent=helper`,
    "#/agents/activity?agent=helper",
    `#/agents/activity?agent=helper&session=${session}`,
  ]) {
    const result = storeRouteFromHash(hash);
    assert.equal(result.kind, "route");
    if (result.kind !== "route") return;
    assert.equal(result.route.repository, "");
    assert.equal(storeRouteToHash(result.route), hash);
  }
});
test("Agents routes reject duplicate, foreign and malformed selections", () => {
  for (const hash of [
    "#/agents?token=secret",
    "#/agents/new?agent=a&agent=b",
    "#/agents/sessions/nope",
    "#/agents/new?agent=%00",
    "#/agents?agent=helper",
    "#/agents/activity?agent=a&agent=b",
    "#/agents/activity?session=nope",
    `#/agents/activity?session=${session}&session=${session}`,
    "#/agents/sessions?session=nope",
  ]) {
    assert.equal(storeRouteFromHash(hash).kind, "invalid");
  }
});

import { expect, test } from "vitest";
import {
  createStoreRouteHistory,
  type StoreRouteBrowser,
} from "../../src/lib/controller/store-route-history.svelte.js";
import { storeRouteToHash } from "../../src/lib/controller/store-route-codec.js";

const repository = "ab".repeat(16);
const operation = "12".repeat(64);
const session = "34".repeat(16);
const thread = "56".repeat(16);

function fakeBrowser(initial: string) {
  let hash = initial;
  let listener: (() => void) | undefined;
  const pushed: string[] = [];
  const replaced: string[] = [];
  const moved: number[] = [];
  const browser: StoreRouteBrowser = {
    hash: () => hash,
    push(value) {
      hash = value;
      pushed.push(value);
    },
    replace(value) {
      hash = value;
      replaced.push(value);
    },
    go(delta) {
      moved.push(delta);
    },
    listen(next) {
      listener = next;
      return () => (listener = undefined);
    },
  };
  return {
    browser,
    pushed,
    replaced,
    moved,
    visit(value: string) {
      hash = value;
      listener?.();
    },
  };
}

test("route history restores, pushes and replaces native browser hashes", () => {
  const initial = `#/repos/${repository}/files?at_op=${operation}&bookmark=main`;
  const navigation = fakeBrowser(initial);
  const history = createStoreRouteHistory(navigation.browser);
  expect(history.route).toMatchObject({
    repository,
    page: "files",
    snapshotOperationId: operation,
    bookmark: "main",
  });

  history.page("log");
  expect(navigation.pushed).toEqual([storeRouteToHash(history.route)]);
  history.replace({ ...history.route, commitId: "34".repeat(64) });
  expect(navigation.replaced).toEqual([storeRouteToHash(history.route)]);
  history.navigate(-1);
  expect(navigation.moved).toEqual([-1]);
  history.destroy();
});

test("external browser navigation restores known and newly entered routes", () => {
  const navigation = fakeBrowser("");
  const history = createStoreRouteHistory(navigation.browser);
  const visited: string[] = [];
  history.onExternalRoute((result) => {
    if (result.kind === "route") visited.push(result.route.page);
  });
  history.connect(repository, operation);
  history.page("files");
  const files = storeRouteToHash(history.route)!;
  history.page("oplog");

  navigation.visit(files);
  expect(history.route.page).toBe("files");
  navigation.visit(`#/repos/${repository}/branches?at_op=${operation}`);
  expect(history.route.page).toBe("branches");
  expect(visited).toEqual(["files", "branches"]);
  history.destroy();
});

test("invalid owned hashes remain explicit and never change selection", () => {
  const navigation = fakeBrowser(`#/repos/${repository}/files`);
  const history = createStoreRouteHistory(navigation.browser);
  const before = history.route;
  navigation.visit(`#/repos/${repository}/files?file=../secret`);
  expect(history.route).toEqual(before);
  expect(history.error).toBe("Invalid repository route value.");
  history.destroy();
});

test("collaboration selections survive reload and browser Back and Forward", () => {
  const agentHash = `#/repos/${repository}/agents?at_op=${operation}&session=${session}`;
  const navigation = fakeBrowser(agentHash);
  const history = createStoreRouteHistory(navigation.browser);
  expect(history.route).toMatchObject({
    page: "agents",
    agentSessionId: session,
    reviewThreadId: null,
  });

  history.page("reviews");
  history.push({ ...history.route, reviewThreadId: thread });
  const reviewHash = storeRouteToHash(history.route)!;
  navigation.visit(agentHash);
  expect(history.route).toMatchObject({
    page: "agents",
    agentSessionId: session,
  });
  navigation.visit(reviewHash);
  expect(history.route).toMatchObject({
    page: "reviews",
    reviewThreadId: thread,
  });

  const reloaded = createStoreRouteHistory(fakeBrowser(reviewHash).browser);
  expect(reloaded.route).toMatchObject({
    page: "reviews",
    reviewThreadId: thread,
  });
  reloaded.destroy();
  history.destroy();
});

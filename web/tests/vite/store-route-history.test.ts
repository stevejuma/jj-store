import { expect, test } from "vitest";
import {
  createStoreRouteHistory,
  type StoreRouteBrowser,
} from "../../src/lib/controller/store-route-history.svelte.js";
import { storeRouteToHash } from "../../src/lib/controller/store-route-codec.js";
import { storeRouteFromHash } from "../../src/lib/controller/store-route-codec.js";

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

test("refreshed repository routes retain Forward after browser Back", () => {
  const navigation = fakeBrowser(
    `#/repos/${repository}/branches?at_op=${operation}`,
  );
  const history = createStoreRouteHistory(navigation.browser);
  history.page("files");
  history.replace({
    ...history.route,
    bookmark: "main",
    commitId: "78".repeat(64),
  });
  const files = storeRouteToHash(history.route)!;
  history.push({ ...history.route, path: "bin" });
  history.refresh(operation);
  const preview = storeRouteToHash(history.route)!;
  navigation.visit(files);
  expect(history.route.path).toBeNull();
  expect(history.canForward).toBe(true);
  navigation.visit(preview);
  expect(history.route.path).toBe("bin");
  expect(history.canBack).toBe(true);
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

test("workspace names survive browser Back, Forward, reload and repository switches", () => {
  const initial = `#/repos/${repository}/files?at_op=${operation}&workspace=default`;
  const navigation = fakeBrowser(initial),
    history = createStoreRouteHistory(navigation.browser);
  history.push({ ...history.route, workspace: "agent/review" });
  const other = storeRouteToHash(history.route)!;
  history.page("agents");
  expect(history.route.workspace).toBe("agent/review");
  navigation.visit(initial);
  expect(history.route.workspace).toBe("default");
  navigation.visit(other);
  expect(history.route.workspace).toBe("agent/review");
  const reload = createStoreRouteHistory(fakeBrowser(other).browser);
  expect(reload.route.workspace).toBe("agent/review");
  history.connect("ef".repeat(16), operation);
  expect(history.route.workspace).toBeNull();
  expect(history.canBack).toBe(true);
  navigation.visit(other);
  history.connect(repository, operation);
  expect(history.route.workspace).toBe("agent/review");
  reload.destroy();
  history.destroy();
});

test("Projects and Evo Log restore independent identities through browser history", () => {
  const commit = "78".repeat(64),
    evolution = "9a".repeat(64),
    project = "bc".repeat(16),
    task = "de".repeat(16);
  const initial = `#/repos/${repository}/log?at_op=${operation}&commit=${commit}&evolution=${evolution}`;
  const navigation = fakeBrowser(initial),
    history = createStoreRouteHistory(navigation.browser);
  history.page("projects");
  expect(history.route).toMatchObject({
    page: "projects",
    snapshotOperationId: operation,
    commitId: null,
    evolutionCommitId: null,
  });
  history.push({ ...history.route, projectId: project, taskId: task });
  const projects = storeRouteToHash(history.route)!;
  history.page("log");
  expect(history.route).toMatchObject({
    page: "log",
    snapshotOperationId: operation,
    projectId: null,
    taskId: null,
  });
  navigation.visit(initial);
  expect(history.route).toMatchObject({
    page: "log",
    commitId: commit,
    evolutionCommitId: evolution,
    snapshotOperationId: operation,
  });
  navigation.visit(projects);
  const reload = createStoreRouteHistory(fakeBrowser(projects).browser);
  for (const restored of [history, reload]) {
    expect(restored.route).toMatchObject({
      page: "projects",
      projectId: project,
      taskId: task,
      snapshotOperationId: operation,
      commitId: null,
      evolutionCommitId: null,
    });
    restored.destroy();
  }
});

test("connecting code from the installation Projects hub preserves one Back destination", () => {
  const navigation = fakeBrowser("#/projects?tab=reviews");
  const history = createStoreRouteHistory(navigation.browser);
  history.connect(repository, operation);
  expect(history.route).toMatchObject({
    repository,
    page: "files",
    projectsHub: false,
  });
  expect(navigation.pushed).toHaveLength(1);
  expect(navigation.replaced).toHaveLength(0);
  navigation.visit("#/projects?tab=reviews");
  expect(history.route).toMatchObject({ page: "projects", workTab: "reviews" });
  history.destroy();
});

test("ordinary activity commit links retain the exact Projects Back and Forward destinations", () => {
  const origin = `#/projects?home=${repository}&project=${"78".repeat(16)}&tab=activity&q=parser&state=active&at_op=${operation}`;
  const navigation = fakeBrowser(origin);
  const history = createStoreRouteHistory(navigation.browser);
  const destination = `#/repos/${repository}/log?at_op=${operation}&commit=${"9a".repeat(64)}`;
  navigation.visit(destination);
  expect(history.canBack).toBe(true);
  history.navigate(-1);
  expect(navigation.moved).toEqual([-1]);
  navigation.visit(origin);
  expect(storeRouteToHash(history.route)).toBe(origin);
  expect(history.canForward).toBe(true);
  history.navigate(1);
  navigation.visit(destination);
  expect(history.route.page).toBe("log");
  expect(history.canBack).toBe(true);
  history.destroy();
});

test("retired review column links restore the Community list or selected detail", () => {
  const initial = `#/repos/${repository}/reviews?at_op=${operation}&thread=${thread}`;
  const legacy = `${initial}&review_view=columns`;
  const history = createStoreRouteHistory(fakeBrowser(legacy).browser);
  expect(history.route.reviewThreadId).toBe(thread);
  expect(storeRouteToHash(history.route)).toBe(initial);
  expect(storeRouteFromHash(legacy.replace("reviews?", "agents?")).kind).toBe(
    "invalid",
  );
  expect(storeRouteFromHash(legacy.replace("columns", "unknown")).kind).toBe(
    "invalid",
  );
  history.destroy();
});

test("repository Projects reload retains its repository destination and code pin", () => {
  const hash = `#/repos/${repository}/projects?at_op=${operation}`;
  const navigation = fakeBrowser(hash);
  const history = createStoreRouteHistory(navigation.browser);
  history.connect(repository, operation);
  expect(storeRouteToHash(history.route)).toBe(hash);
  expect(history.route.projectsHub).not.toBe(true);
  expect(history.canBack).toBe(false);
  history.destroy();
});

test("Stacks round-trips as a separate application route", () => {
  const navigation = fakeBrowser(`#/repos/${repository}/branches`);
  const history = createStoreRouteHistory(navigation.browser);
  history.page("stacks");
  expect(history.route.page).toBe("stacks");
  expect(navigation.pushed).toEqual([
    `#/repos/${repository}/stacks?stack_tab=workspace`,
  ]);
  expect(storeRouteFromHash(navigation.pushed[0]!)).toMatchObject({
    kind: "route",
    route: { repository, page: "stacks" },
  });
  navigation.visit(`#/repos/${repository}/branches`);
  expect(history.route.page).toBe("branches");
  history.destroy();
});

test("focused history survives browser Back, Forward and reload with exact selections", () => {
  const navigation = fakeBrowser(
    `#/repos/${repository}/files?at_op=${operation}&bookmark=main`,
  );
  const history = createStoreRouteHistory(navigation.browser);
  const before = history.route;
  history.page("stacks");
  history.stacks.select("branches");
  const branches = storeRouteToHash(history.route)!;
  history.stacks.select("log");
  history.push({
    ...history.route,
    commitId: "34".repeat(20),
    path: "README.md",
  });
  const commits = storeRouteToHash(history.route)!;
  history.stacks.select("oplog");
  history.push({ ...history.route, inspectedOperationId: operation });
  const operations = storeRouteToHash(history.route)!;
  navigation.visit(branches);
  expect(history.route.stackTab).toBe("branches");
  navigation.visit(commits);
  expect(history.route).toMatchObject({
    page: "log",
    context: "stacks",
    commitId: "34".repeat(20),
    path: "README.md",
  });
  navigation.visit(operations);
  const reloaded = createStoreRouteHistory(navigation.browser);
  expect(storeRouteToHash(reloaded.route)).toBe(
    storeRouteToHash(history.route),
  );
  expect(reloaded.route).toMatchObject({
    page: "oplog",
    context: "stacks",
    inspectedOperationId: operation,
    snapshotOperationId: operation,
  });
  reloaded.stacks.leave();
  expect(reloaded.route).toMatchObject({
    page: "files",
    snapshotOperationId: operation,
  });
  history.stacks.leave();
  expect(history.route).toEqual(before);
  reloaded.destroy();
  history.destroy();
});

test("file display modes restore through Back/Forward and clear on page changes", () => {
  const base = `#/repos/${repository}/files?at_op=${operation}&commit=${"34".repeat(20)}&file=test.ts`;
  const browser = fakeBrowser(base + "&file_view=blame");
  const history = createStoreRouteHistory(browser.browser);
  expect(history.route.fileDisplay).toBe("blame");
  history.push({ ...history.route, fileDisplay: "source" });
  browser.visit(base + "&file_view=blame");
  expect(history.route.fileDisplay).toBe("blame");
  browser.visit(base + "&file_view=source");
  expect(history.route.fileDisplay).toBe("source");
  history.page("log");
  expect(history.route.fileDisplay).toBeUndefined();
  history.destroy();
});

test("focused live and pinned histories survive reload and browser Back/Forward", () => {
  const live = `#/repos/${repository}/log?context=stacks&stack_history=live&at_op=${operation}`;
  const navigation = fakeBrowser(live);
  const history = createStoreRouteHistory(navigation.browser);
  history.selectSnapshot("98".repeat(64));
  const pinned = storeRouteToHash(history.route)!;
  expect(history.route.stackHistory).toBe("pinned");
  navigation.visit(live);
  expect(history.route.stackHistory).toBe("live");
  navigation.visit(pinned);
  expect(history.route.snapshotOperationId).toBe("98".repeat(64));
  expect(history.route.stackHistory).toBe("pinned");
  history.destroy();
  const reloaded = createStoreRouteHistory(navigation.browser);
  expect(reloaded.route.stackHistory).toBe("pinned");
  reloaded.destroy();
});

test("global Agents and recorded repository history survive Back, Forward and reload", () => {
  const hash = `#/repos/${repository}/agents?at_op=${operation}&session=${session}`;
  const nav = fakeBrowser(hash),
    history = createStoreRouteHistory(nav.browser);
  const directory = storeRouteFromHash("#/agents"),
    live = storeRouteFromHash(`#/agents/sessions/${session}`);
  if (directory.kind !== "route" || live.kind !== "route")
    throw new Error("Agents route missing");
  history.push(directory.route);
  history.push(live.route);
  nav.visit("#/agents");
  expect(history.route.agentsHub).toBe(true);
  expect(history.canForward).toBe(true);
  nav.visit(hash);
  expect(history.route.agentSessionId).toBe(session);
  expect(history.route.agentsHub).not.toBe(true);
  nav.visit(`#/agents/sessions/${session}`);
  expect(history.route.executionSessionId).toBe(session);
  const reload = createStoreRouteHistory(fakeBrowser("#/agents").browser);
  expect(reload.route.repository).toBe("");
  expect(reload.route.agentsHub).toBe(true);
  history.connect(repository, operation);
  expect(history.route.page).toBe("files");
  expect(history.route.agentsHub).not.toBe(true);
  history.destroy();
  reload.destroy();
});

test("settings history retains tabs and refuses guarded external owner changes", () => {
  const initial = `#/repos/${repository}/oplog?at_op=${operation}&operation=${operation}`;
  const navigation = fakeBrowser(initial),
    history = createStoreRouteHistory(navigation.browser);
  const entry = history.route;
  history.repositorySettings.open();
  history.repositorySettings.select("search", "scopes");
  const scopes = storeRouteToHash(history.route)!;
  history.repositorySettings.select("storage", "maintenance");
  navigation.visit(scopes);
  expect(history.route.repositorySettingsTab).toBe("scopes");
  history.setNavigationGuard("pending", (current, next) =>
    current.repository !== next.repository ? "Resolve pending storage" : "",
  );
  navigation.visit(`#/repos/${"ef".repeat(16)}/settings?section=general`);
  expect(history.route.repository).toBe(repository);
  expect(navigation.replaced.at(-1)).toBe(scopes);
  expect(history.error).toBe("Resolve pending storage");
  history.repositorySettings.leave();
  expect(history.route).toEqual(entry);
  history.setNavigationGuard("pending");
  navigation.visit(`#/repos/${"ef".repeat(16)}/settings?section=general`);
  expect(history.route.repository).toBe("ef".repeat(16));
  history.destroy();
});

test("a guarded browser Back preserves its original entry for a later accepted Back", () => {
  const initial = `#/repos/${repository}/files?at_op=${operation}`;
  const entries = [{ hash: initial, id: "root" }];
  let position = 0,
    notify: (() => void) | undefined;
  const browser: StoreRouteBrowser = {
    hash: () => entries[position].hash,
    entryId: () => entries[position].id,
    push(hash, id) {
      entries.splice(++position);
      entries.push({ hash, id: id! });
    },
    replace(hash, id) {
      entries[position] = { hash, id: id! };
    },
    go(delta) {
      position += delta;
      notify?.();
      notify?.();
    },
    listen(listener) {
      notify = listener;
      return () => {
        notify = undefined;
      };
    },
  };
  const history = createStoreRouteHistory(browser);
  history.push({ ...history.route, path: "bin" });
  history.setNavigationGuard("draft", () => "Finish editing");
  browser.go(-1);
  expect(history.route.path).toBe("bin");
  expect(entries[0].hash).toBe(initial);
  expect(position).toBe(1);
  expect(history.error).toBe("Finish editing");
  history.setNavigationGuard("draft");
  browser.go(-1);
  expect(history.route.path).toBeNull();
  expect(position).toBe(0);
  history.destroy();
});

import { test } from "node:test";
import assert from "node:assert/strict";
import {
  connectionHash,
  readConnectionHash,
  connectionFetch,
} from "../src/lib/connections/transport.js";
test("connection route defaults and exact selection round trip", () => {
  const id = "a".repeat(32),
    route =
      "#/repos/" +
      "b".repeat(32) +
      "/settings?section=search&tab=scopes&at_op=123";
  assert.deepEqual(readConnectionHash(connectionHash(route, id)), {
    id,
    hash: route,
  });
  assert.deepEqual(readConnectionHash(route), { id: "local", hash: route });
  assert.equal(connectionHash(route, "local"), route);
  assert.throws(() => readConnectionHash("#/projects?server=bad"));
  assert.throws(() =>
    readConnectionHash("#/projects?server=" + id + "&server=" + id),
  );
});
test("gateway fetch captures origin session and never forwards bearer headers locally", async () => {
  const calls: { path: string; headers: Headers }[] = [];
  const id = "a".repeat(32);
  const fetcher = connectionFetch(id, "b".repeat(32), async (input, init) => {
    calls.push({ path: String(input), headers: new Headers(init?.headers) });
    return new Response("{}", {
      headers: { "content-type": "application/json" },
    });
  });
  await fetcher("/v1/repos/" + id + "/files?path=a%20b", {
    headers: { authorization: "Bearer secret", "x-jj-store-protocol": "1" },
  });
  assert.equal(
    calls[0]!.path,
    "/v1/local/connections/" + id + "/api/repos/" + id + "/files?path=a%20b",
  );
  assert.equal(calls[0]!.headers.has("authorization"), false);
  assert.equal(
    calls[0]!.headers.get("x-jj-store-connection-session"),
    "b".repeat(32),
  );
  await assert.rejects(fetcher("https://foreign.example/v1/repos"));
  await assert.rejects(fetcher("/v1/local/compute"));
});

test("public preferences are bounded and layout values are isolated", async () => {
  const { readPreferences, connectionStorage } =
    await import("../src/lib/connections/preferences.js");
  const id = "a".repeat(32);
  assert.deepEqual(
    readPreferences(
      JSON.stringify({
        version: 1,
        selected: id,
        routes: { [id]: "#/settings?section=appearance" },
      }),
    ),
    {
      version: 1,
      selected: id,
      routes: { [id]: "#/settings?section=appearance" },
    },
  );
  assert.equal(
    readPreferences(
      JSON.stringify({ version: 1, selected: "../secret", routes: {} }),
    ).selected,
    "local",
  );
  assert.equal(
    readPreferences(
      JSON.stringify({
        version: 1,
        selected: id,
        routes: { [id]: "x".repeat(5000) },
      }),
    ).selected,
    "local",
  );
  const values = new Map<string, string>();
  const storage: Storage = {
    get length() {
      return values.size;
    },
    key: (i) => [...values.keys()][i] ?? null,
    getItem: (k) => values.get(k) ?? null,
    setItem: (k, v) => {
      values.set(k, v);
    },
    removeItem: (k) => {
      values.delete(k);
    },
    clear: () => values.clear(),
  };
  const local = connectionStorage(storage, "local")!,
    remote = connectionStorage(storage, id)!;
  local.setItem("repo-layout", "local");
  remote.setItem("repo-layout", "remote");
  assert.equal(local.getItem("repo-layout"), "local");
  assert.equal(remote.getItem("repo-layout"), "remote");
  remote.clear();
  assert.equal(local.getItem("repo-layout"), "local");
});

test("inactive history owners retain exact routes without rewriting selected history", async () => {
  const { connectionBrowser } =
    await import("../src/lib/connections/browser.js");
  const id = "a".repeat(32);
  let selected = "local",
    hash = "#/repos/" + "b".repeat(32) + "/files?path=README.md";
  const notifications: (() => void)[] = [];
  const base = {
    hash: () => hash,
    push: (v: string) => {
      hash = v;
    },
    replace: (v: string) => {
      hash = v;
    },
    go: () => {},
    listen: (fn: () => void) => {
      notifications.push(fn);
      return () => {};
    },
  };
  const local = connectionBrowser(
    base,
    "local",
    () => selected === "local",
    hash,
    () => {},
  );
  const remote = connectionBrowser(
    base,
    id,
    () => selected === id,
    "#/settings?section=appearance&tab=preferences",
    () => {},
  );
  remote.replace("#/projects?tab=tasks");
  assert.match(hash, /README/);
  selected = id;
  remote.push("#/settings?section=service&tab=launch");
  assert.equal(readConnectionHash(hash).id, id);
  local.replace("#/repos/" + "b".repeat(32) + "/stacks?stack_tab=workspace");
  assert.match(hash, /section=service/);
  selected = "local";
  base.push(local.hash());
  assert.match(hash, /stacks\?stack_tab=workspace/);
});

test("authenticated agents use public execution while local management is refused", async () => {
  const { authenticatedAgentFetch } =
    await import("../src/lib/connections/agent-transport.js");
  let path = "",
    headers = new Headers();
  const call = authenticatedAgentFetch(
    async (input, init) => {
      path = String(input);
      headers = new Headers(init?.headers);
      return new Response("{}");
    },
    () => "memory-token",
  );
  await call("/v1/local/agents/chat", { method: "POST" });
  assert.equal(path, "/v1/agents/chat");
  assert.equal(headers.get("authorization"), "Bearer memory-token");
  assert.throws(() => call("/v1/local/agents/secrets"));
  assert.throws(() => call("/v1/local/compute"));
});

test("old connection failures cannot invalidate a replacement sign-in", async () => {
  const { connectionInvalidation } =
    await import("../src/lib/connections/management.js");
  const connection = {
    id: "a".repeat(32),
    name: "Builder",
    origin: "https://builder.example",
    installation: "b".repeat(32),
  };
  const sessions = new Map([[connection.id, "old-handle"]]);
  const expired: string[] = [];
  const invalidate = connectionInvalidation(
    sessions,
    { connection, handle: "old-handle", principal: "c".repeat(32) },
    (id) => expired.push(id),
  );
  invalidate();
  sessions.set(connection.id, "replacement-handle");
  await new Promise<void>((resolve) => queueMicrotask(resolve));
  assert.deepEqual(expired, []);
  const current = connectionInvalidation(
    sessions,
    { connection, handle: "replacement-handle", principal: "c".repeat(32) },
    (id) => expired.push(id),
  );
  current();
  await new Promise<void>((resolve) => queueMicrotask(resolve));
  assert.deepEqual(expired, [connection.id]);
});

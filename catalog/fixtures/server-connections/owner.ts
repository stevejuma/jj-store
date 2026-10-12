import { expect, waitFor } from "storybook/test";
import { createServerConnections } from "../../../web/src/lib/connections/owner.svelte.js";
import {
  connectionHash,
  readConnectionHash,
} from "../../../web/src/lib/connections/transport.js";
import { preferenceKey } from "../../../web/src/lib/connections/preferences.js";
import type { StoreRouteBrowser } from "../../../web/src/lib/controller/store-route-browser.js";
function fixture(
  initial = "#/settings?section=appearance&tab=theme",
  saved?: string,
) {
  let index = 0;
  const entries = [initial],
    listeners = new Set<() => void>(),
    values = new Map<string, string>();
  if (saved) values.set(preferenceKey, saved);
  const browser: StoreRouteBrowser = {
    hash: () => entries[index]!,
    push: (hash) => {
      entries.splice(++index);
      entries.push(hash);
    },
    replace: (hash) => {
      entries[index] = hash;
    },
    go: (delta) => {
      index += delta;
      for (const listener of listeners) listener();
    },
    listen: (listener) => {
      listeners.add(listener);
      return () => listeners.delete(listener);
    },
  };
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
  const connection = {
    id: "a".repeat(32),
    name: "Builder",
    origin: "https://builder.example",
    installation: "b".repeat(32),
  };
  const requests: { path: string; headers: Headers }[] = [];
  let generation = 0;
  const fetcher: typeof fetch = async (input, init) => {
    const path = String(input);
    requests.push({ path, headers: new Headers(init?.headers) });
    const json = (value: unknown, status = 200) =>
      new Response(JSON.stringify(value), {
        status,
        headers: { "content-type": "application/json" },
      });
    if (path === "/v1/local/connections") return json([connection]);
    if (path === "/v1/local/connections/connect")
      return json({
        connection,
        handle: String(++generation).padStart(32, "0"),
        principal: "c".repeat(32),
      });
    if (path === "/v1/local/connections/disconnect")
      return json({ disconnected: true });
    if (path.endsWith("/bootstrap"))
      return json({
        version: 1,
        access_mode: path.includes("/api/") ? "authenticated" : "local_trusted",
        storage: { ready: true },
        capabilities: {},
        backups: [],
      });
    if (path.includes("/repositories?"))
      return json({ repositories: [], next_cursor: null });
    return json({ code: "not_available" }, 503);
  };
  const owner = createServerConnections({ browser, storage, fetcher });
  return { owner, browser, storage, connection, requests, entries };
}
export async function retainedOwners() {
  const f = fixture();
  try {
    await waitFor(() => expect(f.owner.supported).toBe(true));
    const local = f.owner.controller;
    local.execution!.drafts.local = "Unsent local draft";
    await f.owner.connect({
      name: f.connection.name,
      origin: f.connection.origin,
      token: "memory-only",
    });
    expect(f.owner.error).toBe("");
    const remote = f.owner.controller;
    expect(remote).not.toBe(local);
    remote.execution!.drafts.remote = "Unsent remote draft";
    await remote.actions.openGlobalSettings("appearance", "preferences");
    const destination = f.browser.hash();
    f.owner.select("local");
    expect(f.owner.controller).toBe(local);
    expect(local.execution!.drafts.local).toBe("Unsent local draft");
    expect(local.state.route.globalSettings?.tab).toBe("theme");
    f.owner.select(f.connection.id);
    expect(f.owner.controller).toBe(remote);
    expect(remote.execution!.drafts.remote).toBe("Unsent remote draft");
    expect(f.browser.hash()).toBe(destination);
    f.browser.go(-1);
    expect(f.owner.selected).toBe("local");
    f.browser.go(1);
    expect(f.owner.selected).toBe(f.connection.id);
    expect(remote.state.route.globalSettings?.tab).toBe("preferences");
    const saved = f.storage.getItem(preferenceKey)!;
    expect(saved).not.toContain("memory-only");
    expect(saved).not.toContain("Unsent");
    expect(f.owner.shareUrl(destination, "http://local.example/")).toBe(
      "https://builder.example/#/settings?section=appearance&tab=preferences",
    );
  } finally {
    f.owner.destroy();
  }
}
export async function guardedOwners() {
  const f = fixture();
  try {
    await waitFor(() => expect(f.owner.supported).toBe(true));
    await f.owner.connect({
      name: f.connection.name,
      origin: f.connection.origin,
      token: "secret",
    });
    const remote = f.owner.controller,
      pending = { path: "agents/chat", body: { request_id: "d".repeat(32) } };
    remote.execution!.pending = pending;
    const release = f.owner.guard(remote, () => false);
    f.owner.select("local");
    expect(f.owner.selected).toBe(f.connection.id);
    expect(remote.execution!.pending?.body).toEqual(pending.body);
    release();
    remote.execution!.pending = null;
    await f.owner.signOut(f.connection.id);
    expect(f.owner.workspaces.some((w) => w.id === f.connection.id)).toBe(
      false,
    );
    expect(f.owner.selected).toBe(f.connection.id);
    expect(f.owner.dialog).toBe(true);
    expect(readConnectionHash(f.browser.hash()).id).toBe(f.connection.id);
  } finally {
    f.owner.destroy();
  }
}
export async function restoredOwners() {
  const id = "a".repeat(32),
    hash = "#/settings?section=appearance&tab=preferences";
  const saved = JSON.stringify({
    version: 1,
    selected: id,
    routes: { [id]: hash },
  });
  const f = fixture("#/", saved);
  try {
    await waitFor(() => expect(f.owner.supported).toBe(true));
    expect(f.owner.selected).toBe(id);
    expect(f.owner.dialog).toBe(true);
    expect(f.browser.hash()).toBe(connectionHash(hash, id));
    expect(f.owner.workspaces).toHaveLength(1);
  } finally {
    f.owner.destroy();
  }
  const explicit = fixture("#/settings?section=appearance&tab=theme", saved);
  try {
    await waitFor(() => expect(explicit.owner.supported).toBe(true));
    expect(explicit.owner.selected).toBe("local");
    expect(explicit.owner.controller.state.route.globalSettings?.tab).toBe(
      "theme",
    );
  } finally {
    explicit.owner.destroy();
  }
}

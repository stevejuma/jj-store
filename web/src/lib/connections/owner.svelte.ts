import {
  connectionManagement,
  withConnectionState,
  awaitConnectionBootstrap,
  connectionOperation,
  connectionBlocked,
  connectionInvalidation,
  type Connected,
} from "./management.js";
import { untrack } from "svelte";
import { readPreferences, preferenceKey } from "./preferences.js";
import { LocalClient } from "../api/local-client.js";
import { ConnectionWorkspaces } from "./workspaces.svelte.js";
import { connectionStartup } from "./startup.js";
import { browserNavigation } from "../controller/store-route-browser.js";
import type { StoreController } from "../controller/contracts.js";
import type { StoreRouteBrowser } from "../controller/store-route-browser.js";

import {
  connectionHash,
  connectionFetch,
  readConnectionHash,
} from "./transport.js";
export type ServerConnection = import("../api/client.js").Model<"Connection">;
export function createServerConnections(
  options: {
    browser?: StoreRouteBrowser;
    storage?: Storage;
    fetcher?: typeof fetch;
  } = {},
) {
  const base = options.browser ?? browserNavigation(false);
  const fetcher: typeof fetch =
    options.fetcher ?? ((...args) => fetch(...args));
  const client = new LocalClient("", fetcher);
  let storage: Storage | undefined = options.storage;
  try {
    storage ??= window.localStorage;
  } catch {
    /* Optional public preferences. */
  }
  const { preferences, parsed } = connectionStartup(base, storage);
  const save = () => {
    try {
      storage?.setItem(preferenceKey, JSON.stringify(preferences));
    } catch {
      /* Session navigation remains usable. */
    }
  };
  let selected = $state(parsed.id);
  let supported = $state(false),
    busy = $state(false),
    error = $state(""),
    dialog = $state(false),
    editing = $state<ServerConnection | null>(null);
  let records = $state<ServerConnection[]>([]);
  const sessions = new Map<string, string>(),
    tokens = new Map<string, string>(),
    guards = new Map<StoreController, () => boolean>();
  let alive = true;
  const remember = (id: string, hash: string) => {
    preferences.routes[id] = hash;
    save();
  };
  const controllers = new ConnectionWorkspaces({
    base,
    storage,
    fetcher,
    selected: () => selected,
    route: (id) => preferences.routes[id] ?? "#/projects?tab=projects",
    remember,
    expired: expire,
  });
  const local = untrack(() => controllers.get("local"));
  function expire(id: string) {
    const handle = sessions.get(id);
    sessions.delete(id);
    tokens.delete(id);
    controllers.remove(id);
    if (handle)
      void client.request("connections/disconnect", { handle }).catch(() => {});
    if (selected === id) {
      error = "Sign in again to use this server.";
      editing = records.find((c) => c.id === id) ?? null;
      dialog = true;
    }
  }
  function permitted() {
    const current = controllers.values.find(
      (w) => w.id === selected,
    )?.controller;
    if (connectionBlocked(current, current ? guards.get(current) : undefined)) {
      error =
        "Finish the current edit or resolve its pending command before switching servers.";
      return false;
    }
    return true;
  }
  function suspend() {
    const previous = controllers.values.find(
      (w) => w.id === selected,
    )?.controller;
    previous?.actions.setConnectionActive?.(false);
  }
  function select(id: string, history = true) {
    if (id === selected) return;
    if (!permitted()) return;
    if (id !== "local" && !records.some((c) => c.id === id)) {
      suspend();
      selected = id;
      error =
        "This saved server is unavailable. Choose a server or add its connection again.";
      return;
    }
    suspend();
    selected = id;
    error = "";
    preferences.selected = id;
    save();
    const next = controllers.values.find((w) => w.id === id)?.controller;
    next?.actions.setConnectionActive?.(true);
    const destination = preferences.routes[id] ?? "#/projects?tab=projects";
    if (history) base?.push(connectionHash(destination, id));
    controllers.activate(id);
    if (id !== "local" && !sessions.has(id)) {
      editing = records.find((c) => c.id === id) ?? null;
      dialog = true;
    }
  }
  const stop = base?.listen(() => {
    try {
      const next = readConnectionHash(base.hash());
      if (next.id !== selected) {
        const before = selected;
        select(next.id, false);
        if (selected === before && next.id !== before)
          base.replace(
            connectionHash(
              preferences.routes[before] ?? "#/projects?tab=projects",
              before,
            ),
          );
      }
    } catch {
      error = "Invalid server selection.";
    }
  });
  void client
    .request<ServerConnection[]>("connections")
    .then((values) => {
      if (!alive) return;
      records = values;
      supported = true;
      const root = !parsed.hash || parsed.hash === "#/" || parsed.hash === "#";
      if (root && preferences.selected !== "local")
        select(preferences.selected);
      else if (selected !== "local") {
        const id = selected;
        selected = "local";
        select(id, false);
      }
    })
    .catch(() => {
      /* Ordinary authenticated standalone servers retain their current app. */
    });
  const state = {
    get busy() {
      return busy;
    },
    set busy(v: boolean) {
      busy = v;
    },
    get error() {
      return error;
    },
    set error(v: string) {
      error = v;
    },
    get editing() {
      return editing;
    },
    set editing(v: ServerConnection | null) {
      editing = v;
    },
    get records() {
      return records;
    },
    set records(v: ServerConnection[]) {
      records = v;
    },
    get dialog() {
      return dialog;
    },
    set dialog(v: boolean) {
      dialog = v;
    },
  };
  const management = connectionManagement({
    client,
    state,
    permitted,
    selected: () => selected,
    select,
    signOut: async (id) => {
      expire(id);
    },
    forget: (id) => {
      delete preferences.routes[id];
      save();
    },
  });
  return withConnectionState(state, {
    ...management,
    get selected() {
      return selected;
    },
    get workspaces() {
      return controllers.values;
    },
    get supported() {
      return supported;
    },
    get controller() {
      return controllers.selected() ?? local;
    },
    select,
    shareUrl(hash: string, localHref: string) {
      const origin = records.find((c) => c.id === selected)?.origin;
      return new URL(
        readConnectionHash(hash).hash,
        origin ? `${origin}/` : localHref,
      ).href;
    },
    guard(controller: StoreController, check: () => boolean) {
      guards.set(controller, check);
      return () => guards.delete(controller);
    },
    add() {
      editing = null;
      dialog = true;
      error = "";
    },
    manage(id: string) {
      editing = records.find((c) => c.id === id) ?? null;
      dialog = true;
      error = "";
    },
    async connect(input: { name: string; origin: string; token: string }) {
      return connectionOperation(state, permitted, async () => {
        const existing = editing;
        const value = await client.request<Connected>("connections/connect", {
          id: existing?.id ?? null,
          ...input,
        });
        if (!alive) return;
        const previousHandle = sessions.get(value.connection.id);
        if (previousHandle)
          void client
            .request("connections/disconnect", { handle: previousHandle })
            .catch(() => {});
        records = [
          ...records.filter((c) => c.id !== value.connection.id),
          value.connection,
        ];
        const remoteFetch = connectionFetch(
          value.connection.id,
          value.handle,
          fetcher,
          connectionInvalidation(sessions, value, expire),
        );
        sessions.set(value.connection.id, value.handle);
        tokens.set(value.connection.id, input.token);
        // Reauthentication destroys protected state; ordinary switching retains its owner.
        controllers.remove(value.connection.id);
        select(value.connection.id);
        dialog = false;
        const controller = untrack(() =>
          controllers.get(value.connection.id, remoteFetch),
        );
        await awaitConnectionBootstrap(controller, () => alive);
        await controller.actions.connect({
          repository: "",
          token: input.token,
        });
      });
    },
    async signOut(id: string) {
      if (permitted()) expire(id);
    },
    destroy() {
      alive = false;
      stop?.();
      client.close();
      controllers.destroy();
      for (const handle of sessions.values())
        void new LocalClient("", fetcher)
          .request("connections/disconnect", { handle })
          .catch(() => {});
      sessions.clear();
      tokens.clear();
    },
  });
}
export type ServerConnections = ReturnType<typeof createServerConnections>;

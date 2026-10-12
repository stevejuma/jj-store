import type { LocalClient } from "../api/local-client.js";
import type { ServerConnection } from "./owner.svelte.js";
/** Device metadata management never owns a protected controller or its transport. */
export function connectionManagement(options: {
  client: LocalClient;
  state: {
    busy: boolean;
    error: string;
    editing: ServerConnection | null;
    records: ServerConnection[];
    dialog: boolean;
  };
  permitted(): boolean;
  selected(): string;
  select(id: string): void;
  signOut(id: string): Promise<void>;
  forget(id: string): void;
}) {
  const { state, client } = options;
  return {
    async rename(name: string) {
      if (!state.editing || state.busy) return;
      state.busy = true;
      try {
        state.records = await client.request<ServerConnection[]>(
          "connections",
          { action: "rename", id: state.editing.id, name },
        );
        state.editing =
          state.records.find((c) => c.id === state.editing?.id) ?? null;
        state.dialog = false;
      } catch {
        state.error = "Unable to rename this server.";
      } finally {
        state.busy = false;
      }
    },
    async remove() {
      if (!state.editing || state.busy || !options.permitted()) return;
      const id = state.editing.id;
      state.busy = true;
      try {
        await options.signOut(id);
        state.records = await client.request<ServerConnection[]>(
          "connections",
          { action: "remove", id },
        );
        options.forget(id);
        if (options.selected() === id) options.select("local");
        state.dialog = false;
      } catch {
        state.error = "Unable to remove this server.";
      } finally {
        state.busy = false;
      }
    },
  };
}
export function withConnectionState<S extends object, A extends object>(
  state: S,
  actions: A,
): S & A {
  return Object.create(Object.getPrototypeOf(actions), {
    ...Object.getOwnPropertyDescriptors(state),
    ...Object.getOwnPropertyDescriptors(actions),
  });
}
export async function awaitConnectionBootstrap(
  controller: import("../controller/contracts.js").StoreController,
  alive: () => boolean,
) {
  const deadline = Date.now() + 35000;
  while (alive() && controller.state.accessMode === "bootstrapping") {
    if (Date.now() > deadline)
      throw new Error("Connection bootstrap timed out");
    await new Promise((resolve) => setTimeout(resolve, 20));
  }
}
export function connectionBlocked(
  controller: import("../controller/contracts.js").StoreController | undefined,
  guard?: () => boolean,
) {
  return (
    guard?.() === false ||
    !!controller?.execution?.pending ||
    !!controller?.directory?.state.pending ||
    Object.values(controller?.configuration?.state.drafts ?? {}).some(
      (draft) => draft.unknown,
    )
  );
}
export function connectionInvalidation(
  sessions: ReadonlyMap<string, string>,
  value: Connected,
  expire: (id: string) => void,
) {
  return () =>
    queueMicrotask(() => {
      if (sessions.get(value.connection.id) === value.handle)
        expire(value.connection.id);
    });
}
export type Connected = {
  connection: ServerConnection;
  handle: string;
  principal: string;
};
export async function connectionOperation(
  state: { busy: boolean; error: string },
  permitted: () => boolean,
  task: () => Promise<void>,
) {
  if (state.busy || !permitted()) return;
  state.busy = true;
  state.error = "";
  try {
    await task();
  } catch (cause) {
    const { ApiError } = await import("../api/client.js");
    state.error =
      cause instanceof ApiError ? cause.code : "connection_unavailable";
  } finally {
    state.busy = false;
  }
}

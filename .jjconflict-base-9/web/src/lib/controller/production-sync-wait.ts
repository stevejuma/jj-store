import { ApiError } from "../api/client.js";
import type { StoreDataSource } from "./native-data-source.js";
import type { SyncSettings } from "./sync-settings.js";
import { storeErrorMessage } from "./http-store-data-source.js";

export function createProductionSyncWait(options: {
  state: () => SyncSettings;
  supported: () => boolean;
  source: () => StoreDataSource;
  version: () => number;
  current: (ticket: number, source: StoreDataSource) => boolean;
}) {
  let pending: AbortController | null = null;
  let generation = 0;
  function stopSyncWait() {
    generation++;
    pending?.abort();
    pending = null;
    const wait = options.state().wait;
    if (wait.state === "capturing" || wait.state === "waiting")
      wait.state = "stopped";
  }
  async function waitForSync(capture: boolean) {
    const state = options.state();
    if (pending || !options.supported() || !state.value?.configured) return;
    if (!capture && (!state.wait.barrier || state.wait.state === "invalid"))
      return;
    const source = options.source(),
      ticket = options.version(),
      token = ++generation;
    const abort = new AbortController();
    pending = abort;
    const current = () =>
      options.current(ticket, source) && token === generation;
    const wait = state.wait;
    wait.error = "";
    try {
      if (!source.sync) throw new Error("Sync management unavailable");
      if (capture) {
        wait.state = "capturing";
        const barrier = await source.sync.capture(abort.signal);
        if (!current()) return;
        wait.barrier = barrier;
      }
      if (!wait.barrier) return;
      wait.state = "waiting";
      const result = await source.sync.wait(wait.barrier, abort.signal);
      if (!current()) return;
      wait.state = result.reached ? "reached" : "pending";
      // Status refresh and waits have independent request generations.
    } catch (cause) {
      if (!current()) return;
      wait.state =
        cause instanceof ApiError && cause.code === "invalid_sync_command"
          ? "invalid"
          : "failed";
      wait.error = storeErrorMessage(cause);
    } finally {
      if (current()) pending = null;
    }
  }
  return { waitForSync, stopSyncWait };
}

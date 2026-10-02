import { pretty } from "../api/client.js";
import type { StoreDataSource } from "./native-data-source.js";
import type { ProductionStoreState } from "./production-store-state.svelte.js";

export async function followProductionStoreEvents(
  active: StoreDataSource,
  isCurrent: () => boolean,
  data: ProductionStoreState,
  signal: AbortSignal,
  changed: () => void = () => {},
) {
  const awaitHead = await active.snapshot().catch(() => null);
  let after: string | undefined;
  let delay = 500;
  while (!signal.aborted && isCurrent()) {
    try {
      data.status.live = "live";
      await active.events(after, signal, (event, id) => {
        if (signal.aborted || !isCurrent()) return;
        after = id;
        delay = 500;
        const retained =
          data.head &&
          ("repository" in data.head
            ? data.head.repository
            : data.head.repo_id) === active.repository
            ? data.head
            : awaitHead;
        if (
          event.type === "reset" ||
          !("snapshot" in event) ||
          pretty(retained) !== pretty(event.snapshot)
        ) {
          data.status.refreshAvailable = true;
          changed();
        }
      });
    } catch {
      if (signal.aborted || !isCurrent()) return;
    }
    if (signal.aborted || !isCurrent()) return;
    data.status.live = "reconnecting";
    await new Promise((resolve) => setTimeout(resolve, delay));
    delay = Math.min(delay * 2, 10_000);
  }
}

import { pretty } from "../api/client.js";
import type { StoreDataSource } from "./native-data-source.js";
import type { ProductionStoreState } from "./production-store-state.svelte.js";

export async function followProductionStoreEvents(
  active: StoreDataSource,
  isCurrent: () => boolean,
  data: ProductionStoreState,
  signal: AbortSignal,
) {
  const awaitHead =
    active.profile === "git_sha1_jj045" ? await active.snapshot() : null;
  let after: string | undefined;
  let delay = 500;
  while (!signal.aborted && isCurrent()) {
    try {
      data.status.live = "live";
      await active.events(after, signal, (event, id) => {
        if (signal.aborted || !isCurrent()) return;
        after = id;
        delay = 500;
        if ("snapshot" in event && "profile" in event.snapshot) {
          const retained =
            data.head &&
            "profile" in data.head &&
            data.head.repository === active.repository
              ? data.head
              : awaitHead;
          if (
            retained &&
            "profile" in retained &&
            (pretty(retained.heads) !== pretty(event.snapshot.heads) ||
              retained.metadata_commit !== event.snapshot.metadata_commit ||
              String(retained.generation) !== String(event.snapshot.generation))
          )
            data.status.refreshAvailable = true;
        } else if (event.type === "ready")
          data.heads = [...event.snapshot.heads];
        else data.status.refreshAvailable = true;
      });
    } catch {
      if (signal.aborted || !isCurrent()) return;
    }
    data.status.live = "reconnecting";
    await new Promise((resolve) => setTimeout(resolve, delay));
    delay = Math.min(delay * 2, 10_000);
  }
}

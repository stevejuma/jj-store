import { createProductionAutomaticRefresh } from "./production-automatic-refresh.js";
import {
  reportProductionStoreError,
  startProductionStoreLifecycle,
} from "./production-store-lifecycle.svelte.js";
import type { ApplicationFreshness } from "./application-freshness.js";
/** Subscribe before the lifecycle discovers repositories or captures selected content. */
export function startProductionSessionEvents(
  options: Omit<
    Parameters<typeof startProductionStoreLifecycle>[0],
    "events"
  > & {
    automatic: Omit<
      Parameters<typeof createProductionAutomaticRefresh>[0],
      "data" | "route" | "source" | "signal" | "freshness" | "report"
    >;
  },
  freshness: ApplicationFreshness,
) {
  const automatic = createProductionAutomaticRefresh({
    ...options.automatic,
    data: options.data,
    route: () => options.history.route,
    source: options.source,
    signal: options.signal,
    freshness,
    report: (cause) => reportProductionStoreError(options.data, cause),
  });
  startProductionStoreLifecycle({
    ...options,
    events: async (supported, token) => {
      freshness.advertise(supported);
      if (token !== undefined) await freshness.configure(supported, token);
    },
  });
  return automatic;
}

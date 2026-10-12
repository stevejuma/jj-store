import { createProductionAutomaticRefresh } from "./production-automatic-refresh.js";
import {
  reportProductionStoreError,
  startProductionStoreLifecycle,
} from "./production-store-lifecycle.svelte.js";
import type { ApplicationFreshness } from "./application-freshness.js";
export function startConnectionEvents(
  options: Omit<
    Parameters<typeof startProductionSessionEvents>[0],
    "automatic"
  > &
    Parameters<typeof startProductionSessionEvents>[0]["automatic"],
  freshness: ApplicationFreshness,
) {
  const { projects, threads, sources, active, refresh, ...lifecycle } = options;
  return startProductionSessionEvents(
    {
      ...lifecycle,
      automatic: { projects, threads, sources, active, refresh },
    },
    freshness,
  );
}
/** Subscribe before the lifecycle discovers repositories or captures selected content. */
export function startProductionSessionEvents(
  options: Omit<
    Parameters<typeof startProductionStoreLifecycle>[0],
    "events"
  > & {
    connectionActive?(): boolean;
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
      if (token !== undefined && (options.connectionActive?.() ?? true))
        await freshness.configure(supported, token);
    },
  });
  return automatic;
}

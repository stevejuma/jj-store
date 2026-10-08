import type { ApplicationFreshness } from "./application-freshness.js";
import type { FreshnessChange } from "../api/application-events.js";
import { createAutomaticRefresh } from "./automatic-refresh.js";
/** One serial dirty reader; timers are only outage or failed-read recovery. */
export function watchAgentChanges(
  freshness: ApplicationFreshness | undefined,
  options: {
    active(): boolean;
    matches(change: FreshnessChange): boolean;
    refresh(): Promise<boolean>;
  },
) {
  if (freshness) return freshness.watch({ ...options, initial: false });
  const lifetime = new AbortController();
  const refresh = createAutomaticRefresh({
    active: options.active,
    refresh: options.refresh,
    poll: async () => true,
    fallback: () => true,
    report: () => {},
    signal: lifetime.signal,
  });
  return { request: refresh.request, stop: () => lifetime.abort() };
}

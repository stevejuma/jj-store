import { createAutomaticRefresh } from "./automatic-refresh.js";
import type { ApplicationFreshness } from "./application-freshness.js";
/** Discussion freshness never depends on runner status, targets or negotiation. */
export function watchAgentDiscussion(
  load: () => Promise<boolean | void>,
  repository: string,
  freshness?: ApplicationFreshness,
) {
  const lifetime = new AbortController();
  const refresh = createAutomaticRefresh({
    active: () => true,
    signal: lifetime.signal,

    refresh: async () => {
      return (await load()) !== false;
    },
    report: () => {},
  });
  const unlisten = freshness?.listen((notice) => {
    if (
      notice.reset ||
      notice.changes.some(
        (change) =>
          change.repository === repository &&
          ["threads", "reactions", "agents", "repository"].includes(
            change.domain,
          ),
      )
    )
      refresh.request();
    refresh.reschedule();
  });
  refresh.request();
  return () => {
    lifetime.abort();
    unlisten?.();
  };
}

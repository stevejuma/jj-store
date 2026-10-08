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
    fallback: () => !freshness?.healthy,
    poll: async () => true,
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
  const reconnect = () => refresh.request();
  if (typeof window !== "undefined")
    window.addEventListener("online", reconnect);
  refresh.request();
  return () => {
    lifetime.abort();
    unlisten?.();
    if (typeof window !== "undefined")
      window.removeEventListener("online", reconnect);
  };
}

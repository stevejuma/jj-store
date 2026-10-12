import type { StoreDataSource } from "./native-data-source.js";
import type { ApplicationFreshness } from "./application-freshness.js";
import type { AgentExecution } from "./agent-execution.svelte.js";
export function connectionActivity(
  pause: (reason: string, paused: boolean) => void,
  feed: () => AbortController,
  stopProjectEvents: () => void,
  source: () => StoreDataSource | null,
  browsing: () => boolean,
  follow: (source: StoreDataSource) => unknown,
  freshness: ApplicationFreshness,
  token: () => string | null,
  execution: AgentExecution,
) {
  return (active: boolean) => {
    pause("connection", !active);
    if (!active) {
      feed().abort();
      stopProjectEvents();
      freshness.stop();
      execution.pauseActivity();
    } else {
      void freshness.configure(freshness.supported, token());
      const current = source();
      if (current && browsing()) follow(current);
      execution.resumeActivity();
    }
  };
}
export function connectionEvents(
  freshness: ApplicationFreshness,
  active: () => boolean = () => true,
) {
  return (token: string) =>
    active()
      ? freshness.configure(freshness.supported, token)
      : Promise.resolve();
}

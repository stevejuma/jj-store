import { expect, test, vi } from "vitest";
import type { Event } from "../../src/lib/api/client.js";
import type { StoreDataSource } from "../../src/lib/controller/native-data-source.js";
import { followProductionStoreEvents } from "../../src/lib/controller/production-store-events.js";
import { createProductionStoreState } from "../../src/lib/controller/production-store-state.svelte.js";

test("collaboration publication events announce freshness without mixing snapshots", async () => {
  vi.useFakeTimers();
  const data = createProductionStoreState();
  data.agentSessions = [{ session_id: "12".repeat(16) }] as never;
  data.reviewThreads = [{ thread_id: "34".repeat(16) }] as never;
  const beforeAgents = data.agentSessions;
  const beforeReviews = data.reviewThreads;
  const abort = new AbortController();
  const changed: Event = {
    version: 1,
    cursor: {
      repository: "56".repeat(16),
      epoch: "78".repeat(16),
      position: 2,
    },
    type: "changed",
  };
  const source = {
    async events(_after, _signal, deliver) {
      deliver(changed, "opaque-event-cursor");
      abort.abort();
    },
  } as StoreDataSource;

  const following = followProductionStoreEvents(
    source,
    () => true,
    data,
    abort.signal,
  );
  await vi.runAllTimersAsync();
  await following;
  expect(data.status.refreshAvailable).toBe(true);
  expect(data.agentSessions).toBe(beforeAgents);
  expect(data.reviewThreads).toBe(beforeReviews);
  vi.useRealTimers();
});

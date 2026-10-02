import { expect, test, vi } from "vitest";
import type { Event } from "../../src/lib/api/client.js";
import type { StoreDataSource } from "../../src/lib/controller/native-data-source.js";
import { followProductionStoreEvents } from "../../src/lib/controller/production-store-events.js";
import { createProductionStoreState } from "../../src/lib/controller/production-store-state.svelte.js";

test("collaboration publication events queue automatic reads without mixing snapshots", async () => {
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
    snapshot: async () => ({ heads: [] }),
    async events(_after, _signal, deliver) {
      deliver(changed, "opaque-event-cursor");
      abort.abort();
    },
  } as StoreDataSource;

  const reload = vi.fn();
  const following = followProductionStoreEvents(
    source,
    () => true,
    data,
    abort.signal,
    reload,
  );
  await vi.runAllTimersAsync();
  await following;
  expect(data.status.refreshAvailable).toBe(true);
  expect(reload).toHaveBeenCalledTimes(1);
  expect(data.agentSessions).toBe(beforeAgents);
  expect(data.reviewThreads).toBe(beforeReviews);
  vi.useRealTimers();
});

test("reconnect ready snapshots trigger a reload when publication was missed", async () => {
  const abort = new AbortController(),
    data = createProductionStoreState(),
    reload = vi.fn();
  const head = {
    version: 1,
    repo_id: "56".repeat(16),
    epoch: "78".repeat(16),
    published_seq: 1,
    manifest_revision: 1,
    code_state_version: 1,
    heads: [],
  };
  data.head = head;
  const source = {
    repository: head.repo_id,
    snapshot: async () => head,
    async events(_after, _signal, deliver) {
      deliver(
        {
          version: 1,
          type: "ready",
          snapshot: { ...head, published_seq: 2 },
          cursor: { repository: head.repo_id, epoch: head.epoch, position: 2 },
        },
        "cursor",
      );
      abort.abort();
    },
  } as StoreDataSource;
  await followProductionStoreEvents(
    source,
    () => true,
    data,
    abort.signal,
    reload,
  );
  expect(reload).toHaveBeenCalledTimes(1);
  expect(data.head.published_seq).toBe(1);
});

test("local companion changes reload collaboration even when code heads are unchanged", async () => {
  const abort = new AbortController(),
    data = createProductionStoreState(),
    reload = vi.fn();
  const head = {
    version: 1,
    profile: "git_sha1_jj045" as const,
    repository: "56".repeat(16),
    generation: 1,
    heads: ["78".repeat(64)],
    metadata_commit: "90".repeat(20),
    actor: null,
  };
  data.head = head;
  const source = {
    repository: head.repository,
    profile: head.profile,
    snapshot: async () => head,
    async events(_after, _signal, deliver) {
      deliver(
        {
          version: 1,
          type: "ready",
          snapshot: { ...head, metadata_commit: "ab".repeat(20) },
        } as never,
        "local-cursor",
      );
      abort.abort();
    },
  } as StoreDataSource;
  await followProductionStoreEvents(
    source,
    () => true,
    data,
    abort.signal,
    reload,
  );
  expect(reload).toHaveBeenCalledTimes(1);
  expect(data.head.metadata_commit).toBe(head.metadata_commit);
});

test("an SSE reset recaptures state even when its head matches the displayed head", async () => {
  const abort = new AbortController(),
    data = createProductionStoreState(),
    reload = vi.fn();
  const head = {
    version: 1,
    repo_id: "56".repeat(16),
    epoch: "78".repeat(16),
    published_seq: 1,
    manifest_revision: 1,
    code_state_version: 1,
    heads: [],
  };
  data.head = head;
  const source = {
    repository: head.repo_id,
    snapshot: async () => head,
    async events(_after, _signal, deliver) {
      deliver(
        {
          version: 1,
          type: "reset",
          reason: "future_position",
          snapshot: head,
          cursor: { repository: head.repo_id, epoch: head.epoch, position: 1 },
        },
        "cursor",
      );
      abort.abort();
    },
  } as StoreDataSource;
  await followProductionStoreEvents(
    source,
    () => true,
    data,
    abort.signal,
    reload,
  );
  expect(reload).toHaveBeenCalledTimes(1);
});

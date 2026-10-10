import { expect, test, vi } from "vitest";
import {
  createThreadTransport,
  THREAD_IDS as ids,
} from "../../../catalog/fixtures/thread-transport.js";
import { createThreadsIntegration } from "../../src/lib/controller/production-threads-integration.js";
import { createProfileRouteRestorer } from "../../src/lib/controller/production-profile-route.js";
import { initialStoreRoute } from "../../src/lib/controller/store-route-history.svelte.js";

const local = "ab".repeat(16);
function fixture(unknownFirst = false) {
  const native = createThreadTransport({ unknownFirst });
  let source: any = null;
  let route = { ...initialStoreRoute(), page: "projects" as const };
  const connected: string[] = [];
  const integration = createThreadsIntegration({
    source: () => source,
    data: {
      repositories: [
        { id: local, profile: "git_sha1_jj045" },
        {
          id: ids.repo,
          profile: "native",
          principal: ids.actor,
          permission: "write",
        },
      ],
    } as any,
    history: {
      get route() {
        return route;
      },
      push(next: any) {
        route = next;
      },
    } as any,
    connectRepository: async (repository) => {
      connected.push(repository);
      source = {
        repository,
        ...(repository === ids.repo ? { threads: native.transport } : {}),
      };
      await integration.connected();
    },
  });
  return {
    integration,
    connected,
    setSource: (value: any) => {
      source = value;
    },
    setRoute: (value: any) => {
      route = value;
    },
    native,
    route: () => route,
  };
}

test("thread_hub_navigation_uses_the_shared_directory_without_connecting", async () => {
  const f = fixture();
  const directory = vi.spyOn(f.native.transport, "list");
  await f.integration.openThreads();
  expect(f.connected).toEqual([]);
  expect(directory).not.toHaveBeenCalled();
  expect(f.route()).toMatchObject({
    page: "threads",
    threadsHub: true,
    threadId: null,
  });
});

test("explicit_thread_navigation_connects_its_native_home", async () => {
  const f = fixture();
  await f.integration.openThreads(ids.thread, ids.repo);
  expect(f.connected).toEqual([ids.repo]);
  expect(f.integration.threads.detail?.thread.value.thread_id).toBe(ids.thread);
});

test("local_profile_switch_preserves_unknown_native_thread_intent", async () => {
  const f = fixture(true);
  f.setSource({ repository: ids.repo, threads: f.native.transport });
  await f.integration.threads.open(ids.thread);
  await expect(
    f.integration.threads.send("Recover the native outcome"),
  ).rejects.toThrow("unknown");
  const pending = f.integration.threads.pending;
  expect(pending).not.toBeNull();
  f.setRoute({ ...initialStoreRoute(), repository: local });
  f.setSource({ repository: local });
  await f.integration.connected();
  expect(f.integration.threads.pending).toBeNull();
  f.setSource({ repository: ids.repo, threads: f.native.transport });
  f.setRoute({
    ...initialStoreRoute(),
    repository: ids.repo,
    page: "threads",
    threadId: ids.thread,
  });
  await f.integration.connected();
  expect(f.integration.threads.pending).toEqual(pending);
  await f.integration.threads.retryPending();
  expect(f.native.requests[1]).toEqual(f.native.requests[0]);
  expect(f.integration.threads.pending).toBeNull();
});

test("unsupported_transport_discards_previous_native_thread_detail", async () => {
  const f = fixture();
  f.setSource({ repository: ids.repo, threads: f.native.transport });
  f.setRoute({
    ...initialStoreRoute(),
    repository: ids.repo,
    page: "threads",
    threadId: ids.thread,
  });
  await f.integration.connected();
  expect(f.integration.threads.detail?.thread.value.thread_id).toBe(ids.thread);
  expect(f.integration.available()).toBe(true);
  f.setRoute({ ...initialStoreRoute(), repository: local });
  f.setSource({ repository: local });
  expect(f.integration.available()).toBe(false);
  await f.integration.connected();
  expect(f.integration.threads.threads).toEqual([]);
  expect(f.integration.threads.detail).toBeUndefined();
  expect(f.integration.threads.events).toEqual([]);
  expect(f.integration.threads.error).toBe("");
});

test("local_thread_capability_comes_from_the_transport", async () => {
  const f = fixture();
  f.setSource({ repository: local, threads: f.native.transport });
  const directory = vi.spyOn(f.native.transport, "list");
  await f.integration.connected();
  expect(f.integration.available()).toBe(true);
  expect(directory).not.toHaveBeenCalled();
});

test("profile_route_reconnects_before_restoring_threads_without_a_code_operation", async () => {
  let source: any = { repository: local };
  const calls: string[] = [];
  const restore = createProfileRouteRestorer({
    data: {} as any,
    route: () => ({
      ...initialStoreRoute(),
      repository: ids.repo,
      page: "threads",
    }),
    source: () => source,
    projects: {
      restore: async () => {
        calls.push("projects");
      },
    },
    connect: async (repository) => {
      source = { repository };
      calls.push("connect");
    },
    token: () => null,
    invalidate: () => {
      calls.push("invalidate");
    },
    restoreThreads: async () => {
      calls.push("threads");
      return true;
    },
    restore: async () => {
      calls.push("code");
    },
  });
  await restore();
  expect(calls).toEqual(["invalidate", "connect"]);
  await restore();
  expect(calls).toEqual(["invalidate", "connect", "invalidate", "threads"]);
});

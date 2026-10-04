import { expect, test } from "vitest";
import {
  createThreadTransport,
  THREAD_IDS as ids,
} from "../../../catalog/fixtures/thread-transport.js";
import { HttpStoreDataSource } from "../../src/lib/controller/http-store-data-source.js";
import { createStoreQueryClient } from "../../src/lib/controller/store-query-client.js";
import type { ReactionTarget } from "../../src/lib/controller/thread-types.js";
import { targetKey } from "../../src/lib/collaboration/thread-presentation.js";

test("Thread queries share session cache and target invalidation retains unrelated data", async () => {
  const f = createThreadTransport(),
    calls: string[] = [],
    queryClient = createStoreQueryClient();
  const target: ReactionTarget = { type: "thread", thread_id: ids.thread };
  const other: ReactionTarget = { type: "thread", thread_id: "77".repeat(16) };
  const fetcher: typeof fetch = async (input, init) => {
    const path = new URL(String(input), "http://local/").pathname;
    const body = init?.body
      ? (JSON.parse(String(init.body)) as { target: ReactionTarget })
      : undefined;
    calls.push(path + (body ? targetKey(body.target) : ""));
    const value = body
      ? await f.transport.reactions(body.target)
      : path.endsWith("reaction-emojis")
        ? await f.transport.emojis()
        : path.endsWith("timeline")
          ? await f.transport.events(ids.thread)
          : path.endsWith("/threads")
            ? await f.transport.list()
            : await f.transport.thread(ids.thread);
    return new Response(JSON.stringify(value), {
      headers: { "content-type": "application/json" },
    });
  };
  const source = new HttpStoreDataSource(
    ids.repo,
    null,
    queryClient,
    "first",
    () => {},
    fetcher,
  );
  const reads = () =>
    Promise.all([
      source.threads.list(),
      source.threads.thread(ids.thread),
      source.threads.events(ids.thread),
      source.threads.emojis(),
      source.threads.reactions(target),
      source.threads.reactions(other),
    ]);
  await Promise.all([reads(), reads()]);
  expect(calls).toHaveLength(6);
  await source.threads.invalidate("reactions", target);
  await reads();
  expect(calls).toHaveLength(7);
  expect(calls.at(-1)).toContain(targetKey(target));
  await source.threads.invalidate("threads");
  await reads();
  expect(calls).toHaveLength(10);
  const second = new HttpStoreDataSource(
    ids.repo,
    null,
    queryClient,
    "second",
    () => {},
    fetcher,
  );
  await second.threads.list();
  expect(calls).toHaveLength(11);
  second.close();
  source.close();
});

test("obsolete conversation reads abort and cannot populate the cache after navigation", async () => {
  const queryClient = createStoreQueryClient();
  let observed!: AbortSignal;
  const source = new HttpStoreDataSource(
    ids.repo,
    null,
    queryClient,
    "first",
    () => {},
    async (_input, init) => {
      observed = init!.signal!;
      return new Promise<Response>((_, reject) =>
        observed.addEventListener(
          "abort",
          () => reject(new DOMException("Aborted", "AbortError")),
          { once: true },
        ),
      );
    },
  );
  const request = source.threads.events(ids.thread).catch((error) => error);
  await source.threads.cancelReads();
  expect(observed.aborted).toBe(true);
  await request;
  expect(
    queryClient
      .getQueryCache()
      .getAll()
      .every((query) => query.state.data === undefined),
  ).toBe(true);
  source.close();
  await expect(source.threads.list()).rejects.toThrow("Closed");
});

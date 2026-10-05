import { expect, test, vi } from "vitest";
import {
  createThreadTransport,
  THREAD_IDS as ids,
} from "../../../catalog/fixtures/thread-transport.js";
import { createThreadsController } from "../../src/lib/controller/production-threads.svelte.js";
import { initialStoreRoute } from "../../src/lib/controller/store-route-history.svelte.js";
import type {
  ReactionTarget,
  ThreadTransport,
} from "../../src/lib/controller/thread-types.js";
function fixture() {
  const f = createThreadTransport();
  const controller = createThreadsController({
    source: () => f.transport,
    repository: () => ids.repo,
    principal: () => ids.actor,
    route: () => ({
      ...initialStoreRoute(),
      repository: ids.repo,
      page: "threads",
      threadId: ids.thread,
    }),
    select: () => {},
  });
  return { ...f, controller };
}

test("directory and selected conversation begin concurrently", async () => {
  const f = fixture();
  const list = f.transport.list;
  let release!: () => void;
  f.transport.list = () =>
    new Promise((resolve) => {
      release = () => void list().then(resolve);
    });
  const read = vi.spyOn(f.transport, "events");
  const restoring = f.controller.restore();
  expect(read).toHaveBeenCalledTimes(1);
  release();
  await restoring;
});

test("confirmed reaction refreshes exactly its target and retains conversation and emoji assets", async () => {
  const f = fixture();
  f.addMessage(ids.thread, "77".repeat(16), "Other message");
  await f.controller.restore();
  const target = f.controller.projection.targets[ids.message]!;
  const other = f.controller.projection.targets["77".repeat(16)]!;
  await Promise.all([
    f.controller.loadReactions(target),
    f.controller.loadReactions(other),
    f.controller.loadEmojis(),
  ]);
  const list = vi.spyOn(f.transport, "list"),
    detail = vi.spyOn(f.transport, "thread"),
    events = vi.spyOn(f.transport, "events"),
    emojis = vi.spyOn(f.transport, "emojis"),
    assets = vi.spyOn(f.transport, "asset"),
    reactions = vi.spyOn(f.transport, "reactions");
  const before = f.controller.events,
    retained = f.controller.getReaction(other),
    urls = f.controller.emojis;
  await f.controller.toggle(target, "👍");
  await Promise.all([
    f.controller.loadReactions(target),
    f.controller.loadReactions(other),
    f.controller.loadEmojis(),
  ]);
  expect(list).not.toHaveBeenCalled();
  expect(detail).not.toHaveBeenCalled();
  expect(events).not.toHaveBeenCalled();
  expect(emojis).not.toHaveBeenCalled();
  expect(assets).not.toHaveBeenCalled();
  expect(reactions).toHaveBeenCalledExactlyOnceWith(target, undefined);
  expect(f.controller.events).toBe(before);
  expect(f.controller.getReaction(other)).toBe(retained);
  expect(f.controller.emojis).toEqual(urls);
  f.controller.clear();
});

test("reaction reads deduplicate main/reply targets and bound concurrency to four", async () => {
  const f = fixture();
  await f.controller.restore();
  const read = f.transport.reactions;
  let active = 0,
    peak = 0;
  const release: (() => void)[] = [];
  f.transport.reactions = (target, cursor) =>
    new Promise((resolve) => {
      active++;
      peak = Math.max(peak, active);
      release.push(() => {
        active--;
        void read(target, cursor).then(resolve);
      });
    });
  const base = f.controller.projection.targets[ids.message]!;
  const targets = Array.from(
    { length: 12 },
    (_, n) =>
      ({
        ...base,
        message_id: n.toString(16).padStart(32, "0"),
      }) as ReactionTarget,
  );
  const pending = targets.map((target) => f.controller.loadReactions(target));
  pending.push(f.controller.loadReactions(targets[0]!));
  await Promise.resolve();
  expect(peak).toBe(4);
  for (let n = 0; n < 12; n++) {
    while (!release.length)
      await new Promise((resolve) => setTimeout(resolve, 0));
    release.shift()!();
  }
  await Promise.all(pending);
  expect(peak).toBe(4);
  f.controller.clear();
});

test("queued reaction reads from an obsolete navigation are skipped", async () => {
  const f = fixture();
  await f.controller.restore();
  const read = f.transport.reactions;
  const releases: (() => void)[] = [];
  const calls: ReactionTarget[] = [];
  f.transport.reactions = (target, cursor) =>
    new Promise((resolve) => {
      calls.push(target);
      releases.push(() => void read(target, cursor).then(resolve));
    });
  const base = f.controller.projection.targets[ids.message]!;
  const pending = Array.from({ length: 12 }, (_, n) =>
    f.controller.loadReactions({
      ...base,
      message_id: n.toString(16).padStart(32, "0"),
    } as ReactionTarget),
  );
  expect(calls).toHaveLength(4);
  f.controller.clear();
  releases.forEach((release) => release());
  await Promise.all(pending);
  expect(calls).toHaveLength(4);
  expect(f.controller.getReaction(base)).toBeUndefined();
});

test("parallel directory failure remains visible after conversation succeeds", async () => {
  const f = fixture();
  f.transport.list = async () => {
    throw Error("directory unavailable");
  };
  await f.controller.restore();
  expect(f.controller.projection.messages).toHaveLength(1);
  expect(f.controller.status).toBe("failed");
  expect(f.controller.error).toBe("directory unavailable");
  f.controller.clear();
});

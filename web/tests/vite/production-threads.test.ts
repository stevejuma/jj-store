import { expect, test, vi } from "vitest";
import {
  createThreadTransport,
  THREAD_IDS as ids,
} from "../../../catalog/fixtures/thread-transport.js";
import { createThreadsController } from "../../src/lib/controller/production-threads.svelte.js";
import {
  initialStoreRoute,
  createStoreRouteHistory,
} from "../../src/lib/controller/store-route-history.svelte.js";
import {
  channelMessages,
  reactionModels,
  targetKey,
} from "../../src/lib/collaboration/thread-presentation.js";
import {
  storeRouteFromHash,
  storeRouteToHash,
} from "../../src/lib/controller/store-route-codec.js";
import type {
  ThreadTransport,
  ReactionTarget,
  ReactionItems,
} from "../../src/lib/controller/thread-types.js";
function fixture(unknownFirst = false) {
  const f = createThreadTransport({ unknownFirst });
  let route = {
    ...initialStoreRoute(),
    repository: ids.repo,
    page: "threads" as const,
    threadId: ids.thread,
  };
  let source: ThreadTransport = f.transport;
  const controller = createThreadsController({
    source: () => source,
    repository: () => route.repository,
    principal: () => ids.actor,
    route: () => route,
    select: (id) => {
      route = { ...route, threadId: id! };
    },
    selectReply: (id) => {
      route = { ...route, replyId: id } as typeof route;
    },
  });
  return {
    ...f,
    controller,
    route: () => route,
    switchSource: (next: ThreadTransport) => {
      source = next;
    },
  };
}
test("human replies retain exact parent revision and edit identity", async () => {
  const f = fixture();
  await f.controller.restore();
  const parent = f.controller.projection.targets[ids.message];
  await f.controller.send("A persisted reply", ids.message);
  const request = f.requests[0];
  expect(request.command).toMatchObject({
    type: "message",
    reply_to: parent.type === "human_message" ? parent.revision : undefined,
  });
  const reply = f.controller.projection.messages.find(
    (m) => m.content === "A persisted reply",
  )!;
  expect(reply.parentMessageId).toBe(ids.message);
  await f.controller.send("Edited reply", undefined, reply.id);
  expect(f.requests[1].command).toMatchObject({
    reply_to: (request.command as { reply_to: unknown }).reply_to,
  });
});
test("unknown outcomes retain and retry the original request, then refresh", async () => {
  const f = fixture(true);
  await f.controller.restore();
  await expect(f.controller.send("Once only")).rejects.toThrow("unknown");
  const pending = f.controller.pending;
  await f.controller.refresh();
  expect(f.controller.pending).toEqual(pending);
  await f.controller.retryPending();
  expect(f.requests[1]).toEqual(f.requests[0]);
  expect(f.controller.pending).toBeNull();
  expect(
    f.controller.projection.messages.filter((m) => m.content === "Once only"),
  ).toHaveLength(1);
});
test("late selections and disconnections discard replies and failures", async () => {
  const f = fixture();
  let resolve!: () => void;
  f.transport.thread = () =>
    new Promise((_, reject) => {
      resolve = () => reject(new Error("old selection"));
    });
  const opening = f.controller.open(ids.thread);
  f.controller.clear();
  resolve();
  await opening;
  expect(f.controller.detail).toBeUndefined();
  expect(f.controller.error).toBe("");
});
test("partial history preserves unloaded parents and edits select newest body", () => {
  const f = createThreadTransport();
  const parent = f.history.get(ids.thread)![0];
  f.addMessage(
    ids.thread,
    "77".repeat(16),
    "reply",
    parent.value.event.type === "message" ? parent.value.event.revision : null,
  );
  const events = f.history.get(ids.thread)!;
  expect(
    channelMessages(events.slice(0, 1)).messages[0].parentMessageId,
  ).toMatch(/^unloaded:/);
  expect(channelMessages(events).messages[1].parentMessageId).toBe(ids.message);
});
test("semantic glyph collisions aggregate while clearing the exact viewer value", async () => {
  const f = fixture();
  await f.controller.restore();
  const target = f.controller.projection.targets[ids.message];
  const items: ReactionItems = {
    type: "reactions",
    target,
    revision: "current",
    conflict_count: 0,
    reactions: [
      { value: { type: "like" }, count: 1, viewer_reacted: false },
      {
        value: { type: "emoji", sequence: "👍" },
        count: 1,
        viewer_reacted: true,
      },
    ],
    actors: [
      {
        actor: ids.actor,
        active: [{ kind: "reaction", id: "99".repeat(32) }],
        value: { type: "emoji", sequence: "👍" },
        conflict: false,
      },
    ],
  };
  f.rows.set(targetKey(target), items);
  expect(
    reactionModels(items, () => ({ emoji: ":custom:", label: "custom" })),
  ).toEqual([{ emoji: "👍", label: "👍", count: 2, selected: true }]);
  await f.controller.toggle(target, "👍", undefined, true);
  expect(f.requests[0].command).toMatchObject({
    type: "clear",
    observed: items.actors[0].active,
  });
});
test("custom emoji loading is shared and bar toggles retain exact custom values", async () => {
  const f = fixture();
  await f.controller.restore();
  const emojis = vi.spyOn(f.transport, "emojis");
  await Promise.all([f.controller.loadEmojis(), f.controller.loadEmojis()]);
  expect(emojis).toHaveBeenCalledTimes(1);
  const target = f.controller.projection.targets[ids.message];
  const custom = f.controller.emojis[0];
  await f.controller.toggle(target, custom.shortcode, custom);
  expect(f.requests[0].command).toMatchObject({
    type: "set",
    value: { type: "custom", emoji_id: ids.emoji, version: f.emojiVersion },
  });
  await f.controller.toggle(target, custom.shortcode, undefined, true);
  expect(f.requests[1].command.type).toBe("clear");
  f.controller.clear();
});
test("Thread and reply routes round trip, and leaving Threads clears selections", () => {
  const route = {
    ...initialStoreRoute(),
    repository: ids.repo,
    page: "threads" as const,
    threadId: ids.thread,
    replyId: ids.message,
  };
  const hash = storeRouteToHash(route)!;
  expect(storeRouteFromHash(hash)).toMatchObject({
    kind: "route",
    route: { threadId: ids.thread, replyId: ids.message },
  });
  const history = createStoreRouteHistory();
  history.push(route);
  history.page("files");
  expect(history.route.threadId).toBeNull();
  expect(history.route.replyId).toBeNull();
  expect(
    storeRouteFromHash(`#/repos/${ids.repo}/threads?reply=${ids.message}`).kind,
  ).toBe("invalid");
});
test("unknown service failures retain intent while explicit rejection releases it", async () => {
  const { ApiError } = await import("../../src/lib/api/client.js");
  const f = fixture();
  await f.controller.restore();
  f.transport.command = async () => {
    throw new ApiError("authority_unavailable");
  };
  await expect(f.controller.send("Uncertain")).rejects.toThrow();
  expect(f.controller.pending).not.toBeNull();
  f.transport.status = async (id) => ({ status: "not_known", request_id: id });
  await f.controller.checkPending();
  expect(f.controller.pending).not.toBeNull();
  f.transport.command = async () => {
    throw new ApiError("collaboration_replication_unsupported");
  };
  await expect(f.controller.retryPending()).rejects.toThrow("WorkStore home");
  expect(f.controller.pending).toBeNull();
});
test("partial emoji loads are transactional and failed previews can be retried", async () => {
  const f = fixture();
  const original = f.transport.asset;
  f.transport.asset = async () => {
    throw new Error("preview unavailable");
  };
  await expect(f.controller.loadEmojis()).rejects.toThrow(
    "preview unavailable",
  );
  expect(f.controller.emojis).toEqual([]);
  f.transport.asset = original;
  await f.controller.loadEmojis();
  expect(f.controller.emojis).toHaveLength(1);
  f.controller.clear();
});
test("in-flight preview responses are discarded after disconnect", async () => {
  const f = fixture();
  let resolve!: (value: Awaited<ReturnType<ThreadTransport["asset"]>>) => void;
  f.transport.asset = () =>
    new Promise((done) => {
      resolve = done;
    });
  const loading = f.controller.loadEmojis();
  await Promise.resolve();
  f.controller.clear();
  resolve({ bytes: new Uint8Array([0]), type: "image/png" });
  await loading;
  expect(f.controller.emojis).toEqual([]);
});
test("status checks address the original domain and request before admitting a receipt", async () => {
  const f = fixture(true);
  await f.controller.restore();
  await expect(f.controller.send("Recovered by status")).rejects.toThrow(
    "unknown",
  );
  const pending = f.controller.pending!,
    status = vi.spyOn(f.transport, "status");
  await f.controller.checkPending();
  expect(status).toHaveBeenCalledWith("threads", pending.request.request_id);
  expect(f.controller.pending).toBeNull();
  expect(
    f.controller.projection.messages.some(
      (m) => m.content === "Recovered by status",
    ),
  ).toBe(true);
});
test("disconnect clears protected drafts and ignores a late unknown outcome", async () => {
  const f = fixture();
  await f.controller.restore();
  let resolve!: (
    value: Awaited<ReturnType<ThreadTransport["command"]>>,
  ) => void;
  f.transport.command = () =>
    new Promise((done) => {
      resolve = done;
    });
  const sending = f.controller.send("Protected draft"),
    id = f.controller.pending!.request.request_id;
  f.controller.clear();
  resolve({ status: "outcome_unknown", request_id: id });
  await sending;
  expect(f.controller.pending).toBeNull();
  expect(f.controller.error).toBe("");
});
test("repository switches may retain a pending intent within the same credential session", async () => {
  const f = fixture(true);
  await f.controller.restore();
  await expect(f.controller.send("Retained")).rejects.toThrow("unknown");
  const intent = f.controller.pending;
  f.controller.clear(true);
  expect(f.controller.pending).toEqual(intent);
  await f.controller.checkPending();
  expect(f.controller.pending).toBeNull();
});
test("opening Threads from Projects binds an authorized repository and clears unrelated selections", async () => {
  const { createThreadsIntegration } =
    await import("../../src/lib/controller/production-threads-integration.js");
  const { createProductionStoreState } =
    await import("../../src/lib/controller/production-store-state.svelte.js");
  const f = createThreadTransport(),
    data = createProductionStoreState(),
    history = createStoreRouteHistory();
  data.repositories = [
    { id: ids.repo, name: "Home", permission: "admin", principal: ids.actor },
  ];
  let source: { repository: string; threads: ThreadTransport } | null = null;
  const connected: string[] = [];
  const integration = createThreadsIntegration({
    data,
    history,
    source: () => source,
    connectRepository: async (id) => {
      connected.push(id);
      source = { repository: id, threads: f.transport };
      history.connect(id, null);
    },
  });
  history.push({ ...initialStoreRoute(), page: "projects", projectsHub: true });
  await integration.openThreads(ids.thread);
  expect(connected).toEqual([ids.repo]);
  expect(history.route).toMatchObject({
    repository: ids.repo,
    page: "threads",
    threadId: ids.thread,
    reviewThreadId: null,
    agentSessionId: null,
    path: null,
  });
  history.push({ ...initialStoreRoute(), page: "projects", projectsHub: true });
  await integration.openThreads(ids.thread);
  expect(history.route.repository).toBe(ids.repo);
  expect(connected).toHaveLength(1);
});

test("search selection loads bounded older pages to locate the exact message", async () => {
  const f = fixture();
  const original = f.transport.events;
  const full = await original(ids.thread);
  const read = vi.fn(async (_id: string, cursor?: string) =>
    cursor
      ? full
      : {
          ...full,
          items: { type: "events" as const, thread_id: ids.thread, events: [] },
          next_cursor: "older",
        },
  );
  f.transport.events = read;
  f.controller.selectReply(ids.message);
  await f.controller.restore();
  expect(read).toHaveBeenCalledWith(ids.thread, "older");
  expect(
    f.controller.projection.messages.find((m) => m.id === ids.message)?.content,
  ).toBe("Welcome to the channel");
});
test("settings commands use exact selected thread revision", async () => {
  const f = fixture();
  await f.controller.restore();
  const before = f.controller.detail!.thread.reference;
  await f.controller.rename("Planning");
  expect(f.requests[0].command).toEqual({
    type: "rename",
    thread_id: ids.thread,
    expected: before,
    title: "Planning",
  });
  await f.controller.lifecycle("archive");
  expect(f.requests[1].command).toMatchObject({
    type: "lifecycle",
    thread_id: ids.thread,
    action: "archive",
  });
});

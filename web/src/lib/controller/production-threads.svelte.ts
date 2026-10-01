import {
  collabRequestId,
  createCollabActions,
} from "./collab-actions.svelte.js";
import { createMessageReactions } from "./message-reactions.svelte.js";
import { ApiError, type Model } from "../api/client.js";
import { channelMessages } from "../collaboration/thread-presentation.js";
import type { ThreadTransport } from "./thread-types.js";
import type { StoreRoute } from "./contracts.js";

export function createThreadsController(options: {
  source: () => ThreadTransport;
  repository: () => string;
  principal: () => string;
  route: () => StoreRoute;
  select: (id: string | null) => void;
  selectReply?: (id: string | null) => void;
}) {
  let generation = 0;
  let threads = $state<Model<"ThreadSummaryV1">[]>([]);
  let detail = $state<Model<"ThreadDetailV1">>();
  let events = $state<Model<"ThreadEventItemV1">[]>([]);
  let listCursor = $state<string | null>(null),
    eventCursor = $state<string | null>(null);
  let status = $state("idle"),
    error = $state("");
  const actions = createCollabActions({
    ...options,
    generation: () => generation,
    valid,
    refresh,
  });
  const richReactions = createMessageReactions({
    source: options.source,
    principal: options.principal,
    generation: () => generation,
    valid,
    execute: actions.execute,
  });
  const projection = $derived(channelMessages(events));
  function valid(ticket: number, transport: ThreadTransport) {
    try {
      return ticket === generation && options.source() === transport;
    } catch {
      return false;
    }
  }
  function clear(preservePending = false) {
    generation++;
    actions.reset(preservePending);
    threads = [];
    detail = undefined;
    events = [];
    listCursor = null;
    eventCursor = null;
    status = "idle";
    error = "";
    richReactions.clear();
  }
  async function list(more = false) {
    const transport = options.source(),
      ticket = generation;
    status = threads.length ? "refreshing" : "loading";
    try {
      const p = await transport.list(
        more ? (listCursor ?? undefined) : undefined,
      );
      if (!valid(ticket, transport)) return;
      if (p.items.type !== "threads") throw new ApiError("invalid_thread_page");
      threads = more ? [...threads, ...p.items.threads] : p.items.threads;
      listCursor = p.next_cursor;
      status = "ready";
      error = "";
    } catch (e) {
      if (valid(ticket, transport)) {
        status = "failed";
        error = String(e instanceof Error ? e.message : e);
      }
    }
  }
  async function read(id: string, more = false) {
    const transport = options.source(),
      ticket = generation;
    status = detail ? "refreshing" : "loading";
    try {
      const [d, p] = await Promise.all([
        more ? Promise.resolve(detail!) : transport.thread(id),
        transport.events(id, more ? (eventCursor ?? undefined) : undefined),
      ]);
      if (!valid(ticket, transport) || options.route().threadId !== id) return;
      if (
        p.items.type !== "events" ||
        p.items.thread_id !== id ||
        p.snapshot.thread_root !== d.snapshot.thread_root
      )
        throw new ApiError("thread_changed_refresh_required");
      detail = d;
      events = more ? [...events, ...p.items.events] : p.items.events;
      eventCursor = p.next_cursor;
      status = "ready";
      error = "";
    } catch (e) {
      if (valid(ticket, transport) && options.route().threadId === id) {
        status = "failed";
        error = String(e instanceof Error ? e.message : e);
      }
    }
  }
  async function restore() {
    generation++;
    if (detail?.thread.value.thread_id !== options.route().threadId) {
      detail = undefined;
      events = [];
      eventCursor = null;
    }
    const ticket = generation;
    await list();
    if (ticket !== generation) return;
    const id = options.route().threadId;
    if (id) await read(id);
  }
  async function open(id: string | null) {
    generation++;
    detail = undefined;
    events = [];
    eventCursor = null;
    error = "";
    options.select(id);
    if (id) await read(id);
    else await list();
  }
  async function refresh() {
    richReactions.invalidate();
    await restore();
  }
  async function create(title: string) {
    const originalRoute = options.route(),
      transport = options.source();
    const thread_id = collabRequestId();
    await actions.execute("threads", {
      version: 1,
      request_id: collabRequestId(),
      command: {
        type: "create",
        thread_id,
        title: title.trim(),
        baseline: null,
      },
    });
    if (options.route() === originalRoute && valid(generation, transport))
      await open(thread_id);
  }
  async function send(body: string, parentId?: string, editId?: string) {
    if (!detail) throw new ApiError("select_thread_first");
    const existing = editId ? projection.targets[editId] : undefined;
    const parent = parentId ? projection.targets[parentId] : undefined;
    const old = editId
      ? events.find((e) => e.message?.message_id === editId)?.message
      : undefined;
    if (parentId && parent?.type !== "human_message")
      throw new ApiError("reply_parent_unavailable");
    await actions.execute("threads", {
      version: 1,
      request_id: collabRequestId(),
      command: {
        type: "message",
        thread_id: detail.thread.value.thread_id,
        expected_thread: detail.thread.reference,
        message_id: editId ?? collabRequestId(),
        expected_revision:
          existing?.type === "human_message" ? existing.revision : null,
        reply_to:
          old?.reply_to ??
          (parent?.type === "human_message" ? parent.revision : null),
        body,
      },
    });
  }
  return {
    get threads() {
      return threads;
    },
    get detail() {
      return detail;
    },
    get projection() {
      return projection;
    },
    get status() {
      return status;
    },
    get error() {
      return actions.error || error;
    },
    get busy() {
      return actions.busy;
    },
    get pending() {
      return actions.pending;
    },
    get hasMore() {
      return !!listCursor;
    },
    get hasOlder() {
      return !!eventCursor;
    },
    get emojis() {
      return richReactions.emojis;
    },
    getReaction: richReactions.getReaction,
    reactionModels: richReactions.reactionModels,
    loadEmojis: richReactions.loadEmojis,
    loadReactions: richReactions.loadReactions,
    toggle: richReactions.toggle,
    clear,
    list,
    read,
    restore,
    open,
    refresh,
    create,
    send,
    checkPending: actions.checkPending,
    retryPending: actions.retryPending,
    selectReply: (id: string | null) => options.selectReply?.(id),
    get reactionRevision() {
      return richReactions.revision;
    },
    more: () => list(true),
    older: () =>
      detail ? read(detail.thread.value.thread_id, true) : Promise.resolve(),
  };
}

import { refreshCommittedThreadAction } from "./thread-committed-refresh.js";
import { createThreadCommands } from "./thread-commands.js";
import { createCollabActions } from "./collab-actions.svelte.js";
import { retainThreadHistory, replyContextLoaded } from "./thread-history.js";
import { createThreadPost } from "./thread-post.svelte.js";
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
  let directoryLoading = $state(false),
    conversationLoading = $state(false);
  let directoryError = $state("");
  let status = $state("idle"),
    error = $state("");
  const actions = createCollabActions({
    ...options,
    generation: () => generation,
    valid,
    refresh: (intent) =>
      refreshCommittedThreadAction(
        intent,
        options.source(),
        richReactions,
        restore,
      ),
    onCommitted: (id) => post.committed(id),
  });
  const richReactions = createMessageReactions({
    source: options.source,
    principal: options.principal,
    generation: () => generation,
    valid,
    execute: actions.execute,
  });
  const post = createThreadPost({
    ...options,
    execute: (request) => actions.execute("threads", request),
    pending: () => !!actions.pending,
    open,
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
    directoryLoading = conversationLoading = false;
    actions.reset(preservePending);
    threads = [];
    detail = undefined;
    events = [];
    listCursor = null;
    eventCursor = null;
    status = "idle";
    directoryError = error = "";
    richReactions.clear();
    if (!preservePending) post.clear();
  }
  async function list(more = false) {
    const transport = options.source(),
      ticket = generation;
    directoryLoading = true;
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
      directoryError = "";
    } catch (e) {
      if (valid(ticket, transport)) {
        status = "failed";
        directoryError = String(e instanceof Error ? e.message : e);
      }
    } finally {
      if (valid(ticket, transport)) directoryLoading = false;
    }
  }
  async function read(id: string, more = false) {
    const transport = options.source(),
      ticket = generation;
    conversationLoading = true;
    status = detail ? "refreshing" : "loading";
    try {
      const retained = more ? [] : projection.messages.map((m) => m.id);
      const [d, first] = await Promise.all([
        more ? Promise.resolve(detail!) : transport.thread(id),
        transport.events(id, more ? (eventCursor ?? undefined) : undefined),
      ]);
      const loaded = await retainThreadHistory({
        transport,
        id,
        root: d.snapshot.thread_root,
        first,
        retained,
        more,
        valid: () =>
          valid(ticket, transport) && options.route().threadId === id,
      });
      if (!loaded) return;
      detail = d;
      events = more ? [...events, ...loaded.events] : loaded.events;
      eventCursor = loaded.cursor;
      status = "ready";
      error = "";
    } catch (e) {
      if (valid(ticket, transport) && options.route().threadId === id) {
        status = "failed";
        error = String(e instanceof Error ? e.message : e);
      }
    } finally {
      if (valid(ticket, transport)) conversationLoading = false;
    }
  }
  async function focusReply(id: string) {
    const target = options.route().replyId,
      ticket = generation;
    for (
      let page = 0;
      target &&
      !replyContextLoaded(projection.messages, target) &&
      eventCursor &&
      page < 20;
      page++
    ) {
      const previous = eventCursor;
      await read(id, true);
      if (
        ticket !== generation ||
        status === "failed" ||
        previous === eventCursor
      )
        return;
    }
    if (
      target &&
      !replyContextLoaded(projection.messages, target) &&
      ticket === generation
    )
      error = eventCursor
        ? "Message is outside loaded history. Load older messages to continue."
        : "This message is unavailable in the current thread.";
  }
  async function restore() {
    generation++;
    directoryLoading = conversationLoading = false;
    if (detail?.thread.value.thread_id !== options.route().threadId) {
      detail = undefined;
      events = [];
      eventCursor = null;
    }
    const ticket = generation;
    const id = options.route().threadId;
    if (options.source().cancelReads) await options.source().cancelReads!();
    if (ticket !== generation) return;
    await Promise.all([list(), id ? read(id) : Promise.resolve()]);
    if (ticket === generation && id && !error) await focusReply(id);
  }
  async function open(id: string | null) {
    generation++;
    directoryLoading = conversationLoading = false;
    detail = undefined;
    events = [];
    eventCursor = null;
    error = "";
    options.select(id);
    const ticket = generation;
    if (options.source().cancelReads) await options.source().cancelReads!();
    if (ticket !== generation) return;
    if (id) {
      await read(id);
      await focusReply(id);
    } else await list();
  }
  async function refresh() {
    await options.source().invalidate?.("all");
    richReactions.invalidate();
    await restore();
  }
  const commands = createThreadCommands({
    detail: () => detail,
    projection: () => projection,
    events: () => events,
    execute: actions.execute,
  });
  return {
    ...commands,
    get threads() {
      return threads;
    },
    get detail() {
      return detail;
    },
    get events() {
      return events;
    },
    get projection() {
      return projection;
    },
    get status() {
      return directoryError || error || status === "failed"
        ? "failed"
        : directoryLoading || conversationLoading
          ? detail || threads.length
            ? "refreshing"
            : "loading"
          : status;
    },
    get error() {
      return (
        (actions.errorKind === "reactions" ? "" : actions.error) ||
        error ||
        directoryError
      );
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
    reactionError: richReactions.getError,
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
    create: post.create,
    checkPending: async () => {
      await actions.checkPending();
      await post.resume();
    },
    retryPending: async () => {
      await actions.retryPending();
      await post.resume();
    },
    selectReply: (id: string | null) => options.selectReply?.(id),
    get reactionRevision() {
      return richReactions.revision;
    },
    more: () => list(true),
    older: () =>
      detail ? read(detail.thread.value.thread_id, true) : Promise.resolve(),
  };
}

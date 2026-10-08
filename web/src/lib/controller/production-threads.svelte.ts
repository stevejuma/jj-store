import { createThreadDirectory } from "./thread-directory.svelte.js";
import { createConversationDiscussionReader } from "../conversations/discussion-reader.svelte.js";
import { createThreadConversations } from "./thread-conversations.svelte.js";

import { refreshCommittedThreadAction } from "./thread-committed-refresh.js";
import { createThreadCommands } from "./thread-commands.js";
import { createCollabActions } from "./collab-actions.svelte.js";
import {
  retainThreadHistory,
  replyContextLoaded,
  loadReplyHistory,
} from "./thread-history.js";
import { createThreadPost } from "./thread-post.svelte.js";
import { createMessageReactions } from "./message-reactions.svelte.js";
import { type Model } from "../api/client.js";
import { channelMessages } from "../collaboration/thread-presentation.js";
import {
  threadSnapshotKey,
  type ThreadDetail,
  type ThreadTransport,
} from "./thread-types.js";
import type { StoreRoute } from "./contracts.js";

export function createThreadsController(options: {
  source: () => ThreadTransport;
  repository: () => string;
  principal: () => string;
  route: () => StoreRoute;
  select: (id: string | null) => void;
  selectReply?: (id: string | null) => void;
}) {
  const conversationHost = createThreadConversations(options, (request) =>
    actions.execute("threads", request),
  );
  const discussionRead = createConversationDiscussionReader();
  let generation = 0;
  let committedRequestId = $state("");
  const directory = createThreadDirectory({
    source: options.source,
    generation: () => generation,
    valid,
    status: (value) => (status = value),
  });
  let detail = $state<ThreadDetail>();
  let events = $state<Model<"ThreadEventItemV1">[]>([]);
  let eventCursor = $state<string | null>(null);
  let conversationLoading = $state(false);
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
    onCommitted: (id) => {
      committedRequestId = id;
      post.committed(id);
    },
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
    directory.loading = conversationLoading = false;
    actions.reset(preservePending);
    directory.clear();
    detail = undefined;
    events = [];
    eventCursor = null;
    status = "idle";
    error = "";
    richReactions.clear();
    if (!preservePending) {
      post.clear();
    }
    discussionRead.clear();
  }
  const list = directory.list;
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
        root: threadSnapshotKey(d.snapshot),
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
      await discussionRead.load(
        transport,
        events,
        threadSnapshotKey(d.snapshot),
        () => valid(ticket, transport) && options.route().threadId === id,
      );
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
    if (!target) return;
    const message = await loadReplyHistory(
      () => ({
        loaded: replyContextLoaded(projection.messages, target),
        cursor: eventCursor,
        valid: ticket === generation && status !== "failed",
      }),
      () => read(id, true),
    );
    if (message) error = message;
  }
  async function restore() {
    generation++;
    directory.loading = conversationLoading = false;
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
    directory.loading = conversationLoading = false;
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
    conversations: conversationHost.conversations,
    get conversationReads() {
      return conversationHost.conversationReads;
    },
    get conversationScope() {
      return conversationHost.conversationScope;
    },
    loadConversation: conversationHost.loadConversation,
    submitConversation: conversationHost.submitConversation,
    readConversation: conversationHost.readConversation,
    ensureConversation: conversationHost.ensureConversation,
    get discussions() {
      return discussionRead.discussions;
    },
    get discussionError() {
      return discussionRead.error;
    },
    get committedRequestId() {
      return committedRequestId;
    },
    get postedThreadId() {
      return post.postedThreadId;
    },
    get threads() {
      return directory.threads;
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
      return directory.error || error || status === "failed"
        ? "failed"
        : directory.loading || conversationLoading
          ? detail || directory.threads.length
            ? "refreshing"
            : "loading"
          : status;
    },
    get error() {
      return (
        (actions.errorKind === "reactions" ? "" : actions.error) ||
        error ||
        directory.error
      );
    },
    get busy() {
      return actions.busy;
    },
    get pending() {
      return actions.pending;
    },
    get hasMore() {
      return directory.hasMore;
    },
    get hasOlder() {
      return !!eventCursor;
    },
    get emojis() {
      return richReactions.emojis;
    },
    invalidateReactions: richReactions.invalidate,
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

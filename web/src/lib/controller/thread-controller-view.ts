import type { Model } from "../api/client.js";
import type { ThreadDetail } from "./thread-types.js";
type Return<F extends (...args: never[]) => unknown> = ReturnType<F>;
export function threadControllerView(context: {
  directory: Return<
    typeof import("./thread-directory.svelte.js").createThreadDirectory
  >;
  actions: Return<
    typeof import("./collab-actions.svelte.js").createCollabActions
  >;
  conversationHost: Return<
    typeof import("./thread-conversations.svelte.js").createThreadConversations
  >;
  discussionRead: Return<
    typeof import("../conversations/discussion-reader.svelte.js").createConversationDiscussionReader
  >;
  post: Return<typeof import("./thread-post.svelte.js").createThreadPost>;
  richReactions: Return<
    typeof import("./message-reactions.svelte.js").createMessageReactions
  >;
  read(): {
    committedRequestId: string;
    detail: ThreadDetail | undefined;
    events: Model<"ThreadEventItemV1">[];
    projection: Return<
      typeof import("../collaboration/thread-presentation.js").channelMessages
    >;
    eventCursor: string | null;
    error: string;
    status: string;
    conversationLoading: boolean;
  };
}) {
  const {
    directory,
    actions,
    conversationHost,
    discussionRead,
    post,
    richReactions,
  } = context;
  return {
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
      return context.read().committedRequestId;
    },
    get postedThreadId() {
      return post.postedThreadId;
    },
    get threads() {
      return directory.threads;
    },
    get detail() {
      return context.read().detail;
    },
    get events() {
      return context.read().events;
    },
    get projection() {
      return context.read().projection;
    },
    get status() {
      const { error, status, detail, conversationLoading } = context.read();
      return directory.error || error || status === "failed"
        ? "failed"
        : directory.loading || conversationLoading
          ? detail || directory.threads.length
            ? "refreshing"
            : "loading"
          : status;
    },
    get error() {
      const { error } = context.read();
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
      return !!context.read().eventCursor;
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
  };
}

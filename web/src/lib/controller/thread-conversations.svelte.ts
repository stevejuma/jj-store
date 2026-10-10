import {
  readConversation,
  type ConversationRead,
} from "../conversations/read-conversation.js";
import { ConversationSessions } from "../conversations/session.svelte.js";
import { ApiError } from "../api/client.js";
import type { ThreadTransport, ThreadRequest } from "./thread-types.js";
export function createThreadConversations(
  options: {
    source: () => ThreadTransport;
    repository: () => string;
    principal: () => string;
  },
  execute: (request: ThreadRequest) => Promise<unknown>,
) {
  const conversations = new ConversationSessions();
  let conversationReads = $state<Record<string, ConversationRead>>({});
  return {
    conversations,
    get conversationReads() {
      return conversationReads;
    },
    loadConversation: async (id: string) => {
      const transport = options.source(),
        scope = JSON.stringify([options.repository(), options.principal()]),
        key = `${scope}:${id}`;
      const validRead = () =>
        options.source() === transport &&
        scope === JSON.stringify([options.repository(), options.principal()]);
      try {
        const result = await readConversation(transport, id, validRead);
        if (validRead()) conversationReads[key] = result;
        return result;
      } catch (cause) {
        if (validRead())
          conversationReads[key] = {
            ...(conversationReads[key] ?? { events: [], discussions: [] }),
            error: cause instanceof Error ? cause.message : String(cause),
          };
        throw cause;
      }
    },
    get conversationScope() {
      return JSON.stringify([options.repository(), options.principal()]);
    },
    submitConversation: (request: import("./thread-types.js").ThreadRequest) =>
      execute(request),
    readConversation: (id: string) => options.source().thread(id),
    ensureConversation: async (id: string, requestId: string) => {
      try {
        return await options.source().thread(id);
      } catch (cause) {
        if (
          !(cause instanceof ApiError) ||
          ![
            "not_found",
            "collaboration_not_found",
            "local_agent_not_found",
          ].includes(cause.code)
        )
          throw cause;
        await execute({
          version: 1,
          request_id: requestId,
          command: {
            type: "create_annotation",
            thread_id: id,
            title: "Agent conversation",
          },
        });
        return options.source().thread(id);
      }
    },
  };
}

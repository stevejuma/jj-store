import type { Model } from "../api/client.js";
import type { ThreadTransport } from "../controller/thread-types.js";
import {
  readConversationDiscussions,
  type DiscussionRead,
} from "./thread-discussions.js";
export function createConversationDiscussionReader() {
  let discussions = $state<DiscussionRead[]>([]),
    error = $state("");
  return {
    get discussions() {
      return discussions;
    },
    get error() {
      return error;
    },
    clear() {
      discussions = [];
      error = "";
    },
    async load(
      transport: ThreadTransport,
      events: Model<"ThreadEventItemV1">[],
      root: string | null,
      valid: () => boolean,
    ) {
      try {
        const next = await readConversationDiscussions(
          transport,
          events,
          root,
          valid,
        );
        if (valid()) {
          discussions = next.discussions;
          error = next.errors.join("\n");
        }
      } catch (cause) {
        if (valid())
          error = cause instanceof Error ? cause.message : String(cause);
      }
    },
  };
}

import { ApiError } from "../api/client.js";
import type { Model } from "../api/client.js";
import {
  threadSnapshotKey,
  type ThreadTransport,
  type ThreadDetail,
} from "../controller/thread-types.js";
import {
  readConversationDiscussions,
  type DiscussionRead,
} from "./thread-discussions.js";
export type ConversationRead = {
  detail?: ThreadDetail;
  events: Model<"ThreadEventItemV1">[];
  discussions: DiscussionRead[];
  error: string;
};
/** Independent of the active thread route; reads never create metadata. */
export async function readConversation(
  transport: ThreadTransport,
  id: string,
  valid: () => boolean,
): Promise<ConversationRead> {
  let detail: ThreadDetail;
  try {
    detail = await transport.thread(id);
  } catch (cause) {
    if (
      cause instanceof ApiError &&
      ["not_found", "collaboration_not_found"].includes(cause.code)
    )
      return { events: [], discussions: [], error: "" };
    throw cause;
  }
  const root = threadSnapshotKey(detail.snapshot),
    events: Model<"ThreadEventItemV1">[] = [];
  let cursor: string | undefined;
  for (let page = 0; page < 20; page++) {
    if (!valid()) throw new Error("Conversation selection changed.");
    const read = await transport.events(id, cursor);
    if (
      read.items.type !== "events" ||
      threadSnapshotKey(read.snapshot) !== root
    )
      throw new Error(
        "Conversation changed during reading. Refresh to continue.",
      );
    events.push(...read.items.events);
    if (events.length > 4096)
      throw new Error("Conversation exceeds the bounded history window.");
    cursor = read.next_cursor ?? undefined;
    if (!cursor) break;
  }
  const discussions = await readConversationDiscussions(
    transport,
    events,
    root,
    valid,
  );
  return {
    detail,
    events,
    discussions: discussions.discussions,
    error: [
      ...(cursor
        ? ["Older conversation history is outside the loaded window."]
        : []),
      ...discussions.errors,
    ].join("\n"),
  };
}

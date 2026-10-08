import type { Model } from "../api/client.js";
import {
  threadSnapshotKey,
  type ThreadTransport,
} from "../controller/thread-types.js";
export type DiscussionRead = {
  thread: Model<"ThreadSummaryV1">;
  events: Model<"ThreadEventItemV1">[];
};
/** Follow explicit batch references at one selection, never a quote index. */
export async function readConversationDiscussions(
  transport: ThreadTransport,
  events: readonly Model<"ThreadEventItemV1">[],
  root?: string | null,
  valid: () => boolean = () => true,
) {
  const links = new Map<
    string,
    NonNullable<Model<"ThreadEventItemV1">["link"]>
  >();
  for (const e of events)
    if (e.link && !links.has(e.link.id)) links.set(e.link.id, e.link);
  const ids = [
    ...new Set(
      [...links.values()].flatMap((l) =>
        l.active && l.target.type === "comment_batch"
          ? l.target.comments.flatMap((c) =>
              c.type === "human_message" ? [c.thread_id] : [],
            )
          : [],
      ),
    ),
  ];
  if (ids.length > 128)
    throw new Error("This conversation exceeds the bounded discussion window.");
  const discussions: DiscussionRead[] = [],
    errors: string[] = [];
  for (const id of ids) {
    if (!valid()) break;
    try {
      const detail = await transport.thread(id),
        items: Model<"ThreadEventItemV1">[] = [];
      const selection = threadSnapshotKey(detail.snapshot);
      if (root !== undefined && selection !== root)
        throw new Error(
          "The conversation changed during reading. Refresh to load its comments.",
        );
      let cursor: string | undefined;
      for (let page = 0; page < 20; page++) {
        if (!valid()) return { discussions: [], errors: [] };
        const read = await transport.events(id, cursor);
        if (
          read.items.type !== "events" ||
          threadSnapshotKey(read.snapshot) !== selection
        )
          throw new Error(
            "Discussion history changed during reading. Refresh to continue.",
          );
        items.push(...read.items.events);
        if (items.length > 4096)
          throw new Error("Discussion history exceeds the bounded window.");
        cursor = read.next_cursor ?? undefined;
        if (!cursor) break;
      }
      if (cursor)
        throw new Error("Discussion history is outside the loaded window.");
      discussions.push({ thread: detail.thread, events: items });
    } catch (cause) {
      errors.push(
        `Discussion ${id.slice(0, 8)}: ${cause instanceof Error ? cause.message : String(cause)}`,
      );
    }
  }
  return { discussions, errors };
}

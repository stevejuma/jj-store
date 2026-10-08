import { ApiError, type Model } from "../api/client.js";
import {
  threadSnapshotKey,
  type ThreadPage,
  type ThreadTransport,
} from "./thread-types.js";

export async function retainThreadHistory(options: {
  transport: ThreadTransport;
  id: string;
  root: string | null;
  first: ThreadPage;
  retained: readonly string[];
  more: boolean;
  valid: () => boolean;
}) {
  let page = options.first;
  const events: Model<"ThreadEventItemV1">[] = [];
  for (let n = 0; n < 20; n++) {
    if (!options.valid()) return;
    if (
      page.items.type !== "events" ||
      page.items.thread_id !== options.id ||
      threadSnapshotKey(page.snapshot) !== options.root
    )
      throw new ApiError("thread_changed_refresh_required");
    events.push(...page.items.events);
    const ids = new Set(events.map((e) => e.message?.message_id));
    const revisions = new Set(
      events.flatMap((e) =>
        e.value.event.type === "message" ? [e.value.event.revision.id] : [],
      ),
    );
    const missingAncestor = events.some(
      (e) => e.message?.reply_to && !revisions.has(e.message.reply_to.id),
    );
    if (
      options.more ||
      !page.next_cursor ||
      (!missingAncestor && options.retained.every((id) => ids.has(id)))
    )
      return { events, cursor: page.next_cursor };
    page = await options.transport.events(options.id, page.next_cursor);
  }
  throw new ApiError("thread_history_limit");
}

export function replyContextLoaded(
  messages: readonly { id: string; parentMessageId?: string }[],
  target: string,
) {
  const seen = new Set<string>();
  let id: string | undefined = target;
  while (id && !seen.has(id)) {
    seen.add(id);
    const message = messages.find((m) => m.id === id);
    if (!message) return false;
    id = message.parentMessageId;
  }
  return !id;
}

export async function loadReplyHistory(
  state: () => { loaded: boolean; cursor: string | null; valid: boolean },
  load: () => Promise<void>,
) {
  for (let page = 0; page < 20; page++) {
    const before = state();
    if (!before.valid || before.loaded) return;
    if (!before.cursor) break;
    await load();
    const after = state();
    if (!after.valid || after.cursor === before.cursor) return;
  }
  const after = state();
  if (after.valid && !after.loaded)
    return after.cursor
      ? "Message is outside loaded history. Load older messages to continue."
      : "This message is unavailable in the current thread.";
}

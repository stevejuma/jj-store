import type {
  CommunityMemberModel,
  CommunityMessageModel,
  CommunityReactionModel,
} from "../community/workspace-model.js";
import { pretty, type Model } from "../api/client.js";
import type {
  ReactionItems,
  ReactionTarget,
} from "../controller/thread-types.js";
import { principalMember } from "./presentation.js";
export const targetKey = (target: ReactionTarget) => pretty(target);
export function channelMessages(events: readonly Model<"ThreadEventItemV1">[]) {
  const latest = new Map<string, Model<"ThreadEventItemV1">>();
  for (const event of events)
    if (event.message && !latest.has(event.message.message_id))
      latest.set(event.message.message_id, event);
  const members = new Map<string, CommunityMemberModel>();
  const targets: Record<string, ReactionTarget> = {};
  const revisions = new Map(
    events
      .filter((e) => e.message)
      .map((e) => [
        e.value.event.type === "message" ? e.value.event.revision.id : "",
        e.message!.message_id,
      ]),
  );
  const messages: CommunityMessageModel[] = [];
  for (const e of [...latest.values()].reverse()) {
    const m = e.message!;
    members.set(m.actor, principalMember(m.actor));
    if (!m.body || e.value.event.type !== "message") continue;
    targets[m.message_id] = {
      type: "human_message",
      thread_id: m.thread_id,
      message_id: m.message_id,
      revision: e.value.event.revision,
    };
    messages.push({
      id: m.message_id,
      authorId: m.actor,
      content: m.body,
      createdAt: new Date(Number(m.accepted_at_millis)).toISOString(),
      timestamp: new Date(Number(m.accepted_at_millis)).toLocaleTimeString([], {
        hour: "2-digit",
        minute: "2-digit",
      }),
      edited: m.previous !== null,
      ...(m.reply_to
        ? {
            parentMessageId:
              revisions.get(m.reply_to.id) ?? `unloaded:${m.reply_to.id}`,
          }
        : {}),
    });
  }
  for (const m of messages) {
    const replies = messages.filter((r) => r.parentMessageId === m.id);
    if (replies.length) {
      m.replyCount = replies.length;
      m.replyParticipantIds = [...new Set(replies.map((r) => r.authorId))];
    }
  }
  return { messages, members: [...members.values()], targets };
}
export function reactionModels(
  items: ReactionItems,
  custom: (value: Extract<Model<"ReactionValueV1">, { type: "custom" }>) => {
    emoji: string;
    url?: string;
    label: string;
  },
): CommunityReactionModel[] {
  const groups = new Map<string, CommunityReactionModel>();
  for (const row of items.reactions) {
    const v = row.value;
    const presentation =
      v.type === "custom"
        ? custom(v)
        : {
            emoji:
              v.type === "like"
                ? "👍"
                : v.type === "dislike"
                  ? "👎"
                  : v.sequence,
            label: v.type === "emoji" ? v.sequence : v.type,
          };
    const previous = groups.get(presentation.emoji);
    groups.set(presentation.emoji, {
      emoji: presentation.emoji,
      label: presentation.label,
      count: (previous?.count ?? 0) + Number(row.count),
      selected: !!previous?.selected || row.viewer_reacted,
      ...("url" in presentation && presentation.url
        ? { customEmojiUrl: presentation.url }
        : {}),
    });
  }
  for (const row of items.actors) {
    if (row.conflict || !row.value || !row.active.length) continue;
    const value = row.value;
    const emoji =
      value.type === "custom"
        ? custom(value).emoji
        : value.type === "like"
          ? "👍"
          : value.type === "dislike"
            ? "👎"
            : value.sequence;
    const group = groups.get(emoji);
    if (group)
      group.actors = [...(group.actors ?? []), principalMember(row.actor)];
  }
  return [...groups.values()];
}

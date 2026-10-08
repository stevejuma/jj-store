import type { Model } from "../api/client.js";
import type {
  CommunityMemberModel,
  CommunityMessageModel,
} from "../community/workspace-model.js";
import { principalMember } from "../collaboration/presentation.js";
import { markdownBlocks } from "../ui/store-agent-chat-body/markdown-blocks.js";
import type { ConversationProjection } from "./agent-adapter.js";
import { attachDiscussions } from "./discussions.js";
import type { DiscussionRead } from "./thread-discussions.js";

export function threadConversationProjection(
  messages: readonly CommunityMessageModel[],
  targets: Record<string, Model<"ReactionTargetV1">>,
  discussions: readonly DiscussionRead[],
  events: readonly Model<"ThreadEventItemV1">[],
  members: readonly CommunityMemberModel[],
  user: CommunityMemberModel,
  parent?: string,
): ConversationProjection {
  const result: ConversationProjection = {
    items: [],
    sources: new Map(),
    threads: [],
    targets: new Map(),
  };
  result.authoritativeDiscussions = new Set(
    discussions.map((d) => d.thread.value.thread_id),
  );
  const positions = new Map<string, number>();
  // Event order is authoritative; edits do not move a message to the bottom.
  [...events].reverse().forEach((e, index) => {
    if (e.message && !positions.has(e.message.message_id))
      positions.set(e.message.message_id, index);
  });
  for (const message of messages) {
    const target = targets[message.id];
    if (target?.type !== "human_message") continue;
    const blocks = markdownBlocks(
      message.id,
      target.revision.id,
      "Message",
      message.content,
    );
    for (const block of blocks)
      result.sources.set(block.id, {
        target,
        part: null,
        text: message.content,
        from: block.sourceFrom ?? 0,
        revision: target.revision.id,
      });
    result.items.push({
      id: message.id,
      observedAt: message.createdAt ? Date.parse(message.createdAt) : undefined,
      message,
      timestamp: { timestamp: message.timestamp, createdAt: message.createdAt },
      kind: "message",
      role: "user",
      label: "Message",
      author:
        members.find((m) => m.id === message.authorId) ??
        principalMember(message.authorId),
      blocks,
      turn: {
        id: message.id,
        author: message.authorId,
        prompt: message.content,
        summary: message.content.slice(0, 240),
      },
    });
  }
  const latestLinks = new Map<string, Model<"ThreadEventItemV1">>();
  for (const event of events)
    if (event.link && !latestLinks.has(event.link.id))
      latestLinks.set(event.link.id, event);
  const batches = [...latestLinks.values()].filter(
    (e) => e.link?.active && e.link.target.type === "comment_batch",
  );
  const replies = discussionReplies(discussions, batches);
  attachDiscussions(result, replies, user);
  for (const event of batches) {
    const batch = event.link!.target;
    if (batch.type !== "comment_batch") continue;
    const message =
      batch.message &&
      events.find(
        (e) =>
          e.value.event.type === "message" &&
          e.value.event.revision.id === batch.message!.id,
      )?.message;
    const item = message
      ? result.items.find((i) => i.id === message.message_id)
      : undefined;
    if (item) item.commentBatchId = batch.request_id;
    else if (
      !message &&
      (!parent
        ? !batch.reply_to
        : events.some(
            (e) =>
              e.message?.message_id === parent &&
              e.value.event.type === "message" &&
              e.value.event.revision.id === batch.reply_to?.id,
          ))
    ) {
      const id = `batch:${batch.request_id}`;
      positions.set(id, events.length - 1 - events.indexOf(event));
      result.items.push({
        id,
        observedAt: Number(event.value.accepted_at_millis),
        kind: "message",
        role: "user",
        label: "Submitted comments",
        author: principalMember(event.link!.actor),
        blocks: [],
        commentBatchId: batch.request_id,
      });
    }
  }
  result.items.sort(
    (a, b) => (positions.get(a.id) ?? 0) - (positions.get(b.id) ?? 0),
  );
  return result;
}

export function discussionReplies(
  discussions: readonly DiscussionRead[],
  batches: readonly Model<"ThreadEventItemV1">[],
) {
  const refs = new Map<string, string[]>();
  for (const event of batches)
    if (event.link?.target.type === "comment_batch")
      for (const target of event.link.target.comments)
        if (target.type === "human_message")
          refs.set(target.revision.id, [
            ...(refs.get(target.revision.id) ?? []),
            event.link.target.request_id,
          ]);
  const replies: Model<"AgentExecutionFeedbackReply">[] = [];
  for (const discussion of discussions) {
    const current = new Map<string, Model<"ThreadEventItemV1">>();
    const links = new Map<
      string,
      NonNullable<Model<"ThreadEventItemV1">["link"]>
    >();
    for (const event of discussion.events) {
      if (event.message && !current.has(event.message.message_id))
        current.set(event.message.message_id, event);
      if (event.link && !links.has(event.link.id))
        links.set(event.link.id, event.link);
    }
    const answers = [...links.values()].flatMap((l) =>
      l.active && l.target.type === "comment_reply" ? [l.target.reply] : [],
    );
    for (const event of discussion.events) {
      const message = event.message;
      if (
        !message?.body ||
        event.value.event.type !== "message" ||
        !current.get(message.message_id)?.message?.body
      )
        continue;
      const revision = event.value.event.revision,
        historical = current.get(message.message_id) !== event;
      if (historical && !refs.has(revision.id)) continue;
      replies.push({
        thread_id: discussion.thread.value.thread_id,
        thread_revision: discussion.thread.reference,
        message,
        revision,
        historical,
        batches: refs.get(revision.id) ?? [],
        source:
          discussion.thread.value.comment_source ??
          (discussion.thread.value.agent_message
            ? {
                target: discussion.thread.value.agent_message,
                part: null,
                selection: null,
                quote: "",
              }
            : null),
        answers: answers.filter((l) =>
          l.comments.some(
            (c) => c.type === "human_message" && c.revision.id === revision.id,
          ),
        ),
      });
    }
  }
  return replies;
}

import type { Model } from "../api/client.js";
import type { CommunityMemberModel } from "../community/workspace-model.js";
import { principalMember } from "../collaboration/presentation.js";
import type { ConversationProjection } from "./agent-adapter.js";
import { sourceAnchor, sourceTargetKey } from "./source-map.js";

export function commentRevisionId(
  message: string,
  revision: Model<"CollabObjectRef">,
) {
  return `${message}:${revision.id}`;
}
export function answerId(answer: Model<"CommentReplyLinkV1">) {
  const target = answer.answer.target;
  return `answer:${target.type === "agent_message" ? target.content.id : sourceTargetKey(target)}:${answer.answer.selection?.start_byte}`;
}
/** The same revision may be visible through several agents. Keep every answer. */
export function mergeReplies(
  replies: readonly Model<"AgentExecutionFeedbackReply">[],
) {
  const result = new Map<string, Model<"AgentExecutionFeedbackReply">>();
  for (const reply of replies) {
    const key = commentRevisionId(reply.message.message_id, reply.revision),
      previous = result.get(key);
    result.set(
      key,
      previous
        ? {
            ...previous,
            batches: [
              ...new Set([
                ...(previous.batches ?? []),
                ...(reply.batches ?? []),
              ]),
            ],
            answers: [
              ...new Map(
                [...previous.answers, ...reply.answers].map((a) => [
                  answerId(a),
                  a,
                ]),
              ).values(),
            ],
          }
        : reply,
    );
  }
  return [...result.values()].sort(
    (a, b) =>
      Number(a.message.accepted_at_millis) -
      Number(b.message.accepted_at_millis),
  );
}
export function attachDiscussions(
  projection: ConversationProjection,
  replies: readonly Model<"AgentExecutionFeedbackReply">[],
  user: CommunityMemberModel,
  labels: ReadonlyMap<string, string> = new Map(),
) {
  projection.answers ??= new Map();
  for (const reply of replies)
    for (const answer of reply.answers)
      projection.answers.set(answerId(answer), answer);
  const blocks = projection.items.flatMap((i) => [
    ...i.blocks,
    ...(i.tools ?? []).flatMap((t) => t.blocks),
  ]);
  for (const reply of mergeReplies(replies)) {
    const source = reply.source;
    const block = blocks.find(
      (b) =>
        source &&
        projection.sources.has(b.id) &&
        sourceAnchor(b, projection.sources.get(b.id)!, source),
    );
    const anchor =
      block && source
        ? sourceAnchor(block, projection.sources.get(block.id)!, source)
        : undefined;
    let thread = projection.threads.find((t) => t.id === reply.thread_id);
    if (!thread) {
      thread = {
        id: reply.thread_id,
        label: block?.label ?? "Original content",
        anchor: anchor ?? {
          blockId: `unavailable:${reply.thread_id}`,
          revision: source ? sourceTargetKey(source) : "whole-message",
          from: 0,
          to: source?.quote.length ?? 0,
          quote: source?.quote ?? "Whole message",
        },
        sourceKind: block?.kind,
        comments: [],
        source,
        revision: reply.thread_revision ?? undefined,
      };
      projection.threads.push(thread);
    }
    thread.comments.push({
      id: commentRevisionId(reply.message.message_id, reply.revision),
      messageId: reply.message.message_id,
      author:
        reply.message.actor === user.id
          ? user
          : principalMember(reply.message.actor),
      text: reply.message.body ?? "",
      status: reply.answers.length ? "replied" : "sent",
      historical: reply.historical,
      revision: reply.revision,
      batchId: reply.batches?.[0],
      batchIds: reply.batches,
      createdAt: Number(reply.message.accepted_at_millis),
      replyItemId: reply.answers[0] ? answerId(reply.answers[0]) : undefined,
      replyDestinations: reply.answers.map((a) => ({
        id: answerId(a),
        label:
          a.answer.target.type === "agent_message"
            ? (labels.get(a.answer.target.session_id) ??
              `Agent ${a.answer.target.session_id.slice(0, 8)} · ${a.answer.target.message_id.slice(0, 8)}`)
            : "Agent answer",
      })),
    });
  }
}

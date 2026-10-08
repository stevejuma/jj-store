import { segments } from "./answer-segments.js";
import { agentMessageTimestamp } from "../collaboration/presentation.js";
import { activityDayDividers } from "../ui/store-agent-execution/presentation.js";
import {
  attachDiscussions,
  answerId,
  commentRevisionId,
} from "./discussions.js";
export { attachDiscussions, answerId } from "./discussions.js";
import { capturedToolProjection } from "./tool-adapter.js";
import type { Model } from "../api/client.js";
import type { ActivityRow } from "../ui/store-agent-execution/presentation.js";
import type { AgentSession } from "../controller/agent-execution-types.js";
import type {
  ChatBodyBlock,
  ChatBodyItem,
  ChatCommentThread,
} from "../ui/store-agent-chat-body/model.js";
import type { CommunityMemberModel } from "../community/workspace-model.js";

import { agentMember } from "../ui/community-agents/model.js";
import { markdownBlocks } from "../ui/store-agent-chat-body/markdown-blocks.js";
import { sourceTargetKey, type ConversationTextSource } from "./source-map.js";

export type AgentConversationRow = ActivityRow & {
  session: AgentSession;
  key: string;
};
export type ConversationProjection = {
  items: ChatBodyItem[];
  sources: Map<string, ConversationTextSource>;
  threads: ChatCommentThread[];
  targets: Map<string, { sessionId: string; rowId: string }>;
  authoritativeDiscussions?: Set<string>;
  answers?: Map<string, Model<"CommentReplyLinkV1">>;
};
export const targetKey = sourceTargetKey;
function revision(target: Model<"ReactionTargetV1">) {
  return target.type === "agent_message" || target.type === "tool_output"
    ? target.content.id
    : target.type === "tool_invocation"
      ? target.snapshot.id
      : targetKey(target);
}
export function agentConversationProjection(
  rows: readonly (ActivityRow & {
    session: Pick<AgentSession, "id" | "setup_id" | "title" | "avatar">;
    key: string;
  })[],
  feedback: Readonly<Record<string, Model<"AgentExecutionFeedback">>>,
  user: CommunityMemberModel,
): ConversationProjection {
  const projection: ConversationProjection = {
    items: [],
    sources: new Map(),
    threads: [],
    targets: new Map(),
  };
  projection.authoritativeDiscussions = new Set(
    Object.values(feedback).flatMap((f) =>
      f.messages.flatMap((m) => m.threads),
    ),
  );
  const days = activityDayDividers(rows.map((r) => ({ ...r, id: r.key })));
  const allReplies: Model<"AgentExecutionFeedbackReply">[] = [];
  const labels = new Map<string, string>();
  const renderedAnswers = new Set<string>();
  for (const row of rows) {
    const captured = feedback[row.session.id]?.messages.find(
      (m) => m.row_id === (row.sourceRowId ?? row.id),
    );
    for (const reply of captured?.replies ?? [])
      allReplies.push({
        ...reply,
        source: reply.source ?? {
          target: captured!.target,
          part: null,
          selection: null,
          quote: "",
        },
      });
    if (captured?.target.type === "agent_message")
      labels.set(
        captured.target.session_id,
        `${row.session.title} · ${row.request.slice(0, 8)}`,
      );
    const author =
      row.kind === "user"
        ? user
        : agentMember({
            id: row.session.setup_id ?? row.session.id,
            name: row.session.title,
            avatar: row.session.avatar,
          });
    const rev = captured
      ? revision(captured.target)
      : `unpublished:${row.text}`;
    const mapBlocks = (
      id: string,
      label: string,
      text: string,
      part:
        | Model<"AgentExecutionFeedbackMessage">["text_parts"][number]
        | undefined,
      base = 0,
    ) => {
      const blocks = markdownBlocks(id, rev, label, text);
      for (const block of blocks) {
        block.commentable = !!part;
        if (part)
          projection.sources.set(block.id, {
            target: captured!.target,
            part: part.part ?? null,
            text: part.text,
            from: base + (block.sourceFrom ?? 0),
            revision: rev,
          });
      }
      return blocks;
    };
    const item = (
      id: string,
      blocks: ChatBodyBlock[],
      answer?: Model<"CommentReplyLinkV1">,
    ): ChatBodyItem => {
      projection.targets.set(id, {
        sessionId: row.session.id,
        rowId: row.sourceRowId ?? row.id,
      });
      const first = answer?.comments[0];
      return {
        id,
        timestamp:
          row.observedAtMillis == null
            ? undefined
            : agentMessageTimestamp(row.observedAtMillis),
        kind: "message",
        label: row.kind === "user" ? "Your message" : "Assistant message",
        role: row.kind === "user" ? "user" : "assistant",
        author,
        blocks,
        replyContext: first
          ? {
              threadId:
                first.type === "human_message"
                  ? first.thread_id
                  : `review:${first.type === "review_comment" ? first.thread_id : ""}`,
              commentIds: answer!.comments.map((c) =>
                c.type === "human_message"
                  ? commentRevisionId(c.message_id, c.revision)
                  : c.type === "review_comment"
                    ? c.comment_id
                    : "",
              ),
            }
          : undefined,
        turn: {
          id: `${row.session.id}:${row.request}`,
          author: user.name,
          prompt:
            rows.find(
              (r) =>
                r.session.id === row.session.id &&
                r.request === row.request &&
                r.kind === "user",
            )?.text ?? "Comments",
          summary: row.text.slice(0, 240),
          agentName: row.session.title,
          createdAt: row.observedAtMillis,
        },
      };
    };
    if (row.kind === "user" || row.kind === "assistant") {
      const part = captured?.text_parts.find((p) =>
        row.sourcePart !== undefined
          ? p.part === row.sourcePart
          : row.kind === "user"
            ? p.part === "/text" || p.part == null
            : p.part == null,
      );
      const from = row.kind === "assistant" ? row.sourceFrom : 0;
      const exact =
        part &&
        from !== undefined &&
        part.text.slice(from, from + row.text.length) === row.text
          ? part
          : undefined;
      if (exact)
        for (const segment of segments(
          exact.text,
          from!,
          from! + row.text.length,
          captured?.answers ?? [],
        )) {
          const destination = segment.answer
            ? answerId(segment.answer)
            : undefined;
          const id = destination
            ? renderedAnswers.has(destination)
              ? `${destination}:fragment:${row.key}:${segment.from}`
              : destination
            : segment.from === from
              ? row.key
              : `${row.key}:${segment.from}`;
          if (destination) renderedAnswers.add(destination);
          const blocks = mapBlocks(
            id,
            row.kind === "user" ? "Your message" : "Assistant message",
            exact.text.slice(segment.from, segment.to),
            exact,
            segment.from,
          );
          if (blocks.length)
            projection.items.push(item(id, blocks, segment.answer));
        }
      else
        projection.items.push(
          item(
            row.key,
            mapBlocks(
              row.key,
              row.kind === "user" ? "Your message" : "Assistant message",
              row.text,
              undefined,
            ),
          ),
        );
    } else if (row.kind === "tool") {
      const { blocks, title } = capturedToolProjection(
        row,
        row.key,
        rev,
        captured,
        projection.sources,
      );
      projection.targets.set(row.key, {
        sessionId: row.session.id,
        rowId: row.sourceRowId ?? row.id,
      });
      projection.items.push({
        id: row.key,
        kind: "tools",
        label: row.text,
        blocks: [],
        tools: [
          {
            id: `${row.key}/tool`,
            revision: rev,
            title,
            status:
              row.value.status === "failed"
                ? "error"
                : row.value.status === "completed"
                  ? "complete"
                  : "running",
            blocks,
            raw: row.value,
            detailPlacement: row.value.kind === "read" ? "sidebar" : "inline",
            commentable: false,
          },
        ],
      });
    } else
      projection.items.push({
        id: row.key,
        kind: "handoff",
        label:
          row.kind === "record" ? `Turn ${row.text}` : row.text || row.kind,
        blocks: [],
        external: true,
      });
  }
  for (const item of projection.items) {
    const target = projection.targets.get(item.id);
    item.observedAt = rows.find(
      (r) =>
        (r.session.id === target?.sessionId &&
          (r.sourceRowId ?? r.id) === target?.rowId) ||
        r.key === item.id,
    )?.observedAtMillis;
  }
  for (const row of rows) {
    const first = projection.items.find(
      (i) =>
        (projection.targets.get(i.id)?.sessionId === row.session.id &&
          projection.targets.get(i.id)?.rowId ===
            (row.sourceRowId ?? row.id)) ||
        i.id === row.key,
    );
    if (first && days.has(row.key)) first.dayLabel = days.get(row.key);
  }
  attachDiscussions(projection, allReplies, user, labels);
  // The context records the submission's canonical request, distinct from each
  // recipient request. Never associate batches by matching authored text.
  for (const row of rows.filter((r) => r.kind === "user")) {
    const context = row.value.sentContext as
      { commentSubmission?: { request_id?: string } } | undefined;
    const batch = context?.commentSubmission?.request_id;
    const item = projection.items.find((i) => i.id === row.key);
    if (item && typeof batch === "string") item.commentBatchId = batch;
  }
  return projection;
}

import type { Model } from "../api/client.js";
import type { CommunityMemberModel } from "../community/workspace-model.js";
import type { ActivityRow } from "../ui/store-agent-execution/presentation.js";
import { attachDiscussions } from "./discussions.js";
import { agentConversationProjection } from "./agent-adapter.js";
import {
  discussionReplies,
  threadConversationProjection,
} from "./thread-adapter.js";
import type { ConversationRead } from "./read-conversation.js";
import { channelMessages } from "../collaboration/thread-presentation.js";
import {
  agentMessageContent,
  agentSentContext,
} from "../collaboration/agent-message-content.js";
import { sourceTargetKey } from "./source-map.js";

function textParts(
  content: Model<"AgentContent">,
): Model<"AgentExecutionFeedbackMessage">["text_parts"] {
  if (content.media_type === "text")
    return [{ part: null, text: content.content }];
  let value: unknown;
  try {
    value = JSON.parse(content.content);
  } catch {
    return [];
  }
  const parts: Model<"AgentExecutionFeedbackMessage">["text_parts"] = [];
  function visit(value: unknown, pointer: string) {
    if (parts.length > 128) return;
    if (typeof value === "string") parts.push({ part: pointer, text: value });
    else if (value && typeof value === "object")
      for (const [key, entry] of Object.entries(value))
        visit(
          entry,
          `${pointer}/${key.replaceAll("~", "~0").replaceAll("/", "~1")}`,
        );
  }
  visit(value, "");
  return parts.length > 128 ||
    parts.reduce((n, p) => n + new TextEncoder().encode(p.text).length, 0) >
      262144
    ? []
    : parts;
}
export function recordedConversationProjection(
  activity: readonly Model<"AgentActivity">[],
  sessionId: string,
  integration: Model<"IntegrationId">,
  agent: CommunityMemberModel,
  user: CommunityMemberModel,
  read?: ConversationRead,
  captured?: Model<"AgentExecutionFeedback">,
) {
  const session = {
    id: sessionId,
    setup_id: null,
    title: agent.name,
    avatar: "",
  };
  const rows: (ActivityRow & { session: typeof session; key: string })[] = [];
  const replies = [
    ...discussionReplies(read?.discussions ?? [], read?.events ?? []),
    ...(captured?.messages ?? []).flatMap((m) =>
      m.replies.map((r) => ({
        ...r,
        source: r.source ?? {
          target: m.target,
          part: null,
          selection: null,
          quote: "",
        },
      })),
    ),
  ];
  const answers = [
    ...new Map(
      [
        ...replies.flatMap((r) => r.answers),
        ...(captured?.messages ?? []).flatMap((m) => m.answers),
      ].map((a) => [sourceTargetKey(a), a]),
    ).values(),
  ];
  const feedback: Model<"AgentExecutionFeedback"> = {
    profile: null,
    write_unavailable: null,
    messages: [],
  };
  for (const entry of activity) {
    const id =
      entry.type === "message"
        ? entry.message_id
        : entry.type === "tool"
          ? entry.tool.tool_call_id
          : entry.event_id;
    const key = `${sessionId}:${id}`;
    const base = {
      id,
      key,
      session,
      request:
        "turn_id" in entry
          ? (entry.turn_id ?? id)
          : entry.type === "tool"
            ? (entry.tool.turn_id ?? id)
            : id,
      sequence: Number(entry.sequence),
    };
    let target: Model<"ReactionTargetV1"> | null | undefined,
      content: Model<"AgentContent"> | undefined;
    if (entry.type === "message") {
      target = entry.reaction_target;
      content = entry.content;
      if (integration === "open_code" && entry.content.media_type === "json") {
        const parts = textParts(content)
          .filter((p) => /^\/\d+\/text$/.test(p.part ?? ""))
          .sort(
            (a, b) =>
              Number(a.part!.split("/")[1]) - Number(b.part!.split("/")[1]),
          );
        if (parts.length)
          for (const part of parts)
            rows.push({
              ...base,
              id: `${id}:${part.part}`,
              key: `${key}:${part.part}`,
              sourceRowId: id,
              sourcePart: part.part,
              sourceFrom: 0,
              kind: entry.role === "user" ? "user" : "assistant",
              text: part.text,
              value: {},
            });
        else
          rows.push({
            ...base,
            kind: entry.role === "user" ? "user" : "assistant",
            text: agentMessageContent(entry.content, integration),
            sourceFrom: 0,
            value: {},
          });
      } else
        rows.push({
          ...base,
          kind: entry.role === "user" ? "user" : "assistant",
          text: agentMessageContent(entry.content, integration),
          sourceFrom: 0,
          value: {
            sentContext:
              entry.role === "user"
                ? agentSentContext(entry.content, integration)
                : undefined,
          },
          observedAtMillis:
            entry.occurred_at_millis == null
              ? undefined
              : Number(entry.occurred_at_millis),
        });
    } else if (entry.type === "tool") {
      target = entry.tool.reaction_target;
      content = entry.tool.output ?? entry.tool.input ?? undefined;
      rows.push({
        ...base,
        kind: "tool",
        text: entry.tool.native_name,
        value: {
          toolCallId: entry.tool.tool_call_id,
          title: entry.tool.native_name,
          kind: entry.tool.kind,
          status:
            entry.tool.state === "succeeded" ? "completed" : entry.tool.state,
          paths: entry.tool.observed_paths,
        },
      });
    } else if (entry.type === "capture_gap")
      rows.push({
        ...base,
        kind: "record",
        text: `Capture gap ${entry.missing_from}–${entry.missing_to}`,
        value: {},
      });
    else if (entry.type === "permission" && entry.state !== "not_observed")
      rows.push({
        ...base,
        kind: "record",
        text: `Tool permission ${entry.state}`,
        value: {},
      });
    if (target)
      feedback.messages.push({
        row_id: id,
        target,
        text_parts: content ? textParts(content) : [],
        threads: [],
        replies: replies.filter(
          (r) =>
            r.source &&
            sourceTargetKey(r.source.target) === sourceTargetKey(target),
        ),
        answers: answers.filter(
          (a) => sourceTargetKey(a.answer.target) === sourceTargetKey(target),
        ),
      });
  }
  const projection = agentConversationProjection(
    rows,
    { [sessionId]: feedback },
    user,
  );
  // Old captures retain recorded message/tool boundaries; completion order is not
  // evidence of the original streaming order. Unsupported rich JSON stays whole.
  for (const item of projection.items) {
    const captured = feedback.messages.find(
      (m) => projection.targets.get(item.id)?.rowId === m.row_id,
    );
    if (captured)
      for (const block of item.blocks)
        if (!projection.sources.has(block.id)) {
          block.commentable = true;
          block.wholeSection = true;
          projection.sources.set(block.id, {
            target: captured.target,
            part: null,
            text: block.text,
            from: 0,
            revision: block.revision,
            wholeSection: true,
          });
        }
    if (item.external) item.external = false;
    if (item.role === "assistant") item.author = agent;
  }
  projection.authoritativeDiscussions = new Set([
    ...(captured?.messages ?? []).flatMap((m) => m.threads),
    ...(read?.discussions ?? []).map((d) => d.thread.value.thread_id),
  ]);
  projection.threads = [];
  attachDiscussions(projection, replies, user);
  if (read) {
    const messages = channelMessages(read.events);
    const human = threadConversationProjection(
      messages.messages,
      messages.targets,
      read.discussions,
      read.events,
      [user],
      user,
    );
    projection.items.push(...human.items);
    for (const [id, source] of human.sources)
      projection.sources.set(id, source);
    // Discussions already attached to captured sources keep their exact mapping.
    for (const thread of human.threads)
      if (!projection.threads.some((t) => t.id === thread.id))
        projection.threads.push(thread);
  }
  return projection;
}

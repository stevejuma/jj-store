import type { Model } from "../api/client.js";
import type {
  CommunityMemberModel,
  CommunityMessageModel,
} from "../community/workspace-model.js";
import { agentAvatarUrl } from "./agent-avatar.js";
import { agentMessageContent } from "./agent-message-content.js";
import type { FileDiffLineContext } from "@lapismd/design-core/diff";
import type { ReviewInlinePatch } from "../controller/review-anchor-patch.js";

export function shortIdentity(value: string, length = 12): string {
  return value.length <= length ? value : `${value.slice(0, length)}…`;
}

export function integrationLabel(
  integration: Model<"AgentSessionSummary">["integration"],
): string {
  if (integration === "open_code") return "OpenCode";
  if (integration === "owned_controller") return "Controller";
  return integration[0]!.toUpperCase() + integration.slice(1);
}

export function agentMember(
  summary: Model<"AgentSessionSummary">,
  captured?: { title: string; avatar: string; connected?: boolean },
): CommunityMemberModel {
  const name = captured?.title || integrationLabel(summary.integration);
  const avatarUrl = captured?.avatar || agentAvatarUrl(summary.integration);
  return {
    id: summary.session_id,
    name,
    handle: `${name.toLowerCase()} agent`,
    initials: name.slice(0, 2).toUpperCase(),
    ...(avatarUrl ? { avatarUrl } : {}),
    kind: "agent",
    presence:
      captured?.connected !== undefined
        ? captured.connected
          ? "online"
          : "offline"
        : summary.state === "active"
          ? "online"
          : "offline",
    status: `${summary.state} · ${summary.coverage.completeness} capture`,
    role: "Agent",
  };
}

export function agentLastActivityAt(summary: Model<"AgentSessionSummary">) {
  const value = summary.last_activity_at_millis;
  if (value === null || value === undefined) return undefined;
  const millis = Number(value);
  return Number.isSafeInteger(millis) &&
    millis >= 0 &&
    millis <= 8_640_000_000_000_000
    ? millis / 1_000
    : undefined;
}

export function anchorAgentSession(
  anchor: Model<"ArtifactAnchorV1">,
): string | undefined {
  const { target } = anchor;
  if (
    target.target === "agent_text" ||
    target.target === "agent_artifact" ||
    target.target === "agent_session"
  )
    return target.value.session_id;
}

export function anchorAgentTool(
  anchor: Model<"ArtifactAnchorV1">,
): { sessionId: string; toolCallId: string } | undefined {
  const { target } = anchor;
  if (target.target !== "agent_artifact" || target.value.owner.kind !== "tool")
    return;
  return {
    sessionId: target.value.session_id,
    toolCallId: target.value.owner.tool_call_id,
  };
}

export function reviewForAgentTool(
  threads: readonly Model<"ReviewThreadSummary">[],
  sessionId: string,
  toolCallId: string,
) {
  return threads.find((thread) => {
    const tool = anchorAgentTool(thread.anchor);
    return tool?.sessionId === sessionId && tool.toolCallId === toolCallId;
  });
}

export function reviewPatchMatchesLine(
  patch: ReviewInlinePatch | undefined,
  context: FileDiffLineContext,
): boolean {
  if (!patch || patch.lineNumber !== context.lineNumber) return false;
  if (patch.path !== context.path) return false;
  return patch.side === "before"
    ? context.variant === "removed"
    : context.variant !== "removed";
}

export function anchorAgentMember(
  anchor: Model<"ArtifactAnchorV1">,
  sessions: readonly Model<"AgentSessionSummary">[],
): CommunityMemberModel | undefined {
  const sessionId = anchorAgentSession(anchor);
  const session = sessions.find(
    (candidate) => candidate.session_id === sessionId,
  );
  return session ? agentMember(session) : undefined;
}

export interface AgentSessionTiming {
  startedAt: number;
  durationMs: number;
  durationLabel: string;
}

export function agentSessionTiming(
  summary: Model<"AgentSessionSummary">,
): AgentSessionTiming | undefined {
  const first = summary.first_activity_at_millis;
  const last = summary.last_activity_at_millis;
  if (first === null || last === null) return;
  const start = Number(first);
  const end = Number(last);
  if (!Number.isSafeInteger(start) || !Number.isSafeInteger(end) || end < start)
    return;
  const durationMs = end - start;
  return {
    startedAt: Math.floor(start / 1_000),
    durationMs,
    durationLabel: formatAgentSessionDuration(durationMs),
  };
}

export function formatAgentSessionDuration(durationMs: number): string {
  const totalSeconds = Math.max(0, Math.round(durationMs / 1_000));
  const days = Math.floor(totalSeconds / 86_400);
  const hours = Math.floor((totalSeconds % 86_400) / 3_600);
  const minutes = Math.floor((totalSeconds % 3_600) / 60);
  const seconds = totalSeconds % 60;
  if (days > 0) return hours > 0 ? `${days}d ${hours}h` : `${days}d`;
  if (hours > 0) return minutes > 0 ? `${hours}h ${minutes}m` : `${hours}h`;
  if (minutes > 0)
    return seconds > 0 ? `${minutes}m ${seconds}s` : `${minutes}m`;
  return `${seconds}s`;
}

export function agentToolDuration(
  duration: number | bigint | null | undefined,
): string | undefined {
  if (duration == null) return undefined;
  const millis = Number(duration);
  if (!Number.isSafeInteger(millis) || millis < 0) return undefined;
  if (millis < 1000) return `${millis}ms`;
  if (millis < 60_000) return `${Number((millis / 1000).toFixed(2))}s`;
  return formatAgentSessionDuration(millis);
}

export function principalMember(principal: string): CommunityMemberModel {
  const local = principal === "local" || principal.endsWith("local");
  return {
    id: principal,
    name: local ? "Local reviewer" : "Repository reviewer",
    handle: local ? "local reviewer" : "repository principal",
    initials: local ? "LR" : principal.slice(0, 2).toUpperCase(),
    presence: "offline",
    role: "Reviewer",
  };
}

export function agentMessageTimestamp(
  occurredAtMillis: number | bigint | null | undefined,
): Pick<CommunityMessageModel, "timestamp" | "createdAt"> {
  if (occurredAtMillis === null || occurredAtMillis === undefined)
    return { timestamp: "Time unknown" };
  const millis = Number(occurredAtMillis);
  const date = new Date(millis);
  if (!Number.isSafeInteger(millis) || !Number.isFinite(date.getTime()))
    return { timestamp: "Time unknown" };
  return {
    timestamp: date.toLocaleTimeString(undefined, {
      hour: "2-digit",
      minute: "2-digit",
      second: "2-digit",
    }),
    createdAt: date.toISOString(),
  };
}

export function transcriptMessage(
  chunk: Model<"AgentTranscriptChunk">,
  summary: Model<"AgentSessionSummary">,
): CommunityMessageModel {
  return {
    id: `${summary.session_id}:${chunk.sequence}`,
    authorId: summary.session_id,
    content: agentMessageContent(chunk, summary.integration),
    ...agentMessageTimestamp(chunk.occurred_at_millis),
  };
}

export function reviewMessage(
  comment: Model<"ReviewComment">,
): CommunityMessageModel {
  const content =
    comment.content.state === "visible"
      ? comment.content.text
      : comment.content.state === "hidden"
        ? `*Comment hidden: ${comment.content.reason}*`
        : `*Comment deleted${comment.content.reason ? `: ${comment.content.reason}` : ""}*`;
  return {
    id: comment.comment_id,
    authorId: comment.author,
    parentMessageId: comment.parent_comment_id ?? undefined,
    content,
    timestamp: new Date(Number(comment.created_at_millis)).toLocaleString(),
    createdAt: new Date(Number(comment.created_at_millis)).toISOString(),
    edited: comment.previous_revision_id !== null,
  };
}

export {
  reviewCommentRows,
  type ReviewCommentRow,
} from "./review-comment-rows.js";

export function anchorLabel(anchor: Model<"ArtifactAnchorV1">): string {
  const { target } = anchor;
  if (target.target === "repository") return "Repository";
  if (target.target === "operation") return "Operation";
  if (target.target === "revision") return "Revision";
  if (target.target === "code_text" || target.target === "code_artifact")
    return target.value.path;
  return "Agent session";
}

export function anchorCommit(
  anchor: Model<"ArtifactAnchorV1">,
): string | undefined {
  const { target } = anchor;
  if (
    target.target === "revision" ||
    target.target === "code_text" ||
    target.target === "code_artifact"
  )
    return target.value.commit_id;
}

export function anchorOperationIds(
  anchor: Model<"ArtifactAnchorV1">,
): readonly string[] {
  const { target } = anchor;
  if (target.target === "operation") return [target.value.operation_id];
  if (target.target === "revision" || target.target === "code_artifact")
    return [target.value.operation_id];
  if (target.target !== "code_text") return [];
  const operations = [target.value.operation_id];
  if (target.value.diff_context) {
    operations.push(
      target.value.diff_context.before_operation_id,
      target.value.diff_context.after_operation_id,
    );
  }
  return [...new Set(operations)];
}

export function anchorMatchesOperation(
  anchor: Model<"ArtifactAnchorV1">,
  operation: string,
): boolean {
  return anchorOperationIds(anchor).includes(operation);
}

export function anchorPath(
  anchor: Model<"ArtifactAnchorV1">,
): string | undefined {
  const { target } = anchor;
  if (target.target === "code_text" || target.target === "code_artifact")
    return target.value.path;
}

export function anchorMatchesPath(
  anchor: Model<"ArtifactAnchorV1">,
  path: string,
): boolean {
  const target = anchor.target;
  if (target.target === "code_artifact") return target.value.path === path;
  if (target.target !== "code_text") return false;
  return (
    target.value.path === path ||
    target.value.diff_context?.before_path === path ||
    target.value.diff_context?.after_path === path
  );
}

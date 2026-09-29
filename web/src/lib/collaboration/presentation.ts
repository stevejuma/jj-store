import type { Model } from "../api/client.js";
import type {
  CommunityMemberModel,
  CommunityMessageModel,
} from "@lapismd/lapis-community/components/community-message";

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
): CommunityMemberModel {
  const name = integrationLabel(summary.integration);
  return {
    id: summary.session_id,
    name,
    handle: `${name.toLowerCase()} agent`,
    initials: name.slice(0, 2).toUpperCase(),
    kind: "agent",
    presence: summary.state === "active" ? "online" : "offline",
    status: `${summary.state} · ${summary.coverage.completeness} capture`,
    role: "Agent",
  };
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

export function transcriptMessage(
  chunk: Model<"AgentTranscriptChunk">,
  summary: Model<"AgentSessionSummary">,
): CommunityMessageModel {
  let content = chunk.content;
  if (chunk.media_type === "json") {
    try {
      content = `\`\`\`json\n${JSON.stringify(JSON.parse(content), null, 2)}\n\`\`\``;
    } catch {
      content = `\`\`\`text\n${content}\n\`\`\``;
    }
  }
  return {
    id: `${summary.session_id}:${chunk.sequence}`,
    authorId: summary.session_id,
    content,
    timestamp: `#${chunk.sequence}`,
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

export function anchorPath(
  anchor: Model<"ArtifactAnchorV1">,
): string | undefined {
  const { target } = anchor;
  if (target.target === "code_text" || target.target === "code_artifact")
    return target.value.path;
}

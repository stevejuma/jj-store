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
    handle: shortIdentity(summary.session_id),
    initials: name.slice(0, 2).toUpperCase(),
    kind: "agent",
    presence: summary.state === "active" ? "online" : "offline",
    status: `${summary.state} · ${summary.coverage.completeness} capture`,
    role: "Agent",
  };
}

export function principalMember(principal: string): CommunityMemberModel {
  const local = principal === "local" || principal.endsWith("local");
  return {
    id: principal,
    name: local ? "Local reviewer" : `Reviewer ${shortIdentity(principal, 8)}`,
    handle: shortIdentity(principal),
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
  if (target.target === "operation")
    return `Operation ${shortIdentity(target.value.operation_id)}`;
  if (target.target === "revision")
    return `Revision ${shortIdentity(target.value.commit_id)}`;
  if (target.target === "code_text" || target.target === "code_artifact")
    return target.value.path;
  return `Agent session ${shortIdentity(target.value.session_id)}`;
}

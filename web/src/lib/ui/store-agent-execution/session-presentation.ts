import type {
  AgentStatus,
  AgentSession,
} from "../../controller/agent-execution-types.js";
import { agentMember } from "../community-agents/model.js";
export function sessionStatus(session: AgentSession, unknown = false): string {
  if (unknown) return "Unknown";
  if (session.connected)
    return session.state === "running" ? "Working" : "Online";
  return ["preparing", "opening"].includes(session.state)
    ? "Starting"
    : "Offline";
}
export function associatedSessions(
  status: AgentStatus | null,
  agent?: string | null,
) {
  return (status?.sessions ?? []).filter((s) => !agent || s.setup_id === agent);
}
export function sessionItems(
  status: AgentStatus | null,
  agent?: string | null,
  unknown = false,
  repositories: readonly { id: string; name: string }[] = [],
) {
  return associatedSessions(status, agent).map((session) => {
    const repositoryName =
      repositories.find((r) => r.id === session.repository)?.name ??
      `repository ${session.repository.slice(0, 12)}`;
    const query = new URLSearchParams({
      at_op: session.workspace_name
        ? (session.workspace_operation ?? session.operation)
        : session.operation,
    });
    if (session.workspace_name) query.set("workspace", session.workspace_name);
    return {
      id: session.id,
      author: {
        ...agentMember({
          id: session.id,
          name: session.conversation_title ?? "New conversation",
          avatar: session.avatar,
        }),
        acpAgentId: session.setup_id ?? undefined,
      },
      excerpt: session.discovery_text ?? undefined,
      context: `${session.setup_id ? (status?.setups.find((s) => s.id === session.setup_id)?.name ?? "Deleted agent") : "Unassigned"} · ${session.runner}`,
      lastActivityAt:
        typeof session.last_activity_at_millis === "number" &&
        Number.isSafeInteger(session.last_activity_at_millis) &&
        session.last_activity_at_millis > 0 &&
        session.last_activity_at_millis <= 8_640_000_000_000_000
          ? session.last_activity_at_millis / 1_000
          : undefined,
      status: sessionStatus(session, unknown),
      repository: {
        id: session.repository,
        name: repositoryName,
        href: `#/repos/${encodeURIComponent(session.repository)}/files?${query}`,
        label: `Open ${repositoryName}`,
      },
    };
  });
}

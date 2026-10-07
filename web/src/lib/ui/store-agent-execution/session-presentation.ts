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
) {
  return associatedSessions(status, agent).map((session) => ({
    id: session.id,
    author: agentMember({
      id: session.id,
      name: session.title,
      avatar: session.avatar,
    }),
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
      href: `#/repos/${encodeURIComponent(session.repository)}/files?${new URLSearchParams({ at_op: session.operation })}`,
      label: `Open workspace repository for ${session.title}`,
    },
  }));
}

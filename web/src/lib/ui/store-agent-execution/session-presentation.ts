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
  return associatedSessions(status, agent).flatMap((session) => {
    const bindings = session.repository_bindings.length
      ? session.repository_bindings
      : [undefined];
    return bindings.map((binding) => {
      const repositoryId = binding?.repository ?? session.repository;
      const repositoryName =
        session.conversation_id && !binding
          ? "Personal"
          : (repositories.find((r) => r.id === repositoryId)?.name ??
            `repository ${repositoryId.slice(0, 12)}`);
      const query = new URLSearchParams({
        at_op:
          binding?.operation ??
          (session.workspace_name
            ? (session.workspace_operation ?? session.operation)
            : session.operation),
      });
      if (binding?.workspace_name)
        query.set("workspace", binding.workspace_name);
      else if (!binding && session.workspace_name)
        query.set("workspace", session.workspace_name);
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
        agents: session.setup_id
          ? [
              {
                id: session.setup_id,
                name:
                  status?.setups.find((s) => s.id === session.setup_id)?.name ??
                  "Deleted agent",
              },
            ]
          : [],
        harness: session.runner,
        repository: {
          id: repositoryId,
          name: repositoryName,
          href: `#/repos/${encodeURIComponent(repositoryId)}/files?${query}`,
          label: `Open ${repositoryName}`,
        },
      };
    });
  });
}

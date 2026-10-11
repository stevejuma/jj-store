import type {
  AgentSession,
  AgentStatus,
} from "../../controller/agent-execution-types.js";
import type { Model } from "../../api/client.js";
import { managedRecordedAgent } from "../../collaboration/managed-agent.js";

/** Explicit code bindings take precedence over the conversation's metadata home. */
export function repositoryExecutionSessions(
  status: AgentStatus | null | undefined,
  repository: string,
  recorded: readonly Model<"AgentSessionSummary">[] = [],
  query = "",
): AgentSession[] {
  const represented = new Set(
    recorded
      .map(
        (session) =>
          managedRecordedAgent(repository, session, status?.sessions)?.id,
      )
      .filter(Boolean),
  );
  const search = query.trim().toLowerCase();
  return (status?.sessions ?? []).filter(
    (session) =>
      (session.repository_bindings.length
        ? session.repository_bindings.some(
            (binding) => binding.repository === repository,
          )
        : session.repository === repository) &&
      !represented.has(session.id) &&
      (!search ||
        [
          session.conversation_title,
          session.title,
          session.discovery_text,
          session.id,
          session.runner,
          session.state,
        ].some((value) => value?.toLowerCase().includes(search))),
  );
}

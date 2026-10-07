import type { Model } from "../api/client.js";
import type { AgentSession } from "../controller/agent-execution-types.js";

/** Enrich presentation only through exact admitted local associations. */
export function managedRecordedAgent(
  repository: string,
  recorded: Model<"AgentSessionSummary">,
  sessions: readonly AgentSession[] = [],
): AgentSession | undefined {
  if (recorded.integration !== "owned_controller") return undefined;
  const matches = sessions.filter(
    (session) =>
      session.repository === repository &&
      session.recorded_session_ids?.includes(recorded.session_id),
  );
  return matches.length === 1 ? matches[0] : undefined;
}

export function managedWorkspaceAgent(
  repository: string,
  workspace: string | null,
  sessions: readonly AgentSession[] = [],
): AgentSession | undefined {
  if (!workspace) return undefined;
  const matches = sessions.filter(
    (session) =>
      session.repository === repository &&
      session.state !== "cleaned" &&
      session.workspace_name === workspace,
  );
  return matches.length === 1 ? matches[0] : undefined;
}

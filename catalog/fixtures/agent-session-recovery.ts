import type { AgentExecution } from "../../web/src/lib/controller/agent-execution.svelte.js";
import type {
  AgentEvent,
  AgentOutcome,
  AgentSession,
} from "../../web/src/lib/controller/agent-execution-types.js";

export function staleResumeOutcome(session: AgentSession): AgentOutcome {
  return {
    version: 1,
    request_id: "bb".repeat(16),
    session_id: session.id,
    state: "dispatching",
    error: null,
  };
}
export function seedStaleResume(
  manager: AgentExecution,
  session: AgentSession,
  active: boolean,
) {
  if (!active) return;
  session.state = "interrupted";
  session.connected = false;
  session.last_error = "agent_rejected";
  manager.outcome = staleResumeOutcome(session);
  manager.pending = {
    path: "agents/command",
    body: {
      version: 1,
      request_id: manager.outcome.request_id,
      session_id: session.id,
      action: { action: "resume" },
    },
  };
}
export function cancelledTurnEvents(
  command?: Record<string, unknown> | null,
): AgentEvent[] {
  const action = command?.action as
    { action?: string; target?: string } | undefined;
  return action?.action === "cancel"
    ? [
        {
          sequence: 3,
          observed_at_millis: null,
          committed: true,
          payload: {
            type: "record",
            requestId: action.target,
            record: { status: "cancelled" },
          },
        },
      ]
    : [];
}

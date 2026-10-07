import type { AgentSession } from "./agent-execution-types.js";
/** Activity opens a live associated conversation without creating one. */
export function activeAgentSession(
  all: readonly AgentSession[],
  agent?: string,
) {
  const sessions = all.filter(
    (session) => !agent || session.setup_id === agent,
  );
  return (
    sessions.find(
      (session) => session.connected && session.state === "running",
    ) ??
    sessions.find((session) => session.connected) ??
    sessions.find((session) => ["preparing", "opening"].includes(session.state))
  );
}

export function openAgentActivity(
  manager: import("./agent-execution.svelte.js").AgentExecution,
  agent?: string,
  session?: string,
) {
  if (session) return manager.openSession(session, agent);
  const active = activeAgentSession(manager.value?.sessions ?? [], agent);
  if (active) manager.openSession(active.id, agent);
  else manager.viewSessions(agent);
}

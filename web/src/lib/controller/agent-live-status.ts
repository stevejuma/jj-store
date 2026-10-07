import type { AgentStatus } from "./agent-execution-types.js";
export type AgentLiveState =
  | "starting"
  | "idle"
  | "queued"
  | "working"
  | "attention"
  | "stopping"
  | "offline"
  | "error"
  | "unknown";
export const agentLiveLabels: Record<AgentLiveState, string> = {
  starting: "Starting",
  idle: "Idle",
  queued: "Queued",
  working: "Working",
  attention: "Needs attention",
  stopping: "Stopping",
  offline: "Offline",
  error: "Error",
  unknown: "Status unknown",
};
/** Only exact installation IDs associate captured avatars with current processes. */
export function agentLiveState(
  status: AgentStatus | null,
  id: string,
  unknown = false,
): AgentLiveState {
  if (unknown || !status) return "unknown";
  const live = status.live_agents?.find((agent) => agent.agent_id === id);
  if (live)
    return live.state in agentLiveLabels
      ? (live.state as AgentLiveState)
      : "unknown";
  if (!status.setups.some((agent) => agent.id === id)) return "unknown";
  const sessions = status.sessions.filter((session) => session.setup_id === id);
  if (
    sessions.some((session) => session.connected && session.state === "running")
  )
    return "working";
  if (sessions.some((session) => session.connected)) return "idle";
  if (
    sessions.some((session) => ["preparing", "opening"].includes(session.state))
  )
    return "starting";
  return "offline";
}

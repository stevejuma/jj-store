import {
  agentLiveState,
  agentLiveLabels,
} from "../../controller/agent-live-status.js";
import type {
  AgentSession,
  AgentSetup,
} from "../../controller/agent-execution-types.js";
import type { CommunityMemberModel } from "../community-workspace/model.js";
export function agentMember(
  agent: Pick<AgentSetup, "id" | "name" | "avatar">,
): CommunityMemberModel {
  return {
    id: agent.id,
    name: agent.name,
    initials: agent.name.slice(0, 2).toUpperCase(),
    handle: "",
    avatarUrl: agent.avatar,
    role: "agent",
    kind: "agent",
    acpAgentId: agent.id,
  };
}
export function agentAvailability(
  id: string,
  sessions: readonly AgentSession[],
  unknown: boolean,
  status?:
    import("../../controller/agent-execution-types.js").AgentStatus | null,
) {
  if (status?.live_agents?.some((a) => a.agent_id === id)) {
    const state = agentLiveState(status, id, unknown);
    return {
      label: agentLiveLabels[state],
      presence:
        state === "offline"
          ? "offline"
          : ["idle", "working"].includes(state)
            ? "online"
            : "unknown",
    } as const;
  }
  const matching = sessions.filter((session) => session.setup_id === id);
  if (unknown) return { label: "Unknown", presence: "unknown" } as const;
  if (matching.some((s) => s.connected && s.state === "running"))
    return { label: "Working", presence: "online" } as const;
  if (matching.some((s) => s.connected))
    return { label: "Online", presence: "online" } as const;
  if (matching.some((s) => ["preparing", "opening"].includes(s.state)))
    return { label: "Starting", presence: "unknown" } as const;
  return { label: "Offline", presence: "offline" } as const;
}

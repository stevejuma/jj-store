import type { LocalClient } from "../api/local-client.js";
import type { AgentStatus, AgentTarget } from "./agent-execution-types.js";
export function readAgentExecutionStatus(
  client: LocalClient,
  repository?: string,
) {
  return Promise.allSettled([
    client.request<AgentStatus>("agents"),
    client.request<AgentTarget[]>(
      `agents/targets${repository ? `?repository=${encodeURIComponent(repository)}` : ""}`,
    ),
  ]).then(([status, targets]) => {
    if (status.status === "rejected") throw status.reason;
    return [
      status.value,
      targets.status === "fulfilled" ? targets.value : [],
      targets.status === "fulfilled"
        ? ""
        : "Repository workspaces unavailable. Retry to refresh selections.",
    ] as const;
  });
}
export const agentControlBody = (id: string, start: boolean) => ({
  version: 1,
  request_id: crypto.randomUUID().replaceAll("-", ""),
  agent_id: id,
  start,
});
export const agentLaunchBody = (
  target: AgentTarget,
  runner: string,
  setupId?: string,
  commit?: string,
  context?: string,
) => ({
  version: 1,
  request_id: crypto.randomUUID().replaceAll("-", ""),
  target_id: target.id,
  repository: target.repository,
  operation: target.operation,
  commit: commit || target.commit,
  runner,
  setup_id: setupId || null,
  context: context || null,
});

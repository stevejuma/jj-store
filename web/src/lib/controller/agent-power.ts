import { browserHost, type HostEnvironment } from "../host/contracts.js";
import type { AgentStatus } from "./agent-execution-types.js";
export async function syncAgentPower(
  status: AgentStatus | null,
  enabled: boolean,
  host: HostEnvironment = browserHost,
) {
  if (!host.agentPower) return;
  const active =
    enabled &&
    (status?.live_agents?.some(
      (agent) =>
        !["offline", "error", "unknown", "stopping"].includes(agent.state),
    ) ??
      false);
  const activity = Math.max(
    0,
    ...(status?.sessions ?? []).map((session) =>
      Number(session.last_activity_at_millis ?? 0),
    ),
  );
  try {
    await host.agentPower(active, activity);
  } catch {
    /* Unsupported host never blocks an agent turn. */
  }
}

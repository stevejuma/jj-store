import { isMacDesktopWindow } from "../desktop-chrome.js";
import type { AgentStatus } from "./agent-execution-types.js";
export async function syncAgentPower(
  status: AgentStatus | null,
  enabled: boolean,
) {
  if (!isMacDesktopWindow()) return;
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
    const { invoke } = await import("@tauri-apps/api/core");
    await invoke("agent_power", { active, activity });
  } catch {
    /* Unsupported host never blocks an agent turn. */
  }
}

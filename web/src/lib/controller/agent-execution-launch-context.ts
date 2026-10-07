import type { AgentExecution } from "./agent-execution.svelte.js";
import {
  agentLaunchDraft,
  type AgentLaunchContext,
} from "./agent-execution-types.js";
export async function showAgentLaunch(
  manager: AgentExecution,
  context: AgentLaunchContext = {},
) {
  if (!manager.available()) return;
  manager.context = context;
  manager.launchDraft = agentLaunchDraft(context.commit);
  manager.setupId = "";
  manager.open = true;
  manager.launchOpen = true;
  manager.onNavigate?.("new");
  await manager.refresh();
}

export function prepareAgentDraft(manager: AgentExecution, id: string) {
  manager.context = {};
  manager.launchDraft = { targetId: "", runner: "codex", commit: "" };
  manager.setupId = id;
  manager.onNavigate?.("new", id);
}

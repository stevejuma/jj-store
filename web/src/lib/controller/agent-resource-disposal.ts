import type { AgentExecution } from "./agent-execution.svelte.js";
import { syncAgentPower } from "./agent-power.js";
export function disposeAgentResources(manager: AgentExecution) {
  void syncAgentPower(null, false);
  manager.chat.destroy();
  manager.catalogs.destroy();
  manager.compute.stop();
  manager.tooling.destroy();
  manager.discussions.destroy();
  manager.client.close();
  manager.drafts = {};
  manager.setupDrafts = {};
  manager.settings.dispose();
  manager.pending = null;
}

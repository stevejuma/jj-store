import type { AgentExecution } from "./agent-execution.svelte.js";
import { syncAgentPower } from "./agent-power.js";
export function disposeAgentResources(manager: AgentExecution) {
  void syncAgentPower(null, false);
  manager.chat.destroy();
  manager.changes.destroy();
  manager.metadata.destroy();
  manager.catalogs.destroy();
  manager.compute.destroy();
  manager.tooling.destroy();
  manager.discussions.destroy();
  manager.client.close();
  manager.drafts = {};
  manager.setupDrafts = {};
  manager.settings.dispose();
  manager.pending = null;
}

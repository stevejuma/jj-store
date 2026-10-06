import { getContext, setContext } from "svelte";
import type { AgentExecution } from "../../controller/agent-execution.svelte.js";
const key = Symbol("store-agent-execution");
export function provideAgentExecution(owner: () => AgentExecution | undefined) {
  setContext(key, owner);
}
export function useAgentExecution() {
  return getContext<(() => AgentExecution | undefined) | undefined>(key)?.();
}

import { untrack, onDestroy } from "svelte";
import { observeAgentChats } from "../../web/src/lib/controller/agent-chat-navigation.svelte.js";
import type { AgentExecution } from "../../web/src/lib/controller/agent-execution.svelte.js";
import type { StoreRoute } from "../../web/src/lib/controller/contracts.js";
import { createStoreShell } from "../../web/src/lib/ui/store-application/create-store-shell.js";
import { createInMemoryStoreController } from "../controller/in-memory-store-controller.svelte.js";
import { createMemoryStoryStorage } from "./memory-storage.js";
export function fixtureAgentShell() {
  return {
    shell: createStoreShell(undefined, false),
    inboxStorage: createMemoryStoryStorage(),
  };
}
/** Exercise the actual application observer with route-reactive controller access. */
export function observeFixtureAgentRuntime(
  manager: AgentExecution,
  route: () => StoreRoute,
) {
  const base = untrack(() => createInMemoryStoreController("agents"));
  observeAgentChats(() => ({
    ...base,
    get execution() {
      route();
      return manager;
    },
    get state() {
      return { ...base.state, route: route() };
    },
  }));
}
export function own(
  manager: AgentExecution,
  receipts: { destroy(): void },
  freshness: { destroy(): void },
  shell: ReturnType<typeof createStoreShell>,
) {
  onDestroy(() => {
    manager.destroy();
    receipts.destroy();
    freshness.destroy();
    void shell.dispose();
  });
}

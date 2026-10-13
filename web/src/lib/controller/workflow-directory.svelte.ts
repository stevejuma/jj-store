import { untrack } from "svelte";
import { SvelteMap } from "svelte/reactivity";
import {
  createWorkflows,
  type WorkflowController,
} from "./workflows.svelte.js";
import type { WorkflowTransport } from "./workflow-types.js";
/** Catalog command owners retain original requests while views change. */
export function createWorkflowDirectory(
  homes: () => string[],
  source: (home: string) => WorkflowTransport | undefined,
) {
  const controllers = new SvelteMap<string, WorkflowController>();
  $effect(() => {
    const selected = homes();
    const owners = untrack(() => {
      for (const [repository, owner] of controllers)
        if (!selected.includes(repository)) {
          owner.stop();
          controllers.delete(repository);
        }
      return selected.flatMap((repository) => {
        let owner = controllers.get(repository);
        if (!owner) {
          const transport = source(repository);
          if (!transport) return [];
          owner = createWorkflows(transport);
          controllers.set(repository, owner);
        }
        return [owner];
      });
    });
    const abort = new AbortController();
    const refresh = () => {
      if (!abort.signal.aborted)
        for (const owner of owners) void owner.refresh();
    };
    refresh();
    const timer = setInterval(refresh, 5000);
    window.addEventListener("focus", refresh);
    window.addEventListener("online", refresh);
    async function events(owner: WorkflowController) {
      while (!abort.signal.aborted && owner.transport.events) {
        try {
          await owner.transport.events(
            abort.signal,
            () => void owner.refresh(),
          );
        } catch {}
        if (!abort.signal.aborted)
          await new Promise((resolve) => setTimeout(resolve, 1000));
      }
    }
    for (const owner of owners) void events(owner);
    return () => {
      abort.abort();
      clearInterval(timer);
      window.removeEventListener("focus", refresh);
      window.removeEventListener("online", refresh);
      for (const owner of owners) owner.stop();
    };
  });
  return controllers;
}

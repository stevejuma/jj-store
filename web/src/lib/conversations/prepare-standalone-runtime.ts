import type { AgentExecution } from "../controller/agent-execution.svelte.js";

/** Preparation may open a session, but only the caller dispatches the captured prompt. */
export async function prepareStandaloneRuntime(
  manager: AgentExecution,
  id: string,
  active: () => boolean,
) {
  if (
    !active() ||
    manager.selected !== id ||
    manager.session?.state === "cleaned"
  )
    return false;
  if (manager.session?.connected) return true;
  const request = await manager.commandSession(id, { action: "resume" });
  if (!request) return false;
  return manager.runtimeObservation.wait(() => {
    if (
      !active() ||
      manager.selected !== id ||
      manager.error ||
      manager.connectionError
    )
      return false;
    const outcome = manager.outcome;
    if (
      outcome?.request_id !== request ||
      (manager.pending && manager.pending.body.request_id !== request)
    )
      return false;
    if (!["dispatching", "opening", "accepted"].includes(outcome.state))
      return false;
    if (
      !manager.pending &&
      manager.session?.connected &&
      ["idle", "running"].includes(manager.session.state)
    )
      return true;
    return undefined;
  }, 90000);
}

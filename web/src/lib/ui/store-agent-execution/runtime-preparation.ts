import type { AgentExecution } from "../../controller/agent-execution.svelte.js";
import {
  object,
  type AgentSession,
} from "../../controller/agent-execution-types.js";

export function runtimeMatches(
  current: AgentSession | undefined,
  wanted: AgentSession["preferences"] | undefined,
) {
  return (
    !!current &&
    !!wanted &&
    (["model", "effort", "mode"] as const).every(
      (key) => (current.preferences[key] ?? null) === (wanted[key] ?? null),
    )
  );
}

/** Observe the exact configure receipt; never poll, retry or replace its identity. */
export async function waitForRuntime(
  manager: AgentExecution,
  sessionId: string,
  requestId: string | undefined,
  wanted: AgentSession["preferences"],
  active: () => boolean,
) {
  if (!requestId) return false;
  return manager.runtimeObservation.wait(() => {
    if (
      !active() ||
      manager.selected !== sessionId ||
      !manager.session?.connected
    )
      return false;
    const outcome = manager.outcome;
    if (manager.error || manager.connectionError) return false;
    if (
      outcome?.request_id !== requestId ||
      (manager.pending && manager.pending.body.request_id !== requestId)
    )
      return false;
    if (outcome.state === "accepted") {
      if (
        !manager.pending &&
        object(manager.session.capabilities).modeReady === true &&
        runtimeMatches(manager.session, wanted)
      )
        return true;
    } else if (outcome.state !== "dispatching") return false;
    return undefined;
  }, 10000);
}

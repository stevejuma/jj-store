import type {
  AgentAction,
  AgentSession,
} from "../../web/src/lib/controller/agent-execution-types.js";

/** Simulates the real asynchronous configure receipt, including uncertainty. */
export function commandReceipts(
  session: () => AgentSession,
  uncertain: boolean,
  settled: () => void,
) {
  const configuring = new Map<
    string,
    { preferences: AgentSession["preferences"]; ready: boolean }
  >();
  const timers = new Set<ReturnType<typeof setTimeout>>();
  return {
    dispatch(
      body: { request_id: string; action?: AgentAction },
      resumeFailure: boolean,
    ) {
      const action = body.action;
      if (action?.action === "configure") {
        configuring.set(body.request_id, {
          preferences: {
            model: action.preferences.model ?? null,
            effort: action.preferences.effort ?? null,
            mode: action.preferences.mode ?? null,
          },
          ready: false,
        });
        const timer = setTimeout(() => {
          timers.delete(timer);
          const saved = configuring.get(body.request_id)!;
          if (!uncertain) {
            saved.ready = true;
            session().preferences = saved.preferences;
            (session().capabilities as { modeReady: boolean }).modeReady = true;
          }
          settled();
        }, 200);
        timers.add(timer);
      }
      return {
        version: 1,
        request_id: body.request_id,
        session_id: session().id,
        state:
          action?.action === "configure"
            ? "dispatching"
            : resumeFailure && action?.action === "resume"
              ? "failed"
              : "queued",
        error:
          resumeFailure && action?.action === "resume"
            ? "agent_rejected"
            : null,
      };
    },
    outcome(request: string) {
      const saved = configuring.get(request);
      return {
        version: 1,
        request_id: request,
        session_id: session().id,
        state: saved
          ? uncertain
            ? "outcome_unknown"
            : saved.ready
              ? "accepted"
              : "dispatching"
          : "outcome_unknown",
        error: uncertain || !saved ? "connection_lost" : null,
      };
    },
    destroy() {
      for (const timer of timers) clearTimeout(timer);
    },
  };
}

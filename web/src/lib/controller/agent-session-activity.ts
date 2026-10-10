import {
  object,
  text,
  type AgentEvent,
  type AgentAction,
  type AgentSession,
  type AgentOutcome,
} from "./agent-execution-types.js";

export type PendingAgentRequest = { body: Record<string, unknown> } | null;

/** Participation state outranks an old dispatch receipt; unrelated requests stay scoped. */
export function sessionActivity(
  session?: AgentSession,
  pending: PendingAgentRequest = null,
  outcome?: AgentOutcome | null,
) {
  const matches =
    !!pending && (!session || pending.body.session_id === session.id);
  if (matches) {
    const state =
      outcome?.request_id === pending.body.request_id
        ? (outcome?.state ?? "checking")
        : "checking";
    if (state === "outcome_unknown" || state === "not_known")
      return {
        state,
        label:
          state === "outcome_unknown" ? "Outcome unknown" : "Request not found",
        streaming: false,
      };
    if (session && ["interrupted", "ended", "cleaned"].includes(session.state))
      return {
        state: session.state,
        label: session.state === "interrupted" ? "Interrupted" : "Stopped",
        streaming: false,
      };
    if (!outcome || ["dispatching", "checking"].includes(state))
      return { state: "starting", label: "Starting", streaming: true };
  }
  if (session?.connected && session.state === "running")
    return { state: "running", label: "Working", streaming: true };
  if (session && ["preparing", "opening"].includes(session.state))
    return { state: "starting", label: "Starting", streaming: true };
}

/** Never cancel a historical turn or a handshake's synthetic request identity. */
export function sessionStopAction(
  session: AgentSession,
  events: readonly AgentEvent[],
): AgentAction | undefined {
  if (session.state === "opening") return { action: "end" };
  if (!session.connected) return;
  if (["opening", "preparing"].includes(session.state))
    return { action: "end" };
  const terminal = new Set<string>();
  let queued: AgentAction | undefined;
  for (const event of [...events].reverse()) {
    const frame = object(event.payload),
      request = text(frame.requestId),
      kind = text(frame.type);
    if (["opened", "continuation"].includes(kind)) break;
    if (["record", "error"].includes(kind)) terminal.add(request);
    if (session.state === "idle" && kind === "started") terminal.add(request);
    if (/^[a-f0-9]{32}$/.test(request) && !terminal.has(request)) {
      if (kind === "started" && session.state === "running")
        return { action: "cancel", target: request };
      if (["queued", "prompt"].includes(kind))
        queued = { action: "cancel", target: request };
    }
  }
  if (queued) return queued;
  if (session.state === "running") return { action: "end" };
}

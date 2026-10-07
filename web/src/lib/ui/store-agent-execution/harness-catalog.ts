import {
  object,
  type AgentSession,
} from "../../controller/agent-execution-types.js";

/** Live negotiation takes precedence; a stopped session retains remembered choices. */
export function harnessCatalog(
  sessions: readonly AgentSession[],
  runner: string,
  discovered?: unknown,
) {
  const matching = sessions.filter((session) => session.runner === runner);
  const session =
    matching.find((session) => session.connected) ??
    matching
      .filter(
        (session) =>
          session.adapter_session_id &&
          Object.keys(object(object(session.capabilities).session)).length,
      )
      .sort(
        (a, b) =>
          Number(b.last_activity_at_millis ?? 0) -
          Number(a.last_activity_at_millis ?? 0),
      )[0];
  return {
    session,
    facts:
      session?.connected || !discovered
        ? object(object(session?.capabilities).session)
        : object(discovered),
    remembered: !!session && !session.connected && !discovered,
  };
}

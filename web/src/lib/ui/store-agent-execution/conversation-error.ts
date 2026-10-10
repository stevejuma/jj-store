import {
  object,
  text,
  type AgentEvent,
  type AgentSession,
} from "../../controller/agent-execution-types.js";

/** Public ACP codes only: raw provider diagnostic data stays behind ACP sanitization. */
export function conversationError(
  session: AgentSession,
  events: readonly AgentEvent[],
) {
  if (!session.last_error || session.connected) return "";
  const latest = object(
    [...events]
      .reverse()
      .find((event) =>
        [
          "error",
          "opened",
          "started",
          "record",
          "closed",
          "continuation",
        ].includes(text(object(event.payload).type)),
      )?.payload,
  );
  const code =
    !["opening", "preparing"].includes(session.state) && latest.type === "error"
      ? text(latest.code)
      : "";
  const error = code || session.last_error;
  return `${error === "agent_rejected" ? "The harness rejected this session request" : error.replaceAll("_", " ")} (${error}). Workspace, saved history and your draft are retained. Send prepares the selected agent automatically. Review any uncertain previous work before sending.`;
}

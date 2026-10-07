import { ApiError } from "../api/client.js";
const refused = new Set([
  "invalid_agent_execution",
  "agent_selection_changed",
  "agent_execution_busy",
  "agent_execution_denied",
  "agent_request_identity_conflict",
  "agent_execution_limit",
  "agent_feedback_limit",
  "agent_feedback_unavailable",
]);
export function isAgentCommandRefusal(cause: unknown): cause is ApiError {
  return cause instanceof ApiError && refused.has(cause.code);
}
export function agentCommandError(cause: ApiError): string {
  if (cause.code === "agent_feedback_limit")
    return "Discussion context exceeds 128 replies or 32 KiB. Shorten or remove eligible discussion before sending again. Your prompt was not sent.";
  if (cause.code === "agent_feedback_unavailable")
    return "Could not read repository discussion. Retry Send after the repository is available. Your prompt was not sent.";
  return cause.message;
}

export function retainUnknownAgentOutcome(owner: {
  outcome: { state: string } | null;
  pending: unknown;
  error: string;
}) {
  if (owner.outcome?.state === "outcome_unknown") {
    owner.pending = null;
    owner.error =
      "The interrupted request remains in session history. Review the workspace before explicitly continuing.";
  }
}

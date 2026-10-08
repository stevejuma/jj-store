import type { AgentExecution } from "../../controller/agent-execution.svelte.js";
import type { NotificationInput } from "../../notifications/notification-service.svelte.js";

type Owner = Pick<
  AgentExecution,
  | "pending"
  | "outcome"
  | "busy"
  | "error"
  | "retainAsUnresolved"
  | "retryRetainedRequest"
  | "openSession"
>;

/** Recovery callbacks stay fenced to the exact request and observed outcome. */
export function requestRecoveryNotice(
  owner: Owner,
): NotificationInput | undefined {
  const pending = owner.pending;
  const state = owner.outcome?.state;
  if (
    !pending ||
    owner.outcome?.request_id !== pending.body.request_id ||
    !["outcome_unknown", "not_known"].includes(state ?? "")
  )
    return;
  const unknown = state === "outcome_unknown";
  const request = String(pending.body.request_id);
  const detail = owner.error || owner.outcome?.error;
  return {
    scope: "agent-execution",
    key: request,
    title: unknown ? "Agent request unresolved" : "Agent request not found",
    message: [
      detail,
      unknown
        ? "Your draft, workspace and request history are retained. Keep this request unresolved and review the workspace before explicitly continuing."
        : "Your draft and original request are retained. Submit that exact request only when you are ready to try again.",
    ]
      .filter(Boolean)
      .join(" "),
    severity: "warning",
    lifetime: "condition",
    action: {
      id: `${unknown ? "retain" : "submit"}:${request}`,
      label: unknown ? "Review workspace" : "Submit retained request",
      run: () => {
        if (
          owner.busy ||
          owner.pending?.body.request_id !== request ||
          owner.outcome?.request_id !== request ||
          owner.outcome?.state !== state
        )
          return;
        if (unknown) {
          owner.retainAsUnresolved();
          if (typeof pending.body.session_id === "string")
            owner.openSession(pending.body.session_id);
        } else void owner.retryRetainedRequest();
      },
    },
  };
}

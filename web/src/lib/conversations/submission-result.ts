import type { Model } from "../api/client.js";
import type { ChatSubmissionResult } from "../ui/store-agent-chat-body/model.js";
/** Publication does not imply delivery. Rechecks use the original receipt IDs. */
export function conversationSubmissionResult(
  receipt: Model<"AgentConversationOutcome">,
  names: ReadonlyMap<string, string> = new Map(),
): ChatSubmissionResult {
  const delivery = receipt.agents.map((a) => ({
    id: a.request_id,
    label: names.get(a.session_id ?? "") ?? a.session_id ?? a.request_id,
    state: a.state,
    error: a.error,
  }));
  const accepted = delivery.every((a) =>
    ["queued", "started", "running", "completed"].includes(a.state),
  );
  const publication = receipt.comment_publication?.state;
  const refusedBeforeDispatch =
    !publication &&
    delivery.every(
      (a) =>
        a.state === "failed" &&
        [
          "queue_full",
          "dispatch_not_sent",
          "workspace_start_snapshot_unavailable",
          "chat_preparation_failed_no_prompt",
          "chat_context_preparation_failed_no_prompt",
        ].includes(a.error ?? ""),
    );
  return {
    publication:
      publication === "published"
        ? "published"
        : publication === "refused"
          ? "refused"
          : refusedBeforeDispatch
            ? "refused"
            : accepted && publication === undefined
              ? "published"
              : "unknown",
    batchId: receipt.comment_publication?.request_id,
    delivery,
    recoveryRequired:
      !accepted || (publication !== undefined && publication !== "published"),
    error: accepted
      ? undefined
      : (delivery.find((a) => a.error)?.error ??
        "Delivery is awaiting confirmation. Check the retained request."),
  };
}

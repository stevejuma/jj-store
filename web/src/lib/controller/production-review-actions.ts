import type {
  StoreDataSource,
  StoreReviewCommandRequest,
} from "./native-data-source.js";
import type { Model } from "../api/client.js";
import type { ProductionStoreState } from "./production-store-state.svelte.js";

interface Context {
  data: ProductionStoreState;
  source(): StoreDataSource;
  report(cause: unknown): void;
  requestId?(): string;
  reloadReviews(): Promise<void>;
  reloadReview(id: string): Promise<void>;
  selectReview(id: string): void;
}

const defaultRequestId = () =>
  [...crypto.getRandomValues(new Uint8Array(16))]
    .map((value) => value.toString(16).padStart(2, "0"))
    .join("");

export function createProductionReviewActions(context: Context) {
  const { data } = context;

  async function apply(command: StoreReviewCommandRequest["command"]) {
    const requestId = (context.requestId ?? defaultRequestId)();
    data.status.reviewAction = "refreshing";
    data.reviewActionMessage = "";
    data.reviewActionRequestId = requestId;
    try {
      const outcome = await context.source().applyReviewCommand({
        version: 1,
        request_id: requestId,
        command,
      });
      if (outcome.status === "outcome_unknown") {
        data.status.reviewAction = "unavailable";
        data.reviewActionMessage =
          "The review outcome is unknown. Check the durable request status before retrying.";
        return;
      }
      data.reviewActionRequestId = null;
      data.status.reviewAction = "ready";
      context.selectReview(outcome.receipt.result.thread_id);
      await context.reloadReviews();
      await context.reloadReview(outcome.receipt.result.thread_id);
    } catch (cause) {
      data.status.reviewAction = "failed";
      context.report(cause);
    }
  }

  async function create(anchor: Model<"ArtifactAnchorV1">, text: string) {
    if (!text.trim()) return;
    await apply({
      command: "create_thread",
      anchor,
      body: {
        schema_version: 1,
        format: "safe_markdown",
        text: text.trim(),
        sanitizer_version: "review-markdown-v1",
      },
    });
  }

  async function reply(text: string, parentCommentId?: string) {
    const detail = data.reviewThread;
    if (!detail || !text.trim()) return;
    await apply({
      command: "append_reply",
      thread_id: detail.thread.thread_id,
      expected_workflow_version: detail.thread.workflow_version,
      parent_comment_id: parentCommentId ?? null,
      body: {
        schema_version: 1,
        format: "safe_markdown",
        text: text.trim(),
        sanitizer_version: "review-markdown-v1",
      },
    });
  }

  async function setResolved(resolved: boolean) {
    const detail = data.reviewThread;
    if (!detail) return;
    await apply({
      command: resolved ? "resolve_thread" : "reopen_thread",
      thread_id: detail.thread.thread_id,
      expected_thread_version: detail.thread.snapshot_id,
      reason: null,
    });
  }

  async function checkAction() {
    const id = data.reviewActionRequestId;
    if (!id) return;
    const status = await context.source().reviewCommandStatus(id);
    if (status.status !== "committed") return;
    data.reviewActionRequestId = null;
    data.reviewActionMessage = "Review update committed.";
    data.status.reviewAction = "ready";
    context.selectReview(status.receipt.result.thread_id);
    await context.reloadReviews();
    await context.reloadReview(status.receipt.result.thread_id);
  }

  return { create, reply, setResolved, checkAction };
}

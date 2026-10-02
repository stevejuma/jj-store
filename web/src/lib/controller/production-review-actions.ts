import type {
  StoreDataSource,
  StoreReviewCommandRequest,
} from "./native-data-source.js";
import { ApiError, type Model } from "../api/client.js";
import type { ProductionStoreState } from "./production-store-state.svelte.js";

interface Context {
  data: ProductionStoreState;
  source(): StoreDataSource;
  report(cause: unknown): void;
  version(): number;
  current(ticket: number, source: StoreDataSource): boolean;
  selectedReviewId(): string | null;
  requestId?(): string;
  reloadReviews(): Promise<void>;
  reloadReview(id: string): Promise<void>;
  selectReview(id: string): void;
}

const defaultRequestId = () =>
  [...crypto.getRandomValues(new Uint8Array(16))]
    .map((value) => value.toString(16).padStart(2, "0"))
    .join("");

function result(
  receipt: Extract<
    import("./repository-read-types.js").ReviewOutcome,
    { status: "committed" }
  >["receipt"],
) {
  const value = "result" in receipt ? receipt.result : receipt.command;
  if (!value) throw new ApiError("snapshot_mismatch");
  return value;
}

export function createProductionReviewActions(context: Context) {
  const { data } = context;

  async function apply(command: StoreReviewCommandRequest["command"]) {
    if (data.reviewActionRequestId || data.status.reviewAction === "refreshing")
      return;
    const source = context.source(),
      ticket = context.version(),
      selectedId = context.selectedReviewId();
    const current = () => context.current(ticket, source);
    const requestId = (context.requestId ?? defaultRequestId)();
    data.status.reviewAction = "refreshing";
    data.reviewActionMessage = "";
    data.reviewActionRequestId = requestId;
    try {
      const outcome = await source.applyReviewCommand({
        version: 1,
        request_id: requestId,
        command,
      });
      if (!current()) return;
      if (outcome.status !== "committed") {
        data.status.reviewAction = "unavailable";
        data.reviewActionMessage =
          "The review outcome is unknown. Check the durable request status before retrying.";
        return;
      }
      data.reviewActionRequestId = null;
      data.committedReviewRequestId = requestId;
      await context.reloadReviews();
      if (!current() || context.selectedReviewId() !== selectedId) return;
      context.selectReview(result(outcome.receipt).thread_id);
      await context.reloadReview(result(outcome.receipt).thread_id);
    } catch (cause) {
      if (!current()) return;
      // The server explicitly rejects these commands without publication.
      if (
        cause instanceof ApiError &&
        [
          "stale_review",
          "stale_local_metadata",
          "local_agent_not_found",
          "unsupported_local_capability",
          "intent_conflict",
          "thread_resolved",
          "thread_open",
          "not_found",
          "invalid_request",
        ].includes(cause.code)
      )
        data.reviewActionRequestId = null;
      data.status.reviewAction = "failed";
      data.reviewActionMessage =
        cause instanceof Error ? cause.message : "Review update failed.";
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

  async function edit(
    commentId: string,
    expectedRevisionId: string,
    text: string,
  ) {
    const detail = data.reviewThread;
    const comment = detail?.comments.find(
      (row) => row.comment_id === commentId,
    );
    const repository = data.repositories.find(
      (row) => row.id === context.source().repository,
    );
    if (
      !comment ||
      comment.content.state !== "visible" ||
      !text.trim() ||
      repository?.principal !== comment.author ||
      (repository.permission !== "write" && repository.permission !== "admin")
    )
      return;
    await apply({
      command: "edit_comment",
      comment_id: commentId,
      expected_revision_id: expectedRevisionId,
      replacement_body: {
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
    const source = context.source(),
      ticket = context.version(),
      selectedId = context.selectedReviewId();
    const status = await source.reviewCommandStatus(id);
    if (!context.current(ticket, source) || id !== data.reviewActionRequestId)
      return;
    if (status.status !== "committed") return;
    data.reviewActionRequestId = null;
    data.committedReviewRequestId = id;
    data.reviewActionMessage = "Review update committed.";
    data.status.reviewAction = "refreshing";
    await context.reloadReviews();
    if (
      !context.current(ticket, source) ||
      context.selectedReviewId() !== selectedId
    )
      return;
    context.selectReview(result(status.receipt).thread_id);
    await context.reloadReview(result(status.receipt).thread_id);
  }

  const createRepositoryReview = (text: string) =>
    create(
      {
        schema_version: 1,
        repository_id: context.source().repository,
        target: { target: "repository" },
        created_context: {
          publication_position: null,
          code_state_version: null,
          agent_root: null,
        },
        sensitivity: "repository",
      },
      text,
    );
  return {
    create,
    createRepositoryReview,
    reply,
    edit,
    setResolved,
    checkAction,
  };
}

import type {
  StoreController,
  StoreLoadStatus,
  StoreRoute,
} from "../../web/src/lib/controller/contracts.js";
import {
  agentDetail,
  agentSessions,
  agentTranscript,
  reviewDetail,
  reviewThreads,
} from "../fixtures/collaboration.js";

export function createInMemoryStoreCollaboration(options: {
  route(): StoreRoute;
  push(route: StoreRoute): void;
  now: number;
  outcome?: "committed" | "unknown";
}) {
  let selectedAgent = $state(agentDetail);
  let selectedReview = $state(reviewDetail());
  let reviewActionMessage = $state("");
  let reviewActionRequestId = $state<string | null>(null);
  let reviewActionStatus = $state<StoreLoadStatus>("ready");
  let pendingResolved = false;
  const actions: Pick<
    StoreController["actions"],
    | "selectAgentSession"
    | "selectReviewThread"
    | "replyToReview"
    | "setReviewResolved"
    | "checkReviewAction"
  > = {
    selectAgentSession(id) {
      const session = agentSessions.find(
        (candidate) => candidate.session_id === id,
      );
      if (id && !session) throw new Error("Unknown fixture agent session");
      selectedAgent = session ? { ...agentDetail, session } : agentDetail;
      options.push({ ...options.route(), agentSessionId: id });
    },
    selectReviewThread(id) {
      const thread = reviewThreads.find(
        (candidate) => candidate.thread_id === id,
      );
      if (id && !thread) throw new Error("Unknown fixture review thread");
      selectedReview = { ...reviewDetail(), ...(thread ? { thread } : {}) };
      options.push({ ...options.route(), reviewThreadId: id });
    },
    async replyToReview(text) {
      if (!text.trim()) return;
      const index = selectedReview.comments.length + 1;
      const byte = index.toString(16).padStart(2, "0");
      selectedReview = {
        ...selectedReview,
        thread: {
          ...selectedReview.thread,
          comment_count: index,
          content_version: Number(selectedReview.thread.content_version) + 1,
          workflow_version: Number(selectedReview.thread.workflow_version) + 1,
        },
        comments: [
          ...selectedReview.comments,
          {
            comment_id: byte.repeat(16),
            revision_id: byte.repeat(32),
            parent_comment_id: null,
            author: "20".repeat(16),
            actor: "20".repeat(16),
            created_at_millis: options.now * 1000,
            previous_revision_id: null,
            content: {
              state: "visible",
              body_id: "21".repeat(32),
              format: "safe_markdown",
              text: text.trim(),
              sanitizer_version: "review-markdown-v1",
            },
          },
        ],
      };
      reviewActionMessage = "Review update committed.";
    },
    async setReviewResolved(resolved) {
      if (options.outcome === "unknown") {
        pendingResolved = resolved;
        reviewActionRequestId = "31".repeat(16);
        reviewActionMessage =
          "The review outcome is unknown. Check the durable request status before retrying.";
        reviewActionStatus = "unavailable";
        return;
      }
      selectedReview = {
        ...selectedReview,
        thread: {
          ...selectedReview.thread,
          status: resolved ? "resolved" : "open",
          workflow_version: Number(selectedReview.thread.workflow_version) + 1,
        },
      };
      reviewActionMessage = "Review update committed.";
    },
    async checkReviewAction() {
      selectedReview = {
        ...selectedReview,
        thread: {
          ...selectedReview.thread,
          status: pendingResolved ? "resolved" : "open",
        },
      };
      reviewActionRequestId = null;
      reviewActionMessage = "Review update committed.";
      reviewActionStatus = "ready";
    },
  };
  return {
    actions,
    get view() {
      const route = options.route();
      return {
        agentSessions,
        agentSession: route.agentSessionId ? selectedAgent : undefined,
        agentTranscript: route.agentSessionId ? agentTranscript : [],
        reviewThreads,
        reviewThread: route.reviewThreadId ? selectedReview : undefined,
        reviewActionMessage,
        reviewActionRequestId,
      };
    },
    get reviewActionStatus() {
      return reviewActionStatus;
    },
  };
}

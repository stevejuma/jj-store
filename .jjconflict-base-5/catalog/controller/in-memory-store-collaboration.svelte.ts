import { createFixtureReviewCommentActions } from "./in-memory-review-comments.js";
import type {
  StoreController,
  StoreLoadStatus,
  StoreRoute,
} from "../../web/src/lib/controller/contracts.js";
import type { ReviewInlinePatch } from "../../web/src/lib/controller/review-anchor-patch.js";
import {
  anchorMatchesOperation,
  anchorMatchesPath,
} from "../../web/src/lib/collaboration/presentation.js";
import {
  agentDetail,
  agentRelations,
  agentSessions,
  agentTranscript,
  reviewDetail,
  reviewPatch,
  reviewThreads,
} from "../fixtures/collaboration.js";
import { appendFixtureReviewComment } from "./fixture-review-edit.js";
import { nativeReviewContext } from "../fixtures/native-activity.js";

export function createInMemoryStoreCollaboration(options: {
  route(): StoreRoute;
  push(route: StoreRoute): void;
  now: number;
  outcome?: "committed" | "unknown";
}) {
  let selectedAgent = $state(agentDetail);
  let selectedReview = $state(reviewDetail());
  let knownReviewThreads = $state([...reviewThreads]);
  const retainedReviews = new Map<string, ReturnType<typeof reviewDetail>>();
  let selectedReviewPatch = $state<ReviewInlinePatch>(reviewPatch);
  let reviewActionMessage = $state("");
  let reviewActionRequestId = $state<string | null>(null);
  let reviewActionStatus = $state<StoreLoadStatus>("ready");
  let committedReviewRequestId = $state<string | null>(null);
  let commentRequest = 64;
  let pendingResolved = false;
  let pendingEdit: (() => void) | null = null;
  function selectReviewDetail(
    thread: (typeof reviewThreads)[number] | undefined,
  ) {
    selectedReview = (thread && retainedReviews.get(thread.thread_id)) ?? {
      ...reviewDetail(),
      ...(thread
        ? {
            thread,
            anchor: {
              object_id: thread.original_anchor_id,
              value: thread.anchor,
            },
          }
        : {}),
    };
  }
  function selectedContext(
    relation: (typeof agentRelations)[number] | undefined,
    thread: (typeof reviewThreads)[number] | undefined,
  ) {
    selectReviewDetail(thread);
    return {
      agentSessionId: relation?.association?.session_id ?? null,
      reviewThreadId: thread?.thread_id ?? null,
    };
  }
  function selectPathReview(
    matches: (thread: (typeof reviewThreads)[number]) => boolean,
  ) {
    const thread = knownReviewThreads.find(matches);
    selectReviewDetail(thread);
    return thread?.thread_id ?? null;
  }
  function publishReview(update: () => void, request?: string) {
    request ??= (++commentRequest).toString(16).padStart(2, "0").repeat(16);
    if (options.outcome === "unknown") {
      pendingEdit = update;
      reviewActionRequestId = request;
      reviewActionStatus = "unavailable";
      reviewActionMessage =
        "The review outcome is unknown. Check the durable request status before retrying.";
    } else {
      update();
      committedReviewRequestId = request;
      reviewActionStatus = "ready";
      reviewActionMessage = "Review update committed.";
    }
  }
  async function appendReview(text: string, parentCommentId?: string) {
    if (!text.trim() || reviewActionRequestId) return;
    const next = appendFixtureReviewComment(
      selectedReview,
      text,
      options.now,
      parentCommentId,
    );
    publishReview(() => {
      selectedReview = next;
      retainReview();
    });
  }
  function retainReview() {
    retainedReviews.set(selectedReview.thread.thread_id, selectedReview);
    knownReviewThreads = knownReviewThreads.map((thread) =>
      thread.thread_id === selectedReview.thread.thread_id
        ? selectedReview.thread
        : thread,
    );
  }
  const actions: Pick<
    StoreController["actions"],
    | "selectAgentSession"
    | "selectReviewThread"
    | "replyToReview"
    | "editReviewComment"
    | "commentOnDiffLine"
    | "commentOnAgentTool"
    | "readAgentToolReview"
    | "readAgentSessionReview"
    | "commentOnAgentSession"
    | "setReviewResolved"
    | "checkReviewAction"
  > = {
    selectAgentSession(id) {
      const session = agentSessions.find(
        (candidate) => candidate.session_id === id,
      );
      if (id && !session) throw new Error("Unknown fixture agent session");
      selectedAgent = session ? { ...agentDetail, session } : agentDetail;
      options.push({
        ...options.route(),
        agentSessionId: id,
        reviewThreadId: null,
      });
    },
    selectReviewThread(id) {
      const thread = knownReviewThreads.find(
        (candidate) => candidate.thread_id === id,
      );
      if (id && !thread) throw new Error("Unknown fixture review thread");
      selectReviewDetail(thread);
      options.push({ ...options.route(), reviewThreadId: id });
    },
    async replyToReview(text, parentCommentId) {
      await appendReview(text, parentCommentId);
    },
    ...createFixtureReviewCommentActions({
      session: () => selectedAgent,
      threads: () => knownReviewThreads,
      setThreads: (value) => {
        knownReviewThreads = value;
      },
      detail: () => selectedReview,
      setDetail: (value) => {
        selectedReview = value;
      },
      selectReview: selectReviewDetail,
      retainReview,
      readDetail: (id) => retainedReviews.get(id),
      selectThread: (id) =>
        options.push({ ...options.route(), reviewThreadId: id }),
      appendReview,
      route: options.route,
      patch: (value) => {
        selectedReviewPatch = value;
      },
      publish: publishReview,
      fail: (message) => {
        reviewActionStatus = "failed";
        reviewActionMessage = message;
      },
      pending: () => !!reviewActionRequestId,
    }),
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
      retainReview();
      reviewActionMessage = "Review update committed.";
    },
    async checkReviewAction() {
      if (pendingEdit) {
        pendingEdit();
        pendingEdit = null;
      } else
        selectedReview = {
          ...selectedReview,
          thread: {
            ...selectedReview.thread,
            status: pendingResolved ? "resolved" : "open",
          },
        };
      committedReviewRequestId = reviewActionRequestId;
      reviewActionRequestId = null;
      reviewActionMessage = "Review update committed.";
      reviewActionStatus = "ready";
      retainReview();
    },
  };
  return {
    actions,
    get view() {
      const route = options.route();
      return {
        agentSessions,
        agentSession: route.agentSessionId ? selectedAgent : undefined,
        agentRelations,
        agentTranscript: route.agentSessionId ? agentTranscript : [],
        reviewThreads: knownReviewThreads,
        reviewThread: route.reviewThreadId ? selectedReview : undefined,
        reviewContext: route.reviewThreadId
          ? nativeReviewContext(selectedReview.anchor.value)
          : undefined,
        reviewPatch:
          route.reviewThreadId &&
          (selectedReview.anchor.value.target.target === "code_text" ||
            selectedReview.anchor.value.target.target === "code_artifact")
            ? selectedReviewPatch
            : undefined,
        reviewActionMessage,
        reviewActionRequestId,
        committedReviewRequestId,
      };
    },
    get reviewActionStatus() {
      return reviewActionStatus;
    },
    revisionContext(commit: string) {
      const relation = agentRelations.find((candidate) =>
        candidate.association?.after_commits.includes(commit),
      );
      const thread = knownReviewThreads.find((candidate) => {
        const target = candidate.anchor.target;
        return (
          (target.target === "revision" ||
            target.target === "code_text" ||
            target.target === "code_artifact") &&
          target.value.commit_id === commit
        );
      });
      return selectedContext(relation, thread);
    },
    operationContext(operation: string) {
      const relation = agentRelations.find((candidate) => {
        const association = candidate.association;
        return (
          association?.before_operations.includes(operation) ||
          association?.after_operations.includes(operation)
        );
      });
      const thread = knownReviewThreads.find((candidate) =>
        anchorMatchesOperation(candidate.anchor, operation),
      );
      return selectedContext(relation, thread);
    },
    pathReview(commit: string, path: string) {
      return selectPathReview((candidate) => {
        const target = candidate.anchor.target;
        return (
          (target.target === "code_text" ||
            target.target === "code_artifact") &&
          target.value.commit_id === commit &&
          target.value.path === path
        );
      });
    },
    operationPathReview(operation: string, path: string) {
      return selectPathReview(
        (candidate) =>
          anchorMatchesOperation(candidate.anchor, operation) &&
          anchorMatchesPath(candidate.anchor, path),
      );
    },
  };
}

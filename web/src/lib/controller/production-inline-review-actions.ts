import type { FileDiffLineContext } from "@lapismd/design-core/diff";
import {
  anchorAgentTool,
  reviewForAgentTool,
  reviewPatchMatchesLine,
} from "../collaboration/presentation.js";
import type { StoreDataSource } from "./native-data-source.js";
import {
  createAgentToolReviewAnchor,
  createCodeLineReviewAnchor,
} from "./review-comment-anchor.js";
import type { createProductionReviewActions } from "./production-review-actions.js";
import type { ProductionStoreState } from "./production-store-state.svelte.js";
import type { createStoreRouteHistory } from "./store-route-history.svelte.js";

interface Context {
  data: ProductionStoreState;
  history: ReturnType<typeof createStoreRouteHistory>;
  source(): StoreDataSource;
  loadReview(id: string): Promise<void>;
  reviewActions: ReturnType<typeof createProductionReviewActions>;
}

export function createProductionInlineReviewActions(context: Context) {
  const { data, history, reviewActions } = context;

  async function commentOnDiffLine(line: FileDiffLineContext, text: string) {
    if (!text.trim()) return;
    if (data.reviewThread && reviewPatchMatchesLine(data.reviewPatch, line)) {
      await reviewActions.reply(text);
      return;
    }
    const anchor = await createCodeLineReviewAnchor(
      context.source(),
      data,
      line,
    );
    if (!anchor) {
      data.status.reviewAction = "failed";
      data.reviewActionMessage =
        "This diff line no longer resolves to one exact native file object.";
      return;
    }
    await reviewActions.create(anchor, text);
  }

  async function commentOnAgentTool(toolCallId: string, text: string) {
    if (!text.trim()) return;
    const sessionId = data.agentSession?.session.session_id;
    if (!sessionId) return;
    const selectedOwner = data.reviewThread
      ? anchorAgentTool(data.reviewThread.anchor.value)
      : undefined;
    if (
      selectedOwner?.sessionId === sessionId &&
      selectedOwner.toolCallId === toolCallId
    ) {
      await reviewActions.reply(text);
      return;
    }
    const existing = reviewForAgentTool(
      data.reviewThreads,
      sessionId,
      toolCallId,
    );
    if (existing) {
      if (data.reviewThread?.thread.thread_id !== existing.thread_id) {
        history.replace({
          ...history.route,
          reviewThreadId: existing.thread_id,
        });
        await context.loadReview(existing.thread_id);
      }
      await reviewActions.reply(text);
      return;
    }
    const anchor = createAgentToolReviewAnchor(
      context.source(),
      data,
      toolCallId,
    );
    if (!anchor) {
      data.status.reviewAction = "failed";
      data.reviewActionMessage =
        "This tool call no longer resolves to one exact captured snapshot.";
      return;
    }
    await reviewActions.create(anchor, text);
  }

  return { commentOnDiffLine, commentOnAgentTool };
}

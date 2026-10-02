import type { FileDiffLineContext } from "@lapismd/design-core/diff";
import {
  anchorAgentTool,
  reviewPatchMatchesLine,
} from "../collaboration/presentation.js";
import type { StoreDataSource } from "./native-data-source.js";
import {
  createAgentToolReviewAnchor,
  createAgentSessionReviewAnchor,
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
  readAgentToolReview(
    toolCallId: string,
  ): Promise<
    import("../api/client.js").Model<"ReviewThreadDetail"> | undefined
  >;
  readAgentSessionReview(): Promise<
    import("../api/client.js").Model<"ReviewThreadDetail"> | undefined
  >;
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
    const existing = (await context.readAgentToolReview(toolCallId))?.thread;
    if (data.agentSession?.session.session_id !== sessionId) return;
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

  async function commentOnAgentSession(text: string) {
    const sessionId = data.agentSession?.session.session_id;
    const source = context.source();
    if (!text.trim() || !sessionId || source.profile === "git_sha1_jj045")
      return;
    const repository = data.repositories.find(
      (row) => row.id === source.repository,
    );
    if (
      repository?.permission !== "write" &&
      repository?.permission !== "admin"
    )
      return;
    const existing = await context.readAgentSessionReview();
    if (
      data.agentSession?.session.session_id !== sessionId ||
      context.source() !== source
    )
      return;
    if (existing) {
      history.replace({
        ...history.route,
        reviewThreadId: existing.thread.thread_id,
      });
      await context.loadReview(existing.thread.thread_id);
      if (data.agentSession?.session.session_id !== sessionId) return;
      await reviewActions.reply(text);
      return;
    }
    const anchor = createAgentSessionReviewAnchor(source, data);
    if (anchor) await reviewActions.create(anchor, text);
  }
  return { commentOnDiffLine, commentOnAgentTool, commentOnAgentSession };
}

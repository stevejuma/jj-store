import type { Model } from "../api/client.js";
import type { StoreDataSource } from "./native-data-source.js";
import type { ProductionStoreState } from "./production-store-state.svelte.js";
import { anchorAgentTool } from "../collaboration/presentation.js";
import { readReviewContext } from "./production-collaboration-context.js";

export function createAgentReviewReader(context: {
  data: ProductionStoreState;
  source(): StoreDataSource;
  version(): number;
  current(ticket: number, source: StoreDataSource): boolean;
}) {
  const { data } = context;
  async function readAgentReview(
    matches: (anchor: Model<"ArtifactAnchorV1">) => boolean,
  ) {
    const source = context.source(),
      ticket = context.version();
    const sessionId = data.agentSession?.session.session_id;
    if (!sessionId || source.profile === "git_sha1_jj045") return;
    const reviews = await readReviewContext(
      source,
      (thread) => matches(thread.anchor),
      data.reviewThreads,
      data.reviewCursor,
    );
    const current = () =>
      context.current(ticket, source) &&
      data.agentSession?.session.session_id === sessionId;
    if (!current() || !reviews.selected) return;
    const detail = await source.reviewThread(reviews.selected.thread_id);
    if (!current()) return;
    if (
      detail.snapshot.repo_id !== source.repository ||
      detail.thread.thread_id !== reviews.selected.thread_id ||
      !matches(detail.anchor.value)
    )
      throw new Error(
        "Comments do not match the selected captured agent target.",
      );
    return detail;
  }
  async function readAgentToolReview(toolCallId: string) {
    const sessionId = data.agentSession?.session.session_id;
    return readAgentReview((anchor) => {
      const owner = anchorAgentTool(anchor);
      return owner?.sessionId === sessionId && owner?.toolCallId === toolCallId;
    });
  }
  async function readAgentSessionReview() {
    const sessionId = data.agentSession?.session.session_id;
    return readAgentReview(
      (anchor) =>
        anchor.target.target === "agent_session" &&
        anchor.target.value.session_id === sessionId,
    );
  }

  return { readAgentToolReview, readAgentSessionReview };
}

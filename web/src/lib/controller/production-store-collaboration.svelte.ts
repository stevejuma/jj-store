import type { StoreDataSource } from "./native-data-source.js";
import type { ProductionStoreState } from "./production-store-state.svelte.js";
import type { createStoreRouteHistory } from "./store-route-history.svelte.js";
import {
  anchorCommit,
  anchorMatchesOperation,
  anchorMatchesPath,
} from "../collaboration/presentation.js";
import {
  readAgentRelations,
  readReviewContext,
} from "./production-collaboration-context.js";
import { readReviewAnchorPatch } from "./review-anchor-patch.js";
import { readNativeReviewContext } from "./native-review-context.js";
import { createProductionReviewActions } from "./production-review-actions.js";
import { createProductionInlineReviewActions } from "./production-inline-review-actions.js";

interface Context {
  data: ProductionStoreState;
  history: ReturnType<typeof createStoreRouteHistory>;
  source(): StoreDataSource;
  version(): number;
  current(ticket: number, source: StoreDataSource): boolean;
  report(cause: unknown): void;
  requestId?(): string;
}

export function createProductionStoreCollaboration(context: Context) {
  const { data, history } = context;
  let reviewGeneration = 0;
  async function loadAgents(more = false) {
    const source = context.source(),
      ticket = context.version();
    data.status.agents = more ? "refreshing" : "loading";
    try {
      const page = await source.agentSessions(
        more ? (data.agentCursor ?? undefined) : undefined,
      );
      if (!context.current(ticket, source)) return;
      data.agentSessions = more
        ? [...data.agentSessions, ...page.sessions]
        : [...page.sessions];
      data.agentCursor = page.next_cursor;
      data.status.agents = data.agentSessions.length ? "ready" : "empty";
    } catch (cause) {
      if (!context.current(ticket, source)) return;
      data.status.agents = "failed";
      context.report(cause);
    }
  }

  async function loadTranscript(more = false) {
    const descriptor = data.transcriptDescriptor;
    const session = data.agentSession?.session.session_id;
    if (!descriptor || !session) return;
    const source = context.source(),
      ticket = context.version();
    data.status.transcript = more ? "refreshing" : "loading";
    try {
      const page = await source.agentTranscript(
        session,
        descriptor,
        more ? (data.transcriptCursor ?? undefined) : undefined,
      );
      if (!context.current(ticket, source)) return;
      data.agentTranscript = more
        ? [...data.agentTranscript, ...page.chunks]
        : [...page.chunks];
      data.transcriptCursor = page.next_cursor;
      data.status.transcript = data.agentTranscript.length ? "ready" : "empty";
    } catch (cause) {
      if (!context.current(ticket, source)) return;
      data.status.transcript = "failed";
      context.report(cause);
    }
  }

  async function loadAgent(id: string) {
    const source = context.source(),
      ticket = context.version();
    data.status.transcript = "loading";
    try {
      const session = await source.agentSession(id);
      if (!context.current(ticket, source)) return;
      data.agentSession = session;
      data.transcriptDescriptor = session.transcripts[0];
      data.agentTranscript = [];
      data.transcriptCursor = null;
      if (data.transcriptDescriptor) await loadTranscript();
      else data.status.transcript = "empty";
    } catch (cause) {
      if (!context.current(ticket, source)) return;
      data.status.transcript = "failed";
      context.report(cause);
    }
  }

  async function loadReviews(more = false) {
    const source = context.source(),
      ticket = context.version();
    data.status.reviews = more ? "refreshing" : "loading";
    try {
      const page = await source.reviewThreads(
        more ? (data.reviewCursor ?? undefined) : undefined,
      );
      if (!context.current(ticket, source)) return;
      data.reviewThreads = more
        ? [...data.reviewThreads, ...page.threads]
        : [...page.threads];
      data.reviewCursor = page.next_cursor;
      data.status.reviews = data.reviewThreads.length ? "ready" : "empty";
    } catch (cause) {
      if (!context.current(ticket, source)) return;
      data.status.reviews = "failed";
      context.report(cause);
    }
  }

  async function loadReview(id: string) {
    const source = context.source(),
      ticket = context.version(),
      selection = ++reviewGeneration;
    const current = () =>
      selection === reviewGeneration && context.current(ticket, source);
    data.status.reviewAction = "loading";
    data.reviewPatch = undefined;
    data.reviewContext = undefined;
    try {
      const thread = await source.reviewThread(id);
      if (!current()) return;
      data.reviewThread = thread;
      if (history.route.page === "reviews") {
        const reviewContext = await readNativeReviewContext(
          source,
          thread.anchor.value,
        );
        if (!current()) return;
        data.reviewContext = reviewContext;
      }
      try {
        const patch = await readReviewAnchorPatch(source, thread.anchor.value);
        if (!current()) return;
        data.reviewPatch = patch;
      } catch {
        if (!current()) return;
        data.reviewPatch = undefined;
      }
      data.status.reviewAction = data.reviewActionRequestId
        ? "unavailable"
        : "ready";
    } catch (cause) {
      if (!current()) return;
      data.status.reviewAction = "failed";
      context.report(cause);
    }
  }

  async function loadContext(
    subject: { commit: string } | { operation: string },
    matches: Parameters<typeof readReviewContext>[1],
  ) {
    const source = context.source(),
      ticket = context.version();
    data.status.agents = "loading";
    data.status.reviews = "loading";
    try {
      const [relations, reviews] = await Promise.all([
        readAgentRelations(source, subject),
        readReviewContext(source, matches),
      ]);
      if (!context.current(ticket, source)) return;
      data.agentRelations = relations;
      data.reviewThreads = reviews.threads;
      data.reviewCursor = reviews.cursor;
      data.status.agents = data.agentRelations.length ? "ready" : "empty";
      data.status.reviews = data.reviewThreads.length ? "ready" : "empty";
      const sessionId = data.agentRelations.find(
        (relation) => relation.association,
      )?.association?.session_id;
      const reviewId = reviews.selected?.thread_id;
      history.replace({
        ...history.route,
        agentSessionId: sessionId ?? null,
        reviewThreadId: reviewId ?? null,
      });
      data.agentSession = undefined;
      data.reviewThread = undefined;
      await Promise.all([
        sessionId ? loadAgent(sessionId) : Promise.resolve(),
        reviewId ? loadReview(reviewId) : Promise.resolve(),
      ]);
    } catch (cause) {
      if (!context.current(ticket, source)) return;
      data.status.agents = "failed";
      data.status.reviews = "failed";
      context.report(cause);
    }
  }

  const loadRevisionContext = (commit: string) =>
    loadContext({ commit }, (thread) => anchorCommit(thread.anchor) === commit);
  const loadOperationContext = (operation: string) =>
    loadContext({ operation }, (thread) =>
      anchorMatchesOperation(thread.anchor, operation),
    );

  async function loadMatchingPathReview(
    matches: Parameters<typeof readReviewContext>[1],
  ) {
    const source = context.source(),
      ticket = context.version();
    data.status.reviews = "loading";
    try {
      const reviews = await readReviewContext(
        source,
        matches,
        data.reviewThreads,
        data.reviewCursor,
      );
      if (!context.current(ticket, source)) return;
      data.reviewThreads = reviews.threads;
      data.reviewCursor = reviews.cursor;
      data.status.reviews = data.reviewThreads.length ? "ready" : "empty";
      const reviewId = reviews.selected?.thread_id;
      history.replace({ ...history.route, reviewThreadId: reviewId ?? null });
      data.reviewThread = undefined;
      if (reviewId) await loadReview(reviewId);
    } catch (cause) {
      if (!context.current(ticket, source)) return;
      data.status.reviews = "failed";
      context.report(cause);
    }
  }

  const loadPathReview = (commit: string, path: string) =>
    loadMatchingPathReview(
      (thread) =>
        anchorCommit(thread.anchor) === commit &&
        anchorMatchesPath(thread.anchor, path),
    );
  const loadOperationPathReview = (operation: string, path: string) =>
    loadMatchingPathReview(
      (thread) =>
        anchorMatchesOperation(thread.anchor, operation) &&
        anchorMatchesPath(thread.anchor, path),
    );

  async function selectAgentSession(id: string | null) {
    history.push({
      ...history.route,
      agentSessionId: id,
      reviewThreadId: null,
    });
    data.agentSession = undefined;
    data.agentTranscript = [];
    data.transcriptDescriptor = undefined;
    data.transcriptCursor = null;
    if (id) await loadAgent(id);
  }

  async function selectReviewThread(id: string | null) {
    reviewGeneration++;
    history.push({ ...history.route, reviewThreadId: id });
    data.reviewThread = undefined;
    data.reviewContext = undefined;
    if (id) await loadReview(id);
  }

  const reviewActions = createProductionReviewActions({
    ...context,
    selectedReviewId: () => history.route.reviewThreadId,
    reloadReviews: () => loadReviews(),
    reloadReview: loadReview,
    selectReview: (id) =>
      history.replace({ ...history.route, reviewThreadId: id }),
  });

  const inlineReviewActions = createProductionInlineReviewActions({
    ...context,
    loadReview,
    reviewActions,
  });

  return {
    loadAgents,
    loadTranscript,
    loadAgent,
    loadReviews,
    loadReview,
    loadRevisionContext,
    loadOperationContext,
    loadPathReview,
    loadOperationPathReview,
    selectAgentSession,
    selectReviewThread,
    ...inlineReviewActions,
    ...reviewActions,
  };
}

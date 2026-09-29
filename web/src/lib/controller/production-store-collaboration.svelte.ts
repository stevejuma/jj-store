import type {
  StoreDataSource,
  StoreReviewCommandRequest,
} from "./native-data-source.js";
import type { ProductionStoreState } from "./production-store-state.svelte.js";
import type { createStoreRouteHistory } from "./store-route-history.svelte.js";
import { anchorCommit, anchorPath } from "../collaboration/presentation.js";
import {
  readRevisionAgentRelations,
  readReviewContext,
} from "./production-collaboration-context.js";

interface Context {
  data: ProductionStoreState;
  history: ReturnType<typeof createStoreRouteHistory>;
  source(): StoreDataSource;
  version(): number;
  current(ticket: number, source: StoreDataSource): boolean;
  report(cause: unknown): void;
  requestId?(): string;
}

const defaultRequestId = () =>
  [...crypto.getRandomValues(new Uint8Array(16))]
    .map((value) => value.toString(16).padStart(2, "0"))
    .join("");

export function createProductionStoreCollaboration(context: Context) {
  const { data, history } = context;
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
      ticket = context.version();
    data.status.reviewAction = "loading";
    try {
      const thread = await source.reviewThread(id);
      if (!context.current(ticket, source)) return;
      data.reviewThread = thread;
      data.status.reviewAction = "ready";
    } catch (cause) {
      if (!context.current(ticket, source)) return;
      data.status.reviewAction = "failed";
      context.report(cause);
    }
  }

  async function loadRevisionContext(commit: string) {
    const source = context.source(),
      ticket = context.version();
    data.status.agents = "loading";
    data.status.reviews = "loading";
    try {
      const [relations, reviews] = await Promise.all([
        readRevisionAgentRelations(source, commit),
        readReviewContext(
          source,
          (thread) => anchorCommit(thread.anchor) === commit,
        ),
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

  async function loadPathReview(commit: string, path: string) {
    const source = context.source(),
      ticket = context.version();
    data.status.reviews = "loading";
    try {
      const reviews = await readReviewContext(
        source,
        (thread) =>
          anchorCommit(thread.anchor) === commit &&
          anchorPath(thread.anchor) === path,
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

  async function selectAgentSession(id: string | null) {
    history.push({ ...history.route, agentSessionId: id });
    data.agentSession = undefined;
    data.agentTranscript = [];
    data.transcriptDescriptor = undefined;
    data.transcriptCursor = null;
    if (id) await loadAgent(id);
  }

  async function selectReviewThread(id: string | null) {
    history.push({ ...history.route, reviewThreadId: id });
    data.reviewThread = undefined;
    if (id) await loadReview(id);
  }

  async function apply(command: StoreReviewCommandRequest["command"]) {
    const source = context.source();
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
      if (outcome.status === "outcome_unknown") {
        data.status.reviewAction = "unavailable";
        data.reviewActionMessage =
          "The review outcome is unknown. Check the durable request status before retrying.";
        return;
      }
      data.reviewActionRequestId = null;
      data.status.reviewAction = "ready";
      await loadReviews();
      await loadReview(outcome.receipt.result.thread_id);
    } catch (cause) {
      data.status.reviewAction = "failed";
      context.report(cause);
    }
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
    await loadReviews();
    await loadReview(status.receipt.result.thread_id);
  }

  return {
    loadAgents,
    loadTranscript,
    loadAgent,
    loadReviews,
    loadReview,
    loadRevisionContext,
    loadPathReview,
    selectAgentSession,
    selectReviewThread,
    reply,
    setResolved,
    checkAction,
  };
}

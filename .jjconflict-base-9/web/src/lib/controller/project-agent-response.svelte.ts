import { ApiError } from "../api/client.js";
import type { StoreController } from "./contracts.js";
import type { StoreDataSource } from "./native-data-source.js";
import type { ProjectAgentResponse } from "./projects-hub-types.js";
import { createProductionStoreState } from "./production-store-state.svelte.js";
import {
  createStoreRouteHistory,
  initialStoreRoute,
} from "./store-route-history.svelte.js";
import { createProductionStoreCollaboration } from "./production-store-collaboration.svelte.js";
import { createThreadsIntegration } from "./production-threads-integration.js";
import { productionStoreView } from "./production-store-view.js";
import { reportProductionStoreError } from "./production-store-lifecycle.svelte.js";

/** A repository-authorized response reader with no global connection or route changes. */
export function createProjectAgentResponse(
  source: StoreDataSource | (() => StoreDataSource),
  base: () => StoreController,
): ProjectAgentResponse {
  const currentSource = () =>
    typeof source === "function" ? source() : source;
  const repository = currentSource().repository;
  const data = createProductionStoreState();
  const history = createStoreRouteHistory({
    hash: () => "",
    push() {},
    replace() {},
    go() {},
    listen: () => () => {},
  });
  let version = 0;
  const authorized = () =>
    base().state.repositories.some((r) => r.id === repository);
  const active = () => {
    if (!authorized()) throw new ApiError("access_denied");
    return currentSource();
  };
  function reset() {
    version++;
    data.agentSession = undefined;
    data.agentTranscript = [];
    data.transcriptCursor = null;
    data.transcriptDescriptor = undefined;
    data.reviewThreads = [];
    data.reviewThread = undefined;
    data.reviewContext = undefined;
    data.reviewPatch = undefined;
    threads.threads.clear(true);
  }
  const collaboration = createProductionStoreCollaboration({
    data,
    history,
    source: active,
    version: () => version,
    current: (ticket, candidate) =>
      ticket === version && authorized() && candidate === currentSource(),
    report(cause) {
      if (
        cause instanceof ApiError &&
        ["access_denied", "bearer_required"].includes(cause.code)
      )
        reset();
      reportProductionStoreError(data, cause);
    },
  });
  const threads = createThreadsIntegration({
    data,
    history,
    source: active,
    connectRepository: async () => {
      throw new ApiError("access_denied");
    },
  });
  const controller: StoreController = {
    get state() {
      const view = productionStoreView(data, history.route, {
        connected: authorized(),
        repository: repository,
        canBack: false,
        canForward: false,
        now: base().state.now,
      });
      return {
        ...view,
        member: base().state.member,
        repository: {
          ...view.repository,
          permission:
            base().state.repositories.find((r) => r.id === repository)
              ?.permission ?? null,
        },
        agentSession: authorized() ? view.agentSession : undefined,
        agentTranscript: authorized() ? view.agentTranscript : [],
        reviewThreads: authorized() ? view.reviewThreads : [],
        reviewThread: authorized() ? view.reviewThread : undefined,
      };
    },
    get threads() {
      return authorized() && currentSource().threads
        ? threads.threads
        : undefined;
    },
    get actions() {
      return {
        ...base().actions,
        selectAgentSession: collaboration.selectAgentSession,
        selectReviewThread: collaboration.selectReviewThread,
        replyToReview: collaboration.reply,
        editReviewComment: collaboration.edit,
        commentOnAgentTool: collaboration.commentOnAgentTool,
        readAgentToolReview: collaboration.readAgentToolReview,
        readAgentSessionReview: collaboration.readAgentSessionReview,
        commentOnAgentSession: collaboration.commentOnAgentSession,
        commentOnDiffLine: collaboration.commentOnDiffLine,
        setReviewResolved: collaboration.setResolved,
        checkReviewAction: collaboration.checkAction,
        async loadMore(
          collection: Parameters<StoreController["actions"]["loadMore"]>[0],
        ) {
          if (collection === "transcript")
            await collaboration.loadTranscript(true);
          if (collection === "reviews") await collaboration.loadReviews(true);
        },
      };
    },
  };
  return {
    controller,
    reset,
    async load(id) {
      reset();
      data.repositories = [...base().state.repositories];
      data.accessMode = base().state.accessMode;
      data.storageReady = base().state.storageReady;
      const ticket = version;
      data.status.error = "";
      data.status.transcript = "loading";
      history.replace({
        ...initialStoreRoute(),
        page: "agents",
        repository: repository,
        agentSessionId: id,
        reviewThreadId: data.reviewActionRequestId
          ? history.route.reviewThreadId
          : null,
      });
      try {
        const candidate = active();
        const head = await candidate.snapshot();
        if (
          ticket !== version ||
          !authorized() ||
          candidate !== currentSource()
        )
          return;
        data.head = head;
        data.heads = [...head.heads];
        history.replace({
          ...history.route,
          snapshotOperationId: head.heads.length === 1 ? head.heads[0]! : null,
        });
        await collaboration.loadAgent(id);
        if (ticket !== version || !authorized()) return;
        if (
          data.agentSession &&
          (data.agentSession.session.session_id !== id ||
            data.agentSession.snapshot.repo_id !== repository)
        ) {
          reset();
          data.status.transcript = "failed";
          reportProductionStoreError(data, new ApiError("snapshot_mismatch"));
          return;
        }
        if (data.agentSession) await collaboration.loadReviews();
      } catch (cause) {
        if (ticket !== version) return;
        data.status.transcript = "failed";
        reportProductionStoreError(data, cause);
      }
    },
  };
}

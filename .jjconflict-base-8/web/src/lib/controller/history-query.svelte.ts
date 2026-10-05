import { ApiError } from "../api/client.js";
import { untrack } from "svelte";
import type {
  HistoryQueryController,
  HistoryQueryInfo,
} from "./history-query-types.js";
import type { ProductionStoreState } from "./production-store-state.svelte.js";
import type { StoreDataSource } from "./native-data-source.js";
import type { createStoreRouteHistory } from "./store-route-history.svelte.js";
import { readGraph } from "./production-store-loaders.js";
import { storeErrorMessage } from "./store-error.js";

export function createHistoryQuery(context: {
  data: ProductionStoreState;
  history: ReturnType<typeof createStoreRouteHistory>;
  source(): StoreDataSource;
  version(): number;
  begin(): number;
  current(ticket: number, source: StoreDataSource): boolean;
}): HistoryQueryController {
  let draft = $state(context.history.route.historyQuery ?? "");
  let email = $state("");
  let emailEdited = false;
  let error = $state("");
  let pending = $state(false);
  let info = $state<HistoryQueryInfo | null>(null);
  let sequence = 0;
  let metadata: {
    source: StoreDataSource;
    operation: string;
    promise: Promise<void>;
  } | null = null;
  let timer: ReturnType<typeof setTimeout> | undefined;
  let abort = new AbortController();
  let repository = context.history.route.repository;
  let draftScope = `${repository}:${context.history.route.page}:${context.history.route.workspace}`;
  let accepted = context.history.route.historyQuery ?? "";
  const scope = $derived(
    `${context.history.route.repository}:${context.history.route.page}:${context.history.route.snapshotOperationId}:${context.history.route.workspace}`,
  );
  const applied = $derived(context.history.route.historyQuery ?? "");
  const invalidate = () => {
    sequence++;
    clearTimeout(timer);
    abort.abort();
    abort = new AbortController();
    pending = false;
  };
  $effect(() => {
    const identity = scope;
    const value = applied;
    const route = untrack(() => context.history.route);
    const nextDraftScope = `${route.repository}:${route.page}:${route.workspace}`;
    void identity;
    invalidate();
    if (draftScope !== nextDraftScope || accepted !== value) {
      draft = value;
      error = "";
    }
    draftScope = nextDraftScope;
    accepted = value;
    if (repository !== route.repository) {
      repository = route.repository;
      email = "";
      emailEdited = false;
      context.data.historyEmail = "";
      info = null;
    }
  });
  $effect(() => {
    const connected = context.data.status.connection === "connected";
    const route = context.history.route;
    if (
      !connected ||
      !route.snapshotOperationId ||
      !["log", "oplog"].includes(route.page)
    )
      return;
    void prepare();
  });
  function prepare(): Promise<void> {
    const source = context.source(),
      operation = context.history.route.snapshotOperationId;
    if (!operation || !source.queryInfo) return Promise.resolve();
    if (metadata?.source === source && metadata.operation === operation)
      return metadata.promise;
    const request = { source, operation, promise: Promise.resolve() };
    metadata = request;
    request.promise = source
      .queryInfo(operation)
      .then((value) => {
        if (
          metadata !== request ||
          context.source() !== source ||
          context.history.route.snapshotOperationId !== operation ||
          context.data.status.connection !== "connected"
        ) {
          if (metadata === request) metadata = null;
          return;
        }
        info = value;
        if (!emailEdited) {
          email = value.default_user_email ?? "";
          context.data.historyEmail = email;
        }
      })
      .catch((cause) => {
        if (metadata === request) {
          metadata = null;
          if (
            context.source() === source &&
            context.history.route.snapshotOperationId === operation &&
            context.data.status.connection === "connected"
          )
            error = storeErrorMessage(cause);
        }
      });
    return request.promise;
  }
  async function run() {
    clearTimeout(timer);
    const route = context.history.route;
    if (
      context.data.status.connection !== "connected" ||
      !route.snapshotOperationId ||
      !["log", "oplog"].includes(route.page)
    )
      return;
    const identity = scope,
      sequenceBeforeMetadata = sequence,
      textBeforeMetadata = draft.trim();
    if (context.source().queryInfo) await prepare();
    if (
      sequenceBeforeMetadata !== sequence ||
      identity !== scope ||
      draft.trim() !== textBeforeMetadata ||
      context.data.status.connection !== "connected"
    )
      return;
    const text = draft.trim(),
      source = context.source(),
      ticket = context.begin();
    const request = ++sequence;
    abort.abort();
    abort = new AbortController();
    const signal = abort.signal;
    pending = true;
    error = "";
    const atTime =
      context.data.graphAtTimeMillis ?? route.historyTime ?? Date.now();
    const workingCopy = context.data.workingCopies.find(
      (copy) => copy.workspace === route.workspace,
    );
    try {
      if (
        new TextEncoder().encode(text).length > (info?.max_query_bytes ?? 4096)
      )
        throw new ApiError("resource_limit", undefined, undefined, {
          kind: "limit",
          message: "Query exceeds the supported byte limit.",
        });
      const value = await readGraph(
        source,
        route.snapshotOperationId,
        route.page === "oplog" ? "operations" : "log",
        context.data.heads,
        atTime,
        undefined,
        undefined,
        workingCopy ? [workingCopy.commitId] : undefined,
        route.workspace ?? undefined,
        { text, email, signal },
      );
      if (
        request !== sequence ||
        signal.aborted ||
        !context.current(ticket, source) ||
        identity !== scope
      )
        return;
      context.data.graph = value.graph;
      context.data.graphCursor = value.cursor;
      context.data.graphPageSnapshot = value.snapshot;
      context.data.graphAtTimeMillis = atTime;
      context.data.status.graph = value.graph.revisions.length
        ? "ready"
        : "empty";
      context.data.historyEmail = email;
      context.history.replace({
        ...route,
        historyQuery: text || undefined,
        historyTime: atTime,
      });
    } catch (cause) {
      if (
        request === sequence &&
        !signal.aborted &&
        context.current(ticket, source)
      )
        error = `Showing previous results. ${storeErrorMessage(cause)}`;
    } finally {
      if (request === sequence) pending = false;
    }
  }
  function setDraft(value: string) {
    invalidate();
    draft = value;
    error = "";
    timer = setTimeout(() => void run(), 250);
  }
  return {
    get draft() {
      return draft;
    },
    get email() {
      return email;
    },
    get error() {
      return error;
    },
    get pending() {
      return pending;
    },
    get info() {
      return info;
    },
    setDraft,
    setEmail(value) {
      emailEdited = true;
      email = value;
      setDraft(draft);
    },
    prepare,
    run,
    clear() {
      invalidate();
      draft = "";
      error = "";
      void run();
    },
    dispose() {
      invalidate();
      metadata = null;
      email = "";
      emailEdited = false;
      context.data.historyEmail = "";
      info = null;
      error = "";
    },
  };
}

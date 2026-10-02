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
  let error = $state("");
  let pending = $state(false);
  let info = $state<HistoryQueryInfo | null>(null);
  let sequence = 0;
  let timer: ReturnType<typeof setTimeout> | undefined;
  let abort = new AbortController();
  let repository = context.history.route.repository;
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
    void identity;
    invalidate();
    draft = value;
    error = "";
    if (repository !== route.repository) {
      repository = route.repository;
      email = "";
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
    const source = context.source(),
      ticket = context.version(),
      operation = route.snapshotOperationId;
    if (source.queryInfo)
      void source
        .queryInfo(operation)
        .then((value) => {
          if (
            context.current(ticket, source) &&
            context.history.route.snapshotOperationId === operation
          )
            info = value;
        })
        .catch((cause) => {
          if (context.current(ticket, source)) error = storeErrorMessage(cause);
        });
  });
  async function run() {
    clearTimeout(timer);
    const route = context.history.route;
    if (
      context.data.status.connection !== "connected" ||
      !route.snapshotOperationId ||
      !["log", "oplog"].includes(route.page)
    )
      return;
    const text = draft.trim(),
      source = context.source(),
      ticket = context.begin();
    const identity = scope;
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
      email = value;
      setDraft(draft);
    },
    run,
    clear() {
      invalidate();
      draft = "";
      error = "";
      void run();
    },
    dispose() {
      invalidate();
      email = "";
      info = null;
      error = "";
    },
  };
}

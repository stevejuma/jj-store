import { createHistoryQuery } from "./history-query.svelte.js";
import { readWorkingCopies } from "./store-workspaces.js";
import { fullId } from "../api/client.js";
import type { StoreDataSource } from "./native-data-source.js";
import { createProductionFileReader } from "./production-file-reader.svelte.js";
import { createProductionEvolutionReader } from "./production-evolution-reader.js";
import {
  normalizeInspectionContinuation,
  readMoreCommitInspection,
  readMoreOperationInspection,
} from "./production-inspection-pages.js";
import {
  readCommitInspection,
  readGraph,
  readOperationInspection,
  readPatch,
  readReferences,
} from "./production-store-loaders.js";
import type { ProductionStoreState } from "./production-store-state.svelte.js";
import { restoreProductionStoreRoute } from "./production-route-restorer.js";
import { createProductionStoreCollaboration } from "./production-store-collaboration.svelte.js";
import type { createStoreRouteHistory } from "./store-route-history.svelte.js";

interface ReaderContext {
  data: ProductionStoreState;
  history: ReturnType<typeof createStoreRouteHistory>;
  source(): StoreDataSource;
  operation(): string | null;
  version(): number;
  current(ticket: number, source: StoreDataSource): boolean;
  report(cause: unknown): void;
}

export function createProductionStoreReaders(
  context: Omit<ReaderContext, "operation"> & { begin(): number },
) {
  const historyQuery = createHistoryQuery(context);
  return {
    reader: createProductionStoreReader(
      {
        ...context,
        operation: () => context.history.route.snapshotOperationId,
      },
      historyQuery.prepare,
    ),
    historyQuery,
  };
}

export function createProductionStoreReader(
  context: ReaderContext,
  prepareHistory?: () => Promise<void>,
) {
  const { data, history } = context;
  const fileReader = createProductionFileReader(context);
  const collaboration = createProductionStoreCollaboration(context);
  const evolution = createProductionEvolutionReader(context);
  async function loadReferences(more = false) {
    const source = context.source(),
      operation = fullId(context.operation() ?? ""),
      ticket = context.version();
    data.status.bookmarks = more ? "refreshing" : "loading";
    try {
      const value = await readReferences(
        source,
        operation,
        more ? (data.refsCursor ?? undefined) : undefined,
        more ? (data.refsPageSnapshot ?? undefined) : undefined,
      );
      const workingCopies = more
        ? data.workingCopies
        : await readWorkingCopies(source, operation, value);
      if (!context.current(ticket, source)) return;
      data.bookmarks = more
        ? [...data.bookmarks, ...value.bookmarks]
        : value.bookmarks;
      data.choices = more ? [...data.choices, ...value.choices] : value.choices;
      data.workingCopies = workingCopies;
      data.refsCursor = value.cursor;
      data.refsPageSnapshot = value.snapshot;
      data.status.bookmarks = data.bookmarks.length ? "ready" : "empty";
    } catch (cause) {
      if (!context.current(ticket, source)) return;
      context.report(cause);
      data.status.bookmarks = "failed";
    }
  }
  async function loadGraph(more = false) {
    const source = context.source(),
      operation = fullId(context.operation() ?? ""),
      ticket = context.version();
    const kind = history.route.page === "oplog" ? "operations" : "log";
    const workspace = data.workingCopies.find(
      (copy) => copy.workspace === history.route.workspace,
    );
    const branchTargets =
      kind !== "log"
        ? undefined
        : history.route.bookmark && history.route.page === "branches"
          ? data.targets
          : workspace && !history.route.bookmark
            ? [workspace.commitId]
            : history.route.page === "branches" &&
                data.workingCopies.some(
                  (copy) => copy.commitId === history.route.commitId,
                )
              ? [history.route.commitId!]
              : undefined;
    data.status.graph = more ? "refreshing" : "loading";
    try {
      if (history.route.historyQuery) await prepareHistory?.();
      if (!context.current(ticket, source)) return;
      data.graphAtTimeMillis ??= history.route.historyTime ?? Date.now();
      const value = await readGraph(
        source,
        operation,
        kind,
        data.heads,
        data.graphAtTimeMillis,
        more ? (data.graphCursor ?? undefined) : undefined,
        more ? (data.graphPageSnapshot ?? undefined) : undefined,
        branchTargets,
        history.route.workspace ?? undefined,
        { text: history.route.historyQuery ?? "", email: data.historyEmail },
      );
      if (!context.current(ticket, source)) return;
      data.graph = more
        ? {
            revisions: [...data.graph.revisions, ...value.graph.revisions],
            devices: value.graph.devices,
          }
        : value.graph;
      data.graphCursor = value.cursor;
      data.graphPageSnapshot = value.snapshot;
      data.status.graph = data.graph.revisions.length ? "ready" : "empty";
    } catch (cause) {
      if (!context.current(ticket, source)) return;
      context.report(cause);
      data.status.graph = "failed";
    }
  }
  async function loadCommit(id: string) {
    const source = context.source(),
      operation = fullId(context.operation() ?? ""),
      ticket = context.version();
    data.status.detail = "loading";
    try {
      const value = await readCommitInspection(source, operation, id);
      if (!context.current(ticket, source)) return;
      data.detail = value.detail;
      data.comparisons = value.comparisons;
      data.conflicts = value.conflicts;
      data.conflictCount = value.conflicts.length;
      data.inspectionContinuation = value.continuation;
      data.status.detail = "ready";
    } catch (cause) {
      if (!context.current(ticket, source)) return;
      context.report(cause);
      data.status.detail = "failed";
    }
  }
  async function loadOperation(id: string, parent?: string) {
    const source = context.source(),
      ticket = context.version();
    data.status.detail = "loading";
    try {
      const value = await readOperationInspection(source, id, parent);
      if (!context.current(ticket, source)) return;
      data.detail = value.detail;
      data.comparisons = value.comparisons;
      data.comparisonParents = value.parents;
      data.unavailableComparisonCount = value.unavailableComparisons;
      data.inspectionContinuation = normalizeInspectionContinuation(
        value.continuation,
      );
      history.replace({
        ...history.route,
        comparisonParentId: value.selectedParent,
      });
      data.status.detail = "ready";
    } catch (cause) {
      if (!context.current(ticket, source)) return;
      context.report(cause);
      data.status.detail = "failed";
    }
  }
  async function loadPatch(path: string) {
    const comparison = data.comparisons.get(path),
      operation = context.operation(),
      source = context.source(),
      ticket = context.version();
    if (!comparison || !operation) return;
    try {
      const patch = await readPatch(source, operation, path, comparison);
      if (!context.current(ticket, source) || !data.detail) return;
      data.detail = {
        ...data.detail,
        patches: {
          ...data.detail.patches,
          ...(patch ? { [path]: patch } : {}),
        },
      };
      history.replace({
        ...history.route,
        comparisonGroupId: comparison.group ?? null,
      });
    } catch (cause) {
      if (context.current(ticket, source)) context.report(cause);
    }
  }
  async function loadMoreDetail() {
    const continuation = data.inspectionContinuation,
      source = context.source(),
      ticket = context.version();
    if (!continuation || !data.detail) return;
    data.status.detail = "refreshing";
    try {
      const page =
        continuation.kind === "commit"
          ? await readMoreCommitInspection(source, continuation)
          : await readMoreOperationInspection(source, continuation);
      if (!context.current(ticket, source) || !data.detail) return;
      data.detail = {
        files: [...data.detail.files, ...page.files],
        patches: data.detail.patches,
      };
      data.comparisons = new Map([...data.comparisons, ...page.comparisons]);
      data.unavailableComparisonCount += page.unavailableComparisons;
      data.inspectionContinuation = normalizeInspectionContinuation(
        page.continuation,
      );
      data.status.detail = data.detail.files.length ? "ready" : "empty";
    } catch (cause) {
      if (!context.current(ticket, source)) return;
      context.report(cause);
      data.status.detail = "failed";
    }
  }
  async function selectRef(value: string, push = true) {
    const choice = data.choices.find((item) => item.value === value);
    if (!choice) return;
    data.targets = [...choice.targets];
    const commit = data.targets.length === 1 ? data.targets[0]! : null;
    const next = {
      ...history.route,
      bookmark: choice.kind === "bookmark" ? choice.label : "",
      commitId: commit,
      evolutionCommitId: null,
      changeId: choice.kind === "change" ? choice.value.slice(7) : null,
      path: null,
      directory: "",
    };
    push ? history.push(next) : history.replace(next);
    data.snapshot = undefined;
    data.blob = undefined;
    if (commit) await fileReader.loadFiles(commit);
  }
  const reads = {
    loadReferences,
    loadGraph,
    ...evolution,
    ...fileReader,
    loadCommit,
    loadOperation,
    loadPatch,
    loadMoreDetail,
    selectRef,
    ...collaboration,
  };
  return {
    ...reads,
    restorePage: () =>
      restoreProductionStoreRoute(history.route, data, history, reads),
  };
}

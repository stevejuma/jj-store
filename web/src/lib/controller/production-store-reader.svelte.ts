import { fullId } from "../api/client.js";
import type { StoreDataSource } from "./native-data-source.js";
import {
  readBlob,
  readCommitInspection,
  readGraph,
  readLandingDocuments,
  readOperationInspection,
  readPatch,
  readPaths,
  readReferences,
} from "./production-store-loaders.js";
import type { ProductionStoreState } from "./production-store-state.svelte.js";
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

export function createProductionStoreReader(context: ReaderContext) {
  const { data, history } = context;
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
      if (!context.current(ticket, source)) return;
      data.bookmarks = more
        ? [...data.bookmarks, ...value.bookmarks]
        : value.bookmarks;
      data.choices = more ? [...data.choices, ...value.choices] : value.choices;
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
    data.status.graph = more ? "refreshing" : "loading";
    try {
      data.graphAtTimeMillis ??= Date.now();
      const value = await readGraph(
        source,
        operation,
        kind,
        data.heads,
        data.graphAtTimeMillis,
        more ? (data.graphCursor ?? undefined) : undefined,
        more ? (data.graphPageSnapshot ?? undefined) : undefined,
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
  async function loadFiles(commit: string, more = false) {
    const source = context.source(),
      operation = fullId(context.operation() ?? ""),
      ticket = context.version();
    data.status.files = more ? "refreshing" : "loading";
    try {
      const value = await readPaths(
        source,
        operation,
        commit,
        data.pathEntries,
        more ? (data.pathsCursor ?? undefined) : undefined,
        more ? (data.pathsPageSnapshot ?? undefined) : undefined,
      );
      if (!context.current(ticket, source)) return;
      data.pathEntries = value.entries;
      data.fileTerms = value.files;
      data.pathsCursor = value.cursor;
      data.pathsPageSnapshot = value.readSnapshot;
      data.snapshot = await readLandingDocuments(
        source,
        operation,
        value.snapshot,
        value.files,
      );
      if (!context.current(ticket, source)) return;
      data.status.files = data.snapshot.files.length ? "ready" : "empty";
    } catch (cause) {
      if (!context.current(ticket, source)) return;
      context.report(cause);
      data.status.files = "failed";
    }
  }
  async function loadBlob(path: string) {
    const source = context.source(),
      id = data.fileTerms.get(path),
      operation = context.operation(),
      ticket = context.version();
    if (!id || !operation) return;
    data.status.detail = "loading";
    try {
      const value = await readBlob(source, operation, path, id);
      if (!context.current(ticket, source)) return;
      data.blob = value;
      data.status.detail = "ready";
    } catch (cause) {
      if (!context.current(ticket, source)) return;
      context.report(cause);
      data.status.detail = "failed";
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
      data.conflictCount = value.conflictCount;
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
  async function selectRef(value: string, push = true) {
    const choice = data.choices.find((item) => item.value === value);
    if (!choice) return;
    data.targets = [...choice.targets];
    const commit = data.targets.length === 1 ? data.targets[0]! : null;
    const next = {
      ...history.route,
      bookmark: choice.kind === "bookmark" ? choice.label : "",
      commitId: commit,
      changeId: choice.kind === "change" ? choice.value.slice(7) : null,
      path: null,
      directory: "",
    };
    push ? history.push(next) : history.replace(next);
    data.snapshot = undefined;
    data.blob = undefined;
    if (commit) await loadFiles(commit);
  }
  async function loadPage() {
    if (!context.operation()) return;
    data.status.error = "";
    if (history.route.page === "files") {
      await loadReferences();
      const selected = data.choices.find(
        (choice) => choice.targets.length === 1,
      );
      if (selected) await selectRef(selected.value, false);
    } else await Promise.all([loadReferences(), loadGraph()]);
  }
  return {
    loadReferences,
    loadGraph,
    loadFiles,
    loadBlob,
    loadCommit,
    loadOperation,
    loadPatch,
    selectRef,
    loadPage,
  };
}

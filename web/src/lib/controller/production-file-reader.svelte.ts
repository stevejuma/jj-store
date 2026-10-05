import { createProductionFileMetadata } from "./production-file-metadata.js";
import type { FileListingViewMode } from "@lapismd/design-core/diff";
import { fullId } from "../api/client.js";
import type { StoreDataSource } from "./native-data-source.js";
import { presentPaths } from "./native-presentation.js";
import {
  nextPathPage,
  pathPageKey,
  storePathPage,
  visiblePathEntries,
  type StoredPathPage,
} from "./production-file-pages.js";
import { readBlob, readPaths } from "./production-store-loaders.js";
import type { ProductionStoreState } from "./production-store-state.svelte.js";
import type { createStoreRouteHistory } from "./store-route-history.svelte.js";

export interface FileReaderContext {
  data: ProductionStoreState;
  history: ReturnType<typeof createStoreRouteHistory>;
  source(): StoreDataSource;
  operation(): string | null;
  version(): number;
  signal?(): AbortSignal;
  current(ticket: number, source: StoreDataSource): boolean;
  report(cause: unknown): void;
}

export function createProductionFileReader(context: FileReaderContext) {
  const { data, history } = context;
  const metadata = createProductionFileMetadata(context, publish);
  let retained:
    | {
        source: StoreDataSource;
        operation: string;
        commit: string;
        ticket: number;
      }
    | undefined;
  let pending:
    { key: string; signal: AbortSignal; promise: Promise<boolean> } | undefined;
  function enrich(
    page: StoredPathPage,
    commit: string,
    operation: string,
    ticket: number,
    source: StoreDataSource,
  ) {
    for (const window of page.metadataWindows ?? []) {
      const paths = new Set(window.paths);
      metadata.load({
        source,
        operation,
        commit,
        ticket,
        window,
        anchor: window.anchor,
        value: {
          ...page,
          entries: page.entries.filter((entry) => paths.has(entry.path)),
          readSnapshot: page.snapshot,
        },
      });
    }
  }
  function publish(commit: string) {
    const presented = presentPaths(
      commit,
      visiblePathEntries(data.pathPages, data.fileViewMode),
    );
    const previous = new Map(
      data.snapshot?.id === commit
        ? data.snapshot.files.map((file) => [file.path, file])
        : [],
    );
    data.snapshot = {
      ...presented.snapshot,
      pathChanges: {
        ...presented.snapshot.pathChanges,
        ...Object.fromEntries(
          [...data.pathPages.values()].flatMap((page) =>
            [...(page.unavailableChanges ?? [])].map((path) => [
              path,
              undefined,
            ]),
          ),
        ),
      },
      files: presented.snapshot.files.map((file) =>
        data.fileTerms.get(file.path) === presented.files.get(file.path)
          ? (previous.get(file.path) ?? file)
          : file,
      ),
    };
    data.fileTerms = presented.files;
  }
  async function loadFiles(
    commit: string,
    options: { path?: string; recursive?: boolean; more?: boolean } = {},
  ) {
    const source = context.source(),
      operation = fullId(context.operation() ?? ""),
      ticket = context.version();
    const path = options.path ?? "";
    const recursive = options.recursive ?? data.fileViewMode !== "tree";
    const more = options.more ?? false;
    if (
      retained?.source !== source ||
      retained.operation !== operation ||
      retained.commit !== commit ||
      retained.ticket !== ticket
    ) {
      retained = { source, operation, commit, ticket };
      data.pathPages = new Map();
      data.knownDirectories = new Set();
      data.collapsedDirectoryPaths = new Set();
      data.fileRevision = undefined;
      data.fileRevisionStatus = "idle";
    }
    const previous = data.pathPages.get(pathPageKey(path, recursive));
    if (more && !previous?.cursor) return false;
    const signal = metadata.bind(
      commit,
      operation,
      pathPageKey(path, recursive),
    );
    if (signal.aborted || !context.current(ticket, source)) return false;
    if (!more && previous) {
      publish(commit);
      data.status.files = previous.entries.length ? "ready" : "empty";
      enrich(previous, commit, operation, ticket, source);
      return true;
    }
    const key = `${pathPageKey(path, recursive)}:${more ? previous?.cursor : ""}`;
    if (pending?.key === key && pending.signal === signal)
      return pending.promise;
    data.status.files = more ? "refreshing" : "loading";
    const promise = load();
    pending = { key, signal, promise };
    try {
      return await promise;
    } finally {
      if (pending?.promise === promise) pending = undefined;
    }
    async function load() {
      try {
        const value = await readPaths(source, operation, commit, {
          path,
          recursive,
          includeLastChange: false,
          signal,
          cursor: more ? (previous?.cursor ?? undefined) : undefined,
          expected: more ? previous?.snapshot : undefined,
        });
        if (!context.current(ticket, source) || signal.aborted) return false;
        data.pathPages = storePathPage(
          data.pathPages,
          {
            path: value.path,
            recursive: value.recursive,
            entries: value.entries,
            cursor: value.cursor,
            snapshot: value.readSnapshot,
          },
          more,
        );
        publish(commit);
        if (!recursive) rememberDirectories(data.snapshot?.directories ?? []);
        data.status.files =
          data.snapshot?.files.length || data.snapshot?.directories?.length
            ? "ready"
            : "empty";
        enrich(
          data.pathPages.get(pathPageKey(value.path, value.recursive))!,
          commit,
          operation,
          ticket,
          source,
        );
        return true;
      } catch (cause) {
        if (!context.current(ticket, source) || signal.aborted) return false;
        context.report(cause);
        data.status.files = "failed";
        return false;
      }
    }
  }
  function rememberDirectories(directories: readonly { path: string }[]) {
    const known = new Set(data.knownDirectories);
    const collapsed = new Set(data.collapsedDirectoryPaths);
    for (const directory of directories) {
      if (!known.has(directory.path)) collapsed.add(directory.path);
      known.add(directory.path);
    }
    data.knownDirectories = known;
    data.collapsedDirectoryPaths = collapsed;
  }
  async function loadBlob(path: string) {
    const source = context.source(),
      id = data.fileTerms.get(path),
      operation = context.operation(),
      ticket = context.version();
    if (!id || !operation) return;
    data.status.detail = "loading";
    try {
      const commit = data.snapshot?.id;
      if (!commit) return;
      const value = await readBlob(
        source,
        operation,
        path,
        id,
        commit,
        context.signal?.(),
      );
      if (!context.current(ticket, source)) return;
      data.blob = value;
      data.status.detail = "ready";
    } catch (cause) {
      if (!context.current(ticket, source)) return;
      context.report(cause);
      data.status.detail = "failed";
    }
  }
  async function setFileViewMode(mode: FileListingViewMode) {
    if (mode === data.fileViewMode) return;
    data.fileViewMode = mode;
    const commit = history.route.commitId;
    if (commit) await loadFiles(commit, { recursive: mode !== "tree" });
  }
  async function toggleDirectory(path: string) {
    const collapsed = new Set(data.collapsedDirectoryPaths);
    if (!collapsed.has(path)) {
      collapsed.add(path);
      data.collapsedDirectoryPaths = collapsed;
      return;
    }
    const commit = history.route.commitId;
    if (commit && !(await loadFiles(commit, { path, recursive: false })))
      return;
    const expanded = new Set(data.collapsedDirectoryPaths);
    expanded.delete(path);
    data.collapsedDirectoryPaths = expanded;
  }
  async function selectDirectory(directory: string) {
    history.push({ ...history.route, directory, path: null });
    const commit = history.route.commitId;
    if (commit && data.fileViewMode === "tree")
      await loadFiles(commit, { path: directory, recursive: false });
  }
  async function loadMoreFiles() {
    if (["loading", "refreshing", "failed"].includes(data.status.files)) return;
    const page = nextPathPage(
      data.pathPages,
      data.fileViewMode,
      history.route.directory,
    );
    const commit = history.route.commitId;
    if (page && commit)
      await loadFiles(commit, {
        path: page.path,
        recursive: page.recursive,
        more: true,
      });
  }
  return {
    loadFiles,
    loadBlob,
    setFileViewMode,
    toggleDirectory,
    selectDirectory,
    loadMoreFiles,
  };
}

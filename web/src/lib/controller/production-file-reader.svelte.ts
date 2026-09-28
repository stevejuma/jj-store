import type { FileListingViewMode } from "@lapismd/design-core/diff";
import { fullId } from "../api/client.js";
import type { StoreDataSource } from "./native-data-source.js";
import { presentPaths } from "./native-presentation.js";
import {
  nextPathPage,
  pathPageKey,
  storePathPage,
  visiblePathEntries,
} from "./production-file-pages.js";
import {
  readBlob,
  readLandingDocuments,
  readPaths,
} from "./production-store-loaders.js";
import type { ProductionStoreState } from "./production-store-state.svelte.js";
import type { createStoreRouteHistory } from "./store-route-history.svelte.js";

interface FileReaderContext {
  data: ProductionStoreState;
  history: ReturnType<typeof createStoreRouteHistory>;
  source(): StoreDataSource;
  operation(): string | null;
  version(): number;
  current(ticket: number, source: StoreDataSource): boolean;
  report(cause: unknown): void;
}

export function createProductionFileReader(context: FileReaderContext) {
  const { data, history } = context;
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
    if (!more && data.snapshot?.id !== commit) {
      data.pathPages = new Map();
      data.knownDirectories = new Set();
      data.collapsedDirectoryPaths = new Set();
    }
    const previous = data.pathPages.get(pathPageKey(path, recursive));
    if (more && !previous?.cursor) return false;
    data.status.files = more ? "refreshing" : "loading";
    try {
      const value = await readPaths(source, operation, commit, {
        path,
        recursive,
        cursor: more ? (previous?.cursor ?? undefined) : undefined,
        expected: more ? previous?.snapshot : undefined,
      });
      if (!context.current(ticket, source)) return false;
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
      const presented = presentPaths(
        commit,
        visiblePathEntries(data.pathPages, data.fileViewMode),
      );
      if (!recursive) rememberDirectories(presented.snapshot.directories ?? []);
      data.fileTerms = presented.files;
      data.snapshot = await readLandingDocuments(
        source,
        operation,
        presented.snapshot,
        presented.files,
      );
      if (!context.current(ticket, source)) return false;
      data.status.files =
        data.snapshot.files.length || data.snapshot.directories?.length
          ? "ready"
          : "empty";
      return true;
    } catch (cause) {
      if (!context.current(ticket, source)) return false;
      context.report(cause);
      data.status.files = "failed";
      return false;
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
    if (
      commit &&
      !data.pathPages.has(pathPageKey(path, false)) &&
      !(await loadFiles(commit, { path, recursive: false }))
    )
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

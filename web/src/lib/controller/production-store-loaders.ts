import { fullId, type Model, type Snapshot } from "../api/client.js";
import type { RepositoryLogGraph } from "../projects/repository-log.js";
import type { Bookmark, RevisionDetail } from "../projects/presentation.js";
import type {
  RepositoryBlob,
  RepositoryChoice,
  RepositorySnapshot,
} from "./contracts.js";
import type { StoreDataSource } from "./native-data-source.js";
import {
  startOperationInspection,
  type InspectionContinuation,
} from "./production-inspection-pages.js";
import { assertPinned, assertSameSnapshot } from "./production-snapshot.js";
import {
  presentChangedFiles,
  presentGraph,
  presentPatch,
  presentReferences,
} from "./native-presentation.js";

export interface Comparison {
  from: string;
  to: string;
  group?: string;
}

export async function readReferences(
  source: StoreDataSource,
  operation: string,
  cursor?: string,
  expected?: Snapshot,
): Promise<{
  bookmarks: Bookmark[];
  choices: RepositoryChoice[];
  cursor: string | null;
  snapshot: Snapshot;
}> {
  const value = await source.references(fullId(operation), cursor);
  assertPinned(value.snapshot, source, operation);
  assertSameSnapshot(value.snapshot, expected);
  return {
    ...presentReferences(value.data.references),
    cursor: value.next_cursor,
    snapshot: value.snapshot,
  };
}

export async function readGraph(
  source: StoreDataSource,
  operation: string,
  kind: "log" | "operations",
  heads: readonly string[],
  atTimeMillis: number,
  cursor?: string,
  expected?: Snapshot,
): Promise<{
  graph: RepositoryLogGraph;
  cursor: string | null;
  snapshot: Snapshot;
}> {
  const value = await source.graph(kind, {
    operation: fullId(operation),
    cursor,
    atTimeMillis,
  });
  assertPinned(value.snapshot, source, operation);
  assertSameSnapshot(value.snapshot, expected);
  return {
    graph: presentGraph(value.data, heads),
    cursor: value.next_cursor,
    snapshot: value.snapshot,
  };
}

export async function readPaths(
  source: StoreDataSource,
  operation: string,
  commit: string,
  query: {
    path: string;
    recursive: boolean;
    cursor?: string;
    expected?: Snapshot;
  },
): Promise<{
  entries: Model<"Entry">[];
  path: string;
  recursive: boolean;
  cursor: string | null;
  readSnapshot: Snapshot;
}> {
  const value = await source.paths(commit, operation, query);
  assertPinned(value.snapshot, source, operation);
  assertSameSnapshot(value.snapshot, query.expected);
  return {
    entries: value.data.entries,
    path: value.data.path,
    recursive: value.data.recursive,
    cursor: value.next_cursor,
    readSnapshot: value.snapshot,
  };
}

export async function readLandingDocuments(
  source: StoreDataSource,
  operation: string,
  snapshot: RepositorySnapshot,
  files: ReadonlyMap<string, string>,
): Promise<RepositorySnapshot> {
  const documents = [...files]
    .filter(([path]) => /(^|\/)(readme|license)(\.[^/]*)?$/iu.test(path))
    .slice(0, 4);
  const loaded = await Promise.all(
    documents.map(
      async ([path, id]) => [path, await source.file(id, operation)] as const,
    ),
  );
  const contents = new Map(loaded);
  return {
    ...snapshot,
    files: snapshot.files.map((file) => {
      const value = contents.get(file.path);
      return value
        ? { ...file, contents: value.text, binary: value.text === undefined }
        : file;
    }),
  };
}

export async function readBlob(
  source: StoreDataSource,
  operation: string,
  path: string,
  fileId: string,
): Promise<RepositoryBlob> {
  const value = await source.file(fileId, operation);
  return { path, contents: value.text, binary: value.text === undefined };
}

export async function readCommitInspection(
  source: StoreDataSource,
  operation: string,
  id: string,
): Promise<{
  detail: RevisionDetail;
  comparisons: Map<string, Comparison>;
  conflicts: Model<"ConflictDescriptor">[];
  continuation: InspectionContinuation | null;
}> {
  const value = await source.commit(id, operation);
  assertPinned(value.snapshot, source, operation);
  const parent = value.data.commit.parents[0];
  const [paths, conflicts] = await Promise.all([
    parent ? source.diffPaths(operation, parent, id) : null,
    source.conflicts(operation, id),
  ]);
  if (paths) assertPinned(paths.snapshot, source, operation);
  assertPinned(conflicts.snapshot, source, operation);
  const changes = paths?.data.changes ?? [];
  return {
    detail: { files: presentChangedFiles(changes), patches: {} },
    comparisons: new Map(
      changes.map((change) => [change.path, { from: parent!, to: id }]),
    ),
    conflicts: conflicts.data.conflicts,
    continuation:
      parent && paths?.next_cursor
        ? {
            kind: "commit",
            operation,
            from: parent,
            to: id,
            cursor: paths.next_cursor,
            snapshot: paths.snapshot,
          }
        : null,
  };
}

export async function readOperationInspection(
  source: StoreDataSource,
  id: string,
  parent?: string,
): Promise<{
  detail: RevisionDetail;
  comparisons: Map<string, Comparison>;
  parents: string[];
  selectedParent: string | null;
  unavailableComparisons: number;
  continuation: InspectionContinuation | null;
}> {
  const [operation, difference] = await Promise.all([
    source.operation(id),
    source.operationDiff(id, parent),
  ]);
  assertPinned(operation.snapshot, source, id);
  assertPinned(difference.snapshot, source, id);
  if (difference.data.data.status === "select_parent")
    return {
      detail: { files: [], patches: {} },
      comparisons: new Map(),
      parents: difference.data.data.parents,
      selectedParent: null,
      unavailableComparisons: 0,
      continuation: null,
    };
  if (difference.data.data.status !== "difference")
    return {
      detail: { files: [], patches: {} },
      comparisons: new Map(),
      parents: [],
      selectedParent: null,
      unavailableComparisons: 0,
      continuation: null,
    };
  const selectedParent = difference.data.data.parent;
  const groups = await source.operationChanges(id, selectedParent);
  const page = await startOperationInspection(
    source,
    id,
    selectedParent,
    groups,
  );
  return {
    detail: { files: page.files, patches: {} },
    comparisons: page.comparisons,
    parents: [],
    selectedParent,
    unavailableComparisons: page.unavailableComparisons,
    continuation: page.continuation,
  };
}

export async function readPatch(
  source: StoreDataSource,
  operation: string,
  path: string,
  comparison: Comparison,
): Promise<string | undefined> {
  const value = await source.diff(
    operation,
    comparison.from,
    comparison.to,
    path,
  );
  assertPinned(value.snapshot, source, operation);
  const change = value.data.changes.find((item) => item.path === path);
  return change ? presentPatch(change) : undefined;
}

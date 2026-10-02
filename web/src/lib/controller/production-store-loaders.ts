import type { CodeSnapshot as Snapshot } from "./repository-profile.js";
import { fullId, type Model } from "../api/client.js";
import type { RepositoryLogGraph } from "../projects/repository-log.js";
import type { RepositoryWorkingCopy } from "../projects/repository-log.js";
import type { Bookmark, RevisionDetail } from "../projects/presentation.js";
import type { RepositoryBlob, RepositoryChoice } from "./contracts.js";
import type { StoreDataSource } from "./native-data-source.js";
import {
  startOperationInspection,
  type InspectionContinuation,
} from "./production-inspection-pages.js";
import { assertPinned, assertSameSnapshot } from "./production-snapshot.js";
import {
  presentChangedFiles,
  presentCommit,
  presentGraph,
  presentPatch,
  presentReferences,
} from "./native-presentation.js";
export { readLandingDocuments } from "./production-landing-documents.js";

export async function readRevision(
  source: StoreDataSource,
  operation: string,
  id: string,
) {
  const value = await source.commit(id, operation);
  assertPinned(value.snapshot, source, operation);
  return presentCommit({
    commit: value.data.commit,
    edges: value.data.commit.parents.map((target) => ({
      target,
      kind: "direct" as const,
    })),
  });
}

export interface Comparison {
  from: string;
  to: string;
  group?: string;
  beforeOperation: string;
  afterOperation: string;
}

export async function readReferences(
  source: StoreDataSource,
  operation: string,
  cursor?: string,
  expected?: Snapshot,
): Promise<{
  bookmarks: Bookmark[];
  choices: RepositoryChoice[];
  workingCopies: RepositoryWorkingCopy[];
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
  targets?: readonly string[],
  workspace?: string,
  query?: { text: string; email: string; signal?: AbortSignal },
): Promise<{
  graph: RepositoryLogGraph;
  cursor: string | null;
  snapshot: Snapshot;
}> {
  const value = await source.graph(kind, {
    operation: fullId(operation),
    cursor,
    atTimeMillis,
    workspace: kind === "log" ? workspace : undefined,
    userEmail: query?.email,
    signal: query?.signal,
    operationQuery: kind === "operations" ? query?.text : undefined,
    revset:
      query?.text && kind === "log"
        ? query.text
        : kind === "log" && targets
          ? targets.length
            ? targets
                .map(
                  (id) =>
                    `::${fullId(id, source.profile === "git_sha1_jj045" ? 40 : 128)}`,
                )
                .join(" | ")
            : "none()"
          : undefined,
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
  entries: import("./repository-read-types.js").CodeEntry[];
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
  conflicts: import("./repository-read-types.js").CodeConflict[];
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
      changes.map((change) => [
        change.path,
        {
          from: parent!,
          to: id,
          beforeOperation: operation,
          afterOperation: operation,
        },
      ]),
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
    comparisons: new Map(
      [...page.comparisons].map(([path, comparison]) => [
        path,
        {
          ...comparison,
          beforeOperation: selectedParent,
          afterOperation: id,
        },
      ]),
    ),
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

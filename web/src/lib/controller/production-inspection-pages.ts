import type { Model, Snapshot } from "../api/client.js";
import type { StoreDataSource } from "./native-data-source.js";
import { presentChangedFiles } from "./native-presentation.js";
import type { Comparison } from "./production-store-loaders.js";
import { assertPinned, assertSameSnapshot } from "./production-snapshot.js";

type ChangeGroup = Model<"Group">;

export interface CommitInspectionContinuation {
  kind: "commit";
  operation: string;
  from: string;
  to: string;
  cursor: string;
  snapshot: Snapshot;
}

interface OperationPathContinuation {
  group: string;
  from: string;
  to: string;
  cursor: string;
}

export interface OperationInspectionContinuation {
  kind: "operation";
  operation: string;
  parent: string;
  groupsCursor: string | null;
  snapshot: Snapshot;
  pendingGroups: ChangeGroup[];
  path: OperationPathContinuation | null;
}

export type InspectionContinuation =
  CommitInspectionContinuation | OperationInspectionContinuation;

export interface InspectionPage {
  files: ReturnType<typeof presentChangedFiles>;
  comparisons: Map<string, Comparison>;
  unavailableComparisons: number;
  continuation: InspectionContinuation | null;
}

function comparisonEntries(
  changes: readonly Model<"FileDiffChange">[],
  comparison: Comparison,
) {
  return new Map(changes.map((change) => [change.path, comparison]));
}

export async function readMoreCommitInspection(
  source: StoreDataSource,
  continuation: CommitInspectionContinuation,
): Promise<InspectionPage> {
  const page = await source.diffPaths(
    continuation.operation,
    continuation.from,
    continuation.to,
    continuation.cursor,
  );
  assertPinned(page.snapshot, source, continuation.operation);
  assertSameSnapshot(page.snapshot, continuation.snapshot);
  const changes = page.data.changes;
  return {
    files: presentChangedFiles(changes),
    comparisons: comparisonEntries(changes, {
      from: continuation.from,
      to: continuation.to,
      beforeOperation: continuation.operation,
      afterOperation: continuation.operation,
    }),
    unavailableComparisons: 0,
    continuation: page.next_cursor
      ? { ...continuation, cursor: page.next_cursor }
      : null,
  };
}

async function readGroupPaths(
  source: StoreDataSource,
  continuation: OperationInspectionContinuation,
  group: string,
  from: string,
  to: string,
  cursor?: string,
): Promise<InspectionPage> {
  const page = await source.diffPaths(continuation.operation, from, to, cursor);
  assertPinned(page.snapshot, source, continuation.operation);
  assertSameSnapshot(page.snapshot, continuation.snapshot);
  const changes = page.data.changes;
  return {
    files: presentChangedFiles(changes),
    comparisons: comparisonEntries(changes, {
      from,
      to,
      group,
      beforeOperation: continuation.parent,
      afterOperation: continuation.operation,
    }),
    unavailableComparisons: 0,
    continuation: {
      ...continuation,
      path: page.next_cursor
        ? { group, from, to, cursor: page.next_cursor }
        : null,
    },
  };
}

export async function readMoreOperationInspection(
  source: StoreDataSource,
  continuation: OperationInspectionContinuation,
): Promise<InspectionPage> {
  let current = continuation;
  let unavailableComparisons = 0;
  while (true) {
    if (current.path) {
      const { group, from, to, cursor } = current.path;
      const page = await readGroupPaths(
        source,
        current,
        group,
        from,
        to,
        cursor,
      );
      if (page.files.length || page.continuation?.kind !== "operation")
        return { ...page, unavailableComparisons };
      current = page.continuation;
      continue;
    }
    const pending = [...current.pendingGroups];
    let exact: Extract<ChangeGroup["comparison"], { status: "exact" }> | null =
      null;
    let change = "";
    while (pending.length && !exact) {
      const group = pending.shift()!;
      if (group.comparison.status === "exact") {
        exact = group.comparison;
        change = group.change;
      } else unavailableComparisons += 1;
    }
    if (exact) {
      const page = await readGroupPaths(
        source,
        { ...current, pendingGroups: pending },
        change,
        exact.from,
        exact.to,
      );
      if (page.files.length || page.continuation?.kind !== "operation")
        return { ...page, unavailableComparisons };
      current = page.continuation;
      continue;
    }
    if (!current.groupsCursor)
      return {
        files: [],
        comparisons: new Map(),
        unavailableComparisons,
        continuation: null,
      };
    const groups = await source.operationChanges(
      current.operation,
      current.parent,
      current.groupsCursor,
    );
    assertPinned(groups.snapshot, source, current.operation);
    assertSameSnapshot(groups.snapshot, current.snapshot);
    current = {
      ...current,
      groupsCursor: groups.next_cursor,
      pendingGroups: groups.data.groups,
    };
  }
}

export async function startOperationInspection(
  source: StoreDataSource,
  operation: string,
  parent: string,
  groups: Model<"Window2">,
): Promise<InspectionPage> {
  assertPinned(groups.snapshot, source, operation);
  return readMoreOperationInspection(source, {
    kind: "operation",
    operation,
    parent,
    groupsCursor: groups.next_cursor,
    snapshot: groups.snapshot,
    pendingGroups: groups.data.groups,
    path: null,
  });
}

export function normalizeInspectionContinuation(
  continuation: InspectionContinuation | null,
): InspectionContinuation | null {
  if (!continuation || continuation.kind === "commit") return continuation;
  return continuation.path ||
    continuation.pendingGroups.length ||
    continuation.groupsCursor
    ? continuation
    : null;
}

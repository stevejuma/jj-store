import assert from "node:assert/strict";
import test from "node:test";
import type { Model, Snapshot } from "../src/lib/api/client.js";
import type { StoreDataSource } from "../src/lib/controller/native-data-source.js";
import {
  normalizeInspectionContinuation,
  readMoreCommitInspection,
  readMoreOperationInspection,
  startOperationInspection,
} from "../src/lib/controller/production-inspection-pages.js";

const repository = "ab".repeat(16);
const operation = "cd".repeat(64);
const parent = "de".repeat(64);
const beforeA = "12".repeat(64);
const afterA = "34".repeat(64);
const beforeB = "56".repeat(64);
const afterB = "78".repeat(64);
const changeA = "9a".repeat(16);
const changeB = "bc".repeat(16);

const snapshot = (): Snapshot => ({
  version: 1,
  repo_id: repository,
  epoch: "epoch-a",
  published_seq: 1,
  manifest_revision: 1,
  code_state_version: 1,
  operation_id: operation,
  heads: [operation],
});

const stats = {
  visited_objects: 1,
  visited_bytes: 1,
  elapsed_millis: 1,
  cache_hits: 0,
};

function change(path: string): Model<"FileDiffChange"> {
  return {
    path,
    before: [],
    after: [],
    sources: [],
    content: { status: "unavailable", reason: "not_requested" },
  };
}

function pathPage(
  from: string,
  to: string,
  changes: Model<"FileDiffChange">[],
  next: string | null,
): Model<"Window4"> {
  return {
    snapshot: snapshot(),
    data: {
      version: 1,
      from,
      to,
      path: "",
      changes,
      previous: null,
      next,
      stats,
    },
    previous_cursor: null,
    next_cursor: next,
  };
}

function groupPage(
  groups: Model<"Group">[],
  next: string | null,
): Model<"Window2"> {
  return {
    snapshot: snapshot(),
    data: {
      version: 1,
      operation,
      parent,
      groups,
      previous: null,
      next,
      stats,
    },
    previous_cursor: null,
    next_cursor: next,
  };
}

function source(calls: {
  groups: string[];
  paths: { from: string; to: string; cursor?: string }[];
}): StoreDataSource {
  return {
    repository,
    operationChanges: async (_operation, _parent, cursor) => {
      calls.groups.push(cursor ?? "");
      assert.equal(cursor, "groups-2");
      return groupPage(
        [
          {
            change: changeB,
            before: [beforeB],
            after: [afterB],
            comparison: { status: "exact", from: beforeB, to: afterB },
          },
        ],
        null,
      );
    },
    diffPaths: async (_operation, from, to, cursor) => {
      calls.paths.push({ from, to, cursor });
      if (from === beforeA && !cursor)
        return pathPage(from, to, [change("src/one.rs")], "paths-2");
      if (from === beforeA && cursor === "paths-2")
        return pathPage(from, to, [change("src/two.rs")], null);
      assert.equal(from, beforeB);
      return pathPage(from, to, [change("README.md")], null);
    },
  } as StoreDataSource;
}

test("commit inspection continues opaque changed-path pages", async () => {
  const calls = { groups: [] as string[], paths: [] as never[] };
  const page = await readMoreCommitInspection(source(calls), {
    kind: "commit",
    operation,
    from: beforeA,
    to: afterA,
    cursor: "paths-2",
    snapshot: snapshot(),
  });
  assert.deepEqual(page.files, [
    { path: "src/two.rs", additions: 0, deletions: 0 },
  ]);
  assert.deepEqual(page.comparisons.get("src/two.rs"), {
    from: beforeA,
    to: afterA,
  });
  assert.equal(page.continuation, null);
});

test("operation inspection pages groups and every exact comparison", async () => {
  const calls = {
    groups: [] as string[],
    paths: [] as { from: string; to: string; cursor?: string }[],
  };
  const data = source(calls);
  const first = await startOperationInspection(
    data,
    operation,
    parent,
    groupPage(
      [
        {
          change: changeA,
          before: [beforeA],
          after: [afterA],
          comparison: { status: "exact", from: beforeA, to: afterA },
        },
        {
          change: "ef".repeat(16),
          before: [beforeA],
          after: [afterB],
          comparison: { before: [beforeA], after: [afterB] },
        },
      ],
      "groups-2",
    ),
  );
  assert.deepEqual(
    first.files.map((file) => file.path),
    ["src/one.rs"],
  );
  assert.equal(first.comparisons.get("src/one.rs")?.group, changeA);

  assert(first.continuation?.kind === "operation");
  const second = await readMoreOperationInspection(data, first.continuation);
  assert.deepEqual(
    second.files.map((file) => file.path),
    ["src/two.rs"],
  );

  assert(second.continuation?.kind === "operation");
  const final = await readMoreOperationInspection(data, second.continuation);
  assert.deepEqual(
    final.files.map((file) => file.path),
    ["README.md"],
  );
  assert.equal(final.unavailableComparisons, 1);
  assert.deepEqual(calls.groups, ["groups-2"]);
  assert.equal(final.comparisons.get("README.md")?.group, changeB);
  assert.equal(normalizeInspectionContinuation(final.continuation), null);
  assert.deepEqual(calls.paths, [
    { from: beforeA, to: afterA, cursor: undefined },
    { from: beforeA, to: afterA, cursor: "paths-2" },
    { from: beforeB, to: afterB, cursor: undefined },
  ]);
});

import assert from "node:assert/strict";
import test from "node:test";
import type {
  Envelope,
  Head,
  Model,
  Snapshot,
  Task,
} from "../src/lib/api/client.js";
import { ApiError } from "../src/lib/api/client.js";
import type {
  GraphQuery,
  StoreDataSource,
} from "../src/lib/controller/native-data-source.js";
import {
  presentChangedFiles,
  presentGraph,
  presentPatch,
  presentPaths,
  presentReferences,
} from "../src/lib/controller/native-presentation.js";
import {
  readGraph,
  readLandingDocuments,
  readOperationInspection,
  readReferences,
} from "../src/lib/controller/production-store-loaders.js";
import { repositoryLandingDocumentLabel } from "../src/lib/ui/community-projects/repository-browser.js";
import { assertSameSnapshot } from "../src/lib/controller/production-snapshot.js";
import {
  nextPathPage,
  storePathPage,
  visiblePathEntries,
} from "../src/lib/controller/production-file-pages.js";

const repository = "ab".repeat(16);
const operation = "cd".repeat(64);
const parent = "de".repeat(64);
const before = "12".repeat(64);
const after = "34".repeat(64);
const change = "56".repeat(16);
const file = "78".repeat(64);
const signature = {
  name: "Maya Chen",
  email: "maya@example.test",
  timestamp: { millis: 60_000, tz_offset: 0 },
};
const stats = {
  visited_objects: 1,
  visited_bytes: 1,
  elapsed_millis: 1,
  cache_hits: 0,
};

function snapshot(id = operation): Snapshot {
  return {
    version: 1,
    repo_id: repository,
    epoch: "epoch-a",
    published_seq: 1,
    manifest_revision: 1,
    code_state_version: 1,
    operation_id: id,
    heads: [id],
  };
}

function source(overrides: Partial<StoreDataSource> = {}): StoreDataSource {
  const unavailable = async (): Promise<never> => {
    throw new Error("unexpected data-source call");
  };
  return {
    repository,
    capabilities: unavailable,
    snapshot: unavailable as (fresh?: boolean) => Promise<Head>,
    references: unavailable,
    graph: unavailable as (
      kind: "log" | "operations" | "evolution",
      query: GraphQuery,
    ) => Promise<Envelope<import("../src/lib/api/client.js").Graph>>,
    commit: unavailable,
    paths: unavailable,
    file: unavailable,
    operation: unavailable,
    operationDiff: unavailable,
    operationChanges: unavailable,
    diffPaths: unavailable,
    diff: unavailable,
    conflicts: unavailable,
    prepareIndex: unavailable as (operation: string) => Promise<Task>,
    cancelIndexTask: unavailable as (id: string) => Promise<Task>,
    indexTask: unavailable as (id: string) => Promise<Task>,
    events: unavailable,
    close() {},
    ...overrides,
  };
}

test("native presentation preserves conflicting targets and exact identifiers", () => {
  const references = presentReferences([
    {
      kind: "bookmark",
      name: "main",
      terms: [after, null, before],
      conflicted: true,
      workspaces: ["default"],
      targets: [
        {
          commit: after,
          change,
          parents: [before],
          description: "Native history",
          author: signature,
          committer: signature,
          conflicted: true,
          workspaces: ["default"],
        },
      ],
    },
  ] as Model<"Reference">[]);

  assert.deepEqual(references.bookmarks[0]?.targets, [after, before]);
  assert.deepEqual(references.choices[0]?.targets, [after, before]);
  assert.equal(references.bookmarks[0]?.updatedAt, 60);
  assert.deepEqual(references.workingCopies, [
    {
      workspace: "default",
      commitId: after,
      changeId: change,
      message: "Native history",
      createdAt: 60,
      author: "Maya Chen",
      authorEmail: "maya@example.test",
    },
  ]);

  const paths = presentPaths(after, [
    {
      path: "src",
      terms: [{ type: "tree", id: before }],
      conflicted: false,
      last_change: { status: "unavailable", reason: "directory" },
    },
    {
      path: "src/lib.rs",
      terms: [{ type: "file", id: file, executable: false }],
      conflicted: false,
      last_change: { status: "unavailable", reason: "budget" },
    },
  ] as Model<"Entry">[]);
  assert.deepEqual(paths.snapshot.directories, [{ path: "src" }]);
  assert.deepEqual(paths.snapshot.files, [{ path: "src/lib.rs" }]);
  assert.equal(paths.files.get("src/lib.rs"), file);
});

test("empty working copies use the native no-description label", () => {
  const references = presentReferences([
    {
      kind: "bookmark",
      name: "main",
      terms: [after],
      conflicted: false,
      workspaces: ["default"],
      targets: [
        {
          commit: after,
          change,
          parents: [before],
          description: "",
          author: signature,
          committer: signature,
          conflicted: false,
          workspaces: ["default"],
        },
      ],
    },
  ] as Model<"Reference">[]);

  assert.equal(
    references.workingCopies[0]?.message,
    "(empty) (no description set)",
  );
});

test("repository landing documents include community files in stable tab order", async () => {
  const requested: string[] = [];
  let activeReads = 0;
  let maximumActiveReads = 0;
  const ids = new Map([
    ["SECURITY.md", "01".repeat(64)],
    ["LICENCE", "02".repeat(64)],
    ["LICENSE.md", "07".repeat(64)],
    ["CONTRIBUTING.md", "03".repeat(64)],
    ["CODE_OF_CONDUCT.md", "04".repeat(64)],
    ["README.markdown", "05".repeat(64)],
    ["docs/README.md", "06".repeat(64)],
  ]);
  const dataSource = source({
    file: async (id) => {
      requested.push(id);
      activeReads += 1;
      maximumActiveReads = Math.max(maximumActiveReads, activeReads);
      await new Promise((resolve) => setTimeout(resolve, 0));
      activeReads -= 1;
      return { text: `contents:${id}` };
    },
  });
  const loaded = await readLandingDocuments(
    dataSource,
    operation,
    {
      id: after,
      files: [...ids].map(([path]) => ({ path })),
    },
    ids,
  );
  assert.deepEqual(requested, [
    ids.get("README.markdown"),
    ids.get("CODE_OF_CONDUCT.md"),
    ids.get("CONTRIBUTING.md"),
    ids.get("LICENCE"),
    ids.get("SECURITY.md"),
  ]);
  assert.equal(maximumActiveReads, 1);
  assert.deepEqual(
    loaded.files
      .filter((candidate) => candidate.contents)
      .map((candidate) => repositoryLandingDocumentLabel(candidate.path)),
    ["Security", "License", "Contributing", "Code of conduct", "README"],
  );
  assert.equal(repositoryLandingDocumentLabel("LICENSE.txt"), "License");
  assert.equal(
    repositoryLandingDocumentLabel("CODE_OF_CONDUCT"),
    "Code of conduct",
  );
  assert.equal(repositoryLandingDocumentLabel("docs/SECURITY.md"), null);
});

test("path pages isolate lazy directories from recursive listing views", () => {
  const root = {
    path: "src",
    terms: [{ type: "tree", id: before }],
    conflicted: false,
  } as Model<"Entry">;
  const child = {
    path: "src/lib.rs",
    terms: [{ type: "file", id: file, executable: false }],
    conflicted: false,
  } as Model<"Entry">;
  let pages = new Map();
  pages = storePathPage(
    pages,
    {
      path: "",
      recursive: false,
      entries: [root],
      cursor: "root-next",
      snapshot: snapshot(),
    },
    false,
  );
  pages = storePathPage(
    pages,
    {
      path: "src",
      recursive: false,
      entries: [child],
      cursor: "src-next",
      snapshot: snapshot(),
    },
    false,
  );
  pages = storePathPage(
    pages,
    {
      path: "",
      recursive: true,
      entries: [child],
      cursor: "recursive-next",
      snapshot: snapshot(),
    },
    false,
  );

  assert.deepEqual(visiblePathEntries(pages, "tree"), [root, child]);
  assert.deepEqual(visiblePathEntries(pages, "list"), [child]);
  assert.equal(nextPathPage(pages, "tree", "src")?.cursor, "src-next");
  assert.equal(nextPathPage(pages, "package", "")?.cursor, "recursive-next");
});

test("graph and diff presentation retain native edges and text statistics", () => {
  const graph = presentGraph(
    {
      version: 1,
      data: {
        graph: "log",
        nodes: [
          {
            commit: {
              id: after,
              change_id: change,
              parents: [before],
              tree_terms: [file],
              description: "Native history",
              author: signature,
              committer: signature,
              predecessors: [],
              conflicted: true,
              workspaces: ["default"],
              bookmarks: [
                {
                  name: "feature/native",
                  remote: null,
                  added: true,
                  removed: false,
                  conflicted: false,
                },
                {
                  name: "feature/native",
                  remote: "origin",
                  added: true,
                  removed: false,
                  conflicted: false,
                },
              ],
              visible: true,
              divergent: true,
            },
            edges: [{ target: before, kind: "indirect" }],
          },
        ],
      },
      stats,
    } as Model<"GraphPage">,
    [operation],
  );
  assert.deepEqual(graph.revisions[0]?.parents, [before]);
  assert.equal(graph.revisions[0]?.changeId, change);
  assert.deepEqual(graph.revisions[0]?.workspaces, ["default"]);
  assert.equal(graph.revisions[0]?.conflicted, true);
  assert.equal(graph.revisions[0]?.divergent, true);
  assert.deepEqual(graph.revisions[0]?.bookmarks, [
    {
      name: "feature/native",
      remote: undefined,
      added: true,
      removed: false,
      conflicted: false,
    },
    {
      name: "feature/native",
      remote: "origin",
      added: true,
      removed: false,
      conflicted: false,
    },
  ]);

  const changed = {
    path: "src/lib.rs",
    before: [],
    after: [],
    sources: [],
    content: {
      status: "text",
      hunks: [
        {
          kind: "different",
          before_start: 0,
          after_start: 0,
          before: "old\nvalue\n",
          after: "new\nvalue\nextra\n",
        },
      ],
    },
  } as Model<"FileDiffChange">;
  assert.deepEqual(presentChangedFiles([changed]), [
    { path: "src/lib.rs", additions: 3, deletions: 2 },
  ]);
  assert.match(presentPatch(changed) ?? "", /@@ -1,2 \+1,3 @@/u);
});

test("native byte offsets become one-based unified diff line coordinates", () => {
  const changed = {
    path: "src/lib.rs",
    before: [],
    after: [],
    sources: [],
    content: {
      status: "text",
      hunks: [
        {
          kind: "matching",
          before_start: 0,
          after_start: 0,
          before: "one\n",
          after: "one\n",
        },
        {
          kind: "different",
          before_start: 4,
          after_start: 4,
          before: "two\n",
          after: "TWO\n",
        },
        {
          kind: "matching",
          before_start: 8,
          after_start: 8,
          before: "three\n",
          after: "three\n",
        },
        {
          kind: "different",
          before_start: 14,
          after_start: 14,
          before: "",
          after: "inserted\n",
        },
        {
          kind: "matching",
          before_start: 14,
          after_start: 23,
          before: "four\n",
          after: "four\n",
        },
      ],
    },
  } as Model<"FileDiffChange">;

  assert.match(
    presentPatch(changed) ?? "",
    /@@ -1,1 \+1,1 @@[\s\S]*@@ -2,1 \+2,1 @@[\s\S]*@@ -3,1 \+3,1 @@[\s\S]*@@ -3,0 \+4,1 @@[\s\S]*@@ -4,1 \+5,1 @@/u,
  );
});

test("branch graph requests bind exact target ancestry", async () => {
  const revsets: (string | undefined)[] = [];
  const graphPage = {
    snapshot: snapshot(),
    data: {
      version: 1,
      data: { graph: "log", nodes: [] },
      stats,
    },
    next_cursor: null,
  } as unknown as Envelope<import("../src/lib/api/client.js").Graph>;
  const dataSource = source({
    graph: async (_kind, query) => {
      revsets.push(query.revset);
      return graphPage;
    },
  });
  await readGraph(dataSource, operation, "log", [], 1, undefined, undefined, [
    after,
    before,
  ]);
  await readGraph(
    dataSource,
    operation,
    "log",
    [],
    1,
    undefined,
    undefined,
    [],
  );
  assert.deepEqual(revsets, [`::${after} | ::${before}`, "none()"]);
});

test("paged reads reject a continuation from another captured snapshot", async () => {
  const expected = snapshot();
  const changed = { ...snapshot(), manifest_revision: 2 };
  assert.throws(() => assertSameSnapshot(changed, expected), {
    name: "Error",
    message: "pagination snapshot mismatch",
  });

  await assert.rejects(
    readReferences(
      source({
        references: async () =>
          ({
            snapshot: changed,
            data: { references: [] },
            previous_cursor: null,
            next_cursor: null,
          }) as Model<"Window">,
      }),
      operation,
      "opaque-cursor",
      expected,
    ),
    (cause) =>
      cause instanceof ApiError &&
      cause.code === "pagination_snapshot_mismatch",
  );
});

test("ambiguous operations require a parent before loading change groups", async () => {
  let groups = 0;
  const inspected = await readOperationInspection(
    source({
      operation: async () =>
        ({ snapshot: snapshot(), data: {} }) as Envelope<
          Model<"OperationDetail">
        >,
      operationDiff: async () =>
        ({
          snapshot: snapshot(),
          data: {
            version: 1,
            data: {
              status: "select_parent",
              operation,
              parents: [parent, before],
            },
            stats,
          },
        }) as Envelope<Model<"OperationDiffResponse">>,
      operationChanges: async () => {
        groups += 1;
        throw new Error("parent was not selected");
      },
    }),
    operation,
  );

  assert.deepEqual(inspected.parents, [parent, before]);
  assert.equal(inspected.selectedParent, null);
  assert.equal(groups, 0);
});

test("explicit operation parent maps each changed path to its comparison", async () => {
  const changedPath = {
    path: "src/lib.rs",
    before: [],
    after: [],
    sources: [],
    content: { status: "unavailable", reason: "not_requested" },
  } as Model<"FileDiffChange">;
  const inspected = await readOperationInspection(
    source({
      operation: async () =>
        ({ snapshot: snapshot(), data: {} }) as Envelope<
          Model<"OperationDetail">
        >,
      operationDiff: async (_id, selected) => {
        assert.equal(selected, parent);
        return {
          snapshot: snapshot(),
          data: {
            version: 1,
            data: {
              status: "difference",
              operation,
              parent,
              view: file,
              parent_view: before,
              delta: {},
            },
            stats,
          },
        } as Envelope<Model<"OperationDiffResponse">>;
      },
      operationChanges: async () =>
        ({
          snapshot: snapshot(),
          data: {
            groups: [
              {
                change,
                before: [before],
                after: [after],
                comparison: { status: "exact", from: before, to: after },
              },
            ],
          },
        }) as Model<"Window2">,
      diffPaths: async () =>
        ({
          snapshot: snapshot(),
          data: { changes: [changedPath] },
        }) as Model<"Window4">,
    }),
    operation,
    parent,
  );

  assert.equal(inspected.selectedParent, parent);
  assert.deepEqual(inspected.detail.files, [
    { path: "src/lib.rs", additions: 0, deletions: 0 },
  ]);
  assert.deepEqual(inspected.comparisons.get("src/lib.rs"), {
    from: before,
    to: after,
    group: change,
    beforeOperation: parent,
    afterOperation: operation,
  });
});

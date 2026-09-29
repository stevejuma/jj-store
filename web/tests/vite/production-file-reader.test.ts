import { expect, test } from "vitest";
import type { StoreDataSource } from "../../src/lib/controller/native-data-source.js";
import { createProductionFileReader } from "../../src/lib/controller/production-file-reader.svelte.js";
import { createProductionStoreState } from "../../src/lib/controller/production-store-state.svelte.js";
import { createStoreRouteHistory } from "../../src/lib/controller/store-route-history.svelte.js";

const repository = "11".repeat(16);
const operation = "12".repeat(64);
const commit = "13".repeat(64);
const change = "14".repeat(16);
const snapshot = {
  version: 1,
  repo_id: repository,
  epoch: "15".repeat(16),
  published_seq: 2,
  manifest_revision: 3,
  code_state_version: 4,
  operation_id: operation,
  heads: [operation],
};
const stats = {
  visited_objects: 1,
  visited_bytes: 1,
  elapsed_millis: 1,
  cache_hits: 0,
};

test("files load the selected commit metadata independently of graph history", async () => {
  const data = createProductionStoreState();
  const history = createStoreRouteHistory(undefined);
  history.connect(repository, operation);
  history.push({
    ...history.route,
    page: "files",
    commitId: commit,
    changeId: change,
  });
  let commitReads = 0;
  const source = {
    repository,
    async paths() {
      return {
        snapshot,
        data: {
          version: 1,
          commit,
          path: "",
          recursive: true,
          entries: [],
          previous: null,
          next: null,
          stats,
        },
        previous_cursor: null,
        next_cursor: null,
      };
    },
    async commit() {
      commitReads++;
      return {
        snapshot,
        data: {
          object: { kind: "commit", id: commit },
          commit: {
            id: commit,
            change_id: change,
            parents: ["16".repeat(64)],
            description: "Expose exact file metadata",
            author: {
              name: "Maya Chen",
              email: "maya@example.test",
              timestamp: { millis: 1_796_000_000_000, offset_minutes: 0 },
            },
            committer: {
              name: "Maya Chen",
              email: "maya@example.test",
              timestamp: { millis: 1_796_000_000_000, offset_minutes: 0 },
            },
            tree_terms: [],
            conflicted: false,
            bookmarks: [],
            workspaces: [],
            visible: true,
            divergent: false,
          },
        },
        next_cursor: null,
      };
    },
  } as unknown as StoreDataSource;
  const reader = createProductionFileReader({
    data,
    history,
    source: () => source,
    operation: () => operation,
    version: () => 1,
    current: () => true,
    report: (cause) => {
      throw cause;
    },
  });

  await reader.loadFiles(commit);

  expect(commitReads).toBe(1);
  expect(data.fileRevision).toMatchObject({
    eventId: commit,
    changeId: change,
    message: "Expose exact file metadata",
    author: "Maya Chen",
  });
});

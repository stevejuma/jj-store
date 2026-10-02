import { expect, test, vi } from "vitest";
import { createStoreRouteHistory } from "../../src/lib/controller/store-route-history.svelte.js";
import {
  workspaceSelectedRoute,
  readWorkingCopies,
} from "../../src/lib/controller/store-workspaces.js";
import { readReferences } from "../../src/lib/controller/production-store-loaders.js";
import { createProductionStoreState } from "../../src/lib/controller/production-store-state.svelte.js";
import { restoreProductionStoreRoute } from "../../src/lib/controller/production-route-restorer.js";
import { createProductionBranchActions } from "../../src/lib/controller/production-branch-actions.js";
import type { StoreDataSource } from "../../src/lib/controller/native-data-source.js";
import type { RepositoryWorkingCopy } from "../../src/lib/projects/repository-log.js";

const repository = "ab".repeat(16),
  operation = "cd".repeat(64),
  commit = "12".repeat(64);
const copy: RepositoryWorkingCopy = {
  workspace: "default",
  commitId: commit,
  changeId: "34".repeat(16),
  message: "Working copy",
  createdAt: 1,
  author: "Maya",
};
const copies = [copy, { ...copy, workspace: "agent/review" }];
const snapshot = {
  version: 1,
  repo_id: repository,
  epoch: "epoch",
  published_seq: 0,
  manifest_revision: 0,
  operation_id: operation,
  heads: [operation],
};
const signature = {
  name: "Maya",
  email: "maya@example.invalid",
  timestamp: { millis: 1, tz_offset: 0 },
};
function window(workspaces: string[], cursor: string | null = null) {
  return {
    snapshot,
    data: {
      references: [
        {
          kind: "change",
          name: "",
          change: copy.changeId,
          terms: [commit],
          workspaces,
          targets: [
            {
              commit,
              change: copy.changeId,
              description: "Working copy",
              author: signature,
              committer: signature,
              workspaces,
            },
          ],
        },
      ],
    },
    next_cursor: cursor,
  };
}
function harness(workspace: string | null) {
  const data = createProductionStoreState();
  const history = createStoreRouteHistory(undefined);
  history.connect(repository, operation);
  history.replace({ ...history.route, page: "files", workspace });
  const reads = {
    loadReferences: vi.fn(async () => {
      data.workingCopies = copies;
      data.status.bookmarks = "ready";
    }),
    loadFiles: vi.fn(async () => true),
    loadBlob: vi.fn(),
    loadCommit: vi.fn(),
    loadOperation: vi.fn(),
    loadPatch: vi.fn(),
    loadGraph: vi.fn(),
    loadAgents: vi.fn(),
    loadAgent: vi.fn(),
    loadReviews: vi.fn(),
    loadReview: vi.fn(),
  };
  return { data, history, reads };
}

test("workspace context resolves the working copy and survives tab, history and reload", async () => {
  const { data, history, reads } = harness("agent/review");
  await restoreProductionStoreRoute(history.route, data, history, reads);
  expect(reads.loadFiles).toHaveBeenCalledWith(commit);
  expect(history.route).toMatchObject({
    workspace: "agent/review",
    commitId: null,
    snapshotOperationId: operation,
  });
  history.push(workspaceSelectedRoute(history.route, "default"));
  history.navigate(-1);
  expect(history.route.workspace).toBe("agent/review");
  history.navigate(1);
  expect(history.route.workspace).toBe("default");
  history.page("oplog");
  expect(history.route.workspace).toBe("default");
  await restoreProductionStoreRoute(history.route, data, history, reads);
  expect(reads.loadGraph).toHaveBeenCalled();
  history.destroy();
});

test("a missing workspace fails without silently loading a different working copy", async () => {
  const { data, history, reads } = harness("forgotten");
  await restoreProductionStoreRoute(history.route, data, history, reads);
  expect(data.status.navigation).toBe("failed");
  expect(data.status.error).toContain("workspace is unavailable");
  expect(reads.loadFiles).not.toHaveBeenCalled();
  expect(history.route.workspace).toBe("forgotten");
  history.destroy();
});

test("workspace context permits independent explicit revision inspection", async () => {
  const { data, history, reads } = harness("default");
  const other = "56".repeat(64);
  history.replace({ ...history.route, commitId: other });
  await restoreProductionStoreRoute(history.route, data, history, reads);
  expect(reads.loadFiles).toHaveBeenCalledWith(other);
  expect(history.route.workspace).toBe("default");
  history.destroy();
});

test("working-copy cards distinguish two workspace names sharing the same commit", async () => {
  const { data, history, reads } = harness(null);
  data.workingCopies = copies;
  history.replace({ ...history.route, page: "branches" });
  const actions = createProductionBranchActions({
    data,
    history,
    reader: reads as never,
    invalidate() {},
  });
  await actions.selectWorkingCopy(commit, "agent/review");
  expect(history.route.workspace).toBe("agent/review");
  await actions.selectWorkingCopy(commit, "default");
  expect(history.route.workspace).toBe("default");
  history.destroy();
});

test("branch selections preserve loaded references and paging while reading only scoped history", async () => {
  const { data, history, reads } = harness(null);
  history.replace({ ...history.route, page: "branches" });
  data.workingCopies = copies;
  data.bookmarks = [
    { name: "main", targets: [commit] },
    { name: "feature", targets: ["56".repeat(64)] },
  ] as typeof data.bookmarks;
  data.refsCursor = "retained-next-page";
  data.refsPageSnapshot = snapshot;
  data.status.bookmarks = "ready";
  const bookmarks = data.bookmarks,
    workingCopies = data.workingCopies,
    pin = data.refsPageSnapshot;
  const targets: string[][] = [];
  reads.loadGraph.mockImplementation(async () => {
    targets.push([...data.targets]);
  });
  const actions = createProductionBranchActions({
    data,
    history,
    reader: reads as never,
    invalidate: vi.fn(),
  });
  await actions.selectBookmark("main");
  await actions.selectBookmark("feature");
  await actions.selectWorkingCopy(commit, "agent/review");
  expect(targets).toEqual([[commit], ["56".repeat(64)], [commit]]);
  expect(reads.loadCommit).toHaveBeenCalledWith(commit);
  expect(reads.loadReferences).not.toHaveBeenCalled();
  expect(data.bookmarks).toBe(bookmarks);
  expect(data.workingCopies).toBe(workingCopies);
  expect(data.refsPageSnapshot).toBe(pin);
  expect(data.refsCursor).toBe("retained-next-page");
  expect(data.status.bookmarks).toBe("ready");
  expect(history.route.snapshotOperationId).toBe(operation);
  history.destroy();
});

test("workspace discovery follows every pinned reference page and preserves shared targets", async () => {
  const references = vi.fn(async (_operation: string, cursor?: string) =>
    window(cursor ? ["agent/review"] : ["default"], cursor ? null : "more"),
  );
  const source = { repository, references } as unknown as StoreDataSource;
  const result = await readWorkingCopies(source, operation);
  expect(result.map((item) => item.workspace)).toEqual([
    "agent/review",
    "default",
  ]);
  expect(references).toHaveBeenNthCalledWith(2, operation, "more");
  references.mockImplementation(async () => ({
    ...window(["default"]),
    snapshot: { ...snapshot, operation_id: "ef".repeat(64) },
  }));
  await expect(readWorkingCopies(source, operation)).rejects.toThrow(
    "snapshot mismatch",
  );
});

test("explicit Refresh follows the workspace's new working copy at the captured operation", async () => {
  const { data, history, reads } = harness("default");
  await restoreProductionStoreRoute(history.route, data, history, reads);
  const nextCommit = "56".repeat(64),
    nextOperation = "ef".repeat(64);
  reads.loadReferences.mockImplementation(async () => {
    data.workingCopies = [{ ...copy, commitId: nextCommit }];
    data.status.bookmarks = "ready";
  });
  history.refresh(nextOperation);
  await restoreProductionStoreRoute(history.route, data, history, reads);
  expect(reads.loadFiles).toHaveBeenLastCalledWith(nextCommit);
  expect(history.route).toMatchObject({
    workspace: "default",
    commitId: null,
    snapshotOperationId: nextOperation,
  });
  history.destroy();
});

test("workspace discovery continues the displayed first page without recapturing its snapshot", async () => {
  const references = vi.fn(async (_operation: string, cursor?: string) =>
    window(cursor ? ["agent/review"] : ["default"], cursor ? null : "more"),
  );
  const source = { repository, references } as unknown as StoreDataSource;
  const first = await readReferences(source, operation);
  const result = await readWorkingCopies(source, operation, first);
  expect(references).toHaveBeenCalledTimes(2);
  expect(result).toHaveLength(2);
  references.mockImplementation(async () => ({
    ...window(["agent/review"]),
    snapshot: { ...snapshot, published_seq: 1 },
  }));
  await expect(readWorkingCopies(source, operation, first)).rejects.toThrow(
    "pagination snapshot mismatch",
  );
});

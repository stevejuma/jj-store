import { expect, test, vi } from "vitest";
import { createWorkflowRunSource } from "../../src/lib/ui/store-workflows/workflow-run-source.svelte.js";
import {
  workflowRunReader,
  runOperation,
  previousOperation,
  runCommit,
} from "../../../catalog/fixtures/workflow-run-source.js";
import { code, home } from "../../../catalog/fixtures/workflows.js";
import type { StoreDataSource } from "../../src/lib/controller/native-data-source.js";

test("run choices read source operations and page commits at one explicit operation and clock", async () => {
  const reader = workflowRunReader(code),
    graph = vi.fn(reader.graph);
  // Production retains one acquired lease identity.
  const lease = { ...reader, graph },
    choices = createWorkflowRunSource(
      () => lease,
      () => runOperation,
    );
  await choices.loadOperations();
  expect(choices.operations.rows.map((row) => row.id)).toEqual([
    runOperation,
    previousOperation,
  ]);
  const page = await reader.graph("log", { operation: runOperation });
  graph
    .mockResolvedValueOnce({ ...page, next_cursor: "next-commits" })
    .mockResolvedValueOnce(page);
  choices.resetRevisions();
  await choices.loadRevisions("commits");
  await choices.loadRevisions("commits", true);
  expect(graph.mock.calls.at(-1)?.[1]).toMatchObject({
    operation: runOperation,
    cursor: "next-commits",
    atTimeMillis: graph.mock.calls.at(-2)?.[1].atTimeMillis,
  });
  expect(choices.commits.rows[0]?.id).toBe(runCommit);
  expect(choices.commits.cursor).toBeNull();
});
test("bookmarks preserve exact targets and refuse conflicted automatic selection", async () => {
  const lease = workflowRunReader(code),
    choices = createWorkflowRunSource(
      () => lease,
      () => runOperation,
    );
  await choices.loadRevisions("bookmarks");
  expect(choices.bookmarks.rows[0]).toMatchObject({
    id: "main",
    commit: runCommit,
    disabled: false,
  });
  expect(choices.bookmarks.rows[1]).toMatchObject({
    id: "ambiguous",
    disabled: true,
  });
  expect(choices.bookmarks.rows[1]?.commit).toBeUndefined();
});
test("operation changes ignore old commit and bookmark responses", async () => {
  const lease = workflowRunReader(code);
  let operation = runOperation;
  let release!: (page: Awaited<ReturnType<StoreDataSource["graph"]>>) => void;
  let releaseBookmarks!: (
    page: Awaited<ReturnType<StoreDataSource["references"]>>,
  ) => void;
  const page = await lease.graph("log", { operation });
  const references = await lease.references(operation);
  lease.graph = () =>
    new Promise((resolve) => {
      release = resolve;
    });
  lease.references = () =>
    new Promise((resolve) => {
      releaseBookmarks = resolve;
    });
  const choices = createWorkflowRunSource(
      () => lease,
      () => operation,
    ),
    pending = choices.loadRevisions("commits"),
    pendingBookmarks = choices.loadRevisions("bookmarks");
  operation = previousOperation;
  choices.resetRevisions();
  release(page);
  releaseBookmarks(references);
  await Promise.all([pending, pendingBookmarks]);
  expect(choices.commits.rows).toEqual([]);
  expect(choices.commits.loading).toBe(false);
  expect(choices.bookmarks.rows).toEqual([]);
  expect(choices.bookmarks.loading).toBe(false);
});
test("a source snapshot from another repository is rejected with retryable error", async () => {
  const lease = workflowRunReader(code),
    other = workflowRunReader(home);
  lease.graph = other.graph;
  const choices = createWorkflowRunSource(
    () => lease,
    () => runOperation,
  );
  await choices.loadOperations();
  expect(choices.operations.rows).toEqual([]);
  expect(choices.operations.error).toContain("snapshot changed");
  await choices.loadRevisions("commits");
  expect(choices.commits.rows).toEqual([]);
  expect(choices.commits.error).toContain("snapshot changed");
});

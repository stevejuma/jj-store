import { expect, test, vi } from "vitest";
import { workflowFiles } from "../../src/lib/ui/store-workflows/workflow-files.js";
import type { StoreDataSource } from "../../src/lib/controller/native-data-source.js";
const repository = "01".repeat(16),
  operation = "02".repeat(64),
  commit = "03".repeat(64);
function source() {
  const snapshot = { repository, operation };
  return {
    repository,
    snapshot: vi.fn(async () => ({ repository, heads: [operation] })),
    graph: vi.fn(async () => ({
      snapshot,
      data: { data: { graph: "log", nodes: [{ commit: { id: commit } }] } },
    })),
    paths: vi.fn(async () => ({
      snapshot,
      data: {
        entries: [
          { path: ".github/workflows/ci.yml" },
          { path: ".github/workflows/check.yaml" },
          { path: ".github/workflows/README.md" },
        ],
      },
      next_cursor: null,
    })),
  };
}
test("file discovery uses a single pinned source and lists YAML only", async () => {
  const reader = source();
  expect(await workflowFiles(reader as unknown as StoreDataSource)).toEqual([
    {
      id: ".github/workflows/ci.yml",
      name: "ci.yml",
      description: ".github/workflows/ci.yml",
    },
    {
      id: ".github/workflows/check.yaml",
      name: "check.yaml",
      description: ".github/workflows/check.yaml",
    },
  ]);
  expect(reader.paths).toHaveBeenCalledWith(
    commit,
    operation,
    expect.objectContaining({ path: ".github/workflows", recursive: true }),
  );
});
test("foreign snapshots are rejected and discovery does not change the saved path", async () => {
  const reader = source();
  reader.paths.mockResolvedValue({
    snapshot: { repository: "ff".repeat(16), operation },
    data: { entries: [] },
    next_cursor: null,
  });
  await expect(
    workflowFiles(reader as unknown as StoreDataSource),
  ).rejects.toThrow("snapshot changed");
  const foreignGraph = source();
  const graph = await foreignGraph.graph();
  foreignGraph.graph.mockResolvedValue({
    ...graph,
    snapshot: { repository: "ff".repeat(16), operation },
  });
  await expect(
    workflowFiles(foreignGraph as unknown as StoreDataSource),
  ).rejects.toThrow("revision changed");
});

import { expect, test, vi } from "vitest";
import { BranchFileOwners } from "../../src/lib/controller/stack-workspace/branch-file-owners.js";
import { workspaceBranchSource } from "../../src/lib/controller/stack-workspace/files.js";
import { workspaceFixture } from "../../../catalog/fixtures/stack-workspace/data.js";
import type { WorkspaceFile } from "../../src/lib/controller/stack-workspace/types.js";

function fixture() {
  const snapshot = workspaceFixture();
  const stack = snapshot.stacks[0]!;
  const commits = stack.commits.slice(0, 2);
  stack.commits = commits;
  for (const [index, commit] of commits.entries()) {
    commit.immutable = false;
    commit.graph = {
      parents: [index ? commits[index - 1]!.id : "base"],
      bookmarks: [],
      workspaces: [],
      edges: [],
      conflicted: false,
      divergent: false,
    };
  }
  snapshot.readKey = "operation-1";
  const range = { from: "base", to: commits[1]!.id };
  const source = workspaceBranchSource(stack.id, stack.name);
  const file: WorkspaceFile = {
    path: "src/owned.ts",
    hunks: [],
    content: { status: "unloaded" },
    change: {
      before: [{ type: "file", id: "a".repeat(40), executable: false }],
      after: [{ type: "file", id: "b".repeat(40), executable: false }],
      sources: [],
    },
  };
  const read = vi.fn(async ({ to }: { to: string }) => ({
    files: to === commits[0]!.id ? [file] : [],
  }));
  const owners = new BranchFileOwners();
  const resolve = () =>
    owners.resolve(snapshot, source, range, [file], read, () => {});
  return {
    snapshot,
    stack,
    commits,
    source,
    range,
    file,
    read,
    owners,
    resolve,
  };
}

test("branch drags resolve unique native file terms without loading patches and cache within the read key", async () => {
  const f = fixture();
  expect((await f.resolve())[0]?.selectionSource).toBe(f.commits[0]!.id);
  expect(f.read).toHaveBeenCalledTimes(2);
  await f.resolve();
  expect(f.read).toHaveBeenCalledTimes(2);
  f.snapshot.readKey = "operation-2";
  await f.resolve();
  expect(f.read).toHaveBeenCalledTimes(4);
});

test("multiple changing commits remain ambiguous even when a patch or net endpoints match", async () => {
  const f = fixture();
  f.read.mockImplementation(async () => ({ files: [f.file] }));
  expect((await f.resolve())[0]?.selectionSource).toBeUndefined();
});

test("immutable owners and different file modes, identities or provenance remain read-only", async () => {
  for (const field of ["immutable", "before", "after", "sources"] as const) {
    const f = fixture();
    if (field === "immutable") f.commits[0]!.immutable = true;
    else {
      const changed = structuredClone(f.file);
      if (field === "sources") changed.change!.sources = ["previous.ts"];
      else
        changed.change![field] = [
          { type: "file", id: "c".repeat(40), executable: true },
        ];
      f.read.mockImplementation(async ({ to }) => ({
        files: to === f.commits[0]!.id ? [changed] : [],
      }));
    }
    expect((await f.resolve())[0]?.selectionSource, field).toBeUndefined();
  }
});

test("missing, merge, cyclic or conflicted ancestry cannot manufacture a branch owner", async () => {
  for (const shape of ["missing", "merge", "cycle", "conflict", "divergent"]) {
    const f = fixture(),
      graph = f.commits[0]!.graph!;
    if (shape === "missing") graph.parents = ["unloaded"];
    if (shape === "merge") graph.parents.push("other");
    if (shape === "cycle") graph.parents = [f.commits[1]!.id];
    if (shape === "conflict") graph.conflicted = true;
    if (shape === "divergent") graph.divergent = true;
    expect((await f.resolve())[0]?.selectionSource, shape).toBeUndefined();
    expect(f.read).not.toHaveBeenCalled();
  }
});

test("resolution requires qualified capabilities and complete bounded file pages", async () => {
  const f = fixture();
  f.snapshot.capabilities = [];
  expect((await f.resolve())[0]?.selectionSource).toBeUndefined();
  expect(f.read).not.toHaveBeenCalled();
  f.snapshot.capabilities = ["transfer"];
  let pages = 0;
  const read = vi.fn(async () => ({
    files: [f.file],
    nextCursor: `page-${++pages}`,
  }));
  const result = await f.owners.resolve(
    f.snapshot,
    f.source,
    f.range,
    [f.file],
    read,
    () => {},
  );
  expect(result[0]?.selectionSource).toBeUndefined();
  expect(read).toHaveBeenCalledTimes(50);
});

test("scope invalidation during resolution rejects late native file evidence", async () => {
  const f = fixture();
  await expect(
    f.owners.resolve(f.snapshot, f.source, f.range, [f.file], f.read, () => {
      throw new Error("snapshot_mismatch");
    }),
  ).rejects.toThrow("snapshot_mismatch");
});

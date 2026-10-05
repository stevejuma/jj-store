import { expect, test } from "vitest";
import { projectStackRange } from "../../src/lib/controller/stack-workspace/stack-segments.js";
import type {
  StackCommit,
  WorkspaceStack,
} from "../../src/lib/controller/stack-workspace/types.js";
const node = (
  id: string,
  parents: string[],
  names: string[] = [],
): StackCommit => ({
  id,
  message: id,
  author: "Test",
  email: "test@example.test",
  createdAt: 1,
  files: [],
  graph: {
    parents,
    edges: parents.map((target) => ({ target, kind: "direct" })),
    bookmarks: names.map((name) => ({
      name,
      added: true,
      removed: false,
      conflicted: false,
    })),
    workspaces: [],
    conflicted: false,
    divergent: false,
  },
});
const stack: WorkspaceStack = {
  id: "upper-id",
  name: "upper",
  base: "main",
  heads: [{ name: "upper", commit: "tip" }],
  commits: [],
  conflicts: false,
};
test("dependent segments use native parent links and compare each head with its lower boundary", () => {
  const projected = projectStackRange(stack, "base", [
    node("tip", ["middle"], ["upper"]),
    node("lower", ["base"], ["lower"]),
    node("middle", ["lower"]),
  ]);
  expect(projected.heads).toEqual([
    { name: "upper", commit: "tip" },
    { name: "lower", commit: "lower" },
  ]);
  expect(
    projected.segments?.map((s) => [
      s.head.name,
      s.baseCommit,
      s.commits.map((c) => c.id),
    ]),
  ).toEqual([
    ["upper", "lower", ["tip", "middle"]],
    ["lower", "base", ["lower"]],
  ]);
});
test("partial segment boundaries remain unavailable until bounded continuation proves them", () => {
  const partial = projectStackRange(
    stack,
    "base",
    [node("tip", ["middle"], ["upper"]), node("middle", ["lower"])],
    "more",
  );
  expect(partial.segments?.[0]?.baseCommit).toBeNull();
  expect(partial.segments?.[0]?.nextCursor).toBe("more");
  const complete = projectStackRange(partial, "base", [
    node("tip", ["middle"], ["upper"]),
    node("middle", ["lower"]),
    node("lower", ["base"], ["lower", "alias", "jj-store/workspaces/hidden"]),
  ]);
  expect(complete.segments).toHaveLength(2);
  expect(complete.segments?.flatMap((s) => s.commits.map((c) => c.id))).toEqual(
    ["tip", "middle", "lower"],
  );
  expect(complete.segments?.[0]?.baseCommit).toBe("lower");
});
test("merge or elided ancestry keeps the provider graph and does not invent a linear stack", () => {
  const commits = [
    node("tip", ["left", "right"], ["upper"]),
    node("left", ["base"], ["lower"]),
    node("right", ["base"]),
  ];
  const projected = projectStackRange(stack, "base", commits);
  expect(projected.heads).toEqual(stack.heads);
  expect(projected.segments).toHaveLength(1);
  expect(projected.segments?.[0]?.commits).toEqual(commits);
  expect(projected.commits.map((c) => c.id)).toEqual(["right", "left", "tip"]);
});

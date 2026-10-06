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
  expect(projected.commits.map((commit) => commit.id)).toEqual([
    "lower",
    "middle",
    "tip",
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
test("a native missing edge into excluded target ancestry retains proven branch cards", () => {
  const lower = node("lower", ["fork"], ["lower", "alias"]);
  lower.graph!.edges = [{ target: "fork", kind: "missing" }];
  const projected = projectStackRange(stack, "advanced-target", [
    node("tip", ["lower"], ["upper"]),
    lower,
  ]);
  expect(
    projected.segments?.map((s) => [
      s.head.name,
      s.baseCommit,
      s.commits.map((c) => c.id),
    ]),
  ).toEqual([
    ["upper", "lower", ["tip"]],
    ["alias", "fork", ["lower"]],
  ]);
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

test("native topology keeps unloaded lower cards and their exact boundaries beyond page one", () => {
  const scoped = {
    ...stack,
    topologyHeads: [
      { name: "upper", commit: "tip", baseCommit: "lower" },
      {
        name: "lower",
        commit: "lower",
        aliases: ["alias"],
        baseCommit: "fork",
        shared: true,
      },
    ],
  };
  const result = projectStackRange(
    scoped,
    "advanced-target",
    [node("tip", ["next"])],
    "page-two",
  );
  expect(
    result.segments?.map((s) => [s.head.name, s.baseCommit, s.unloaded]),
  ).toEqual([
    ["upper", "lower", false],
    ["lower", "fork", true],
  ]);
  expect(result.segments?.at(-1)?.nextCursor).toBe("page-two");
});

test("a merge in the lowest card does not erase the valid card above it", () => {
  const result = projectStackRange(
    {
      ...stack,
      topologyHeads: [
        { name: "upper", commit: "tip", baseCommit: "merge" },
        { name: "merged", commit: "merge", baseCommit: null },
      ],
    },
    "base",
    [
      node("tip", ["merge"]),
      node("merge", ["left", "right"]),
      node("left", ["base"]),
      node("right", ["base"]),
    ],
  );
  expect(
    result.segments?.map((s) => [s.head.name, s.commits.map((c) => c.id)]),
  ).toEqual([
    ["upper", ["tip"]],
    ["merged", ["merge", "left", "right"]],
  ]);
  expect(result.segments?.[1]?.baseCommit).toBeNull();
  expect(result.segments?.[1]?.commits[0]?.graph?.parents).toEqual([
    "left",
    "right",
  ]);
});

test("an exhausted integrated range is empty rather than an unloaded skeleton or bookmark-only history", () => {
  const result = projectStackRange(
    { ...stack, commits: [node("tip", ["base"])] },
    "advanced-target",
    [],
  );
  expect(result.commits).toEqual([]);
  expect(result.segments?.[0]?.unloaded).toBe(false);
});

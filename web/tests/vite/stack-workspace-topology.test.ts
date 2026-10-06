import { workspaceBranchSource } from "../../src/lib/controller/stack-workspace/files.js";
import { stackCardSelection } from "../../src/lib/controller/stack-workspace/card-selection.js";
import { expect, test } from "vitest";
import { workspaceFixture } from "../../../catalog/fixtures/stack-workspace/data.js";
import {
  projectLocalStacks,
  localStacksCommand,
} from "../../src/lib/controller/stack-workspace/local-state.js";
import {
  appliedStacks,
  stackLaneMembers,
} from "../../src/lib/controller/stack-workspace/presentation.js";
import { WorkspaceCommitDrafts } from "../../src/lib/controller/stack-workspace/commit-drafts.svelte.js";
import { retainedWorkspaceDiffs } from "../../src/lib/controller/stack-workspace/retained-diffs.js";
import { projectStackRange } from "../../src/lib/controller/stack-workspace/stack-segments.js";
import type { Model } from "../../src/lib/api/client.js";

function fixture() {
  const snapshot = workspaceFixture();
  const template = snapshot.stacks[0]!;
  snapshot.stacks = [
    "upper",
    "lower",
    "alias",
    "sibling",
    "integrated",
    "empty",
  ].map((name, index) => ({
    ...structuredClone(template),
    id: `branch:${JSON.stringify([name])}`,
    name,
    heads: [{ name, commit: name === "alias" ? "lower" : name }],
    commits: template.commits.map((commit) => ({
      ...commit,
      id: `${name}-${commit.id}`,
    })),
  }));
  snapshot.targetOptions = [snapshot.target];
  const state: Model<"LocalStacksState"> = {
    version: 1,
    operation: "a".repeat(128),
    revision: 2,
    active: snapshot.active,
    warning: null,
    conflicts: [],
    can_undo: false,
    can_redo: false,
    workspaces: [
      {
        id: snapshot.active,
        name: "Work",
        applied: snapshot.stacks.map((s) => s.name),
        target: snapshot.target.name,
        target_commit: snapshot.target.commit,
        merge_commit: "merge",
        wip_commit: "wip",
        conflicted: false,
        membership_changed: false,
      },
    ],
    topology: {
      complete: true,
      integrated: ["integrated"],
      lanes: [
        {
          applied: ["upper", "lower", "alias"],
          heads: [
            { names: ["upper"], commit: "upper" },
            { names: ["lower", "alias"], commit: "lower" },
          ],
        },
        {
          applied: ["sibling"],
          heads: [{ names: ["sibling"], commit: "sibling" }],
        },
        {
          applied: ["empty"],
          heads: [{ names: ["empty"], commit: snapshot.target.commit }],
        },
      ],
    },
  };
  return { snapshot, state };
}
test("workspace lanes use operation topology, independent of loaded pages, without changing inventory or saved membership", () => {
  const { snapshot, state } = fixture();
  const result = projectLocalStacks(snapshot, state);
  expect(result.stacks).toHaveLength(6);
  expect(result.workspaces[0]!.applied).toHaveLength(6);
  const lanes = appliedStacks(result);
  expect(lanes.map((lane) => lane.name)).toEqual(["upper", "sibling", "empty"]);
  expect(lanes[0]!.heads).toEqual([
    { name: "upper", commit: "upper", aliases: [] },
    { name: "lower", commit: "lower", aliases: ["alias"] },
  ]);
  expect(
    localStacksCommand(result, { kind: "unapply_stack", stack: lanes[0]!.id }),
  ).toEqual({
    kind: "unapply_branches",
    branches: ["upper", "lower", "alias"],
  });
  expect(
    localStacksCommand(result, {
      kind: "unapply_stack",
      stack: lanes[0]!.id,
      branch: "alias",
    }),
  ).toEqual({ kind: "unapply", branch: "alias" });
  expect(
    localStacksCommand(result, {
      kind: "apply_stack",
      stack: result.stacks[1]!.id,
    }),
  ).toEqual({ kind: "apply", branch: "lower" });
  result.topology!.complete = false;
  expect(appliedStacks(result)).toEqual([]);
});

test("regrouping retains distinct branch drafts, exact mutation destinations and both adjacent diffs", () => {
  const { snapshot, state } = fixture();
  snapshot.workspaces[0]!.applied = snapshot.stacks.map((s) => s.id);
  const drafts = new WorkspaceCommitDrafts();
  const upper = snapshot.stacks[0]!,
    lower = snapshot.stacks[1]!;
  drafts.open(snapshot, upper.id);
  drafts.open(snapshot, lower.id);
  drafts.get(snapshot.active, lower.id)!.title = "Lower draft";
  const diffs = [upper, lower].map((stack) => ({
    origin: `stack-${stack.id}`,
    source: stack.commits[0]!.id,
    path: "file.ts",
  }));
  const result = projectLocalStacks(snapshot, state);
  const retained = retainedWorkspaceDiffs(
    result,
    snapshot.active,
    diffs,
    () => [],
  );
  expect(retained).toEqual(diffs);
  expect(
    stackLaneMembers(result, appliedStacks(result)[0]!).map((s) => s.name),
  ).toEqual(["upper", "lower", "alias"]);
  expect(drafts.get(snapshot.active, lower.id)!.title).toBe("Lower draft");
  expect(
    localStacksCommand(result, {
      kind: "create_commit",
      stack: lower.id,
      message: "Lower draft",
    }),
  ).toMatchObject({ branch: "lower" });
});

test("native aliases share one segment and commit rows while merge parents stay intact", () => {
  const { snapshot, state } = fixture();
  const stack = appliedStacks(projectLocalStacks(snapshot, state))[0]!;
  const node = (id: string, parents: string[]) => ({
    ...stack.commits[0]!,
    id,
    graph: {
      parents,
      edges: parents.map((target) => ({ target, kind: "direct" as const })),
      bookmarks: [],
      workspaces: [],
      conflicted: false,
      divergent: false,
    },
  });
  const projected = projectStackRange(stack, "base", [
    node("upper", ["lower"]),
    node("lower", ["base"]),
  ]);
  expect(projected.segments?.map((segment) => segment.head.name)).toEqual([
    "upper",
    "lower",
  ]);
  expect(projected.segments?.[1]?.head.aliases).toEqual(["alias"]);
  expect(
    projected.segments?.flatMap((segment) => segment.commits),
  ).toHaveLength(2);
});

test("regrouped cards render a retained lower branch selection under its original owner", () => {
  const { snapshot, state } = fixture();
  const result = projectLocalStacks(snapshot, state);
  const lower = result.stacks[1]!;
  const selected = lower.commits[0]!;
  const card = stackCardSelection(
    result,
    { [lower.id]: selected.id },
    appliedStacks(result)[0]!,
    "lower",
    ["alias"],
    [selected],
  );
  expect(card.owner).toBe(lower.id);
  expect(card.commit?.id).toBe(selected.id);
});

test("another branch card selection does not redirect a lower branch click", () => {
  const { snapshot, state } = fixture();
  const result = projectLocalStacks(snapshot, state);
  const upper = appliedStacks(result)[0]!;
  const lower = result.stacks[1]!;
  const source = workspaceBranchSource(upper.id, upper.name);
  const card = stackCardSelection(
    result,
    { [upper.id]: source },
    upper,
    "lower",
    ["alias"],
    lower.commits,
  );
  expect(card.owner).toBe(lower.id);
  expect(card.branch).toBe("lower");
  expect(card.branchSelected).toBe(false);
});

test("complete inventory hides dependent entries, searches aliases and preserves a partially applied lower slice", async () => {
  const { stackBookmarks, browsingStack } =
    await import("../../src/lib/controller/stack-workspace/presentation.js");
  const { snapshot, state } = fixture();
  state.workspaces[0]!.applied = ["lower"];
  state.topology!.inventory = structuredClone(state.topology!.lanes);
  state.topology!.lanes = [
    {
      heads: [state.topology!.inventory[0]!.heads[1]!],
      applied: ["lower"],
      unapply: ["lower"],
    },
  ];
  snapshot.bookmarks = snapshot.stacks.map((stack) => ({
    name: stack.name,
    targets: [stack.heads[0]!.commit],
  }));
  const result = projectLocalStacks(snapshot, state);
  const bookmarks = stackBookmarks(result);
  expect(bookmarks.map((b) => b.name)).toEqual([
    "upper",
    "sibling",
    "integrated",
    "empty",
  ]);
  expect(bookmarks[0]).toMatchObject({
    workspace: true,
    partiallyApplied: true,
    searchTerms: ["upper", "lower", "alias"],
  });
  expect(appliedStacks(result).map((s) => s.name)).toEqual(["lower"]);
  expect(browsingStack(result, result.stacks[1]!.id)?.name).toBe("upper");
  expect(
    localStacksCommand(result, {
      kind: "apply_stack",
      stack: result.stacks[0]!.id,
    }),
  ).toEqual({ kind: "apply", branch: "upper" });
  result.topology!.complete = false;
  expect(stackBookmarks(result).map((b) => b.name)).toContain("lower");
});

test("discovered lower destinations retain drafts, exact comparisons and exclusive grouped unapply", async () => {
  const { stackSourceRange } =
    await import("../../src/lib/controller/stack-workspace/source-range.js");
  const { workspaceDropFeedback } =
    await import("../../src/lib/controller/stack-workspace/drag.js");
  const { snapshot, state } = fixture();
  state.workspaces[0]!.applied = ["upper", "lower", "sibling"];
  const upper = state.topology!.lanes[0]!;
  upper.heads[0]!.base_commit = "lower";
  upper.heads[1]!.base_commit = "actual-fork";
  upper.heads[1]!.shared = true;
  upper.applied = ["upper", "lower"];
  upper.unapply = ["upper"];
  state.topology!.lanes[1]!.heads.push(upper.heads[1]!);
  state.topology!.inventory = structuredClone(state.topology!.lanes);
  const result = projectLocalStacks(snapshot, state);
  const lower = result.stacks[1]!;
  expect(
    stackSourceRange(
      result,
      new Map(),
      workspaceBranchSource(lower.id, "lower"),
    ),
  ).toEqual({ from: "actual-fork", to: "lower" });
  expect(
    localStacksCommand(result, {
      kind: "unapply_stack",
      stack: result.stacks[0]!.id,
    }),
  ).toEqual({ kind: "unapply_branches", branches: ["upper"] });
  // Now only the upper head is saved: discovery still permits the lower composer.
  result.workspaces[0]!.applied = [result.stacks[0]!.id];
  const drafts = new WorkspaceCommitDrafts();
  expect(drafts.open(result, lower.id)).toBe(true);
  const selection = { source: "unassigned", path: "new.txt" };
  result.workspaces[0]!.unassigned = [{ path: selection.path, hunks: [] }];
  expect(
    workspaceDropFeedback(
      result,
      JSON.stringify({ kind: "selection", selection }),
      { kind: "stack", id: lower.id },
    ).state,
  ).toBe("valid");
});

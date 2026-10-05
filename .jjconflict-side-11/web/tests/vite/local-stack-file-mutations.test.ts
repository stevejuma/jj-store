import { expect, test } from "vitest";
import { localStacksCommand } from "../../src/lib/controller/stack-workspace/local-state.js";
import { workspaceFixture } from "../../../catalog/fixtures/stack-workspace/data.js";

test("local file mutations map displayed sources to exact native commit selections", () => {
  const snapshot = workspaceFixture(),
    stack = snapshot.stacks[0]!,
    source = stack.commits[0]!,
    target = stack.commits.at(-1)!;
  const workspace = snapshot.workspaces.find((w) => w.id === snapshot.active)!;
  workspace.wipCommit = "wip";
  expect(
    localStacksCommand(snapshot, {
      kind: "transfer",
      target: target.id,
      selection: { source: "unassigned", path: "draft.txt" },
    }),
  ).toEqual({
    kind: "transfer",
    target: target.id,
    selections: [{ source: "wip", path: "draft.txt" }],
  });
  expect(
    localStacksCommand(snapshot, {
      kind: "split",
      commit: source.id,
      message: "Part",
      selection: { source: source.id, path: "part.txt" },
    }),
  ).toEqual({
    kind: "split",
    commit: source.id,
    message: "Part",
    selections: [{ source: source.id, path: "part.txt" }],
  });
  expect(
    localStacksCommand(snapshot, {
      kind: "edit_commit",
      commit: target.id,
      message: "Edited",
      selections: [{ source: "unassigned", path: "draft.txt" }],
    }),
  ).toMatchObject({
    kind: "edit",
    commit: target.id,
    message: "Edited",
    selections: [{ source: "wip", path: "draft.txt" }],
  });
  expect(
    localStacksCommand(snapshot, {
      kind: "create_commit",
      stack: stack.id,
      message: "Extraction",
      selection: { source: source.id, path: "part.txt" },
    }),
  ).toMatchObject({
    kind: "commit",
    selections: [{ source: source.id, path: "part.txt" }],
  });
  expect(() =>
    localStacksCommand(snapshot, {
      kind: "transfer",
      target: target.id,
      selection: { source: "branch:unknown", path: "part.txt" },
    }),
  ).toThrow("Net branch diffs");
  expect(() =>
    localStacksCommand(snapshot, {
      kind: "transfer",
      target: target.id,
      selection: {
        source: source.id,
        path: "part.txt",
        hunks: ["presentation:untrusted"],
      },
    }),
  ).toThrow("whole files");
});

import { expect, test } from "vitest";
import { MockStackWorkspaceAdapter } from "../../../catalog/fixtures/stack-workspace/adapter.js";
import { workspaceFixture } from "../../../catalog/fixtures/stack-workspace/data.js";
import { StackWorkspaceController } from "../../src/lib/controller/stack-workspace/controller.svelte.js";
import { workspaceDrop } from "../../src/lib/controller/stack-workspace/drag.js";
import { commitDropPlacement } from "../../src/lib/controller/stack-workspace/commit-placement.js";
import type { WorkspaceCommand } from "../../src/lib/controller/stack-workspace/types.js";

async function setup(snapshot = workspaceFixture()) {
  const controller = new StackWorkspaceController(
    new MockStackWorkspaceAdapter(snapshot),
  );
  await controller.refresh();
  return controller;
}
test("commit row edge drops preserve above and below placement instead of squashing", () => {
  const payload = JSON.stringify({ kind: "commit", commit: "api1" });
  for (const placement of ["above", "below"] as const)
    expect(
      workspaceDrop(payload, "commit", "api2", undefined, placement),
    ).toEqual({
      kind: "reorder_commit",
      commit: "api1",
      target: "api2",
      placement,
    });
  expect(workspaceDrop(payload, "commit", "api2")).toEqual({
    kind: "squash",
    commit: "api1",
    target: "api2",
  });
});
test("exact row sorting retains identities and rejects immutable rewrites and redundant positions", async () => {
  const controller = await setup();
  const files = structuredClone(
    controller.snapshot!.stacks[0]!.commits[0]!.files,
  );
  const command = {
    kind: "reorder_commit",
    commit: "api1",
    target: "api2",
    placement: "above",
  } as WorkspaceCommand;
  await controller.prepare(command);
  expect(controller.preview, controller.validation).not.toBeNull();
  expect(
    controller.snapshot!.stacks[0]!.commits.map((commit) => commit.id),
  ).toEqual(["api1", "api2"]);
  await controller.apply();
  expect(
    controller.snapshot!.stacks[0]!.commits.map((commit) => commit.id),
  ).toEqual(["api2", "api1"]);
  expect(controller.snapshot!.stacks[0]!.commits[1]!.files).toEqual(files);
  await controller.prepare(command);
  expect(controller.preview).toBeNull();
  expect(controller.validation).toContain("already");
  await controller.prepare({ kind: "undo" });
  await controller.apply();
  const snapshot = workspaceFixture();
  snapshot.stacks[0]!.commits[1]!.immutable = true;
  const immutable = await setup(snapshot);
  await immutable.prepare(command);
  expect(immutable.preview).toBeNull();
  expect(immutable.validation).toContain("immutable");
});

test("sorting thresholds use rendered quarters and leave file drags in the center", () => {
  const bounds = { top: 100, height: 80 };
  const payload = JSON.stringify({ kind: "commit", commit: "api1" });
  expect(commitDropPlacement(payload, bounds, 119)).toBe("above");
  expect(commitDropPlacement(payload, bounds, 120)).toBeUndefined();
  expect(commitDropPlacement(payload, bounds, 160)).toBeUndefined();
  expect(commitDropPlacement(payload, bounds, 161)).toBe("below");
  expect(
    commitDropPlacement(JSON.stringify({ kind: "selection" }), bounds, 100),
  ).toBeUndefined();
  expect(commitDropPlacement("malformed", bounds, 100)).toBeUndefined();
});

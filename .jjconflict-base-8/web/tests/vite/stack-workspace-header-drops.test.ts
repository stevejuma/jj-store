import { expect, test } from "vitest";
import { MockStackWorkspaceAdapter } from "../../../catalog/fixtures/stack-workspace/adapter.js";
import { workspaceFixture } from "../../../catalog/fixtures/stack-workspace/data.js";
import { StackWorkspaceController } from "../../src/lib/controller/stack-workspace/controller.svelte.js";
import { WorkspaceDragSession } from "../../src/lib/controller/stack-workspace/drag-session.svelte.js";
import {
  workspaceDrop,
  workspaceDropFeedback,
} from "../../src/lib/controller/stack-workspace/drag.js";
import { stackGroups } from "../../src/lib/controller/stack-workspace/presentation.js";
import type { WorkspaceCommand } from "../../src/lib/controller/stack-workspace/types.js";

const payload = JSON.stringify({ kind: "commit", commit: "ui1" });
const target = { kind: "stack" as const, id: "api", branch: "feature/model" };
async function setup(snapshot = workspaceFixture()) {
  const controller = new StackWorkspaceController(
    new MockStackWorkspaceAdapter(snapshot),
  );
  await controller.refresh();
  return controller;
}
async function apply(
  controller: StackWorkspaceController,
  command: WorkspaceCommand,
) {
  await controller.prepare(command);
  expect(controller.preview, controller.validation).not.toBeNull();
  await controller.apply();
  expect(controller.error || controller.validation).toBe("");
}

test("commit header drops retain the exact branch and isolate feedback per header", () => {
  const command = {
    kind: "move_commit",
    commit: "ui1",
    stack: "api",
    branch: "feature/model",
  };
  expect(workspaceDrop(payload, "stack", "api", "feature/model")).toEqual(
    command,
  );
  expect(
    workspaceDropFeedback(workspaceFixture(), payload, target),
  ).toMatchObject({ state: "valid", command });
  const drag = new WorkspaceDragSession();
  expect(drag.key(target)).not.toBe(
    drag.key({ ...target, branch: "feature/api" }),
  );
});

test("moving into a dependent header preserves upper commits and supports undo", async () => {
  const controller = await setup();
  const before = controller.snapshot!;
  const source = structuredClone(before.stacks[1]!.commits[0]);
  await controller.prepare({
    kind: "move_commit",
    commit: "ui1",
    stack: "api",
    branch: "feature/model",
  });
  expect(controller.preview?.summary).toContain("feature/model");
  expect(controller.snapshot).toEqual(before);
  await controller.apply();
  expect(
    stackGroups(controller.snapshot!.stacks[0]!).map((group) =>
      group.commits.map((commit) => commit.id),
    ),
  ).toEqual([["api2"], ["ui1", "api1"]]);
  expect(controller.snapshot!.stacks[0]!.commits[1]).toEqual(source);
  expect(controller.snapshot!.stacks[1]!.commits).toEqual([]);
  await apply(controller, { kind: "undo" });
  expect(controller.snapshot!.stacks).toEqual(before.stacks);
});

test("commits move between branch segments but reject their current segment", async () => {
  const controller = await setup();
  const same = workspaceDropFeedback(
    controller.snapshot,
    JSON.stringify({ kind: "commit", commit: "api2" }),
    { ...target, branch: "feature/api" },
  );
  expect(same.state).toBe("invalid");
  await apply(controller, {
    kind: "move_commit",
    commit: "api2",
    stack: "api",
    branch: "feature/model",
  });
  expect(
    stackGroups(controller.snapshot!.stacks[0]!).map((group) =>
      group.commits.map((commit) => commit.id),
    ),
  ).toEqual([[], ["api2", "api1"]]);
  await controller.prepare({
    kind: "move_commit",
    commit: "api2",
    stack: "api",
    branch: "feature/model",
  });
  expect(controller.preview).toBeNull();
  expect(controller.validation).toContain("another branch");
});

test("stale and immutable branch header targets reject drops and previews", async () => {
  const snapshot = workspaceFixture();
  snapshot.stacks[0]!.commits[1]!.immutable = true;
  const controller = await setup(snapshot);
  for (const branch of ["missing", "feature/model"]) {
    const feedback = workspaceDropFeedback(controller.snapshot, payload, {
      ...target,
      branch,
    });
    expect(feedback.state).toBe("invalid");
    await controller.prepare({
      kind: "move_commit",
      commit: "ui1",
      stack: "api",
      branch,
    });
    expect(controller.preview).toBeNull();
    expect(controller.snapshot!.stacks[1]!.commits[0]!.id).toBe("ui1");
  }
  expect(
    workspaceDropFeedback(controller.snapshot, payload, target, true).state,
  ).toBe("invalid");
});

test("header moves can append above immutable ancestors without rewriting them", async () => {
  const snapshot = workspaceFixture();
  snapshot.stacks[0]!.commits[0]!.immutable = true;
  const ancestor = structuredClone(snapshot.stacks[0]!.commits[0]);
  const controller = await setup(snapshot);
  expect(
    workspaceDropFeedback(controller.snapshot, payload, target).state,
  ).toBe("valid");
  await apply(controller, {
    kind: "move_commit",
    commit: "ui1",
    stack: "api",
    branch: "feature/model",
  });
  expect(controller.snapshot!.stacks[0]!.commits[0]).toEqual(ancestor);
  expect(
    stackGroups(controller.snapshot!.stacks[0]!).map((group) =>
      group.commits.map((commit) => commit.id),
    ),
  ).toEqual([["api2"], ["ui1", "api1"]]);
});

import { expect, test } from "vitest";
import { MockStackWorkspaceAdapter } from "../../../catalog/fixtures/stack-workspace/adapter.js";
import { workspaceFixture } from "../../../catalog/fixtures/stack-workspace/data.js";
import { StackWorkspaceController } from "../../src/lib/controller/stack-workspace/controller.svelte.js";
import { workspaceDropFeedback } from "../../src/lib/controller/stack-workspace/drag.js";
import { cloneWorkspace } from "../../src/lib/controller/stack-workspace/types.js";

const selection = {
  source: "unassigned",
  path: "src/settings.ts",
  hunks: ["src/settings.ts:add keyboard shortcut"],
};
const payload = JSON.stringify({ kind: "selection", selection });
const target = { kind: "new-stack" as const };

test("new branch spacer retains exact hunks and rejects stale or foreign drags", () => {
  const snapshot = workspaceFixture();
  expect(workspaceDropFeedback(snapshot, payload, target)).toMatchObject({
    state: "valid",
    command: { kind: "create_stack", name: "", selections: [selection] },
  });
  expect(
    workspaceDropFeedback(
      snapshot,
      JSON.stringify({ kind: "commit", commit: "api1" }),
      target,
    ).state,
  ).toBe("invalid");
  expect(
    workspaceDropFeedback(
      snapshot,
      JSON.stringify({
        kind: "selection",
        selection: { ...selection, hunks: ["missing"] },
      }),
      target,
    ).state,
  ).toBe("invalid");
  expect(workspaceDropFeedback(snapshot, payload, target, true).state).toBe(
    "invalid",
  );
  snapshot.capabilities = snapshot.capabilities.filter(
    (value) => value !== "create_commit",
  );
  expect(workspaceDropFeedback(snapshot, payload, target).state).toBe(
    "invalid",
  );
});

test("confirmed branch creation stages exact selections without moving source files", async () => {
  const adapter = new MockStackWorkspaceAdapter();
  const controller = new StackWorkspaceController(adapter);
  await controller.refresh();
  const before = cloneWorkspace(controller.snapshot!);
  await controller.prepare({
    kind: "create_stack",
    name: "feature/dropped",
    selections: [selection],
  });
  expect(controller.snapshot).toEqual(before);
  await controller.apply();
  const stack = controller.snapshot!.stacks.find(
    (stack) => stack.name === "feature/dropped",
  )!;
  expect(controller.selectedStack).toBe(stack.id);
  expect(controller.selectedBranch).toBe("feature/dropped");
  expect(stack.commits).toEqual([]);
  expect(controller.commitDrafts.get("daily", stack.id)?.selections).toEqual([
    selection,
  ]);
  expect(controller.snapshot!.workspaces[0]!.unassigned).toEqual(
    before.workspaces[0]!.unassigned,
  );
  controller.close();
});

test("unknown branch creation retains request and stages once only after status confirmation", async () => {
  const adapter = new MockStackWorkspaceAdapter(workspaceFixture(), "unknown");
  const controller = new StackWorkspaceController(adapter);
  await controller.refresh();
  await controller.prepare({
    kind: "create_stack",
    name: "feature/recover",
    selections: [selection],
  });
  const request = cloneWorkspace(controller.preview!.request);
  await controller.apply();
  expect(controller.pending).toEqual(request);
  expect(controller.commitDrafts.drafts).toEqual({});
  await controller.checkStatus();
  const stack = controller.snapshot!.stacks.find(
    (stack) => stack.name === "feature/recover",
  )!;
  expect(controller.pending).toBeNull();
  expect(controller.commitDrafts.get("daily", stack.id)?.selections).toEqual([
    selection,
  ]);
  expect(adapter.calls).toHaveLength(1);
  controller.close();
});

test("dependent branch drop preserves selections and stages in the chosen stack", async () => {
  const controller = new StackWorkspaceController(
    new MockStackWorkspaceAdapter(),
  );
  await controller.refresh();
  await controller.prepare({
    kind: "create_stack",
    name: "feature/dependent-drop",
    parent: "api",
    selections: [selection],
  });
  await controller.apply();
  expect(controller.selectedStack).toBe("api");
  expect(controller.selectedBranch).toBe("feature/dependent-drop");
  expect(controller.commitDrafts.get("daily", "api")?.selections).toEqual([
    selection,
  ]);
  controller.close();
});

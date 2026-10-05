import { expect, test, vi } from "vitest";
import { StackWorkspaceController } from "../../src/lib/controller/stack-workspace/controller.svelte.js";
import {
  appliedStacks,
  stackBookmarks,
} from "../../src/lib/controller/stack-workspace/presentation.js";
import {
  cloneWorkspace,
  type StackWorkspaceAdapter,
  type WorkspaceSnapshot,
} from "../../src/lib/controller/stack-workspace/types.js";
import { workspaceFixture } from "../../../catalog/fixtures/stack-workspace/data.js";
function fixture() {
  const initial = workspaceFixture();
  const target = initial.target;
  const snapshot: WorkspaceSnapshot = {
    ...cloneWorkspace(initial),
    active: null,
    workspaces: [],
    target: null,
    base: null,
    capabilities: [],
  };
  const calls: string[] = [];
  const adapter: StackWorkspaceAdapter = {
    read: async () => cloneWorkspace(snapshot),
    readTargetHistory: async () => ({ commits: [] }),
    preview: async () => {
      calls.push("preview");
      throw new Error("No executor");
    },
    apply: async () => {
      calls.push("apply");
      return { status: "unknown" };
    },
    status: async () => ({ status: "unknown" }),
  };
  return { snapshot, target, adapter, calls };
}
test("an unconfigured checkout loads without inventing a target or combination", async () => {
  const { adapter, calls } = fixture();
  const controller = new StackWorkspaceController(adapter);
  await controller.refresh();
  expect(controller.error).toBe("");
  expect(controller.snapshot?.target).toBeNull();
  expect(controller.snapshot?.workspaces).toEqual([]);
  expect(appliedStacks(controller.snapshot!)).toEqual([]);
  expect(
    stackBookmarks(controller.snapshot!).every((branch) => !branch.workspace),
  ).toBe(true);
  expect(calls).toEqual([]);
  controller.close();
});
test("advertised mutations cannot bypass missing target and combination setup", async () => {
  const { adapter, snapshot, target, calls } = fixture();
  snapshot.capabilities = ["create_commit", "create_workspace"];
  const controller = new StackWorkspaceController(adapter);
  await controller.refresh();
  expect(controller.can("create_workspace")).toBe(false);
  snapshot.target = target;
  snapshot.base = target.name;
  await controller.refresh();
  expect(controller.can("create_workspace")).toBe(true);
  expect(controller.can("create_commit")).toBe(false);
  await controller.prepare({
    kind: "create_commit",
    stack: "api",
    message: "blocked",
  });
  expect(calls).toEqual([]);
  controller.close();
});
test("target selection uses the current read context and never calls the executor", async () => {
  const { adapter, target, calls } = fixture();
  const choose = vi.fn(async () => ({
    ...(await adapter.read()),
    target,
    base: target.name,
  }));
  Object.assign(adapter, { chooseTarget: choose });
  const controller = new StackWorkspaceController(adapter);
  await controller.refresh();
  await controller.chooseTarget(target);
  expect(choose).toHaveBeenCalledWith({
    repository: "local-demo",
    revision: 1,
    target: { name: target.name, commit: target.commit },
  });
  expect(controller.snapshot?.active).toBeNull();
  expect(controller.snapshot?.target).toEqual(target);
  expect(calls).toEqual([]);
  controller.close();
});

test.each(["refresh", "close"] as const)(
  "%s discards a late target choice",
  async (action) => {
    const { adapter, target, snapshot } = fixture();
    let finish!: (snapshot: WorkspaceSnapshot) => void;
    adapter.chooseTarget = () =>
      new Promise((resolve) => {
        finish = resolve;
      });
    const controller = new StackWorkspaceController(adapter);
    await controller.refresh();
    const pending = controller.chooseTarget(target);
    if (action === "refresh") await controller.refresh();
    else controller.close();
    finish({ ...snapshot, target, base: target.name });
    await pending;
    expect(controller.snapshot?.target).toBeNull();
    controller.close();
  },
);

test("an unknown active combination ID cannot imply applied membership or enable mutations", async () => {
  const { adapter, snapshot, target } = fixture();
  Object.assign(snapshot, {
    target,
    base: target.name,
    active: "missing",
    capabilities: ["create_commit"],
  });
  const controller = new StackWorkspaceController(adapter);
  await controller.refresh();
  expect(controller.can("create_commit")).toBe(false);
  expect(appliedStacks(controller.snapshot!)).toEqual([]);
  expect(
    stackBookmarks(controller.snapshot!).every((branch) => !branch.workspace),
  ).toBe(true);
  expect(controller.commitDrafts.edit(controller.snapshot!, "api2")).toBe(
    false,
  );
  controller.close();
});

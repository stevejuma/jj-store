import { expect, test, vi } from "vitest";
import { MockStackWorkspaceAdapter } from "../../../catalog/fixtures/stack-workspace/adapter.js";
import { workspaceFixture } from "../../../catalog/fixtures/stack-workspace/data.js";
import { applyWorkspaceMembership } from "../../src/lib/controller/stack-workspace/command-request.js";
import { StackWorkspaceController } from "../../src/lib/controller/stack-workspace/controller.svelte.js";

async function setup(fault: "none" | "unknown" | "stale" = "none") {
  const adapter = new MockStackWorkspaceAdapter(workspaceFixture(), fault);
  const controller = new StackWorkspaceController(adapter);
  await controller.refresh();
  return { adapter, controller };
}

test("one click applies and unapplies the exact branch without requesting a preview", async () => {
  const { adapter, controller } = await setup();
  const preview = vi.spyOn(adapter, "preview");
  const before = structuredClone(
    controller.snapshot!.workspaces[0]!.unassigned,
  );
  controller.commitDrafts.open(controller.snapshot!, "api");
  controller.commitDrafts.get(controller.snapshot!.active!, "api")!.title =
    "Keep my draft";
  const revision = controller.snapshot!.revision;
  await applyWorkspaceMembership(controller, {
    kind: "apply_stack",
    stack: "search",
  });
  expect(controller.snapshot!.workspaces[0]!.applied).toContain("search");
  expect(adapter.calls[0]).toMatchObject({
    revision,
    command: { kind: "apply_stack", stack: "search" },
  });
  await applyWorkspaceMembership(controller, {
    kind: "unapply_stack",
    stack: "search",
  });
  expect(controller.snapshot!.workspaces[0]!.applied).not.toContain("search");
  expect(controller.snapshot!.workspaces[0]!.unassigned).toEqual(before);
  expect(
    controller.commitDrafts.get(controller.snapshot!.active!, "api")?.title,
  ).toBe("Keep my draft");
  expect(preview).not.toHaveBeenCalled();
  expect(controller.preview).toBeNull();
  expect(controller.pending).toBeNull();
  expect(controller.error || controller.validation).toBe("");
  controller.close();
});

test("pending membership blocks duplicate clicks and disposal cannot install the result", async () => {
  const { adapter, controller } = await setup();
  const apply = adapter.apply.bind(adapter);
  let release!: () => void;
  const gate = new Promise<void>((resolve) => {
    release = resolve;
  });
  const calls = vi
    .spyOn(adapter, "apply")
    .mockImplementation(async (request) => {
      await gate;
      return apply(request);
    });
  const before = controller.snapshot;
  const running = applyWorkspaceMembership(controller, {
    kind: "unapply_stack",
    stack: "api",
  });
  expect(controller.locked).toBe(true);
  await applyWorkspaceMembership(controller, {
    kind: "unapply_stack",
    stack: "ui",
  });
  expect(calls).toHaveBeenCalledTimes(1);
  expect(controller.preview).toBeNull();
  controller.close();
  release();
  await running;
  expect(controller.snapshot).toBe(before);
});

test("unknown membership retains its exact request and never retries a local executor automatically", async () => {
  const { adapter, controller } = await setup("unknown");
  Object.assign(adapter, { retryPending: false });
  await applyWorkspaceMembership(controller, {
    kind: "unapply_stack",
    stack: "api",
  });
  const pending = controller.pending!;
  expect(pending).toEqual(adapter.calls[0]);
  expect(controller.preview).toBeNull();
  expect(controller.locked).toBe(true);
  await applyWorkspaceMembership(controller, {
    kind: "apply_stack",
    stack: "search",
  });
  await controller.apply();
  expect(adapter.calls).toHaveLength(1);
  expect(controller.pending).toEqual(pending);
  await controller.checkStatus();
  expect(controller.pending).toBeNull();
  expect(controller.snapshot!.workspaces[0]!.applied).not.toContain("api");
  controller.close();
});

test("stale membership is rejected without a stuck preview and can refresh", async () => {
  const { adapter, controller } = await setup("stale");
  await applyWorkspaceMembership(controller, {
    kind: "unapply_stack",
    stack: "api",
  });
  expect(controller.validation).toContain("workspace changed");
  expect(controller.preview).toBeNull();
  expect(controller.pending).toBeNull();
  expect(controller.locked).toBe(false);
  expect(controller.snapshot!.workspaces[0]!.applied).toContain("api");
  await controller.refresh();
  expect(controller.snapshot!.revision).toBe(adapter.snapshot.revision);
  controller.close();
});

test("capability denial and non-membership commands cannot use immediate execution", async () => {
  const { adapter, controller } = await setup();
  await applyWorkspaceMembership(controller, {
    kind: "delete_stack",
    stack: "api",
  });
  controller.snapshot!.capabilities = [];
  await applyWorkspaceMembership(controller, {
    kind: "unapply_stack",
    stack: "api",
  });
  expect(controller.validation).not.toBe("");
  expect(adapter.calls).toEqual([]);
  controller.close();
});

import { expect, test, vi } from "vitest";
import {
  productionStacksHttp,
  stackOperation,
} from "../../../catalog/fixtures/stack-workspace/production-http.js";
import { createProductionStoreController } from "../../src/lib/controller/production-store-controller.svelte.js";

test("real bootstrap injects a verified Stacks controller and its route never reads the ordinary graph", async () => {
  const fixture = productionStacksHttp();
  const controller = createProductionStoreController(fixture);
  try {
    await vi.waitFor(() =>
      expect(controller.stackWorkspace?.snapshot?.targetOptions).toHaveLength(
        2,
      ),
    );
    const stacks = controller.stackWorkspace!;
    expect(controller.state.route.page).toBe("stacks");
    expect(stacks.snapshot?.active).toBeNull();
    expect(stacks.physical.observation).toBeNull();
    expect(
      fixture.requests.some(
        (request) =>
          request.path.endsWith("/stacks") &&
          request.body?.command?.kind === "refresh",
      ),
    ).toBe(true);
    expect(stacks.snapshot?.capabilities).toEqual(["create_workspace"]);
    expect(stacks.snapshot?.workspaces).toEqual([]);
    await stacks.chooseTarget(stacks.snapshot!.targetOptions![0]!);
    const branch = stacks.snapshot!.stacks[0]!;
    stacks.selectStack(branch.id);
    await vi.waitFor(() =>
      expect(stacks.histories.state(branch.id).loaded).toBe(true),
    );
    expect(stacks.snapshot!.stacks[0]!.commits).toHaveLength(2);
    await controller.actions.refresh();
    expect(controller.stackWorkspace).toBe(stacks);
    expect(stacks.snapshot!.stacks[0]!.commits).toHaveLength(2);
    expect(
      fixture.requests.filter(
        (request) => request.body?.request?.kind === "graph",
      ),
    ).toEqual([]);
    expect(
      fixture.requests.filter((request) => request.path.includes("/v1/repos/")),
    ).toEqual([]);
    controller.actions.disconnect();
    expect(controller.stackWorkspace).toBeUndefined();
    expect(stacks.physical.observation).toBeNull();
  } finally {
    controller.actions.destroy();
  }
});
test("explicit index preparation restores Stacks at its exact operation and performs no code command", async () => {
  const fixture = productionStacksHttp(true);
  const controller = createProductionStoreController(fixture);
  try {
    await vi.waitFor(() =>
      expect(controller.state.status.error).toContain("index rebuild required"),
    );
    expect(controller.stackWorkspace?.error).toBe("");
    expect(
      fixture.requests.filter((request) =>
        request.path.endsWith("/index/tasks"),
      ),
    ).toEqual([]);
    await controller.actions.prepareIndex();
    await vi.waitFor(() =>
      expect(controller.stackWorkspace!.snapshot!.targetOptions).toHaveLength(
        2,
      ),
    );
    expect(controller.state.status.error).toBe("");
    expect(controller.state.route.snapshotOperationId).toBe(stackOperation);
    expect(
      fixture.requests.filter((request) =>
        request.path.endsWith("/index/tasks"),
      ),
    ).toHaveLength(1);
    expect(
      fixture.requests.filter((request) => request.path.includes("/commands")),
    ).toEqual([]);
  } finally {
    controller.actions.destroy();
  }
});

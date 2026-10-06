import { expect, test, vi } from "vitest";
import {
  productionStacksHttp,
  stackOperation,
  stackRepository,
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
    expect(stacks.snapshot?.capabilities).toEqual([
      "create_workspace",
      "undo",
      "redo",
    ]);
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

test("a direct focused history link initializes workspace context without loading stack history", async () => {
  const fixture = productionStacksHttp();
  fixture.browser.push(
    `#/repos/${stackRepository}/log?context=stacks&at_op=${stackOperation}`,
  );
  const controller = createProductionStoreController(fixture);
  try {
    await vi.waitFor(() =>
      expect(controller.state.status.navigation).toBe("ready"),
    );
    expect(controller.state.status.error).toBe("");
    expect(controller.stackWorkspace?.snapshot?.targetOptions).toHaveLength(2);
    expect(
      fixture.requests.some(
        (request) => request.body?.request?.kind === "graph",
      ),
    ).toBe(true);
    expect(
      fixture.requests.some(
        (request) => request.body?.request?.kind === "stack_history",
      ),
    ).toBe(false);
  } finally {
    controller.actions.destroy();
  }
});

test("focused histories read the ordinary repository graph and retain the Stacks owner", async () => {
  const fixture = productionStacksHttp();
  const controller = createProductionStoreController(fixture);
  try {
    await vi.waitFor(() =>
      expect(controller.stackWorkspace?.snapshot?.targetOptions).toHaveLength(
        2,
      ),
    );
    const stacks = controller.stackWorkspace!;
    const refreshes = fixture.requests.filter((request) =>
      request.path.endsWith("/stacks"),
    ).length;
    await controller.actions.selectStacksView("log");
    expect(controller.state.route).toMatchObject({
      page: "log",
      context: "stacks",
      workspace: null,
    });
    expect(controller.state.status.error).toBe("");
    expect(controller.stackWorkspace).toBe(stacks);
    const graph = fixture.requests.find(
      (request) => request.body?.request?.kind === "graph",
    )!;
    expect(graph.body.request.query[0]).toMatchObject({
      graph: "log",
      revset: "all()",
      workspace: null,
    });
    expect(
      fixture.requests.filter((request) => request.path.endsWith("/stacks")),
    ).toHaveLength(refreshes);
    await controller.actions.selectStacksView("oplog");
    expect(controller.state.status.error).toBe("");
    expect(controller.stackWorkspace).toBe(stacks);
    expect(
      fixture.requests.some(
        (request) => request.body?.request?.query?.[0]?.graph === "operations",
      ),
    ).toBe(true);
    controller.state.status.error = "index rebuild required";
    await controller.actions.selectStacksView("workspace");
    expect(controller.state.status.error).toBe("");
    expect(controller.state.status.navigation).toBe("ready");
    await controller.actions.selectStacksView("branches");
    expect(controller.stackWorkspace).toBe(stacks);
    expect(
      fixture.requests.filter((request) => request.path.endsWith("/stacks")),
    ).toHaveLength(refreshes);
    controller.actions.disconnect();
    expect(controller.stackWorkspace).toBeUndefined();
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

test.each(["live", "pinned"] as const)(
  "focused %s history uses the correct operation after refresh and tab entry",
  async (mode) => {
    const fixture = productionStacksHttp();
    let latest = stackOperation;
    const fetcher: typeof fetch = async (input, init) => {
      const response = await fixture.fetcher(input, init);
      if (!response.headers.get("content-type")?.includes("application/json"))
        return response;
      const request = init?.body ? JSON.parse(String(init.body)) : null;
      const operation = String(input).endsWith("/query")
        ? (request?.operation ?? latest)
        : latest;
      const json = JSON.stringify(await response.json()).replaceAll(
        stackOperation,
        operation,
      );
      return new Response(json, {
        status: response.status,
        headers: response.headers,
      });
    };
    fixture.browser.push(
      `#/repos/${stackRepository}/log?context=stacks&stack_history=${mode}&at_op=${stackOperation}`,
    );
    const controller = createProductionStoreController({ ...fixture, fetcher });
    try {
      await vi.waitFor(() =>
        expect(controller.state.status.navigation).toBe("ready"),
      );
      const stacks = controller.stackWorkspace;
      latest = "d".repeat(128);
      await controller.actions.refresh();
      expect(controller.state.route.snapshotOperationId).toBe(
        mode === "live" ? latest : stackOperation,
      );
      expect(controller.state.status.error).toBe("");
      await controller.actions.selectStacksView("workspace");
      latest = "e".repeat(128);
      await controller.actions.selectStacksView("log");
      expect(controller.state.route.snapshotOperationId).toBe(
        mode === "live" ? latest : stackOperation,
      );
      expect(controller.stackWorkspace).toBe(stacks);
      expect(controller.state.status.error).toBe("");
      const graphs = fixture.requests.filter(
        (request) => request.body?.request?.kind === "graph",
      );
      expect(graphs.at(-1)?.body.operation).toBe(
        mode === "live" ? latest : stackOperation,
      );
    } finally {
      controller.actions.destroy();
    }
  },
);

test("an accepted workspace mutation advances both focused history tabs without eager graph reads", async () => {
  const fixture = productionStacksHttp();
  let latest = stackOperation;
  const fetcher: typeof fetch = async (input, init) => {
    const request = init?.body ? JSON.parse(String(init.body)) : null;
    if (
      String(input).endsWith("/stacks") &&
      ["setup", "create"].includes(request?.command?.kind)
    )
      latest = "f".repeat(128);
    const response = await fixture.fetcher(input, init);
    if (!response.headers.get("content-type")?.includes("application/json"))
      return response;
    const value = await response.json();
    if (String(input).endsWith("/stacks") && latest !== stackOperation)
      value.revision = 1;
    return new Response(
      JSON.stringify(value).replaceAll(stackOperation, latest),
      { status: response.status, headers: response.headers },
    );
  };
  const controller = createProductionStoreController({ ...fixture, fetcher });
  try {
    await vi.waitFor(() =>
      expect(controller.stackWorkspace?.snapshot?.targetOptions).toHaveLength(
        2,
      ),
    );
    const stacks = controller.stackWorkspace!;
    await stacks.chooseTarget(stacks.snapshot!.targetOptions![0]!);
    await stacks.prepare({ kind: "create_workspace", name: "New workspace" });
    expect(stacks.preview).not.toBeNull();
    await stacks.apply();
    expect(stacks.snapshot?.operation).toBe(latest);
    expect(latest).not.toBe(stackOperation);
    expect(
      fixture.requests.filter(
        (request) => request.body?.request?.kind === "graph",
      ),
    ).toHaveLength(0);
    for (const tab of ["log", "oplog"] as const) {
      await controller.actions.selectStacksView(tab);
      expect(controller.stackWorkspace).toBe(stacks);
      expect(controller.state.route.snapshotOperationId).toBe(latest);
      const reads = fixture.requests.filter(
        (request) => request.body?.request?.kind === "graph",
      );
      expect(reads.at(-1)?.body.operation).toBe(latest);
    }
  } finally {
    controller.actions.destroy();
  }
});

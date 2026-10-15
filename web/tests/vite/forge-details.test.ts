import { expect, it } from "vitest";
import { workspaceFixture } from "../../../catalog/fixtures/stack-workspace/data.js";
import { MockStackWorkspaceAdapter } from "../../../catalog/fixtures/stack-workspace/adapter.js";
import { workspaceForgeTransport } from "../../../catalog/fixtures/forge/workspace-transport.js";
import { ForgeController } from "../../src/lib/controller/forge/controller.svelte.js";
import { StackWorkspaceController } from "../../src/lib/controller/stack-workspace/controller.svelte.js";
import { workspaceBranchSource } from "../../src/lib/controller/stack-workspace/files.js";
import {
  stackDetailsColumns,
  workspaceSourceBranch,
} from "../../src/lib/controller/stack-workspace/details-columns.js";

function fixture() {
  const snapshot = workspaceFixture();
  const remote = workspaceForgeTransport(true);
  const forge = new ForgeController(remote.transport);
  forge.data = remote.initial;
  const adapter = Object.assign(new MockStackWorkspaceAdapter(snapshot), {
    forge,
  });
  return {
    snapshot,
    forge,
    controller: new StackWorkspaceController(adapter, { initial: snapshot }),
  };
}

it("a PR-only view and its file share one origin, while closing the file retains the PR", () => {
  const { controller, forge, snapshot } = fixture();
  const stack = snapshot.stacks[0]!;
  forge.select("feature/api", "inline");
  expect(stackDetailsColumns(controller, stack)).toEqual([
    { origin: "stack-api", branch: "feature/api", diff: undefined },
  ]);
  controller.selectFile(
    "api2",
    "src/api.ts",
    ["src/api.ts:query endpoint"],
    "api",
  );
  const [detail] = stackDetailsColumns(controller, stack);
  expect(detail?.diff?.source).toBe("api2");
  expect(detail?.diff?.hunks).toEqual(["src/api.ts:query endpoint"]);
  expect(stackDetailsColumns(controller, stack)).toHaveLength(1);
  controller.closeDiff("stack-api");
  expect(stackDetailsColumns(controller, stack)[0]?.diff).toBeUndefined();
  expect(forge.selected?.number).toBe(42);
  forge.close();
  controller.close();
});

it("file selection resolves the precise bookmark PR and leaves other diff origins intact", () => {
  const { controller, forge, snapshot } = fixture();
  controller.selectFile("ui1", "src/toolbar.ts", undefined, "ui");
  const other = controller.diffs[0];
  controller.selectFile(
    workspaceBranchSource("api", "feature/model"),
    "src/model.ts",
    undefined,
    "api",
  );
  expect(forge.selected?.number).toBe(41);
  expect(forge.open).toBe(false);
  expect(stackDetailsColumns(controller, snapshot.stacks[0]!)[0]?.branch).toBe(
    "feature/model",
  );
  forge.detailsExpanded = false;
  controller.selectFile("api1", "src/model.ts", undefined, "api");
  expect(forge.detailsExpanded).toBe(false);
  expect(controller.diffs[0]).toBe(other);
  controller.selectFile("api2", "src/api.ts", undefined, "api");
  expect(forge.selected?.number).toBe(42);
  expect(forge.detailsExpanded).toBe(true);
  expect(workspaceSourceBranch(snapshot, "unassigned")).toBeUndefined();
  expect(workspaceSourceBranch(snapshot, "branch:bad-json")).toBeUndefined();
  forge.close();
  controller.close();
});

it("file browsing cannot replace an unknown publication plan", async () => {
  const { controller, forge } = fixture();
  await forge.inline.start("feature/workspace");
  const plan = forge.plan!;
  forge.pending = { kind: "submit", plan };
  controller.selectFile("api2", "src/api.ts", undefined, "api");
  expect(forge.pending?.kind).toBe("submit");
  expect(forge.plan).toBe(plan);
  expect(forge.inline.creating).toBe("feature/workspace");
  forge.close();
  controller.close();
});

import { expect, test, vi } from "vitest";
import { StackWorkspaceController } from "../../src/lib/controller/stack-workspace/controller.svelte.js";
import {
  toggleStackSource,
  toggleStackBranch,
  toggleTargetBranch,
  toggleTargetCommit,
} from "../../src/lib/controller/stack-workspace/selection-actions.js";
import { MockStackWorkspaceAdapter } from "../../../catalog/fixtures/stack-workspace/adapter.js";
import { workspaceFixture } from "../../../catalog/fixtures/stack-workspace/data.js";

async function setup() {
  const adapter = new MockStackWorkspaceAdapter(workspaceFixture());
  const controller = new StackWorkspaceController(adapter);
  await controller.refresh();
  return { controller, adapter };
}

test("deselect only the activated stack source, retaining drafts, other selections and diffs", async () => {
  const { controller, adapter } = await setup();
  controller.commitDrafts.open(controller.snapshot!, "api");
  controller.commitDrafts.get("daily", "api")!.title = "Retained draft";
  toggleStackSource(controller, "api", { commit: "api2" });
  controller.selectFile("api2", "src/api.ts");
  toggleStackSource(controller, "ui", { commit: "ui1" });
  const load = vi.spyOn(controller.details, "loadFiles");
  toggleStackSource(controller, "api", { commit: "api2" });
  expect(controller.stackSelections).toEqual({ ui: "ui1" });
  expect(controller.selectedCommit).toBeNull();
  expect(controller.diffs).toHaveLength(1);
  expect(controller.commitDrafts.get("daily", "api")!.title).toBe(
    "Retained draft",
  );
  expect(load).not.toHaveBeenCalled();
  toggleStackSource(controller, "api", { branch: "feature/model" });
  toggleStackSource(controller, "api", { branch: "feature/model" });
  expect(controller.stackSelections).toEqual({ ui: "ui1" });
  await controller.refresh();
  expect(controller.stackSelections).toEqual({ ui: "ui1" });
  expect(adapter.calls).toEqual([]);
  controller.close();
});

test("branch and target deselection preserves cached target history without commands", async () => {
  const { controller, adapter } = await setup();
  controller.selectStack("api", "feature/api");
  toggleStackBranch(controller, "api", "feature/api");
  expect(controller.selectedStack).toBeNull();
  expect(controller.selectedBranch).toBeNull();
  await controller.selectTargetBranch();
  const commits = [...controller.targetHistory.commits];
  const id = commits[0]!.id;
  toggleTargetCommit(controller, id);
  toggleTargetCommit(controller, id);
  expect(controller.targetHistory.selectedId).toBeNull();
  toggleTargetBranch(controller);
  expect(controller.targetHistory.active).toBe(false);
  expect(controller.targetHistory.commits).toEqual(commits);
  const read = vi.spyOn(adapter, "readTargetHistory");
  toggleTargetBranch(controller);
  expect(read).not.toHaveBeenCalled();
  expect(controller.targetHistory.active).toBe(true);
  expect(adapter.calls).toEqual([]);
  controller.close();
});

test("locked selections cannot be toggled while a command owns the UI", async () => {
  const { controller } = await setup();
  controller.selectStack("api", "feature/api");
  controller.selectCommit("api2");
  controller.busy = true;
  toggleStackSource(controller, "api", { commit: "api2" });
  toggleStackBranch(controller, "api", "feature/api");
  toggleTargetBranch(controller);
  expect(controller.stackSelections).toEqual({ api: "api2" });
  expect(controller.selectedStack).toBe("api");
  expect(controller.targetHistory.active).toBe(false);
  controller.close();
});

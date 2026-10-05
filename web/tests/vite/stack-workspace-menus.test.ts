import { describe, expect, it } from "vitest";
import { MockStackWorkspaceAdapter } from "../../../catalog/fixtures/stack-workspace/adapter.js";
import { workspaceFixture } from "../../../catalog/fixtures/stack-workspace/data.js";
import { StackWorkspaceController } from "../../src/lib/controller/stack-workspace/controller.svelte.js";
import { stackGroups } from "../../src/lib/controller/stack-workspace/presentation.js";
import type { WorkspaceCommand } from "../../src/lib/controller/stack-workspace/types.js";

async function setup() {
  const controller = new StackWorkspaceController(
    new MockStackWorkspaceAdapter(workspaceFixture()),
  );
  await controller.refresh();
  return controller;
}
async function run(controller: StackWorkspaceController, command: unknown) {
  await controller.prepare(command as WorkspaceCommand);
  expect(controller.preview, controller.validation).not.toBeNull();
  await controller.apply();
  expect(controller.error || controller.validation).toBe("");
}
describe("stack creation and Changeyard menu semantics", () => {
  it("creates independent stacks and dependent branch segments without duplicating lower commits", async () => {
    const controller = await setup();
    await run(controller, { kind: "create_stack", name: "independent" });
    expect(controller.snapshot!.stacks).toHaveLength(4);
    await run(controller, {
      kind: "create_stack",
      name: "dependent",
      parent: "api",
    });
    expect(controller.snapshot!.stacks).toHaveLength(4);
    const stack = controller.snapshot!.stacks[0]!;
    expect(stack.name).toBe("dependent");
    expect(stack.heads.map((head) => head.name)).toEqual([
      "dependent",
      "feature/api",
      "feature/model",
    ]);
    expect(
      stackGroups(stack).map((group) =>
        group.commits.map((commit) => commit.id),
      ),
    ).toEqual([[], ["api2"], ["api1"]]);
    await run(controller, {
      kind: "create_commit",
      stack: "api",
      message: "Dependent work",
    });
    expect(
      stackGroups(controller.snapshot!.stacks[0]!)[0]!.commits[0]!.message,
    ).toBe("Dependent work");
  });
  it.each(["above", "below"])(
    "inserts an empty commit %s at the exact selected position",
    async (placement) => {
      const controller = await setup();
      await run(controller, {
        kind: "add_empty_commit",
        commit: "api2",
        placement,
        message: "Empty checkpoint",
      });
      const commits = controller.snapshot!.stacks[0]!.commits;
      expect(commits.map((commit) => commit.message)).toEqual(
        placement === "above"
          ? [
              "Add repository model",
              "Expose repository API",
              "Empty checkpoint",
            ]
          : [
              "Add repository model",
              "Empty checkpoint",
              "Expose repository API",
            ],
      );
      expect(
        commits.find((commit) => commit.message === "Empty checkpoint")!.files,
      ).toEqual([]);
    },
  );
  it.each(["above", "below"])(
    "creates a branch %s without moving existing heads",
    async (placement) => {
      const controller = await setup();
      await run(controller, {
        kind: "create_branch",
        stack: "api",
        commit: "api2",
        placement,
        name: "new-branch",
      });
      const stack = controller.snapshot!.stacks[0]!;
      expect(
        stack.heads.find((head) => head.name === "new-branch")!.commit,
      ).toBe(placement === "above" ? "api2" : "api1");
      expect(
        stack.heads.find((head) => head.name === "feature/api")!.commit,
      ).toBe("api2");
      expect(
        stackGroups(stack)
          .flatMap((group) => group.commits)
          .map((commit) => commit.id),
      ).toEqual(["api2", "api1"]);
    },
  );
  it("rejects stale parents and duplicate branch names", async () => {
    const controller = await setup();
    await controller.prepare({
      kind: "create_stack",
      name: "dependent",
      parent: "missing",
    } as WorkspaceCommand);
    expect(controller.preview).toBeNull();
    expect(controller.validation).toMatch(/existing stack/);
    await controller.prepare({ kind: "create_stack", name: "feature/model" });
    expect(controller.preview).toBeNull();
    expect(controller.validation).toMatch(/already in use/);
  });
  it("deletes a commit without returning its files and undo restores the exact state", async () => {
    const controller = await setup();
    const before = structuredClone(controller.snapshot);
    await run(controller, { kind: "delete_commit", commit: "api2" });
    expect(
      controller.snapshot!.stacks[0]!.commits.map((commit) => commit.id),
    ).toEqual(["api1"]);
    expect(controller.snapshot!.workspaces[0]!.unassigned).toEqual(
      before!.workspaces[0]!.unassigned,
    );
    await run(controller, { kind: "undo" });
    expect(controller.snapshot!.stacks).toEqual(before!.stacks);
  });
  it("squashes the selected branch segment and retains lower branch changes", async () => {
    const controller = await setup();
    await run(controller, {
      kind: "add_empty_commit",
      commit: "api2",
      placement: "above",
      message: "Additional API work",
    });
    await run(controller, {
      kind: "squash_branch",
      stack: "api",
      name: "feature/api",
    });
    const stack = controller.snapshot!.stacks[0]!;
    expect(stack.commits).toHaveLength(2);
    expect(stack.commits[0]!.message).toBe("Add repository model");
    expect(stack.commits[1]!.message).toContain("Additional API work");
    expect(stack.commits[1]!.files[0]!.path).toBe("src/api.ts");
  });
  it("edits in a retained inline draft and saves only explicitly staged additions", async () => {
    const controller = await setup();
    expect(controller.commitDrafts.edit(controller.snapshot!, "api2")).toBe(
      true,
    );
    const draft = controller.commitDrafts.get("daily", "api")!;
    expect(draft.commit).toBe("api2");
    expect(draft.title).toBe("Expose repository API");
    expect(draft.selections[0]!.source).toBe("api2");
    expect(controller.snapshot!.stacks[0]!.commits[1]!.message).toBe(
      "Expose repository API",
    );
    controller.commitDrafts.open(controller.snapshot!, "api", {
      source: "unassigned",
      path: "README.md",
    });
    await run(controller, {
      kind: "edit_commit",
      commit: "api2",
      message: "Edited API",
      selections: controller.commitDrafts.get("daily", "api")!.selections,
    });
    expect(controller.commitDrafts.get("daily", "api")).toBeUndefined();
    expect(
      controller.snapshot!.stacks[0]!.commits[1]!.files.map(
        (file) => file.path,
      ),
    ).toEqual(["src/api.ts", "README.md"]);
    expect(
      controller.snapshot!.workspaces[0]!.unassigned.map((file) => file.path),
    ).toEqual(["src/settings.ts"]);
  });
});

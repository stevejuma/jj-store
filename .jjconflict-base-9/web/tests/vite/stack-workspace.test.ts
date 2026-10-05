import { describe, expect, it, vi } from "vitest";
import { StackWorkspaceController } from "../../src/lib/controller/stack-workspace/controller.svelte.js";
import { WorkspaceDragSession } from "../../src/lib/controller/stack-workspace/drag-session.svelte.js";
import {
  activeWorkspace,
  stackGroups,
} from "../../src/lib/controller/stack-workspace/presentation.js";
import {
  combinedWorkspaceFiles,
  workspaceSourceFiles,
  workspaceHunkSelection,
  workspaceFileSelection,
  workspaceBranchSource,
} from "../../src/lib/controller/stack-workspace/files.js";
import {
  workspaceDrop,
  workspaceDropFeedback,
} from "../../src/lib/controller/stack-workspace/drag.js";
import type { WorkspaceCommand } from "../../src/lib/controller/stack-workspace/types.js";
import { MockStackWorkspaceAdapter } from "../../../catalog/fixtures/stack-workspace/adapter.js";
import { workspaceFixture } from "../../../catalog/fixtures/stack-workspace/data.js";

async function setup(
  fault: ConstructorParameters<typeof MockStackWorkspaceAdapter>[1] = "none",
) {
  const adapter = new MockStackWorkspaceAdapter(workspaceFixture(), fault);
  const controller = new StackWorkspaceController(adapter);
  await controller.refresh();
  return { adapter, controller };
}
async function run(
  controller: StackWorkspaceController,
  command: WorkspaceCommand,
) {
  await controller.prepare(command);
  expect(controller.preview, controller.validation).not.toBeNull();
  await controller.apply();
  expect(controller.error || controller.validation).toBe("");
}
describe("stack workspace UI controller and fixture", () => {
  it("sets native drag cursor feedback and clears ownership on teardown", () => {
    const drag = new WorkspaceDragSession();
    const snapshot = workspaceFixture();
    const payload = JSON.stringify({
      kind: "selection",
      selection: { source: "unassigned", path: "README.md" },
    });
    drag.payload = payload;
    const dataTransfer = {
      types: ["application/vnd.lapis.stack-workspace+json"],
      dropEffect: "none",
      getData: () => payload,
    };
    const event = {
      dataTransfer,
      preventDefault: vi.fn(),
      stopPropagation: vi.fn(),
    } as unknown as DragEvent;
    drag.over(event, { kind: "stage", id: "ui" }, snapshot, false);
    expect(dataTransfer.dropEffect).toBe("move");
    expect(event.preventDefault).toHaveBeenCalledOnce();
    snapshot.stacks[1]!.commits[0]!.immutable = true;
    drag.over(event, { kind: "commit", id: "ui1" }, snapshot, false);
    expect(dataTransfer.dropEffect).toBe("none");
    drag.clear();
    expect(drag.payload).toBeNull();
    expect(drag.feedback).toBeNull();
  });
  it("resolves whole files only when every hunk has one mutable origin", () => {
    const snapshot = workspaceFixture();
    expect(
      workspaceFileSelection(snapshot, "unassigned", "src/settings.ts")?.hunks,
    ).toHaveLength(2);
    expect(
      workspaceFileSelection(snapshot, "combined", "src/settings.ts")?.source,
    ).toBe("unassigned");
    snapshot.stacks[1]!.commits[0]!.files.push(
      structuredClone(snapshot.workspaces[0]!.unassigned[0]!),
    );
    expect(
      workspaceFileSelection(snapshot, "combined", "src/settings.ts"),
    ).toBeNull();
    snapshot.stacks[0]!.commits[0]!.immutable = true;
    expect(
      workspaceFileSelection(snapshot, "stack:api", "src/model.ts"),
    ).toBeNull();
  });
  it("describes valid and rejected Changeyard-style targets before mutating", () => {
    const snapshot = workspaceFixture();
    const payload = JSON.stringify({
      kind: "selection",
      selection: { source: "unassigned", path: "README.md" },
    });
    expect(
      workspaceDropFeedback(snapshot, payload, { kind: "stage", id: "ui" }),
    ).toMatchObject({
      state: "valid",
      command: { kind: "create_commit", stack: "ui" },
    });
    expect(
      workspaceDropFeedback(snapshot, payload, { kind: "commit", id: "ui1" }),
    ).toMatchObject({
      state: "valid",
      command: { kind: "transfer", target: "ui1" },
    });
    expect(
      workspaceDropFeedback(snapshot, payload, { kind: "working" }),
    ).toMatchObject({ state: "invalid" });
    snapshot.stacks[1]!.commits[0]!.immutable = true;
    expect(
      workspaceDropFeedback(snapshot, payload, { kind: "commit", id: "ui1" }),
    ).toMatchObject({
      state: "invalid",
      message: expect.stringContaining("immutable"),
    });
    expect(
      workspaceDropFeedback(
        snapshot,
        payload,
        { kind: "stage", id: "ui" },
        true,
      ).state,
    ).toBe("invalid");
    expect(
      workspaceDropFeedback(
        snapshot,
        JSON.stringify({
          kind: "selection",
          selection: { source: "unassigned", path: "README.md", hunks: [] },
        }),
        { kind: "stage", id: "ui" },
      ).state,
    ).toBe("invalid");
  });
  it("moves a folder's exact selections together and returns committed changes to Unstaged", async () => {
    const { controller } = await setup();
    const selections = activeWorkspace(controller.snapshot!).unassigned.map(
      (file) => ({ source: "unassigned", path: file.path }),
    );
    const command = workspaceDrop(
      JSON.stringify({ kind: "folder", selections }),
      "commit",
      "ui1",
    )!;
    await run(controller, command);
    expect(activeWorkspace(controller.snapshot!).unassigned).toHaveLength(0);
    expect(
      controller.snapshot!.stacks[1]!.commits[0]!.files.map(
        (file) => file.path,
      ),
    ).toEqual(["src/toolbar.ts", "src/settings.ts", "README.md"]);
    await run(controller, {
      kind: "transfer",
      target: "unassigned",
      selection: {
        source: "ui1",
        path: "src/settings.ts",
        hunks: ["src/settings.ts:remember preference"],
      },
    });
    expect(
      activeWorkspace(controller.snapshot!).unassigned[0]!.hunks.map(
        (hunk) => hunk.id,
      ),
    ).toEqual(["src/settings.ts:remember preference"]);
    expect(
      controller.snapshot!.stacks[1]!.commits[0]!.files.find(
        (file) => file.path === "src/settings.ts",
      )!.hunks,
    ).toHaveLength(1);
  });
  it("stages multiple exact file/hunk drops without changing the snapshot", async () => {
    const { controller } = await setup();
    const before = JSON.stringify(controller.snapshot);
    const snapshot = controller.snapshot!;
    controller.commitDrafts.open(snapshot, "ui", {
      source: "unassigned",
      path: "src/settings.ts",
      hunks: ["src/settings.ts:add keyboard shortcut"],
    });
    controller.commitDrafts.open(snapshot, "ui", {
      source: "unassigned",
      path: "README.md",
    });
    controller.commitDrafts.open(snapshot, "ui", {
      source: "unassigned",
      path: "src/settings.ts",
      hunks: ["src/settings.ts:add keyboard shortcut"],
    });
    const draft = controller.commitDrafts.get("daily", "ui")!;
    expect(draft.selections).toHaveLength(2);
    expect(JSON.stringify(controller.snapshot)).toBe(before);
    draft.title = "Keyboard shortcut";
    await run(controller, {
      kind: "create_commit",
      stack: "ui",
      message: draft.title,
      selections: draft.selections,
    });
    const commit = controller
      .snapshot!.stacks.find((stack) => stack.id === "ui")!
      .commits.at(-1)!;
    expect(commit.files.map((file) => file.path)).toEqual([
      "src/settings.ts",
      "README.md",
    ]);
    expect(commit.files[0]!.hunks.map((hunk) => hunk.id)).toEqual([
      "src/settings.ts:add keyboard shortcut",
    ]);
    expect(
      activeWorkspace(controller.snapshot!).unassigned[0]!.hunks.map(
        (hunk) => hunk.id,
      ),
    ).toEqual(["src/settings.ts:remember preference"]);
    expect(controller.commitDrafts.get("daily", "ui")).toBeUndefined();
  });
  it("merges hunk drops and lets a whole-file selection supersede them", async () => {
    const { controller } = await setup();
    const snapshot = controller.snapshot!;
    for (const id of ["add keyboard shortcut", "remember preference"])
      controller.commitDrafts.open(snapshot, "ui", {
        source: "unassigned",
        path: "src/settings.ts",
        hunks: [`src/settings.ts:${id}`],
      });
    expect(
      controller.commitDrafts.get("daily", "ui")!.selections[0]!.hunks,
    ).toHaveLength(2);
    controller.commitDrafts.open(snapshot, "ui", {
      source: "unassigned",
      path: "src/settings.ts",
    });
    controller.commitDrafts.open(snapshot, "ui", {
      source: "unassigned",
      path: "src/settings.ts",
      hunks: ["src/settings.ts:remember preference"],
    });
    expect(controller.commitDrafts.get("daily", "ui")!.selections).toEqual([
      { source: "unassigned", path: "src/settings.ts" },
    ]);
  });
  it("retains scoped drafts through switching and cancellation", async () => {
    const { controller } = await setup();
    controller.commitDrafts.open(controller.snapshot!, "ui");
    controller.commitDrafts.get("daily", "ui")!.title = "Daily draft";
    await controller.prepare({
      kind: "create_commit",
      stack: "ui",
      message: "Daily draft",
    });
    controller.cancelPreview();
    expect(controller.commitDrafts.get("daily", "ui")!.title).toBe(
      "Daily draft",
    );
    await run(controller, {
      kind: "switch_workspace",
      workspace: "experiment",
    });
    expect(controller.commitDrafts.get("experiment", "ui")).toBeUndefined();
    controller.commitDrafts.open(controller.snapshot!, "ui");
    controller.commitDrafts.get("experiment", "ui")!.title = "Experiment draft";
    controller.commitDrafts.clear("experiment", "ui");
    expect(controller.commitDrafts.get("daily", "ui")!.title).toBe(
      "Daily draft",
    );
    await run(controller, { kind: "switch_workspace", workspace: "daily" });
    expect(controller.commitDrafts.get("daily", "ui")!.title).toBe(
      "Daily draft",
    );
  });
  it("retains a staged draft and exact request until unknown outcome recovery", async () => {
    const { controller, adapter } = await setup("unknown");
    controller.commitDrafts.open(controller.snapshot!, "ui", {
      source: "unassigned",
      path: "README.md",
    });
    const draft = controller.commitDrafts.get("daily", "ui")!;
    draft.title = "Document shortcut";
    await controller.prepare({
      kind: "create_commit",
      stack: "ui",
      message: draft.title,
      selections: draft.selections,
    });
    const request = controller.preview!.request;
    await controller.apply();
    expect(controller.pending).toEqual(request);
    expect(controller.commitDrafts.get("daily", "ui")).toEqual(draft);
    await controller.apply();
    expect(adapter.calls).toEqual([request, request]);
    expect(controller.commitDrafts.get("daily", "ui")).toBeUndefined();
    expect(
      controller
        .snapshot!.stacks.find((stack) => stack.id === "ui")!
        .commits.filter((commit) => commit.message === draft.title),
    ).toHaveLength(1);
  });
  it("rejects unavailable and immutable drops without opening a draft", async () => {
    const { controller } = await setup();
    const snapshot = controller.snapshot!;
    snapshot.stacks[0]!.commits[0]!.immutable = true;
    for (const selection of [
      { source: "missing", path: "README.md" },
      { source: "api1", path: "src/model.ts" },
      { source: "unassigned", path: "src/settings.ts", hunks: ["stale"] },
    ])
      expect(controller.commitDrafts.open(snapshot, "ui", selection)).toBe(
        false,
      );
    expect(controller.commitDrafts.open(snapshot, "search")).toBe(false);
    expect(controller.commitDrafts.get("daily", "ui")).toBeUndefined();
  });
  it("keeps independent diff selections beside their originating columns", async () => {
    const { controller } = await setup();
    controller.selectFile("unassigned", "README.md");
    controller.selectFile("api2", "src/api.ts", [
      "src/api.ts:mutation endpoint",
    ]);
    controller.selectStack("ui");
    controller.selectCommit("ui1");
    controller.selectFile("ui1", "src/toolbar.ts");
    expect(controller.diffs.map((diff) => diff.origin)).toEqual([
      "working",
      "stack-api",
      "stack-ui",
    ]);
    expect(controller.diffs[1]!.hunks).toEqual([
      "src/api.ts:mutation endpoint",
    ]);
    controller.closeDiff("stack-ui");
    expect(controller.diffs.map((diff) => diff.origin)).toEqual([
      "working",
      "stack-api",
    ]);
    controller.selectFile("ui1", "src/toolbar.ts");
    expect(controller.diffs).toHaveLength(3);
    await run(controller, {
      kind: "reword",
      commit: "ui1",
      message: "Updated workspace",
    });
    expect(controller.diffs).toHaveLength(3);
    expect(controller.diffs[1]!.hunks).toEqual([
      "src/api.ts:mutation endpoint",
    ]);
  });
  it("reconciles obsolete diffs without clearing other stacks on refresh or recovery", async () => {
    const { adapter, controller } = await setup("unknown");
    controller.selectFile("unassigned", "README.md");
    controller.selectFile("api2", "src/api.ts");
    controller.selectFile("ui1", "src/toolbar.ts");
    await controller.prepare({ kind: "uncommit", commit: "api2" });
    await controller.apply();
    expect(controller.diffs).toHaveLength(3);
    await controller.checkStatus();
    expect(controller.diffs.map((diff) => diff.origin)).toEqual([
      "working",
      "stack-ui",
    ]);
    await run(controller, {
      kind: "switch_workspace",
      workspace: "experiment",
    });
    expect(controller.diffs.map((diff) => diff.origin)).toEqual(["stack-ui"]);
    adapter.snapshot.stacks = adapter.snapshot.stacks.filter(
      (stack) => stack.id !== "ui",
    );
    await controller.refresh();
    expect(controller.diffs).toEqual([]);
  });
  it("retains only available selected hunks after a split", async () => {
    const { controller } = await setup();
    controller.selectFile("api2", "src/api.ts", [
      "src/api.ts:mutation endpoint",
    ]);
    await run(controller, {
      kind: "split",
      commit: "api2",
      message: "Mutation",
      selection: controller.selection!,
    });
    expect(controller.diffs[0]).toMatchObject({
      origin: "stack-api",
      source: "api2",
      path: "src/api.ts",
      hunks: [],
    });
  });
  it("selects branch files inline and preserves independent commit browsing", async () => {
    const { controller } = await setup();
    const source = workspaceBranchSource("api", "feature/model");
    controller.selectStackFiles("api", "feature/model");
    expect(
      workspaceSourceFiles(controller.snapshot!, source).map(
        (file) => file.path,
      ),
    ).toEqual(["src/model.ts"]);
    controller.selectFile(source, "src/model.ts");
    expect(controller.selection).toBeNull();
    expect(controller.diffs[0]).toMatchObject({ origin: "stack-api", source });
    controller.selectCommit("ui1");
    expect(controller.stackSelections.api).toBe(source);
    controller.selectCommit("api2");
    expect(controller.selectedCommitFor("ui")).toBe("ui1");
    expect(controller.selectedCommitFor("api")).toBe("api2");
    await run(controller, { kind: "uncommit", commit: "api1" });
    expect(controller.diffs).toEqual([]);
    expect(controller.selectedCommitFor("ui")).toBe("ui1");
  });
  it("groups dependent bookmark segments without duplicate commits", () => {
    const groups = stackGroups(workspaceFixture().stacks[0]!);
    expect(
      groups.map((group) => [
        group.head.name,
        group.commits.map((commit) => commit.id),
      ]),
    ).toEqual([
      ["feature/api", ["api2"]],
      ["feature/model", ["api1"]],
    ]);
  });
  it.each(["git_sha1_jj045", "native"] as const)(
    "%s retains WIP through apply/unapply and base-only state",
    async (profile) => {
      const adapter = new MockStackWorkspaceAdapter(workspaceFixture(profile));
      const controller = new StackWorkspaceController(adapter);
      await controller.refresh();
      const before = activeWorkspace(controller.snapshot!).unassigned;
      await run(controller, { kind: "unapply_stack", stack: "api" });
      await run(controller, { kind: "unapply_stack", stack: "ui" });
      expect(activeWorkspace(controller.snapshot!).applied).toEqual([]);
      expect(activeWorkspace(controller.snapshot!).unassigned).toEqual(before);
      expect(
        combinedWorkspaceFiles(controller.snapshot!).map((file) => file.path),
      ).toEqual(["README.md", "src/settings.ts"]);
      await run(controller, { kind: "apply_stack", stack: "api" });
      expect(
        combinedWorkspaceFiles(controller.snapshot!).map((file) => file.path),
      ).toContain("src/model.ts");
    },
  );
  it("previews do not mutate and named combinations retain independent WIP", async () => {
    const { adapter, controller } = await setup();
    const before = await adapter.read();
    await controller.prepare({
      kind: "switch_workspace",
      workspace: "experiment",
    });
    expect(await adapter.read()).toEqual(before);
    await controller.apply();
    expect(activeWorkspace(controller.snapshot!).unassigned[0]!.path).toBe(
      "src/experiment.ts",
    );
    await run(controller, { kind: "switch_workspace", workspace: "daily" });
    expect(activeWorkspace(controller.snapshot!).unassigned).toEqual(
      before.workspaces[0]!.unassigned,
    );
  });
  it("selected hunk commit preserves unselected and unrelated changes", async () => {
    const { controller } = await setup();
    await run(controller, {
      kind: "create_commit",
      stack: "api",
      message: "Shortcut",
      selection: {
        source: "unassigned",
        path: "src/settings.ts",
        hunks: ["src/settings.ts:add keyboard shortcut"],
      },
    });
    expect(
      activeWorkspace(controller.snapshot!).unassigned[0]!.hunks.map(
        (hunk) => hunk.label,
      ),
    ).toEqual(["remember preference"]);
    expect(activeWorkspace(controller.snapshot!).unassigned[1]!.path).toBe(
      "README.md",
    );
    expect(
      controller
        .snapshot!.stacks[0]!.commits.at(-1)!
        .files[0]!.hunks.map((hunk) => hunk.label),
    ).toEqual(["add keyboard shortcut"]);
  });
  it("amends a commit with an exact file selection", async () => {
    const { controller } = await setup();
    await run(controller, {
      kind: "transfer",
      selection: { source: "unassigned", path: "README.md" },
      target: "ui1",
    });
    expect(
      activeWorkspace(controller.snapshot!).unassigned.map((file) => file.path),
    ).not.toContain("README.md");
    expect(
      controller.snapshot!.stacks[1]!.commits[0]!.files.map(
        (file) => file.path,
      ),
    ).toContain("README.md");
  });
  it("splits, reorders, squashes and moves commits while retaining identity", async () => {
    const { controller } = await setup();
    await run(controller, {
      kind: "split",
      commit: "api2",
      message: "Mutation",
      selection: {
        source: "api2",
        path: "src/api.ts",
        hunks: ["src/api.ts:mutation endpoint"],
      },
    });
    const id = controller.snapshot!.stacks[0]!.commits.at(-1)!.id;
    expect(
      controller.snapshot!.stacks[0]!.commits[1]!.files[0]!.hunks,
    ).toHaveLength(1);
    await run(controller, {
      kind: "reorder_commit",
      commit: id,
      direction: "down",
    });
    expect(controller.snapshot!.stacks[0]!.commits[1]!.id).toBe(id);
    await run(controller, { kind: "squash", commit: id, target: "api2" });
    expect(controller.snapshot!.stacks[0]!.commits).toHaveLength(2);
    await run(controller, { kind: "move_commit", commit: "api2", stack: "ui" });
    expect(controller.snapshot!.stacks[1]!.commits.at(-1)!.id).toBe("api2");
  });
  it("rewords, uncommits and undoes the last operation without losing other work", async () => {
    const { controller } = await setup();
    await run(controller, {
      kind: "reword",
      commit: "api2",
      message: "New API message",
    });
    await run(controller, { kind: "uncommit", commit: "api2" });
    expect(
      activeWorkspace(controller.snapshot!).unassigned.map((file) => file.path),
    ).toContain("src/api.ts");
    await run(controller, { kind: "undo" });
    expect(controller.snapshot!.stacks[0]!.commits[1]!.message).toBe(
      "New API message",
    );
    expect(
      activeWorkspace(controller.snapshot!).unassigned.map((file) => file.path),
    ).not.toContain("src/api.ts");
  });
  it("creates, switches and previews deletion of active and inactive combinations", async () => {
    const { controller } = await setup();
    await run(controller, { kind: "create_workspace", name: "Review" });
    const id = controller.snapshot!.active;
    await controller.prepare({ kind: "delete_workspace", workspace: id });
    expect(controller.preview?.request.command).toEqual({
      kind: "delete_workspace",
      workspace: id,
    });
    controller.cancelPreview();
    await run(controller, { kind: "switch_workspace", workspace: "daily" });
    await run(controller, { kind: "delete_workspace", workspace: id });
    expect(controller.snapshot!.workspaces).toHaveLength(2);
    await run(controller, { kind: "create_stack", name: "feature/new" });
    const stack = controller.snapshot!.stacks.at(-1)!;
    await run(controller, {
      kind: "rename_stack",
      stack: stack.id,
      name: "feature/renamed",
    });
    await run(controller, { kind: "rebase", stack: stack.id });
    await run(controller, { kind: "delete_stack", stack: stack.id });
    expect(controller.snapshot!.stacks).toHaveLength(3);
  });
  it("preserves original unknown requests and resolves by receipt before stale checks", async () => {
    const { adapter, controller } = await setup("unknown");
    const command: WorkspaceCommand = {
      kind: "rename_stack",
      stack: "api",
      name: "feature/renamed",
    };
    await controller.prepare(command);
    command.name = "different";
    await controller.apply();
    const id = controller.pending!.id;
    expect(controller.snapshot!.stacks[0]!.name).toBe("feature/api");
    await controller.prepare({ kind: "undo" });
    expect(controller.pending!.id).toBe(id);
    await controller.apply();
    expect(adapter.calls.map((call) => call.id)).toEqual([id, id]);
    expect(controller.snapshot!.stacks[0]!.name).toBe("feature/renamed");
    expect(controller.pending).toBeNull();
  });
  it("recovers an unknown outcome through status with no duplicate execution", async () => {
    const { adapter, controller } = await setup("unknown");
    await controller.prepare({ kind: "unapply_stack", stack: "api" });
    await controller.apply();
    await controller.checkStatus();
    expect(adapter.calls).toHaveLength(1);
    expect(activeWorkspace(controller.snapshot!).applied).toEqual(["ui"]);
  });
  it("stale apply retains the preview, exact selection and original draft", async () => {
    const { controller } = await setup("stale");
    controller.selectFile("unassigned", "README.md");
    await controller.prepare({
      kind: "create_commit",
      stack: "api",
      message: "Retained draft",
      selection: controller.selection!,
    });
    await controller.apply();
    expect(controller.validation).toContain("workspace changed");
    expect(controller.preview!.request.command).toMatchObject({
      message: "Retained draft",
    });
    expect(controller.selection!.path).toBe("README.md");
    controller.cancelPreview();
    await controller.refresh();
    expect(controller.snapshot!.revision).toBe(2);
  });
  it("known failure leaves fixture and preview intact for retry", async () => {
    const { adapter, controller } = await setup("failed");
    const before = await adapter.read();
    await controller.prepare({ kind: "unapply_stack", stack: "api" });
    await controller.apply();
    expect(controller.error).toContain("retained");
    expect(await adapter.read()).toEqual(before);
    await controller.apply();
    expect(controller.preview).toBeNull();
  });
  it("previews conflicts and preserves unassigned edits when resolving", async () => {
    const { controller } = await setup();
    await controller.prepare({ kind: "apply_stack", stack: "search" });
    expect(controller.preview!.conflicts).toEqual(["src/settings.ts"]);
    await controller.apply();
    expect(controller.snapshot!.conflicts).toHaveLength(1);
    await run(controller, {
      kind: "resolve_conflict",
      path: "src/settings.ts",
      content: "resolved",
    });
    expect(controller.snapshot!.conflicts).toEqual([]);
    expect(
      activeWorkspace(controller.snapshot!).unassigned[0]!.hunks,
    ).toHaveLength(3);
  });
  it("rejects immutable commits, malformed selections, same destinations and unavailable checkouts", async () => {
    const { adapter, controller } = await setup();
    adapter.snapshot.stacks[0]!.commits[0]!.immutable = true;
    await controller.refresh();
    await controller.prepare({
      kind: "reword",
      commit: "api1",
      message: "Unsafe",
    });
    expect(controller.validation).toContain("immutable");
    await controller.prepare({
      kind: "transfer",
      selection: { source: "api2", path: "src/api.ts" },
      target: "api2",
    });
    expect(controller.validation).toContain("different");
    await controller.prepare({
      kind: "transfer",
      selection: {
        source: "unassigned",
        path: "README.md",
        hunks: ["missing"],
      },
      target: "ui1",
    });
    expect(controller.validation).toContain("stale");
    adapter.snapshot.availability = "unlinked";
    await controller.refresh();
    await controller.prepare({ kind: "undo" });
    expect(controller.validation).toContain("available local checkout");
  });
  it("rejects malformed/foreign drags and maps valid drops to previewable commands", () => {
    expect(workspaceDrop("broken", "commit", "ui1")).toBeNull();
    expect(
      workspaceDrop(
        JSON.stringify({ kind: "commit", commit: "ui1" }),
        "commit",
        "ui1",
      ),
    ).toBeNull();
    expect(
      workspaceDrop(
        JSON.stringify({ kind: "stack", stack: "api" }),
        "workspace",
      ),
    ).toEqual({ kind: "apply_stack", stack: "api" });
    expect(
      workspaceDrop(
        JSON.stringify({
          kind: "selection",
          selection: { source: "unassigned", path: "README.md" },
        }),
        "commit",
        "ui1",
      ),
    ).toMatchObject({ kind: "transfer", target: "ui1" });
  });
  it("suppresses a late read after the controller closes", async () => {
    const { adapter, controller } = await setup();
    let resolve!: (snapshot: ReturnType<typeof workspaceFixture>) => void;
    adapter.read = () =>
      new Promise((done) => {
        resolve = done;
      });
    const pending = controller.refresh();
    controller.close();
    const newer = workspaceFixture();
    newer.revision = 100;
    resolve(newer);
    await pending;
    expect(controller.snapshot!.revision).toBe(1);
  });
  it("requires a physical folder even when an adapter reports ready", async () => {
    const { adapter, controller } = await setup();
    adapter.snapshot.folder = null;
    await controller.refresh();
    await controller.prepare({ kind: "create_stack", name: "Unavailable" });
    expect(controller.preview).toBeNull();
    expect(controller.validation).toContain("available local checkout");
    expect(adapter.calls).toHaveLength(0);
  });
  it("retains the original request when apply acknowledgement throws", async () => {
    const { adapter, controller } = await setup();
    const apply = adapter.apply.bind(adapter);
    adapter.apply = async (request) => {
      await apply(request);
      throw new Error("Acknowledgement lost");
    };
    await controller.prepare({
      kind: "create_commit",
      stack: "api",
      message: "Exact intent",
    });
    const request = controller.preview!.request;
    await controller.apply();
    expect(controller.pending).toEqual(request);
    expect(controller.locked).toBe(true);
    await controller.checkStatus();
    expect(controller.pending).toBeNull();
    expect(
      controller.snapshot!.stacks[0]!.commits.filter(
        (commit) => commit.message === "Exact intent",
      ),
    ).toHaveLength(1);
  });
  it("owns preview intent even if an adapter changes its request copy", async () => {
    const { adapter, controller } = await setup();
    const preview = adapter.preview.bind(adapter);
    adapter.preview = async (request) => {
      const result = await preview(request);
      result.request.command = { kind: "delete_stack", stack: "api" };
      return result;
    };
    await controller.prepare({
      kind: "create_commit",
      stack: "api",
      message: "Owned intent",
    });
    expect(controller.preview!.request.command).toEqual({
      kind: "create_commit",
      stack: "api",
      message: "Owned intent",
    });
    await controller.apply();
    expect(controller.snapshot!.stacks[0]!.commits.at(-1)!.message).toBe(
      "Owned intent",
    );
  });
});

describe("aggregate hunk assignment", () => {
  it("resolves a unique source and transfers only that hunk", async () => {
    const { controller } = await setup();
    const selection = workspaceHunkSelection(
      controller.snapshot!,
      "combined",
      "src/settings.ts",
      "src/settings.ts:add keyboard shortcut",
    );
    expect(selection?.source).toBe("unassigned");
    await run(controller, {
      kind: "transfer",
      target: "ui1",
      selection: selection!,
    });
    expect(
      activeWorkspace(controller.snapshot!)
        .unassigned.find((file) => file.path === "src/settings.ts")!
        .hunks.map((hunk) => hunk.label),
    ).toEqual(["remember preference"]);
  });
  it("rejects ambiguous union identities and immutable origins", () => {
    const snapshot = workspaceFixture();
    const commit = snapshot.stacks[0]!.commits[1]!;
    const path = "src/api.ts",
      hunk = "src/api.ts:query endpoint";
    expect(
      workspaceHunkSelection(snapshot, "stack:api", path, hunk)?.source,
    ).toBe("api2");
    commit.immutable = true;
    expect(
      workspaceHunkSelection(snapshot, "stack:api", path, hunk),
    ).toBeNull();
    commit.immutable = false;
    snapshot.workspaces[0]!.unassigned.push(structuredClone(commit.files[0]!));
    expect(workspaceHunkSelection(snapshot, "combined", path, hunk)).toBeNull();
  });
});

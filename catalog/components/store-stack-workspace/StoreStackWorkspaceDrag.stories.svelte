<script module lang="ts">
  import { defineMeta } from "@storybook/addon-svelte-csf";
  import {
    expect,
    fireEvent,
    userEvent,
    waitFor,
    within,
  } from "storybook/test";
  import StackWorkspaceFixture from "../../fixtures/StackWorkspaceFixture.svelte";
  import { WORKSPACE_DRAG_TYPE } from "../../../web/src/lib/controller/stack-workspace/drag.js";
  import {
    applyPreview,
    dropWorkspaceChange,
    workspaceCanvas,
    workspacePortal,
  } from "../../fixtures/stack-workspace/story-actions.js";
  const { Story } = defineMeta({
    title: "JJ Store/Stacks/Drag and drop",
    component: StackWorkspaceFixture,
    tags: ["visual-pending"],
    parameters: { layout: "fullscreen" },
  });
  async function start(source: HTMLElement, target: HTMLElement) {
    const dataTransfer = new DataTransfer();
    await fireEvent(
      source,
      new DragEvent("dragstart", { bubbles: true, dataTransfer }),
    );
    const over = new DragEvent("dragover", {
      bubbles: true,
      cancelable: true,
      dataTransfer,
    });
    await fireEvent(target, over);
    expect(over.defaultPrevented).toBe(true);
    return dataTransfer;
  }
  async function drop(target: HTMLElement, dataTransfer: DataTransfer) {
    await fireEvent(
      target,
      new DragEvent("drop", { bubbles: true, cancelable: true, dataTransfer }),
    );
  }
</script>

<Story
  name="Unstaged files stage in any stack with guides"
  play={async ({ canvasElement }) => {
    const canvas = workspaceCanvas(canvasElement);
    const unstaged = await canvas.findByRole("region", {
      name: "Unstaged files",
    });
    expect(
      parseFloat(
        getComputedStyle(
          unstaged.querySelector('[data-ui-part="column-body"]')!,
        ).paddingLeft,
      ),
    ).toBe(8);
    const file = within(unstaged).getByRole("button", {
      name: "src/settings.ts",
    });
    expect(file).toHaveAttribute("draggable", "true");
    const stage = canvas.getByRole("group", {
      name: "Commit composer for feature/workspace",
    });
    const transfer = await start(file, stage);
    expect(
      JSON.parse(transfer.getData(WORKSPACE_DRAG_TYPE)).selection.hunks,
    ).toHaveLength(2);
    expect(stage).toHaveAttribute("data-drop-target-state", "valid");
    expect(getComputedStyle(stage, "::after").borderTopStyle).toBe("dashed");
    expect(
      within(stage).getByRole("status", { name: "Drop guide" }),
    ).toHaveTextContent("Stage changes for a new commit");
    const child = within(stage).getByRole("button", {
      name: "Assign to feature/workspace",
    });
    await fireEvent(
      stage,
      new DragEvent("dragleave", {
        bubbles: true,
        relatedTarget: child,
        dataTransfer: transfer,
      }),
    );
    expect(stage).toHaveAttribute("data-drop-target-state", "valid");
    await drop(stage, transfer);
    expect(stage).toHaveAttribute("data-drop-target-state", "idle");
    const form = within(stage).getByRole("form");
    expect(
      within(form).getByRole("button", { name: "src/settings.ts" }),
    ).toBeVisible();
    expect(
      within(unstaged).getByRole("button", { name: "src/settings.ts" }),
    ).toBeVisible();
    await userEvent.click(within(form).getByRole("button", { name: "Cancel" }));
    const api = canvas.getByRole("region", { name: "Stack feature/api" });
    const body = api.querySelector<HTMLElement>(
      '[data-ui-part="column-body"]',
    )!;
    await dropWorkspaceChange(
      within(unstaged).getByRole("button", { name: "README.md" }),
      body,
    );
    expect(
      within(api).getByRole("form", { name: "Create commit in feature/api" }),
    ).toBeVisible();
    expect(
      within(api).getByRole("button", { name: "README.md" }),
    ).toBeVisible();
  }}
/>

<Story
  name="Folder drops amend commits and files return to Unstaged"
  play={async ({ canvasElement }) => {
    const canvas = workspaceCanvas(canvasElement),
      portal = workspacePortal(canvasElement);
    const unstaged = await canvas.findByRole("region", {
      name: "Unstaged files",
    });
    const folder = within(unstaged).getByRole("button", {
      name: "src",
    });
    expect(folder).toHaveAttribute("draggable", "true");
    const row = canvas
      .getByRole("button", { name: "Add workspace toolbar" })
      .closest<HTMLElement>("[data-revision-id]")!;
    const transfer = await start(folder, row);
    expect(row).toHaveAttribute("data-drop-target-state", "valid");
    expect(
      within(row).getByRole("status", { name: "Drop guide" }),
    ).toHaveTextContent("Move changes into this commit");
    expect(JSON.parse(transfer.getData(WORKSPACE_DRAG_TYPE)).kind).toBe(
      "folder",
    );
    await drop(row, transfer);
    expect(
      portal.getByRole("list", { name: "Selected changes" }),
    ).toHaveTextContent("src/settings.ts");
    await userEvent.click(
      portal.getByRole("button", { name: "Preview command" }),
    );
    await applyPreview(canvasElement);
    expect(
      within(unstaged).queryByRole("button", { name: "src/settings.ts" }),
    ).toBeNull();
    await userEvent.click(
      canvas.getByRole("button", { name: "Add workspace toolbar" }),
    );
    const stack = canvas.getByRole("region", {
      name: "Stack feature/workspace",
    });
    const file = within(stack).getByRole("button", { name: "src/settings.ts" });
    const body = unstaged.querySelector<HTMLElement>(
      '[data-ui-part="column-body"]',
    )!;
    const back = await start(file, body);
    expect(body).toHaveAttribute("data-drop-target-state", "valid");
    expect(
      within(body).getByRole("status", { name: "Drop guide" }),
    ).toHaveTextContent("Move changes back to Unstaged");
    await drop(body, back);
    await userEvent.click(
      portal.getByRole("button", { name: "Preview command" }),
    );
    await applyPreview(canvasElement);
    expect(
      within(unstaged).getByRole("button", { name: "src/settings.ts" }),
    ).toBeVisible();
    expect(
      within(stack).queryByRole("button", { name: "src/settings.ts" }),
    ).toBeNull();
  }}
/>

<Story
  name="Rejected and cancelled drops clear their guides"
  args={{ immutable: true }}
  play={async ({ canvasElement }) => {
    const canvas = workspaceCanvas(canvasElement);
    const unstaged = await canvas.findByRole("region", {
      name: "Unstaged files",
    });
    const file = within(unstaged).getByRole("button", { name: "README.md" });
    const row = canvas
      .getByRole("button", { name: "Prepare add repository model" })
      .closest<HTMLElement>("[data-revision-id]")!;
    const transfer = await start(file, row);
    expect(row).toHaveAttribute("data-drop-target-state", "invalid");
    expect(
      within(row).getByRole("status", { name: "Drop guide" }),
    ).toHaveTextContent("immutable");
    expect(transfer.dropEffect).toBe("none");
    await drop(row, transfer);
    expect(row).toHaveAttribute("data-drop-target-state", "idle");
    expect(workspacePortal(canvasElement).queryByRole("dialog")).toBeNull();
    expect(
      within(unstaged).getByRole("button", { name: "README.md" }),
    ).toBeVisible();
    const stage = canvas.getByRole("group", {
      name: "Commit composer for feature/workspace",
    });
    const next = await start(file, stage);
    await fireEvent(
      file,
      new DragEvent("dragend", { bubbles: true, dataTransfer: next }),
    );
    await waitFor(() =>
      expect(stage).toHaveAttribute("data-drop-target-state", "idle"),
    );
    const foreign = new DataTransfer();
    foreign.setData("text/plain", "foreign");
    await fireEvent(
      stage,
      new DragEvent("dragover", {
        bubbles: true,
        cancelable: true,
        dataTransfer: foreign,
      }),
    );
    expect(stage).toHaveAttribute("data-drop-target-state", "idle");
    expect(within(stage).queryByRole("form")).toBeNull();
  }}
/>

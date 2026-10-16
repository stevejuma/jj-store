<script module lang="ts">
  import { verifyDraggableHunks } from "../../fixtures/stack-workspace/draggable-hunks-play.js";
  import { defineMeta } from "@storybook/addon-svelte-csf";
  import {
    expect,
    fireEvent,
    userEvent,
    within,
    waitFor,
  } from "storybook/test";
  import StackWorkspaceFixture from "../../fixtures/StackWorkspaceFixture.svelte";
  import { WORKSPACE_DRAG_TYPE } from "../../../web/src/lib/controller/stack-workspace/drag.js";
  import {
    applyPreview,
    dropWorkspaceChange,
    commandMenu,
    messagePreview,
    unassignedFile,
    workspaceCanvas,
    workspacePortal,
  } from "../../fixtures/stack-workspace/story-actions.js";
  const { Story } = defineMeta({
    title: "JJ Store/Stack Workspace/Commands",
    component: StackWorkspaceFixture,
    tags: ["visual-pending"],
    parameters: { layout: "fullscreen" },
  });
</script>

<Story
  name="Inline commit with staged files and hunks"
  play={async ({ canvasElement }) => {
    const canvas = workspaceCanvas(canvasElement),
      portal = workspacePortal(canvasElement);
    const stack = within(
      await canvas.findByRole("region", { name: "Stack feature/workspace" }),
    );
    await userEvent.click(
      stack.getByRole("button", { name: "Create commit in feature/workspace" }),
    );
    let form = stack.getByRole("form", {
      name: "Create commit in feature/workspace",
    });
    expect(portal.queryByRole("dialog")).toBeNull();
    expect(
      within(form).getByRole("button", { name: "Create commit" }),
    ).toBeDisabled();
    expect(
      within(form).getByLabelText("Commit title").getBoundingClientRect()
        .height,
    ).toBeGreaterThanOrEqual(36);
    expect(
      getComputedStyle(
        within(form).getByLabelText("Commit title").parentElement!,
      ).borderBottomWidth,
    ).toBe("1px");
    const titleRow = within(form).getByLabelText("Commit title").parentElement!;
    const card = form.closest<HTMLElement>('[data-ui-part="card"]')!;
    const titleBounds = titleRow.getBoundingClientRect();
    const cardBounds = card.getBoundingClientRect();
    expect(
      Math.abs(titleBounds.left - (cardBounds.left + 1)),
    ).toBeLessThanOrEqual(1);
    expect(
      Math.abs(titleBounds.right - (cardBounds.right - 1)),
    ).toBeLessThanOrEqual(1);
    expect(form.querySelector(".community-chat-composer")).not.toBeNull();
    await userEvent.type(
      within(form).getByLabelText("Commit title"),
      "Discarded draft",
    );
    await userEvent.click(within(form).getByRole("button", { name: "Cancel" }));
    expect(stack.queryByRole("form")).toBeNull();
    await unassignedFile(canvasElement, "src/settings.ts");
    const transfer = new DataTransfer();
    await fireEvent(
      canvas.getByRole("group", {
        name: "Draggable hunk add keyboard shortcut",
      }),
      new DragEvent("dragstart", { bubbles: true, dataTransfer: transfer }),
    );
    await fireEvent(
      stack.getByRole("button", { name: "Assign to feature/workspace" }),
      new DragEvent("drop", {
        bubbles: true,
        cancelable: true,
        dataTransfer: transfer,
      }),
    );
    form = stack.getByRole("form", {
      name: "Create commit in feature/workspace",
    });
    const staged = within(form).getByRole("group", {
      name: "Staged files for feature/workspace",
    });
    expect(
      within(staged).getByRole("button", { name: "src/settings.ts" }),
    ).toBeVisible();
    expect(within(form).getByLabelText("Commit title")).toHaveValue("");
    // A second drop retains both exact staged selections.
    await unassignedFile(canvasElement, "README.md");
    await dropWorkspaceChange(
      canvas.getByRole("group", { name: "Draggable hunk document shortcut" }),
      staged,
    );
    expect(
      within(staged).getByRole("button", { name: "README.md" }),
    ).toBeVisible();
    await userEvent.type(
      within(form).getByLabelText("Commit title"),
      "Shortcut and documentation",
    );
    await userEvent.type(
      within(
        within(form).getByRole("group", { name: "Commit description" }),
      ).getByRole("textbox"),
      "Keep the preference hunk unstaged.",
    );
    await userEvent.click(
      within(form).getByRole("button", { name: "Create commit" }),
    );
    await waitFor(async () => {
      await expect(
        await portal.findByText(
          /using only the staged changes in src\/settings.ts, README.md/,
        ),
      ).toBeVisible();
    });
    await userEvent.click(
      portal.getByRole("button", { name: "Cancel preview" }),
    );
    expect(within(form).getByLabelText("Commit title")).toHaveValue(
      "Shortcut and documentation",
    );
    await userEvent.click(
      within(form).getByRole("button", { name: "Create commit" }),
    );
    await applyPreview(canvasElement);
    expect(stack.queryByRole("form")).toBeNull();
    await userEvent.click(
      stack.getByRole("button", {
        name: "Shortcut and documentation Keep the preference hunk unstaged.",
      }),
    );
    expect(stack.getByRole("button", { name: "README.md" })).toBeVisible();
    const unstaged = within(
      canvas.getByRole("region", { name: "Unstaged files" }),
    );
    expect(unstaged.queryByRole("button", { name: "README.md" })).toBeNull();
    await userEvent.click(
      unstaged.getByRole("button", { name: "src/settings.ts" }),
    );
    expect(
      canvas.queryByRole("checkbox", { name: "add keyboard shortcut" }),
    ).toBeNull();
    expect(
      canvas.getByRole("checkbox", { name: "remember preference" }),
    ).toBeVisible();
  }}
/>

<Story
  name="Drop changes onto existing commit"
  play={async ({ canvasElement }) => {
    const canvas = workspaceCanvas(canvasElement),
      portal = workspacePortal(canvasElement);
    await unassignedFile(canvasElement, "README.md");
    await dropWorkspaceChange(
      canvas.getByRole("group", { name: "Draggable hunk document shortcut" }),
      canvas.getByRole("button", { name: "Prepare add workspace toolbar" }),
    );
    expect(
      portal.getByRole("combobox", { name: "Destination" }),
    ).toHaveTextContent("Prepare add workspace toolbar");
    await userEvent.click(
      portal.getByRole("button", { name: "Preview command" }),
    );
    await applyPreview(canvasElement);
    await expect(
      canvas.queryByRole("button", { name: "README.md" }),
    ).not.toBeInTheDocument();
    await userEvent.click(
      canvas.getByRole("button", {
        name: "Prepare add workspace toolbar",
      }),
    );
    await expect(
      within(
        canvas.getByRole("region", { name: "Stack feature/workspace" }),
      ).getByRole("button", { name: "README.md" }),
    ).toBeVisible();
  }}
/>

<Story
  name="Split reorder squash and move"
  play={async ({ canvasElement }) => {
    const canvas = workspaceCanvas(canvasElement),
      portal = workspacePortal(canvasElement);
    await userEvent.click(
      await canvas.findByRole("button", {
        name: "Expose repository API",
      }),
    );
    await userEvent.click(canvas.getByRole("button", { name: "src/api.ts" }));
    await userEvent.click(
      canvas.getByRole("checkbox", { name: "query endpoint" }),
    );
    await userEvent.click(
      canvas.getByRole("button", {
        name: "Split selected changes",
      }),
    );
    await messagePreview(canvasElement, "Mutation endpoint");
    await applyPreview(canvasElement);
    await commandMenu(
      canvasElement,
      "Actions for Mutation endpoint",
      "Move down",
    );
    await applyPreview(canvasElement);
    await expect(
      canvas.getByRole("button", { name: "Mutation endpoint" }),
    ).toBeVisible();
    await commandMenu(
      canvasElement,
      "Actions for Expose repository API",
      "Squash into previous",
    );
    await applyPreview(canvasElement);
    await expect(
      canvas.queryByRole("button", {
        name: "Expose repository API",
      }),
    ).not.toBeInTheDocument();
    await commandMenu(
      canvasElement,
      "Actions for Mutation endpoint",
      "Move to stack",
    );
    await userEvent.click(
      await portal.findByRole("combobox", { name: "Destination" }),
    );
    await userEvent.click(
      await portal.findByRole("option", {
        name: /feature\/workspace/,
      }),
    );
    await userEvent.click(
      portal.getByRole("button", { name: "Preview command" }),
    );
    await applyPreview(canvasElement);
    await expect(
      within(
        canvas.getByRole("region", { name: "Stack feature/workspace" }),
      ).getByRole("button", {
        name: "Mutation endpoint Expose repository API",
      }),
    ).toBeVisible();
  }}
/>

<Story
  name="Drag a hunk to a commit"
  play={async ({ canvasElement }) => {
    const canvas = workspaceCanvas(canvasElement),
      portal = workspacePortal(canvasElement);
    await unassignedFile(canvasElement, "src/settings.ts");
    const dataTransfer = new DataTransfer();
    await fireEvent(
      canvas.getByRole("group", {
        name: "Draggable hunk add keyboard shortcut",
      }),
      new DragEvent("dragstart", { bubbles: true, dataTransfer }),
    );
    await expect(dataTransfer.getData(WORKSPACE_DRAG_TYPE)).toContain(
      "add keyboard shortcut",
    );
    await fireEvent(
      canvas.getByRole("button", {
        name: "Add workspace toolbar",
      }),
      new DragEvent("drop", { bubbles: true, cancelable: true, dataTransfer }),
    );
    await expect(
      await portal.findByRole("list", { name: "Selected changes" }),
    ).toHaveTextContent("src/settings.ts · 1 hunk(s)");
    await userEvent.click(
      portal.getByRole("button", { name: "Preview command" }),
    );
    await applyPreview(canvasElement);
    await unassignedFile(canvasElement, "src/settings.ts");
    await expect(
      canvas.queryByRole("checkbox", { name: "add keyboard shortcut" }),
    ).not.toBeInTheDocument();
    await expect(
      canvas.getByRole("checkbox", { name: "remember preference" }),
    ).toBeVisible();
  }}
/>

<Story
  name="Rename rebase uncommit and delete stack"
  play={async ({ canvasElement }) => {
    const canvas = workspaceCanvas(canvasElement),
      portal = workspacePortal(canvasElement);
    await commandMenu(canvasElement, "Actions for feature/api", "Rename stack");
    const name = await portal.findByLabelText("Name");
    await userEvent.clear(name);
    await userEvent.type(name, "feature/local-api");
    await userEvent.click(
      portal.getByRole("button", { name: "Preview command" }),
    );
    await applyPreview(canvasElement);
    await commandMenu(
      canvasElement,
      "Actions for feature/local-api",
      "Rebase onto base",
    );
    await applyPreview(canvasElement);
    await commandMenu(
      canvasElement,
      "Actions for Expose repository API",
      "Uncommit",
    );
    await applyPreview(canvasElement);
    await expect(
      canvas.getByRole("button", { name: "src/api.ts" }),
    ).toBeVisible();
    await commandMenu(
      canvasElement,
      "Actions for feature/local-api",
      "Delete stack",
    );
    await applyPreview(canvasElement);
    await expect(
      canvas.queryByRole("region", { name: "Stack feature/local-api" }),
    ).not.toBeInTheDocument();
    await expect(
      canvas.getByRole("button", { name: "src/api.ts" }),
    ).toBeVisible();
  }}
/>

<Story name="Workspace draggable diff hunks" play={verifyDraggableHunks} />

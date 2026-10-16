<script module lang="ts">
  import { defineMeta } from "@storybook/addon-svelte-csf";
  import {
    expect,
    fireEvent,
    userEvent,
    within,
    waitFor,
  } from "storybook/test";
  import StackWorkspaceProductionFixture from "../../fixtures/StackWorkspaceProductionFixture.svelte";
  import { WORKSPACE_DRAG_TYPE } from "../../../web/src/lib/controller/stack-workspace/drag.js";
  const { Story } = defineMeta({
    title: "JJ Store/Stacks/Native drag and drop",
    component: StackWorkspaceProductionFixture,
    tags: ["visual-pending"],
    parameters: { layout: "fullscreen", jjStoreShell: true },
    args: { editing: true },
  });
  async function drag(source: HTMLElement, target: HTMLElement) {
    const dataTransfer = new DataTransfer();
    await fireEvent(
      source,
      new DragEvent("dragstart", { bubbles: true, dataTransfer }),
    );
    await fireEvent(
      target,
      new DragEvent("dragover", {
        bubbles: true,
        cancelable: true,
        dataTransfer,
      }),
    );
    expect(target).toHaveAttribute("data-drop-target-state", "valid");
    await fireEvent(
      target,
      new DragEvent("drop", { bubbles: true, cancelable: true, dataTransfer }),
    );
    return JSON.parse(dataTransfer.getData(WORKSPACE_DRAG_TYPE)).selection;
  }
</script>

<Story
  name="Native branch files stage and hunks preview exact transfers"
  play={async ({ canvasElement }) => {
    const canvas = within(canvasElement),
      portal = within(document.body);
    const stack = await canvas.findByRole("region", { name: "Stack topic" });
    await within(stack).findByRole("button", { name: "Topic ancestor" });
    await userEvent.click(
      within(stack).getByRole("button", { name: "Show all changes in topic" }),
    );
    const files = await within(stack).findByRole("region", {
      name: "Changed files in branch topic",
    });
    const file = await within(files).findByRole("button", {
      name: "src/topic.ts",
    });
    expect(file).toHaveAttribute("draggable", "true");
    const composer = canvas.getByRole("group", {
      name: "Commit composer for topic",
    });
    expect(await drag(file, composer)).toEqual({
      source: "3".repeat(40),
      path: "src/topic.ts",
    });
    const form = within(composer).getByRole("form");
    expect(
      within(form).getByRole("button", { name: "src/topic.ts" }),
    ).toBeVisible();
    await userEvent.click(within(form).getByRole("button", { name: "Cancel" }));
    await userEvent.click(file);
    const hunk = await canvas.findByRole("group", {
      name: "Draggable hunk @@ -1 +1 @@",
    });
    expect(hunk).toHaveAttribute("draggable", "true");
    const row = within(stack)
      .getByRole("button", { name: "Topic tip" })
      .closest<HTMLElement>("[data-revision-id]")!;
    expect(await drag(hunk, row)).toEqual({
      source: "3".repeat(40),
      path: "src/topic.ts",
      hunks: ["native:" + "a".repeat(64)],
    });
    await userEvent.click(
      portal.getByRole("button", { name: "Preview command" }),
    );
    await waitFor(() =>
      expect(
        portal.getByRole("button", { name: "Apply command" }),
      ).toBeVisible(),
    );
    expect(
      portal.getByText("Move src/topic.ts from " + "3".repeat(40)),
    ).toBeVisible();
    await userEvent.click(
      portal.getByRole("button", { name: "Cancel preview" }),
    );
    const unstaged = canvas.getByRole("region", { name: "Unstaged files" });
    const working = unstaged.querySelector<HTMLElement>(
      '[data-ui-part="column-body"]',
    )!;
    expect(await drag(hunk, working)).toMatchObject({
      source: "3".repeat(40),
      hunks: ["native:" + "a".repeat(64)],
    });
    await userEvent.click(
      portal.getByRole("button", { name: "Preview command" }),
    );
    await waitFor(() =>
      expect(
        portal.getByRole("button", { name: "Apply command" }),
      ).toBeVisible(),
    );
    expect(
      within(files).getByRole("button", { name: "src/topic.ts" }),
    ).toBeVisible();
  }}
/>

<Story
  name="Ambiguous native branch comparisons remain read only"
  args={{ dragRead: "ambiguous" }}
  play={async ({ canvasElement }) => {
    const canvas = within(canvasElement);
    const stack = await canvas.findByRole("region", { name: "Stack topic" });
    await within(stack).findByRole("button", { name: "Topic ancestor" });
    await userEvent.click(
      within(stack).getByRole("button", { name: "Show all changes in topic" }),
    );
    const files = await within(stack).findByRole("region", {
      name: "Changed files in branch topic",
    });
    const file = await within(files).findByRole("button", {
      name: "src/topic.ts",
    });
    expect(file).toHaveAttribute("draggable", "false");
    await userEvent.click(file);
    const hunk = await canvas.findByRole("group", { name: /^Draggable hunk/ });
    expect(hunk).toHaveAttribute("draggable", "false");
    expect(
      canvas.getByText(/Select an individual commit to move/),
    ).toBeVisible();
  }}
/>

<Story
  name="Ownership read failures use application feedback"
  args={{ dragRead: "failure" }}
  play={async ({ canvasElement }) => {
    const canvas = within(canvasElement),
      portal = within(document.body);
    const stack = await canvas.findByRole("region", { name: "Stack topic" });
    await within(stack).findByRole("button", { name: "Topic ancestor" });
    await userEvent.click(
      within(stack).getByRole("button", { name: "Show all changes in topic" }),
    );
    await waitFor(() =>
      expect(
        portal
          .getAllByRole("alert")
          .some((alert) =>
            alert.textContent?.includes("Repository request failed"),
          ),
      ).toBe(true),
    );
    expect(
      within(stack).getByRole("button", { name: "Topic ancestor" }),
    ).toBeEnabled();
  }}
/>

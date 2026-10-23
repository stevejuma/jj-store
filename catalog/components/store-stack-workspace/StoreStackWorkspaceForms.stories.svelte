<script module lang="ts">
  import { workspaceFileName } from "../../fixtures/stack-workspace/story-actions.js";
  import { defineMeta } from "@storybook/addon-svelte-csf";
  import { expect, userEvent, within, waitFor } from "storybook/test";
  import StackWorkspaceFixture from "../../fixtures/StackWorkspaceFixture.svelte";
  import {
    applyPreview,
    unassignedFile,
    workspaceCanvas,
    workspacePortal,
  } from "../../fixtures/stack-workspace/story-actions.js";
  const { Story } = defineMeta({
    title: "JJ Store/Stack Workspace/Forms",
    component: StackWorkspaceFixture,
    tags: ["visual-pending", "application"],
    parameters: { jjStoreStory: { kind: "application" }, layout: "fullscreen" },
  });
  async function settingsRows(dialog: HTMLElement, count: number) {
    await waitFor(() => {
      const group = dialog.querySelector<HTMLElement>(
        '.community-settings__option-group[data-ui-part="field-group"]',
      )!;
      const rows = group.querySelectorAll<HTMLElement>(
        ':scope > [data-ui-part="field"]',
      );
      expect(rows).toHaveLength(count);
      expect(getComputedStyle(group).borderTopWidth).toBe("1px");
      let previous: DOMRect | undefined;
      for (const row of rows) {
        expect(row).toHaveClass("community-settings__option-row");
        const style = getComputedStyle(row);
        expect(parseFloat(style.paddingLeft)).toBeGreaterThanOrEqual(16);
        expect(parseFloat(style.paddingTop)).toBeGreaterThanOrEqual(12);
        const rect = row.getBoundingClientRect();
        expect(rect.height).toBeGreaterThanOrEqual(68);
        if (previous) {
          expect(Math.abs(rect.top - previous.bottom)).toBeLessThan(1);
          expect(style.borderTopWidth).toBe("1px");
        }
        previous = rect;
      }
    });
  }
</script>

<Story
  name="Assign selected changes with settings rows and branch command picker"
  play={async ({ canvasElement }) => {
    const canvas = workspaceCanvas(canvasElement),
      portal = workspacePortal(canvasElement);
    await unassignedFile(canvasElement, "README.md");
    await userEvent.click(
      canvas.getByRole("button", { name: "Assign hunk document shortcut" }),
    );
    const dialog = portal.getByRole("dialog", {
      name: "Assign selected changes",
    });
    await userEvent.click(
      within(dialog).getByRole("combobox", { name: "Destination" }),
    );
    await settingsRows(dialog, 2);
    expect(
      within(dialog).getByRole("list", { name: "Selected changes" }),
    ).toHaveTextContent("README.md · 1 hunk(s)");
    await userEvent.type(
      await portal.findByLabelText("Search destinations"),
      "model",
    );
    expect(portal.getAllByRole("option")).toHaveLength(1);
    expect(
      portal.getByRole("option", { name: /feature\/model/ }),
    ).toHaveTextContent("Add repository model");
    await userEvent.keyboard("{ArrowDown}{Enter}");
    expect(
      within(dialog).getByRole("combobox", { name: "Destination" }),
    ).toHaveTextContent("feature/model");
    await userEvent.click(
      within(dialog).getByRole("button", { name: "Preview command" }),
    );
    await waitFor(async () => {
      expect(
        await portal.findByText(/move to commit “Add repository model”/),
      ).toBeVisible();
    });
    await applyPreview(canvasElement);
    await userEvent.click(
      canvas.getByRole("button", { name: "Add repository model" }),
    );
    expect(
      within(
        canvas.getByRole("region", { name: "Stack feature/api" }),
      ).getByRole("button", { name: workspaceFileName("README.md") }),
    ).toBeVisible();
    expect(
      within(
        canvas.getByRole("region", { name: "Unstaged files" }),
      ).queryByRole("button", { name: workspaceFileName("README.md") }),
    ).toBeNull();
  }}
/>

<Story
  name="Create independent stack"
  play={async ({ canvasElement }) => {
    const canvas = workspaceCanvas(canvasElement),
      portal = workspacePortal(canvasElement);
    await userEvent.click(
      await canvas.findByRole("button", { name: "Create stack" }),
    );
    expect(portal.getByRole("radio", { name: "Independent" })).toBeChecked();
    await settingsRows(portal.getByRole("dialog", { name: "Create stack" }), 2);
    expect(portal.queryByRole("combobox", { name: "Add to stack" })).toBeNull();
    await userEvent.type(portal.getByLabelText("Name"), "feature/independent");
    await userEvent.click(
      portal.getByRole("button", { name: "Preview command" }),
    );
    await waitFor(async () => {
      expect(await portal.findByText(/at “main” and apply/)).toBeVisible();
    });
    await applyPreview(canvasElement);
    expect(
      canvas.getByRole("region", { name: "Stack feature/independent" }),
    ).toBeVisible();
  }}
/>

<Story
  name="Create dependent stack with command picker"
  play={async ({ canvasElement }) => {
    const canvas = workspaceCanvas(canvasElement),
      portal = workspacePortal(canvasElement);
    await userEvent.click(
      await canvas.findByRole("button", { name: "Create stack" }),
    );
    await userEvent.type(portal.getByLabelText("Name"), "feature/dependent");
    await userEvent.click(portal.getByRole("radio", { name: "Dependent" }));
    await settingsRows(portal.getByRole("dialog", { name: "Create stack" }), 3);
    await userEvent.click(
      portal.getByRole("button", { name: "Preview command" }),
    );
    expect(portal.getByRole("alert")).toHaveTextContent(
      "Choose an existing stack",
    );
    await userEvent.click(
      portal.getByRole("combobox", { name: "Add to stack" }),
    );
    await userEvent.type(portal.getByLabelText("Search stacks"), "api");
    expect(portal.getAllByRole("option")).toHaveLength(1);
    await userEvent.click(portal.getByRole("option", { name: /feature\/api/ }));
    await userEvent.click(
      portal.getByRole("button", { name: "Preview command" }),
    );
    await waitFor(async () => {
      expect(
        await portal.findByText(/as a dependent branch above “feature\/api”/),
      ).toBeVisible();
    });
    await applyPreview(canvasElement);
    const stack = within(
      canvas.getByRole("region", { name: "Stack feature/dependent" }),
    );
    expect(
      stack.getByRole("button", {
        name: "Show all changes in feature/dependent",
      }),
    ).toBeVisible();
    expect(
      stack.getByRole("button", { name: "Show all changes in feature/api" }),
    ).toBeVisible();
    expect(
      stack.getByRole("button", { name: "Show all changes in feature/model" }),
    ).toBeVisible();
    expect(
      canvas.queryByRole("region", { name: "Stack feature/api" }),
    ).toBeNull();
  }}
/>

<script module lang="ts">
  import { defineMeta } from "@storybook/addon-svelte-csf";
  import { expect, userEvent, within, waitFor } from "storybook/test";
  import StackWorkspaceSetupApplicationFixture from "../../fixtures/StackWorkspaceSetupApplicationFixture.svelte";
  const { Story } = defineMeta({
    title: "JJ Store/Stacks/Setup",
    component: StackWorkspaceSetupApplicationFixture,
    tags: ["visual-pending"],
    parameters: { layout: "fullscreen", jjStoreShell: true },
  });
  async function choose(canvasElement: HTMLElement) {
    const canvas = within(canvasElement);
    await userEvent.click(
      await canvas.findByRole("combobox", { name: "Workspace target branch" }),
    );
    await userEvent.click(
      await within(canvasElement.ownerDocument.body).findByRole("option", {
        name: /^main/,
      }),
    );
    await waitFor(() =>
      expect(
        canvas.queryByRole("combobox", { name: "Workspace target branch" }),
      ).toBeNull(),
    );
    expect(canvas.queryByRole("dialog")).toBeNull();
    return canvas;
  }
</script>

<Story
  name="Unconfigured workspace"
  play={async ({ canvasElement }) => {
    const canvas = within(canvasElement);
    await canvas.findByRole("combobox", { name: "Workspace target branch" });
    expect(
      canvas.getByText("No stack workspace", {
        selector: '[data-ui-part="empty-title"]',
      }),
    ).toBeVisible();
    expect(
      canvas.getByRole("button", { name: "Create workspace" }),
    ).toBeDisabled();
    expect(canvas.queryByRole("region", { name: "Unstaged files" })).toBeNull();
    expect(canvas.getByRole("tab", { name: "Branches" })).toBeEnabled();
    await choose(canvasElement);
    expect(
      canvas.getByText("No stack workspace", {
        selector: '[data-ui-part="empty-title"]',
      }),
    ).toBeVisible();
    expect(
      canvas.getByRole("button", { name: "Create workspace" }),
    ).toBeDisabled();
  }}
/>
<Story
  name="Explicit target enables browsing without setup"
  args={{ page: "branches" }}
  play={async ({ canvasElement }) => {
    const canvas = await choose(canvasElement);
    await userEvent.click(
      await canvas.findByRole("option", { name: "feature/api" }),
    );
    await canvas.findByRole("region", { name: "Stack feature/api" });
    expect(
      canvas.getByRole("button", { name: "Apply feature/api" }),
    ).toBeDisabled();
    await userEvent.click(
      within(
        canvas.getByRole("navigation", { name: "Stack workspace navigation" }),
      ).getByRole("tab", { name: "Workspace" }),
    );
    expect(canvas.queryByRole("region", { name: "Unstaged files" })).toBeNull();
    expect(
      canvas.getByText("No stack workspace", {
        selector: '[data-ui-part="empty-title"]',
      }),
    ).toBeVisible();
  }}
/>

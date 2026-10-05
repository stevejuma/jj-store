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
  function assertSetupMargins(canvasElement: HTMLElement) {
    const picker = within(canvasElement).getByRole("combobox", {
      name: "Workspace target branch",
    });
    const form = picker.closest<HTMLElement>(
      ".community-settings__option-group",
    )!;
    const style = getComputedStyle(form);
    for (const margin of [
      style.marginTop,
      style.marginRight,
      style.marginBottom,
      style.marginLeft,
    ])
      expect(Number.parseFloat(margin)).toBe(12);
    const parent = form.parentElement!.getBoundingClientRect();
    const bounds = form.getBoundingClientRect();
    expect(Math.abs(bounds.left - parent.left - 12)).toBeLessThanOrEqual(1);
    expect(Math.abs(parent.right - bounds.right - 12)).toBeLessThanOrEqual(1);
    expect(Math.abs(bounds.top - parent.top - 12)).toBeLessThanOrEqual(1);
  }
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
    assertSetupMargins(canvasElement);
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
    await within(canvasElement).findByRole("combobox", {
      name: "Workspace target branch",
    });
    assertSetupMargins(canvasElement);
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

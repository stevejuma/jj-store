<script module lang="ts">
  import { defineMeta } from "@storybook/addon-svelte-csf";
  import { expect, userEvent, within, waitFor } from "storybook/test";
  import Fixture from "../../fixtures/TerminalFixture.svelte";
  const { Story } = defineMeta({
    title: "JJ Store/Desktop terminal header",
    component: Fixture,
    tags: ["visual-pending"],
    parameters: { layout: "fullscreen" },
  });
  async function open(canvasElement: HTMLElement) {
    const ui = within(canvasElement);
    await userEvent.click(ui.getByRole("button", { name: "Show terminal" }));
    await waitFor(() => expect(ui.getAllByRole("tab")).toHaveLength(1));
    return ui;
  }
  async function hoverControls(canvasElement: HTMLElement) {
    const ui = await open(canvasElement);
    if (import.meta.env.MODE !== "test") return;
    const { userEvent: pointer } = await import("vitest/browser");
    const controls = [
      ui.getByRole("tab"),
      ...within(
        ui.getByRole("region", { name: "Repository terminal" }),
      ).getAllByRole("button"),
    ];
    for (const control of controls) {
      await pointer.hover(ui.getByRole("button", { name: "Repository A" }));
      const rest = getComputedStyle(control).backgroundColor;
      await pointer.hover(control);
      await waitFor(() =>
        expect(getComputedStyle(control).backgroundColor).not.toBe(rest),
      );
    }
    await pointer.hover(ui.getByRole("button", { name: "Repository A" }));
  }
</script>

<Story
  name="Inline close and retained tab navigation"
  play={async ({ canvasElement }) => {
    const ui = await open(canvasElement);
    const tab = ui.getByRole("tab");
    const close = ui.getByRole("button", { name: "Close terminal 1" });
    const tabRect = tab.getBoundingClientRect();
    const closeRect = close.getBoundingClientRect();
    await expect(closeRect.left).toBeGreaterThanOrEqual(tabRect.left);
    await expect(closeRect.right).toBeLessThanOrEqual(tabRect.right);
    await expect(closeRect.top).toBeGreaterThanOrEqual(tabRect.top);
    await expect(closeRect.bottom).toBeLessThanOrEqual(tabRect.bottom);
    await expect(
      ui.queryByText("/workspace/repo-a/1", { exact: true }),
    ).not.toBeInTheDocument();
    await expect(tab).toHaveAttribute("title", "/workspace/repo-a/1");
    await userEvent.click(ui.getByRole("button", { name: "New terminal" }));
    await waitFor(() => expect(ui.getAllByRole("tab")).toHaveLength(2));
    const tabs = ui.getAllByRole("tab");
    tabs[1].focus();
    await userEvent.keyboard("{ArrowLeft}");
    await expect(tabs[0]).toHaveFocus();
    await expect(tabs[0]).toHaveAttribute("aria-selected", "true");
    await userEvent.keyboard("{End}");
    await expect(tabs[1]).toHaveFocus();
    await expect(tabs[1]).toHaveAttribute("aria-selected", "true");
    await userEvent.click(ui.getByRole("button", { name: "Close terminal 1" }));
    await waitFor(() => expect(ui.getAllByRole("tab")).toHaveLength(1));
    await expect(tabs[1]).toHaveAttribute("aria-selected", "true");
  }}
/>
<Story
  name="Thin resize separator"
  play={async ({ canvasElement }) => {
    const ui = await open(canvasElement);
    const handle = ui.getByRole("separator", { name: "Resize terminal" });
    await expect(handle.getBoundingClientRect().height).toBe(1);
    const before = handle.getAttribute("aria-valuenow");
    handle.focus();
    await userEvent.keyboard("{ArrowUp}");
    await expect(handle).toHaveFocus();
    await expect(handle.getAttribute("aria-valuenow")).not.toBe(before);
  }}
/>
<Story
  name="Maximize closes details"
  play={async ({ canvasElement }) => {
    const ui = await open(canvasElement);
    await expect(
      ui.getByRole("complementary", { name: "Details sidebar" }),
    ).toBeVisible();
    await userEvent.click(
      ui.getByRole("button", { name: "Maximize terminal" }),
    );
    await expect(
      ui.queryByRole("complementary", { name: "Details sidebar" }),
    ).not.toBeInTheDocument();
    await userEvent.click(ui.getByRole("button", { name: "Restore terminal" }));
    await expect(
      ui.queryByRole("complementary", { name: "Details sidebar" }),
    ).not.toBeInTheDocument();
  }}
/>
<Story
  name="Visible header hover states"
  play={async ({ canvasElement }) => hoverControls(canvasElement)}
/>
<Story
  name="Visible header hover states in dark mode"
  globals={{ colorMode: "dark" }}
  play={async ({ canvasElement }) => hoverControls(canvasElement)}
/>

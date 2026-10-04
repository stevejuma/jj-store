<script module lang="ts">
  import { defineMeta } from "@storybook/addon-svelte-csf";
  import { expect, userEvent, waitFor, within } from "storybook/test";
  import StoreApplication from "../../fixtures/StoreApplicationFixture.svelte";
  async function aligned(header: HTMLElement, toggle: HTMLElement) {
    const center = () => {
      const box = toggle.getBoundingClientRect();
      return box.top + box.height / 2 - header.getBoundingClientRect().top;
    };
    await expect(header.getBoundingClientRect().height).toBe(40);
    await expect(center()).toBeCloseTo(23, 1);
    const root = header.ownerDocument.documentElement;
    const before = root.style.fontSize;
    try {
      root.style.fontSize = "20px";
      await expect(header.getBoundingClientRect().height).toBe(40);
      await expect(center()).toBeCloseTo(23, 1);
      await expect(toggle.getBoundingClientRect().height).toBe(28);
    } finally {
      root.style.fontSize = before;
    }
  }
  const { Story } = defineMeta({
    title: "JJ Store/Desktop chrome",
    component: StoreApplication,
    tags: ["visual-pending"],
    parameters: { layout: "fullscreen", jjStoreShell: true },
  });
</script>

<Story
  name="MacDesktopHeader"
  args={{ desktopChrome: true, displayMode: "desktop" }}
  play={async ({ canvasElement }) => {
    const canvas = within(canvasElement);
    const header = canvas.getByRole("navigation", {
      name: "Application navigation",
    });
    await expect(header).toHaveAttribute("data-tauri-drag-region", "deep");
    const sidebarToggle = canvas.getByRole("button", {
      name: "Expand left sidebar",
    });
    await expect(
      sidebarToggle.getBoundingClientRect().left -
        header.getBoundingClientRect().left,
    ).toBeGreaterThanOrEqual(80);
    await expect(header.getBoundingClientRect().height).toBeGreaterThanOrEqual(
      40,
    );
    await aligned(header, sidebarToggle);
    await userEvent.click(sidebarToggle);
    await expect(
      canvas.getByRole("button", { name: "Collapse left sidebar" }),
    ).toBeVisible();
    await userEvent.click(canvas.getByRole("tab", { name: "Commits" }));
    await userEvent.click(canvas.getByRole("button", { name: "Back" }));
    await expect(
      canvas.getByText("jj-store · Branches", { exact: true }),
    ).toBeVisible();
  }}
/>

<Story
  name="NarrowMacDesktopHeader"
  args={{ desktopChrome: true, displayMode: "auto" }}
  play={async ({ canvasElement }) => {
    const canvas = within(canvasElement);
    const shell = canvasElement.querySelector<HTMLElement>(
      '[data-ui-component="community-app-shell"]',
    )!;
    shell.style.width = "900px";
    await waitFor(async () => {
      await expect(shell).toHaveAttribute("data-display-mode", "mobile");
    });
    const header = canvas.getByRole("navigation", {
      name: "Application navigation",
    });
    await aligned(
      header,
      canvas.getByRole("button", { name: "Expand left sidebar" }),
    );
    await expect(header).toBeVisible();
    await expect(header).toHaveAttribute("data-tauri-drag-region", "deep");
    await expect(header.getBoundingClientRect().height).toBeGreaterThanOrEqual(
      40,
    );
    await expect(
      canvas
        .getByRole("button", { name: "Expand left sidebar" })
        .getBoundingClientRect().left - header.getBoundingClientRect().left,
    ).toBeGreaterThanOrEqual(80);
  }}
/>

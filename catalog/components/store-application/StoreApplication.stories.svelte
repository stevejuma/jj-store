<script module lang="ts">
  import { defineMeta } from "@storybook/addon-svelte-csf";
  import { expect, userEvent, waitFor, within } from "storybook/test";
  import StoreApplication from "../../fixtures/StoreApplicationFixture.svelte";
  const { Story } = defineMeta({
    title: "JJ Store/Shell",
    component: StoreApplication,
    tags: ["visual-pending"],
    parameters: {
      layout: "fullscreen",
      jjStoreShell: true,
      docs: {
        story: { inline: false, iframeHeight: 700 },
        source: {
          code: '<script>\n  import { StoreApplication } from "./catalog/index.js";\n<\/script>\n<StoreApplication />',
        },
      },
    },
  });
</script>

<Story
  name="Default"
  play={async ({ canvasElement }) => {
    const canvas = within(canvasElement);
    const header = canvas.getByRole("navigation", {
      name: "Application navigation",
    });
    await expect(header).not.toHaveAttribute("data-tauri-drag-region");
    await expect(
      canvas
        .getByRole("button", { name: "Expand left sidebar" })
        .getBoundingClientRect().left - header.getBoundingClientRect().left,
    ).toBeLessThan(16);
    const bodyTitle = canvasElement.querySelector<HTMLElement>(
      ".store-application__title",
    )!;
    await expect(bodyTitle).toBeVisible();
    await expect(
      canvas.getByRole("combobox", { name: "Repository workspace" }),
    ).toBeVisible();
    const mainBody = canvasElement.querySelector<HTMLElement>(
      '[data-ui-component="app-shell"][data-ui-part="body"]',
    )!;
    await expect(mainBody).toHaveAttribute("data-layout", "regions");
    await expect(getComputedStyle(mainBody).overflow).toBe("hidden");
    await expect(
      mainBody.querySelector(".ui-minimal-app-shell__body-scroll-area"),
    ).toBeNull();
    await expect(
      canvas.getByText("jj-store · Branches", { exact: true }),
    ).toBeVisible();
    await expect(
      canvas.getByRole("button", { name: "Expand left sidebar" }),
    ).toBeVisible();
    await userEvent.click(canvas.getByRole("tab", { name: "Commits" }));
    await expect(
      canvas.getByText("jj-store · Commits", { exact: true }),
    ).toBeVisible();
    await userEvent.click(canvas.getByRole("tab", { name: "Op Log" }));
    await expect(
      canvas.getByText("jj-store · Op Log", { exact: true }),
    ).toBeVisible();
    await userEvent.click(canvas.getByRole("button", { name: "Back" }));
    await expect(
      canvas.getByText("jj-store · Commits", { exact: true }),
    ).toBeVisible();
    await userEvent.click(canvas.getByRole("button", { name: "Forward" }));
    await expect(
      canvas.getByText("jj-store · Op Log", { exact: true }),
    ).toBeVisible();
  }}
/>
<Story
  name="Expanded"
  args={{ initiallyCollapsed: false }}
  play={async ({ canvasElement }) => {
    const canvas = within(canvasElement);
    await userEvent.click(canvas.getByRole("button", { name: "Repositories" }));
    await waitFor(() =>
      expect(
        canvas.queryByRole("button", { name: "native-demo" }),
      ).not.toBeInTheDocument(),
    );
    await userEvent.click(canvas.getByRole("button", { name: "Repositories" }));
    await userEvent.click(canvas.getByRole("button", { name: "native-demo" }));
    await expect(canvas.getByText("native-demo · Overview")).toBeVisible();
    await userEvent.click(
      canvas.getByRole("button", { name: "Toggle details" }),
    );
    await expect(
      canvas.getByRole("complementary", { name: "Repository details" }),
    ).toBeVisible();
    await expect(canvas.getByText("Files/Repository")).toBeVisible();
    await expect(
      canvas
        .getByRole("complementary", { name: "Repository details" })
        .querySelectorAll('[data-ui-part="sidebar-section"]').length,
    ).toBeGreaterThanOrEqual(2);
    await expect(
      canvas.queryByRole("button", { name: "Close details" }),
    ).not.toBeInTheDocument();
    await userEvent.click(
      canvas.getByRole("button", { name: "Toggle details" }),
    );
    await waitFor(() =>
      expect(
        canvas.queryByRole("complementary", { name: "Repository details" }),
      ).not.toBeInTheDocument(),
    );
    const resize = canvas.getByRole("slider", {
      name: "Resize repository navigation sidebar",
    });
    const before = Number(resize.getAttribute("aria-valuenow"));
    resize.focus();
    await userEvent.keyboard("{ArrowRight}");
    await waitFor(() =>
      expect(Number(resize.getAttribute("aria-valuenow"))).toBeGreaterThan(
        before,
      ),
    );
  }}
/>
<Story
  name="Compact repository picker"
  play={async ({ canvasElement }) => {
    const canvas = within(canvasElement);
    const trigger = canvas.getByRole("button", { name: "Repositories" });
    await userEvent.click(trigger);
    const body = within(canvasElement.ownerDocument.body);
    const search = await body.findByPlaceholderText("Search repositories");
    await userEvent.type(search, "native-demo");
    await expect(
      body.getByRole("option", { name: "native-demo" }),
    ).toBeVisible();
    await userEvent.keyboard("{Escape}");
    await waitFor(() => expect(trigger).toHaveFocus());
  }}
/>
<Story
  name="Mobile"
  args={{ displayMode: "mobile" }}
  play={async ({ canvasElement }) => {
    const canvas = within(canvasElement);
    await expect(
      canvas.getByText("jj-store · Branches", { exact: true }),
    ).toBeVisible();
    await userEvent.click(
      canvas.getByRole("button", { name: "Open details sidebar" }),
    );
    await expect(
      canvas.getByRole("navigation", { name: "Repository views" }),
    ).toBeVisible();
    await userEvent.click(canvas.getByRole("button", { name: "JJ Log" }));
    await expect(
      canvas.getByText("jj-store · Commits", { exact: true }),
    ).toBeVisible();
    await expect(
      canvas.queryByRole("navigation", { name: "Repository views" }),
    ).not.toBeInTheDocument();
  }}
/>

<Story
  name="Hover reveal"
  play={async ({ canvasElement }) => {
    const canvas = within(canvasElement);
    const edge = canvas.getByRole("button", {
      name: "Preview repository navigation sidebar",
    });
    await userEvent.hover(edge);
    await waitFor(() => expect(edge).toHaveAttribute("aria-expanded", "true"));
    await expect(
      canvas.getByRole("button", { name: "native-demo" }),
    ).toBeVisible();
    await userEvent.unhover(edge);
    await userEvent.hover(
      canvas.getByText("jj-store · Branches", { exact: true }),
    );
    await waitFor(() => expect(edge).toHaveAttribute("aria-expanded", "false"));
  }}
/>

<Story
  name="Mobile header controls stay at toolbar end"
  args={{ displayMode: "mobile", initialPage: "log" }}
  play={async ({ canvasElement }) => {
    const c = within(canvasElement);
    const toggle = c.getByRole("button", { name: "Open details sidebar" });
    const toolbar = toggle.closest('[data-ui-part="topbar"]')!;
    const bounds = toolbar.getBoundingClientRect(),
      button = toggle.getBoundingClientRect();
    const menu = c
      .getByRole("button", { name: "Application menu" })
      .getBoundingClientRect();
    await expect(bounds.right - menu.right).toBeLessThan(30);
    await expect(button.right).toBeLessThanOrEqual(menu.left);
    await expect(button.left).toBeGreaterThan(bounds.left + bounds.width * 0.8);
  }}
/>

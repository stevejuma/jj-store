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
  name="Settings"
  args={{ initiallyCollapsed: false }}
  play={async ({ canvasElement }) => {
    const canvas = within(canvasElement);
    const beforeHash = canvasElement.ownerDocument.defaultView?.location.hash;
    await userEvent.click(
      canvas.getByRole("button", { name: "Open profile menu for Maya Chen" }),
    );
    await userEvent.click(
      await within(canvasElement.ownerDocument.body).findByRole("menuitem", {
        name: "Settings",
      }),
    );
    await expect(
      await canvas.findByRole("heading", {
        name: "Local service",
        level: 1,
      }),
    ).toBeVisible();
    await expect(canvas.getByText("Repository token")).toBeVisible();
    await expect(
      canvas.queryByRole("button", { name: "Toggle details" }),
    ).not.toBeInTheDocument();
    await userEvent.click(canvas.getByRole("button", { name: "Repository" }));
    const tablist = canvas.getByRole("tablist");
    const heading = canvas.getByRole("heading", { level: 1 });
    await expect(tablist.getBoundingClientRect().bottom).toBeLessThan(
      heading.getBoundingClientRect().top,
    );
    await expect(canvas.getByText("admin")).toBeVisible();
    await expect(
      canvas.getByRole("textbox", { name: "Repository ID" }),
    ).toHaveAttribute("readonly");
    await userEvent.click(canvas.getByRole("tab", { name: "Advanced" }));
    await expect(canvas.getByText("jj-store-native-v1")).toBeVisible();
    await userEvent.click(canvas.getByRole("button", { name: "Search" }));
    await userEvent.click(canvas.getByRole("tab", { name: "Health" }));
    await expect(canvas.getByText("Fresh")).toBeVisible();
    await expect(canvas.getAllByText("18", { exact: true })).toHaveLength(3);
    await userEvent.click(
      canvas.getByRole("button", { name: "Back to jj-store" }),
    );
    await expect(
      canvas.getByText("jj-store · Branches", { exact: true }),
    ).toBeVisible();
    await expect(canvasElement.ownerDocument.defaultView?.location.hash).toBe(
      beforeHash,
    );
  }}
/>
<Story
  name="Settings collapsed"
  args={{ initiallyCollapsed: false }}
  play={async ({ canvasElement }) => {
    const canvas = within(canvasElement);
    await userEvent.click(
      canvas.getByRole("button", { name: "Open profile menu for Maya Chen" }),
    );
    await userEvent.click(
      await within(canvasElement.ownerDocument.body).findByRole("menuitem", {
        name: "Settings",
      }),
    );
    await canvas.findByRole("heading", { name: "Local service", level: 1 });
    const sidebar = await canvas.findByLabelText("Settings sidebar");
    const navigation = within(sidebar).getByRole("navigation", {
      name: "Settings navigation",
    });
    await expect(navigation).not.toHaveAttribute("data-compact");
    await userEvent.click(
      canvas.getByRole("button", { name: "Collapse left sidebar" }),
    );
    await waitFor(() =>
      expect(sidebar).toHaveAttribute("data-state", "collapsed"),
    );
    await expect(navigation).toHaveAttribute("data-compact", "true");
    for (const label of [
      "Back to jj-store",
      "Local service",
      "Repository",
      "Search",
    ]) {
      const item = within(navigation).getByRole("button", { name: label });
      await expect(item).toBeVisible();
      await expect(item).toHaveAttribute("title", label);
      await expect(item.querySelector("svg")).toBeVisible();
      await expect(item.querySelector("span")).not.toBeVisible();
    }
  }}
/>
<Story
  name="Settings degraded search"
  args={{ initiallyCollapsed: false, settingsSearch: "degraded" }}
  play={async ({ canvasElement }) => {
    const canvas = within(canvasElement);
    await userEvent.click(
      canvas.getByRole("button", { name: "Open profile menu for Maya Chen" }),
    );
    await userEvent.click(
      await within(canvasElement.ownerDocument.body).findByRole("menuitem", {
        name: "Settings",
      }),
    );
    await canvas.findByRole("heading", { name: "Local service", level: 1 });
    await userEvent.click(canvas.getByRole("button", { name: "Search" }));
    await userEvent.click(canvas.getByRole("tab", { name: "Health" }));
    await expect(canvas.getByText("Degraded")).toBeVisible();
    await expect(
      canvas.queryByRole("button", { name: "Prepare graph index" }),
    ).not.toBeInTheDocument();
    await userEvent.click(canvas.getByRole("button", { name: "Repository" }));
    await userEvent.click(canvas.getByRole("tab", { name: "Advanced" }));
    await userEvent.click(
      canvas.getByRole("button", { name: "Prepare graph index" }),
    );
    await expect(
      canvas.getByText("Graph index preparation completed."),
    ).toBeVisible();
    await userEvent.click(canvas.getByRole("button", { name: "Search" }));
    await userEvent.click(canvas.getByRole("tab", { name: "Health" }));
    await expect(canvas.getByText("Degraded")).toBeVisible();
  }}
/>
<Story
  name="Settings unavailable search"
  args={{ initiallyCollapsed: false, settingsSearch: "unavailable" }}
  play={async ({ canvasElement }) => {
    const canvas = within(canvasElement);
    await userEvent.click(
      canvas.getByRole("button", { name: "Open profile menu for Maya Chen" }),
    );
    await userEvent.click(
      await within(canvasElement.ownerDocument.body).findByRole("menuitem", {
        name: "Settings",
      }),
    );
    await canvas.findByRole("heading", { name: "Local service", level: 1 });
    await userEvent.click(canvas.getByRole("button", { name: "Search" }));
    await userEvent.click(canvas.getByRole("tab", { name: "Health" }));
    await expect(
      canvas.getByText("Search is not configured for this repository."),
    ).toBeVisible();
    await expect(
      canvas.queryByRole("button", { name: "Prepare graph index" }),
    ).not.toBeInTheDocument();
    await userEvent.click(canvas.getByRole("button", { name: "Repository" }));
    await userEvent.click(canvas.getByRole("tab", { name: "Advanced" }));
    await userEvent.click(
      canvas.getByRole("button", { name: "Prepare graph index" }),
    );
    await expect(
      canvas.getByText("Graph index preparation completed."),
    ).toBeVisible();
  }}
/>
<Story
  name="Settings status failure"
  args={{ initiallyCollapsed: false, settingsSearch: "failed" }}
  play={async ({ canvasElement }) => {
    const canvas = within(canvasElement);
    await userEvent.click(
      canvas.getByRole("button", { name: "Open profile menu for Maya Chen" }),
    );
    await userEvent.click(
      await within(canvasElement.ownerDocument.body).findByRole("menuitem", {
        name: "Settings",
      }),
    );
    await canvas.findByRole("heading", { name: "Local service", level: 1 });
    await userEvent.click(canvas.getByRole("button", { name: "Search" }));
    await userEvent.click(canvas.getByRole("tab", { name: "Health" }));
    await expect(
      canvas.getByText("Search status could not be loaded."),
    ).toBeVisible();
    await expect(
      canvas.getByRole("button", { name: "Refresh status" }),
    ).toBeEnabled();
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
  name="Settings mobile"
  args={{ displayMode: "mobile" }}
  play={async ({ canvasElement }) => {
    const canvas = within(canvasElement);
    await userEvent.click(
      canvas.getByRole("button", { name: "Open navigation sidebar" }),
    );
    await userEvent.click(
      canvas.getByRole("button", { name: "Open profile menu for Maya Chen" }),
    );
    await userEvent.click(
      await within(canvasElement.ownerDocument.body).findByRole("menuitem", {
        name: "Settings",
      }),
    );
    await expect(
      await canvas.findByRole("heading", { name: "Local service", level: 1 }),
    ).toBeVisible();
    await userEvent.click(
      canvas.getByRole("button", { name: "Open settings sidebar" }),
    );
    await userEvent.click(canvas.getByRole("button", { name: "Search" }));
    await userEvent.click(canvas.getByRole("tab", { name: "Health" }));
    await expect(
      await canvas.findByRole("heading", { name: "Search", level: 1 }),
    ).toBeVisible();
    await expect(canvas.getByText("Fresh")).toBeVisible();
    await userEvent.click(
      canvas.getByRole("button", { name: "Open settings sidebar" }),
    );
    await userEvent.click(
      canvas.getByRole("button", { name: "Back to jj-store" }),
    );
    await expect(
      canvas.getByText("jj-store · Branches", { exact: true }),
    ).toBeVisible();
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
  name="Mobile details control stays at toolbar end"
  args={{ displayMode: "mobile", initialPage: "log" }}
  play={async ({ canvasElement }) => {
    const c = within(canvasElement);
    const toggle = c.getByRole("button", { name: "Open details sidebar" });
    const toolbar = toggle.closest("header")!;
    const bounds = toolbar.getBoundingClientRect(),
      button = toggle.getBoundingClientRect();
    await expect(bounds.right - button.right).toBeLessThan(30);
    await expect(button.left).toBeGreaterThan(bounds.left + bounds.width * 0.8);
  }}
/>

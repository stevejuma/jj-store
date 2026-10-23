<script module lang="ts">
  import { storyControl } from "../../fixtures/story-controls.js";
  import { defineMeta } from "@storybook/addon-svelte-csf";
  import { expect, within, userEvent, waitFor } from "storybook/test";
  import WorkflowsFixture from "../../fixtures/WorkflowsFixture.svelte";
  const { Story } = defineMeta({
    title: "JJ Store/Workflow history navigation",
    component: WorkflowsFixture,
    tags: ["visual-pending", "application"],
    parameters: { jjStoreStory: { kind: "application" }, layout: "fullscreen" },
  });
  const initialHash = "#/workflows?tab=history",
    home = "41".repeat(16),
    workflow = "11".repeat(16);
</script>

<Story
  name="Repository workflow navigation keeps paged runs and author filters"
  args={{ initialHash, historyExamples: true }}
  play={async ({ canvasElement }) => {
    const canvas = within(canvasElement),
      body = within(document.body);
    const nav = within(
      await canvas.findByRole("navigation", {
        name: "Workflows by repository",
      }),
    );
    await expect(
      await nav.findByRole("group", { name: "Parser workflows" }),
    ).toBeVisible();
    await expect(
      await nav.findByRole("group", { name: "Automation home workflows" }),
    ).toBeVisible();
    await userEvent.click(
      await canvas.findByRole("button", { name: "More runs for Build parser" }),
    );
    await waitFor(() =>
      expect(
        canvas.queryByRole("button", {
          name: /Open run Build parser 53535353/,
        }),
        "After loading a continuation",
      ).not.toBeNull(),
    );
    await userEvent.click(nav.getByRole("button", { name: "Build parser" }));
    await expect(canvas.getByLabelText("Workflow location")).toHaveTextContent(
      `scope=${home}&workflow=${workflow}`,
    );
    await waitFor(() =>
      expect(
        canvas.queryByRole("button", {
          name: /Open run Build parser 53535353/,
        }),
        "After selecting a workflow",
      ).not.toBeNull(),
    );
    await expect(
      canvas.queryByRole("button", { name: /Open run Publish preview/ }),
    ).toBeNull();
    await expect(
      nav.getByRole("button", { name: "Build parser" }),
    ).toHaveAttribute("aria-current", "page");
    await userEvent.click(nav.getByRole("button", { name: "All workflows" }));
    await waitFor(async () => {
      await expect(
        await canvas.findByRole("button", { name: /Open run Publish preview/ }),
      ).toBeVisible();
    });
    await userEvent.click(
      canvas.getByRole("combobox", { name: "Run author filter" }),
    );
    const author = await body.findByRole("option", { name: /^Server author/ });
    await expect(
      author.querySelector('[data-ui-component="community-member-avatar"]'),
    ).not.toBeNull();
    await userEvent.click(author);
    await expect(canvas.getByLabelText("Workflow location")).toHaveTextContent(
      "author=" + "22".repeat(16),
    );
    await expect(
      canvas.queryByRole("button", { name: /Open run Build parser 53535353/ }),
    ).toBeNull();
    await expect(
      canvas.getByRole("button", { name: /Open run Publish preview/ }),
    ).toBeVisible();
    await userEvent.click(
      canvas.getByRole("button", { name: /Open run Build parser 51515151/ }),
    );
    await waitFor(async () => {
      await expect(
        await canvas.findByRole("article", { name: "Run details" }),
      ).toBeVisible();
    });
    await expect(
      canvas.queryByRole("navigation", { name: "Workflows by repository" }),
    ).toBeNull();
    await userEvent.click(
      canvas.getByRole("button", { name: "Back to history" }),
    );
    await expect(canvas.getByLabelText("Workflow location")).toHaveTextContent(
      "author=" + "22".repeat(16),
    );
    await userEvent.click(
      canvas.getByRole("button", { name: "Clear history filters" }),
    );
    await waitFor(() =>
      expect(
        canvas.queryByRole("button", {
          name: /Open run Build parser 53535353/,
        }),
        "After returning from details and clearing filters",
      ).not.toBeNull(),
    );
    await expect(canvas.queryByRole("button", { name: /^Mine/ })).toBeNull();
    await expect(
      canvas.queryByRole("button", { name: /^Other runs/ }),
    ).toBeNull();
  }}
/>

<Story
  name="Recorded trigger and bookmark filters with icon-bearing row menu"
  args={{ initialHash, historyExamples: true }}
  play={async ({ canvasElement }) => {
    const canvas = within(canvasElement),
      body = within(document.body);
    await canvas.findByRole("button", { name: /Open run Publish preview/ });
    await userEvent.click(
      canvas.getByRole("combobox", { name: "Run trigger filter" }),
    );
    await userEvent.click(
      await body.findByRole("option", { name: /^Repository event/ }),
    );
    await expect(
      canvas.queryByRole("button", { name: /Open run Build parser/ }),
    ).toBeNull();
    await userEvent.click(
      canvas.getByRole("combobox", { name: "Run bookmark filter" }),
    );
    await userEvent.click(
      await body.findByRole("option", { name: /^release/ }),
    );
    await expect(canvas.getByLabelText("Workflow location")).toHaveTextContent(
      "trigger=event&bookmark=release",
    );
    await userEvent.click(
      canvas.getByRole("button", { name: "Actions for run 55555555" }),
    );
    const menu = await body.findByRole("menuitem", {
      name: "View workflow definition",
    });
    await expect(menu.querySelector("svg")).not.toBeNull();
    await expect(body.queryByRole("menuitem", { name: /Delete/ })).toBeNull();
    await userEvent.keyboard("{Escape}");
    await waitFor(() =>
      expect(getComputedStyle(document.body).pointerEvents).not.toBe("none"),
    );
    await userEvent.click(
      canvas.getByRole("button", { name: "Clear history filters" }),
    );
    await userEvent.click(
      canvas.getByRole("button", { name: "Show runs for Automation home" }),
    );
    await expect(canvas.getByLabelText("Workflow location")).toHaveTextContent(
      "source=" + home,
    );
    await expect(
      canvas.getByRole("button", { name: /Open run Publish preview/ }),
    ).toBeVisible();
    await expect(
      canvas.queryByRole("button", { name: /Open run Build parser/ }),
    ).toBeNull();
    await userEvent.click(
      canvas
        .getByRole("navigation", { name: "Workflows by repository" })
        .querySelector('button[title^="Agent review"]')!,
    );
    await waitFor(async () => {
      await expect(await canvas.findByText("No matching runs")).toBeVisible();
    });
  }}
/>

<Story
  name="Workflow navigation is keyboard collapsible and restores its selected group"
  args={{ initialHash, historyExamples: true }}
  play={async ({ canvasElement }) => {
    const canvas = within(canvasElement);
    const toggle = await canvas.findByRole("button", {
      name: "Collapse Parser workflows",
    });
    toggle.focus();
    await userEvent.keyboard("{Enter}");
    await expect(
      canvas.getByRole("button", { name: "Expand Parser workflows" }),
    ).toHaveAttribute("aria-expanded", "false");
    await userEvent.click(
      canvas.getByRole("button", { name: "Toggle workflow navigation" }),
    );
    await expect(
      canvas.queryByRole("navigation", { name: "Workflows by repository" }),
    ).toBeNull();
    await userEvent.click(
      canvas.getByRole("button", { name: "Toggle workflow navigation" }),
    );
    await userEvent.click(
      canvas.getByRole("button", { name: "Expand Parser workflows" }),
    );
    await userEvent.click(canvas.getByRole("button", { name: "Build parser" }));
    await storyControl(document.body, "Fixture back");
    await waitFor(() =>
      expect(
        canvas.getByRole("button", { name: "All workflows" }),
      ).toHaveAttribute("aria-current", "page"),
    );
    await storyControl(document.body, "Fixture forward");
    await waitFor(() =>
      expect(
        canvas.getByRole("button", { name: "Build parser" }),
      ).toHaveAttribute("aria-current", "page"),
    );
  }}
/>

<Story
  name="Mobile history keeps workflow navigation and filters reachable"
  parameters={{
    viewport: { defaultViewport: "mobile390" },
    jjStoreShell: true,
  }}
  args={{ initialHash, historyExamples: true, shell: true, width: 390 }}
  play={async ({ canvasElement }) => {
    const canvas = within(canvasElement),
      body = within(document.body);
    await waitFor(async () => {
      await expect(
        await canvas.findByRole("button", { name: /Open run Publish preview/ }),
      ).toBeVisible();
    });
    await waitFor(() =>
      expect(
        canvas
          .getByTestId("workflow-frame")
          .querySelector('[data-ui-part="root"]'),
      ).toHaveAttribute("data-display-mode", "mobile"),
    );
    await userEvent.click(
      canvas.getByRole("button", { name: "Toggle workflow navigation" }),
    );
    await userEvent.click(
      await body.findByRole("button", { name: "Build parser" }),
    );
    await waitFor(async () => {
      await expect(
        await canvas.findByRole("button", {
          name: /Open run Build parser 51515151/,
        }),
      ).toBeVisible();
    });
    const frame = canvas.getByTestId("workflow-frame");
    await expect(frame.scrollWidth).toBeLessThanOrEqual(frame.clientWidth);
    await userEvent.click(
      canvas.getByRole("combobox", { name: "Run status filter" }),
    );
    await waitFor(async () => {
      await expect(
        await body.findByRole("option", { name: /^skipped/ }),
      ).toBeVisible();
    });
    await userEvent.keyboard("{Escape}");
  }}
/>

<Story
  name="Resizable white workflow panel and aligned avatar rows"
  args={{ initialHash, historyExamples: true }}
  play={async ({ canvasElement }) => {
    const canvas = within(canvasElement);
    const list = await canvas.findByRole("list", { name: "Workflow runs" });
    const sidebar = canvas.getByRole("complementary", {
      name: "Workflow navigation",
    });
    const header = canvas
      .getByRole("heading", { name: "All workflows" })
      .closest("header")!;
    await expect(
      header.querySelector('[data-ui-part="sidebar-toggle-icon"]'),
    ).not.toBeNull();
    await expect(
      Math.abs(
        sidebar.getBoundingClientRect().top -
          header.getBoundingClientRect().top,
      ),
    ).toBeLessThan(2);
    await expect(getComputedStyle(sidebar).backgroundColor).toBe(
      getComputedStyle(header).backgroundColor,
    );
    const handle = canvas.getByRole("button", {
      name: "Resize workflow navigation",
    });
    const original = sidebar.getBoundingClientRect().width;
    handle.focus();
    await userEvent.keyboard("{ArrowRight}");
    await waitFor(() =>
      expect(sidebar.getBoundingClientRect().width).toBe(original + 16),
    );
    await userEvent.keyboard("{Home}");
    await waitFor(() =>
      expect(sidebar.getBoundingClientRect().width).toBe(224),
    );
    await userEvent.dblClick(handle);
    await waitFor(() =>
      expect(sidebar.getBoundingClientRect().width).toBe(256),
    );
    const articles = list.querySelectorAll("article");
    const positions = [...articles].map((row) => {
      const status = row.querySelector(".workflow-run-status")!,
        avatar = row.querySelector('button[aria-label^="View "]')!,
        text = row.querySelector(".workflow-history-row__summary")!;
      const a = avatar.getBoundingClientRect(),
        t = text.getBoundingClientRect();
      expect(status.getBoundingClientRect().left).toBeLessThan(a.left);
      expect(
        Math.abs(a.top + a.height / 2 - t.top - t.height / 2),
      ).toBeLessThan(2);
      expect(text.textContent).not.toContain("Server author");
      return a.left;
    });
    await expect(new Set(positions).size).toBe(1);
    const filter = canvas.getByRole("combobox", { name: "Run author filter" }),
      initial = getComputedStyle(filter).backgroundColor;
    await userEvent.keyboard("{Tab}");
    filter.focus();
    await waitFor(() =>
      expect(getComputedStyle(filter).backgroundColor).not.toBe(initial),
    );
  }}
/>

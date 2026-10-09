<script module lang="ts">
  import { defineMeta } from "@storybook/addon-svelte-csf";
  import { expect, userEvent, within, waitFor } from "storybook/test";
  import { page as browser } from "vitest/browser";
  import Fixture from "../../fixtures/WorkspaceFilesFixture.svelte";
  const { Story } = defineMeta({
    title: "JJ Store/Conversation workspace files",
    component: Fixture,
    tags: ["visual-pending"],
    parameters: { layout: "fullscreen" },
  });
</script>

<Story
  name="Standalone footer opens files and diffs without changing the conversation"
  play={async ({ canvasElement }) => {
    const c = within(canvasElement),
      page = within(document.body);
    await userEvent.click(await c.findByRole("button", { name: /Workspace:/ }));
    await expect(
      await within(document.querySelector(".workspace-popover")!).findByText(
        "/fixture/repository/agent-workspace",
      ),
    ).toBeVisible();
    await userEvent.click(page.getByRole("button", { name: "View files" }));
    const panel = await c.findByRole("complementary", {
      name: "Workspace files",
    });
    const rail = within(panel);
    const main = panel.parentElement!.querySelector<HTMLElement>(
      ".ui-minimal-app-shell__main",
    )!;
    await waitFor(() => {
      expect(panel.parentElement).toHaveClass(
        "ui-minimal-app-shell__composition",
      );
      expect(
        panel.querySelector('[data-ui-part="sidebar-header"] [role="tablist"]'),
      ).not.toBeNull();
      expect(getComputedStyle(panel).backgroundColor).toBe(
        getComputedStyle(main).backgroundColor,
      );
      expect(getComputedStyle(main).borderStartEndRadius).toBe("0px");
      expect(getComputedStyle(main).borderEndEndRadius).toBe("0px");
      expect(
        Math.abs(
          panel.getBoundingClientRect().top - main.getBoundingClientRect().top,
        ),
      ).toBeLessThanOrEqual(1);
      expect(
        Math.abs(
          panel.getBoundingClientRect().bottom -
            main.getBoundingClientRect().bottom,
        ),
      ).toBeLessThanOrEqual(1);
    });
    const refresh = rail.getByRole("button", {
      name: "Refresh workspace files",
    });
    await expect(
      panel.getBoundingClientRect().right -
        refresh.getBoundingClientRect().right,
    ).toBeLessThanOrEqual(13);
    await expect(
      getComputedStyle(rail.getByRole("tab", { name: "Files" })).borderRadius,
    ).toBe("999px");
    await userEvent.click(
      await rail.findByRole("button", { name: "src/unchanged.ts" }),
    );
    const inspector = within(
      await c.findByRole("complementary", { name: "Workspace file" }),
    );
    await expect(
      await inspector.findByRole("textbox", {
        name: "src/unchanged.ts",
      }),
    ).toHaveTextContent("unchanged = true");
    const inspectorElement = c.getByRole("complementary", {
      name: "Workspace file",
    });
    const fileContent = inspectorElement.querySelector<HTMLElement>(
      ".workspace-file-content",
    )!;
    await expect(getComputedStyle(fileContent).paddingInlineStart).toBe("12px");
    if (inspectorElement.dataset.presentation === "inline") {
      await expect(
        Math.abs(
          inspectorElement.getBoundingClientRect().right -
            panel.getBoundingClientRect().left,
        ),
      ).toBeLessThanOrEqual(1);
      await expect(
        getComputedStyle(inspectorElement).borderStartEndRadius,
      ).toBe("0px");
    }
    await userEvent.click(
      inspector.getByRole("button", { name: "Close workspace file" }),
    );
    rail.getByRole("tab", { name: "Files" }).focus();
    await userEvent.keyboard("{ArrowRight}");
    await expect(rail.getByRole("tab", { name: "Changes" })).toHaveAttribute(
      "aria-selected",
      "true",
    );
    await expect(
      await rail.findByRole("button", { name: "Comparison range" }),
    ).toHaveTextContent("Branch");
    await waitFor(() =>
      expect(
        rail.queryByRole("button", { name: "src/unchanged.ts" }),
      ).not.toBeInTheDocument(),
    );
    await userEvent.click(
      await rail.findByRole("button", { name: "src/example.ts" }),
    );
    const diff = within(
      await c.findByRole("complementary", { name: "Workspace file" }),
    );
    await expect(
      await diff.findByRole("radio", { name: "Before" }),
    ).toBeVisible();
    await userEvent.click(diff.getByRole("radio", { name: "After" }));
    await expect(
      await diff.findByRole("textbox", { name: "src/example.ts" }),
    ).toHaveTextContent("café 🦀");
    await expect(document.documentElement.scrollWidth).toBeLessThanOrEqual(
      window.innerWidth + 1,
    );
  }}
/>

<Story
  name="Workspace refresh retains files without a loading-more row"
  args={{ controlReads: true }}
  play={async ({ canvasElement }) => {
    const c = within(canvasElement);
    await userEvent.click(await c.findByRole("button", { name: /Workspace:/ }));
    await userEvent.click(
      await within(document.body).findByRole("button", { name: "View files" }),
    );
    const panel = within(
      await c.findByRole("complementary", { name: "Workspace files" }),
    );
    await panel.findByRole("button", { name: "src/unchanged.ts" });
    const refresh = panel.getByRole("button", {
      name: "Refresh workspace files",
    });
    await waitFor(() => expect(refresh).toBeEnabled());
    await userEvent.click(
      c.getByRole("button", { name: "Hold next workspace read" }),
    );
    await userEvent.click(refresh);
    await c.findByRole("button", { name: "Complete workspace read" });
    await expect(refresh).toBeDisabled();
    await expect(panel.getByRole("tabpanel")).toHaveAttribute(
      "aria-busy",
      "true",
    );
    await expect(
      panel.getByRole("button", { name: "src/unchanged.ts" }),
    ).toBeVisible();
    await expect(
      panel.queryByText("Loading more repository files"),
    ).not.toBeInTheDocument();
    await userEvent.click(
      c.getByRole("button", { name: "Complete workspace read" }),
    );
    await waitFor(() => expect(refresh).toBeEnabled());
    await expect(panel.getByRole("tabpanel")).toHaveAttribute(
      "aria-busy",
      "false",
    );
  }}
/>

<Story
  name="Workspace panels stay in their owning inbox and preserve other rails"
  args={{ routeNavigation: true }}
  play={async ({ canvasElement }) => {
    const c = within(canvasElement);
    const originalRail = () =>
      c.getByRole("complementary", {
        name: "Repository details",
        hidden: true,
      });
    const appearance = (el: HTMLElement) => {
      const style = getComputedStyle(el);
      return {
        background: style.backgroundColor,
        margin: style.marginInlineEnd,
        radius: style.borderStartEndRadius,
        width: el.getBoundingClientRect().width,
      };
    };
    await userEvent.click(c.getByRole("button", { name: "Fixture projects" }));
    await expect(
      c.getByRole("button", { name: "Toggle details" }),
    ).toHaveAttribute("aria-expanded", "false");
    await userEvent.click(c.getByRole("button", { name: "Toggle details" }));
    await waitFor(() => expect(originalRail()).toBeVisible());
    const before = appearance(originalRail());
    await expect(originalRail()).toHaveAttribute("data-variant", "rail");
    await userEvent.click(c.getByRole("button", { name: "Fixture inbox" }));
    await userEvent.click(await c.findByRole("button", { name: /Workspace:/ }));
    await userEvent.click(
      await within(document.body).findByRole("button", { name: "View files" }),
    );
    const files = within(
      await c.findByRole("complementary", { name: "Workspace files" }),
    );
    await userEvent.click(
      await files.findByRole("button", { name: "src/unchanged.ts" }),
    );
    await within(
      await c.findByRole("complementary", { name: "Workspace file" }),
    ).findByRole("textbox", { name: "src/unchanged.ts" });
    for (const destination of ["Fixture projects", "Fixture repository"]) {
      await userEvent.click(c.getByRole("button", { name: destination }));
      await waitFor(() => expect(originalRail()).toBeVisible());
      await expect(
        c.queryByRole("complementary", {
          name: "Workspace files",
          hidden: true,
        }),
      ).not.toBeInTheDocument();
      await expect(
        c.queryByRole("complementary", {
          name: "Workspace file",
          hidden: true,
        }),
      ).not.toBeInTheDocument();
      await expect(originalRail()).toHaveAttribute("data-variant", "rail");
      await expect(appearance(originalRail())).toEqual(before);
      await expect(
        c.getByRole("button", { name: "Toggle details" }),
      ).toHaveAttribute("aria-expanded", "true");
    }
    await userEvent.click(c.getByRole("button", { name: "Toggle details" }));
    await userEvent.click(c.getByRole("button", { name: "Fixture inbox" }));
    await expect(
      await c.findByRole("complementary", { name: "Workspace files" }),
    ).toBeVisible();
    await expect(
      await within(
        await c.findByRole("complementary", { name: "Workspace file" }),
      ).findByRole("textbox", { name: "src/unchanged.ts" }),
    ).toHaveTextContent("unchanged = true");
    await userEvent.click(
      c.getByRole("button", { name: "Fixture other session" }),
    );
    await expect(
      c.queryByRole("complementary", { name: "Workspace files", hidden: true }),
    ).not.toBeInTheDocument();
    await expect(
      c.queryByRole("complementary", { name: "Workspace file", hidden: true }),
    ).not.toBeInTheDocument();
    await userEvent.click(c.getByRole("button", { name: "Fixture projects" }));
    await expect(
      c.getByRole("button", { name: "Toggle details" }),
    ).toHaveAttribute("aria-expanded", "false");
  }}
/>

<Story
  name="Inbox header toggles workspace files and file loads show skeletons"
  args={{ controlReads: true }}
  play={async ({ canvasElement }) => {
    const c = within(canvasElement);
    const toggle = await c.findByRole("button", {
      name: "Toggle workspace files",
    });
    await waitFor(() => expect(toggle).toBeEnabled());
    await expect(toggle).toHaveAttribute("aria-expanded", "false");
    await userEvent.click(toggle);
    const rail = within(
      await c.findByRole("complementary", { name: "Workspace files" }),
    );
    await rail.findByRole("button", { name: "src/unchanged.ts" });
    await waitFor(() =>
      expect(
        rail.getByRole("button", { name: "Refresh workspace files" }),
      ).toBeEnabled(),
    );
    await expect(toggle).toHaveAttribute("aria-expanded", "true");
    await userEvent.click(toggle);
    await expect(toggle).toHaveAttribute("aria-expanded", "false");
    await expect(
      c.queryByRole("complementary", { name: "Workspace files" }),
    ).not.toBeInTheDocument();
    await userEvent.click(toggle);
    await waitFor(() =>
      expect(
        rail.getByRole("button", { name: "Refresh workspace files" }),
      ).toBeEnabled(),
    );
    await userEvent.click(
      c.getByRole("button", { name: "Hold next workspace read" }),
    );
    await userEvent.click(
      rail.getByRole("button", { name: "src/unchanged.ts" }),
    );
    const inspector = within(
      await c.findByRole("complementary", { name: "Workspace file" }),
    );
    const loading = await inspector.findByRole("status", {
      name: "Loading src/unchanged.ts",
    });
    const placeholders = [
      ...loading.querySelectorAll('[data-ui-component="skeleton"]'),
    ];
    await expect(placeholders.length).toBeGreaterThan(3);
    for (const placeholder of placeholders)
      await expect(placeholder.getBoundingClientRect().height).toBeGreaterThan(
        0,
      );
    await expect(
      inspector.queryByText("Loading file…"),
    ).not.toBeInTheDocument();
    await expect(inspector.getByText("src/unchanged.ts")).toBeVisible();
    await userEvent.click(
      c.getByRole("button", { name: "Complete workspace read" }),
    );
    await expect(
      await inspector.findByRole("textbox", { name: "src/unchanged.ts" }),
    ).toHaveTextContent("unchanged = true");
    await expect(
      inspector.queryByRole("status", { name: "Loading src/unchanged.ts" }),
    ).not.toBeInTheDocument();
  }}
/>

<Story
  name="Workspace rail header and file divider span the panel"
  play={async ({ canvasElement }) => {
    const c = within(canvasElement);
    await userEvent.click(
      await c.findByRole("button", { name: "Toggle workspace files" }),
    );
    const panel = await c.findByRole("complementary", {
      name: "Workspace files",
    });
    await within(panel).findByRole("button", { name: "README.md" });
    const header = panel.querySelector<HTMLElement>(
      '[data-ui-part="sidebar-header"]',
    )!;
    const toolbar = panel.querySelector<HTMLElement>(
      ".community-projects__file-tree-actions",
    )!;
    await expect({
      headerBorder: getComputedStyle(header).borderBottomWidth,
      startsAtPanelEdge:
        Math.abs(
          toolbar.getBoundingClientRect().left -
            panel.getBoundingClientRect().left,
        ) <= 1,
      endsAtPanelEdge:
        Math.abs(
          toolbar.getBoundingClientRect().right -
            panel.getBoundingClientRect().right,
        ) <= 1,
    }).toEqual({
      headerBorder: "0px",
      startsAtPanelEdge: true,
      endsAtPanelEdge: true,
    });
    await expect(getComputedStyle(toolbar).borderBottomWidth).toBe("1px");
  }}
/>

<Story
  name="Workspace inspector joins the rail at wide and constrained widths"
  play={async ({ canvasElement }) => {
    const c = within(canvasElement);
    try {
      await browser.viewport(1800, 988);
      await userEvent.click(
        await c.findByRole("button", { name: "Toggle workspace files" }),
      );
      const panel = await c.findByRole("complementary", {
        name: "Workspace files",
      });
      await userEvent.click(
        await within(panel).findByRole("button", { name: "README.md" }),
      );
      const inspector = await c.findByRole("complementary", {
        name: "Workspace file",
      });
      await within(inspector).findByRole("textbox", { name: "README.md" });
      for (const [width, presentation] of [
        [1800, "inline"],
        [1070, "replace-main"],
      ] as const) {
        await browser.viewport(width, 988);
        await waitFor(() =>
          expect(inspector).toHaveAttribute("data-presentation", presentation),
        );
        await expect(panel).toHaveAttribute("data-presentation", "inline");
        await waitFor(() =>
          expect(
            Math.abs(
              inspector.getBoundingClientRect().right -
                panel.getBoundingClientRect().left,
            ),
          ).toBeLessThanOrEqual(1),
        );
        await expect(getComputedStyle(inspector).borderStartEndRadius).toBe(
          "0px",
        );
        await expect(getComputedStyle(inspector).borderEndEndRadius).toBe(
          "0px",
        );
        await expect(document.documentElement.scrollWidth).toBeLessThanOrEqual(
          window.innerWidth + 1,
        );
      }
    } finally {
      await browser.viewport(1280, 800);
    }
  }}
/>

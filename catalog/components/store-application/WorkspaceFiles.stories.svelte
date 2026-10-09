<script module lang="ts">
  import { defineMeta } from "@storybook/addon-svelte-csf";
  import { expect, fn, userEvent, within, waitFor } from "storybook/test";
  import { page as browser, userEvent as browserEvent } from "vitest/browser";
  import Fixture from "../../fixtures/WorkspaceFilesFixture.svelte";
  const longBranch =
    "feature/retain-exact-workspace-source-and-conversation-reply-identities";
  async function viewFiles() {
    const page = within(document.body);
    await userEvent.click(
      await page.findByRole("menuitem", { name: "View files" }),
    );
    await waitFor(() => {
      expect(page.queryByRole("menu", { name: "Workspace" })).toBeNull();
      expect(getComputedStyle(document.body).pointerEvents).not.toBe("none");
    });
  }
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
    await viewFiles();
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
    await viewFiles();
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
    await viewFiles();
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
    const reopened = within(
      await c.findByRole("complementary", { name: "Workspace files" }),
    );
    await reopened.findByRole("button", { name: "src/unchanged.ts" });
    await userEvent.click(
      c.getByRole("button", { name: "Hold next workspace read" }),
    );
    await browser.getByRole("button", { name: /^src\/unchanged\.ts$/ }).click();
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
      headerBorder: "1px",
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

<Story
  name="Workspace source reader has syntax and gutters without extra comment chrome"
  play={async ({ canvasElement }) => {
    const c = within(canvasElement);
    await userEvent.click(
      await c.findByRole("button", { name: "Toggle workspace files" }),
    );
    const rail = within(
      await c.findByRole("complementary", { name: "Workspace files" }),
    );
    await userEvent.click(
      await rail.findByRole("button", { name: "README.md" }),
    );
    const panel = await c.findByRole("complementary", {
      name: "Workspace file",
    });
    const inspector = within(panel);
    await inspector.findByRole("textbox", { name: "README.md" });
    await expect({
      headerBorder: getComputedStyle(
        panel.querySelector('[data-ui-part="sidebar-header"]')!,
      ).borderBottomWidth,
      lineNumbers: !!panel.querySelector(".cm-lineNumbers"),
      heading: !!panel.querySelector(".cm-heading"),
      background: getComputedStyle(panel.querySelector(".chat-text-block")!)
        .backgroundColor,
      extraCommentRow: !!inspector.queryByRole("button", {
        name: "Comment on file",
      }),
    }).toEqual({
      headerBorder: "1px",
      lineNumbers: true,
      heading: true,
      background: "rgba(0, 0, 0, 0)",
      extraCommentRow: false,
    });
    await userEvent.click(rail.getByRole("button", { name: "src/example.ts" }));
    const source = await inspector.findByRole("textbox", {
      name: "src/example.ts",
    });
    await waitFor(() =>
      expect(panel.querySelector(".cm-keyword")).not.toBeNull(),
    );
    await expect(panel.querySelector(".cm-string")).toHaveTextContent(
      "café 🦀",
    );
    source.focus();
    await userEvent.keyboard("{Control>}a{/Control}");
    await userEvent.click(
      inspector.getByRole("button", {
        name: "Comment on src/example.ts",
      }),
    );
    const editor = await inspector.findByRole("textbox", {
      name: "Comment on src/example.ts",
    });
    await userEvent.click(editor);
    await userEvent.keyboard("Keep this Unicode greeting.");
    await expect(source).toHaveTextContent(
      "export const greeting = 'café 🦀';",
    );
    await expect(
      inspector.getByRole("textbox", { name: "Comment on src/example.ts" }),
    ).toHaveTextContent("Keep this Unicode greeting.");
  }}
/>

<Story
  name="Workspace comparisons use compact adjacent controls and an explicit base placeholder"
  args={{ missingBase: true, baseName: longBranch }}
  play={async ({ canvasElement }) => {
    const c = within(canvasElement);
    await userEvent.click(
      await c.findByRole("button", { name: "Toggle workspace files" }),
    );
    const panel = await c.findByRole("complementary", {
      name: "Workspace files",
    });
    const rail = within(panel);
    await userEvent.click(rail.getByRole("tab", { name: "Changes" }));
    const range = await rail.findByRole("button", { name: "Comparison range" });
    const base = await rail.findByRole("button", { name: "Comparison base" });
    await waitFor(() =>
      expect(
        rail.queryByRole("status", { name: "Loading workspace files" }),
      ).not.toBeInTheDocument(),
    );
    await expect(
      rail.queryByText(
        "Choose a comparison base. No default branch is configured.",
      ),
    ).not.toBeInTheDocument();
    await expect(base).toHaveTextContent("Choose branch");
    await expect(range).toHaveAttribute("data-size", "sm");
    await expect(base).toHaveAttribute("data-size", "sm");
    await expect(
      Math.abs(
        range.getBoundingClientRect().top - base.getBoundingClientRect().top,
      ),
    ).toBeLessThanOrEqual(1);
    await expect(base.getBoundingClientRect().left).toBeGreaterThan(
      range.getBoundingClientRect().right,
    );
    await expect(base.getBoundingClientRect().right).toBeLessThanOrEqual(
      panel.getBoundingClientRect().right,
    );
    await userEvent.click(base);
    await userEvent.click(
      await within(document.body).findByRole("option", {
        name: longBranch,
      }),
    );
    await expect(base).toHaveTextContent(longBranch);
    await expect(base).toHaveAttribute("title", longBranch);
    const label = base.querySelector<HTMLElement>(".truncate")!;
    await expect(getComputedStyle(label).textOverflow).toBe("ellipsis");
    await expect(label.scrollWidth).toBeGreaterThan(label.clientWidth);
    await expect(
      range.querySelector(".lucide-arrow-left-right"),
    ).not.toBeNull();
    await expect(base.querySelector(".lucide-git-branch")).not.toBeNull();
    const resize = c.getByRole("slider", { name: "Resize workspace files" });
    resize.focus();
    await userEvent.keyboard("{Home}");
    await waitFor(() =>
      expect(
        Math.abs(
          base.getBoundingClientRect().top - range.getBoundingClientRect().top,
        ),
      ).toBeLessThanOrEqual(1),
    );
    await expect(base.getBoundingClientRect().right).toBeLessThanOrEqual(
      panel.getBoundingClientRect().right,
    );
    await expect(document.documentElement.scrollWidth).toBeLessThanOrEqual(
      window.innerWidth + 1,
    );
    resize.focus();
    await userEvent.keyboard("{End}");
    await waitFor(() =>
      expect(
        Math.abs(
          base.getBoundingClientRect().top - range.getBoundingClientRect().top,
        ),
      ).toBeLessThanOrEqual(1),
    );
  }}
/>

<Story
  name="Workspace footer offers compact left aligned actions and a copy submenu"
  play={async ({ canvasElement }) => {
    const c = within(canvasElement),
      page = within(document.body);
    const trigger = await c.findByRole("button", { name: /Workspace:/ });
    const write = fn();
    const descriptor = Object.getOwnPropertyDescriptor(navigator, "clipboard");
    Object.defineProperty(navigator, "clipboard", {
      configurable: true,
      value: { writeText: write },
    });
    try {
      await expect(parseFloat(getComputedStyle(trigger).fontSize)).toBeLessThan(
        14,
      );
      const rest = getComputedStyle(trigger).backgroundColor;
      await expect(rest).not.toBe("rgba(0, 0, 0, 0)");
      await browser.getByRole("button", { name: /Workspace:/ }).hover();
      await waitFor(() =>
        expect(getComputedStyle(trigger).backgroundColor).not.toBe(rest),
      );
      await browserEvent.click(trigger);
      const view = await page.findByRole("menuitem", { name: "View files" });
      const menu = page.getByRole("menu", { name: "Workspace" });
      await expect(
        within(menu).queryByText("/fixture/repository/agent-workspace"),
      ).toBeNull();
      await expect(
        Math.abs(
          menu.getBoundingClientRect().left -
            trigger.getBoundingClientRect().left,
        ),
      ).toBeLessThanOrEqual(1);
      await expect(getComputedStyle(view).justifyContent).not.toBe("center");
      await browserEvent.click(page.getByRole("menuitem", { name: "Copy" }));
      await browserEvent.click(
        await page.findByRole("menuitem", { name: "Project path" }),
      );
      await waitFor(() =>
        expect(write).toHaveBeenCalledWith(
          "/fixture/repository/agent-workspace",
        ),
      );
      await waitFor(() =>
        expect(page.queryByRole("menu", { name: "Workspace" })).toBeNull(),
      );
      await waitFor(() =>
        expect(getComputedStyle(trigger).pointerEvents).not.toBe("none"),
      );
      await browserEvent.click(trigger);
      const copy = await page.findByRole("menuitem", { name: "Copy" });
      copy.focus();
      await browserEvent.keyboard("{ArrowRight}");
      const name = await page.findByRole("menuitem", {
        name: "Workspace name",
      });
      await waitFor(() =>
        expect(
          page.getByRole("menuitem", { name: "Project path" }),
        ).toHaveFocus(),
      );
      await browserEvent.keyboard("{ArrowDown}");
      await waitFor(() => expect(name).toHaveFocus());
      await browserEvent.keyboard("{Enter}");
      await waitFor(() => expect(write).toHaveBeenCalledTimes(2));
      await expect(write).toHaveBeenLastCalledWith("agent-" + "44".repeat(16));
      await waitFor(() => expect(trigger).toHaveFocus());
    } finally {
      if (descriptor) Object.defineProperty(navigator, "clipboard", descriptor);
      else Reflect.deleteProperty(navigator, "clipboard");
    }
  }}
/>

<Story
  name="Workspace menu keeps multi repository inspection scoped to the selection"
  args={{ multipleBindings: true }}
  play={async ({ canvasElement }) => {
    const c = within(canvasElement),
      page = within(document.body);
    const trigger = await c.findByRole("button", { name: /Workspace:/ });
    await browserEvent.click(trigger);
    const select = await page.findByRole("menuitem", {
      name: "Inspect workspace",
    });
    select.focus();
    await browserEvent.keyboard("{ArrowRight}");
    const option = await page.findByRole("menuitemradio", {
      name: /other-workspace/,
    });
    await waitFor(() =>
      expect(
        page.getByRole("menuitemradio", { name: /agent-workspace/ }),
      ).toHaveFocus(),
    );
    await browserEvent.keyboard("{ArrowDown}");
    await waitFor(() => expect(option).toHaveFocus());
    await browserEvent.keyboard("{Enter}");
    await waitFor(() => expect(trigger).toHaveTextContent("other-workspace"));
    await waitFor(() =>
      expect(getComputedStyle(trigger).pointerEvents).not.toBe("none"),
    );
    await browserEvent.click(trigger);
    await viewFiles();
    await expect(
      await c.findByRole("complementary", { name: "Workspace files" }),
    ).toBeVisible();
    await expect(
      await c.findByRole("button", { name: "src/unchanged.ts" }),
    ).toBeVisible();
  }}
/>

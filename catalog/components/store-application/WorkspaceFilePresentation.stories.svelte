<script module lang="ts">
  import { verifyWorkspaceRailHeader } from "../../fixtures/workspace-rail-header-play.js";
  import { views } from "../../../web/src/lib/ui/store-application/application-views.js";
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
    beforeEach: async () => {
      await Promise.all([
        views.agents(),
        views.agentChatSidebar(),
        views.workspaceFiles(),
        views.workspaceInspector(),
      ]);
    },
    tags: ["visual-pending"],
    parameters: { layout: "fullscreen" },
  });
</script>

<Story
  name="Workspace rail header and file divider span the panel"
  play={verifyWorkspaceRailHeader}
/>

<Story
  name="Workspace inspector joins the rail at wide and constrained widths"
  play={async ({ canvasElement }) => {
    const c = within(canvasElement);

    try {
      await browser.viewport(1800, 988);
      await waitFor(() =>
        expect(
          c.getByRole("button", { name: "Toggle workspace files" }),
        ).toBeEnabled(),
      );
      await userEvent.click(
        c.getByRole("button", { name: "Toggle workspace files" }),
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

    await waitFor(() =>
      expect(
        c.getByRole("button", { name: "Toggle workspace files" }),
      ).toBeEnabled(),
    );
    await userEvent.click(
      c.getByRole("button", { name: "Toggle workspace files" }),
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

    await waitFor(() =>
      expect(
        c.getByRole("button", { name: "Toggle workspace files" }),
      ).toBeEnabled(),
    );
    await userEvent.click(
      c.getByRole("button", { name: "Toggle workspace files" }),
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
      await waitFor(() =>
        expect(
          Math.abs(
            menu.getBoundingClientRect().left -
              trigger.getBoundingClientRect().left,
          ),
        ).toBeLessThanOrEqual(1),
      );
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
      await waitFor(() => {
        const remaining = page.queryByRole("menu", { name: "Workspace" });
        const style = remaining ? getComputedStyle(remaining) : null;
        expect(
          remaining,
          JSON.stringify({
            state: remaining?.getAttribute("data-state"),
            animation: style?.animation,
            transitions: style?.transition,
            pointer: getComputedStyle(document.body).pointerEvents,
          }),
        ).toBeNull();
      });
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
      await waitFor(() =>
        expect(page.queryByRole("menu", { name: "Workspace" })).toBeNull(),
      );
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
      name: "Workspace",
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

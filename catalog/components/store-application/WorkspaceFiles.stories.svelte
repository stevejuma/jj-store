<script module lang="ts">
  import { defineMeta } from "@storybook/addon-svelte-csf";
  import { expect, userEvent, within, waitFor } from "storybook/test";
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

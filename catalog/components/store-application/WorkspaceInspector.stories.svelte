<script module lang="ts">
  import { defineMeta } from "@storybook/addon-svelte-csf";
  import { expect, userEvent, within, waitFor } from "storybook/test";
  import { page as browser } from "vitest/browser";
  import Fixture from "../../fixtures/WorkspaceFilesFixture.svelte";
  const { Story } = defineMeta({
    title: "JJ Store/Conversation workspace inspector",
    component: Fixture,
    tags: ["visual-pending"],
    parameters: { layout: "fullscreen" },
  });
</script>

<Story
  name="Changed file header owns responsive presentation tabs and icon diff controls"
  play={async ({ canvasElement }) => {
    const c = within(canvasElement);
    await c.findByRole("button", { name: "Toggle workspace files" });
    await waitFor(() =>
      expect(
        c.getByRole("button", {
          name: "Toggle workspace files",
        }),
      ).toBeEnabled(),
    );
    await userEvent.click(
      c.getByRole("button", { name: "Toggle workspace files" }),
    );
    const rail = within(
      await c.findByRole("complementary", { name: "Workspace files" }),
    );
    await userEvent.click(rail.getByRole("tab", { name: "Changes" }));
    await userEvent.click(
      await rail.findByRole("button", { name: "example.ts +1 -1" }),
    );
    const inspector = await c.findByRole("complementary", {
      name: "Workspace file",
    });
    const headerElement = await waitFor(() => {
      const element = inspector.querySelector<HTMLElement>(
        '[data-ui-part="sidebar-header"]',
      );
      expect(element).not.toBeNull();
      return element!;
    });
    const header = within(headerElement);
    await waitFor(async () => {
      await expect(
        await header.findByRole("tab", { name: "Diff" }),
      ).toBeVisible();
    });
    await expect(header.getByRole("tab", { name: "Before" })).toBeVisible();
    const split = header.getByRole("radio", { name: "Show split diff" });
    await expect(split).toBeVisible();
    await expect(split).toHaveTextContent("");
    await expect(split.querySelector("svg")).not.toBeNull();
    await expect(
      header.getByText("src/example.ts").parentElement!.querySelector("svg"),
    ).not.toBeNull();
    await expect(
      rail.getByRole("button", { name: "example.ts +1 -1" }),
    ).toHaveTextContent("+1");
    await expect(
      rail.getByRole("button", { name: "example.ts +1 -1" }),
    ).toHaveTextContent("-1");
    await browser.viewport(1800, 988);
    try {
      const resize = await c.findByRole("slider", {
        name: "Resize workspace file",
      });
      resize.focus();
      await userEvent.keyboard("{Home}");
      const more = await header.findByRole("button", {
        name: "More file presentation",
      });
      await userEvent.click(more);
      const menu = within(document.body);
      for (const name of ["Diff", "Before", "After"])
        await waitFor(async () => {
          await expect(
            await menu.findByRole("menuitem", { name }),
          ).toBeVisible();
        });
      await userEvent.click(menu.getByRole("menuitem", { name: "Before" }));
      await expect(
        await within(inspector).findByRole("textbox", {
          name: "src/example.ts",
        }),
      ).toHaveTextContent("Hello");
      await expect(
        header.queryByRole("radio", { name: "Show split diff" }),
      ).not.toBeInTheDocument();
      resize.focus();
      await userEvent.keyboard("{End}");
      await expect(
        await header.findByRole("tab", { name: "Before" }),
      ).toHaveAttribute("aria-selected", "true");
      await userEvent.click(header.getByRole("tab", { name: "Diff" }));
      await waitFor(async () => {
        await expect(
          await header.findByRole("radio", { name: "Show split diff" }),
        ).toBeVisible();
      });
      await expect(headerElement.scrollWidth).toBeLessThanOrEqual(
        headerElement.clientWidth + 1,
      );
    } finally {
      await browser.viewport(1280, 800);
    }
  }}
/>

<Story
  name="Files and Changes retain compact root icons and nested indentation"
  args={{ rootChanges: true }}
  play={async ({ canvasElement }) => {
    const c = within(canvasElement);
    await c.findByRole("button", { name: "Toggle workspace files" });
    await waitFor(() =>
      expect(
        c.getByRole("button", {
          name: "Toggle workspace files",
        }),
      ).toBeEnabled(),
    );
    await userEvent.click(
      c.getByRole("button", { name: "Toggle workspace files" }),
    );
    const rail = within(
      await c.findByRole("complementary", { name: "Workspace files" }),
    );
    for (const tab of ["Files", "Changes"]) {
      await userEvent.click(rail.getByRole("tab", { name: tab }));
      for (const name of ["Show files as folders", "Show files as packages"]) {
        await userEvent.click(rail.getByRole("radio", { name }));
        await waitFor(() => {
          const root = rail.getByRole("button", {
            name: `README.md ${tab === "Files" ? "Repository file" : "+1 -1"}`,
          });
          const nested = rail.getByRole("button", {
            name: `example.ts ${tab === "Files" ? "Repository file" : "+1 -1"}`,
          });
          expect(nested).toHaveAttribute("data-file-path", "src/example.ts");
          const inset = (row: HTMLElement) =>
            row
              .querySelector(".ui-diff-file-listing__icon")!
              .getBoundingClientRect().left - row.getBoundingClientRect().left;
          expect(inset(root)).toBeCloseTo(8, 1);
          expect(inset(nested)).toBeCloseTo(40, 1);
        });
      }
    }
  }}
/>

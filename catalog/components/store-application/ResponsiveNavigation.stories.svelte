<script module lang="ts">
  import { defineMeta } from "@storybook/addon-svelte-csf";
  import { expect, userEvent, waitFor, within } from "storybook/test";
  import Fixture from "../../fixtures/ResponsiveNavigationFixture.svelte";
  const { Story } = defineMeta({
    title: "JJ Store/Responsive navigation",
    component: Fixture,
    tags: ["visual-pending"],
  });
</script>

<Story
  name="Overflow restores tabs and preserves the enclosing panel and disabled options"
  play={async ({ canvasElement }) => {
    const c = within(canvasElement),
      body = within(document.body);
    const more = await c.findByRole("button", { name: "More fixture views" });
    await userEvent.click(more);
    await expect(
      body.getByRole("menuitem", { name: "Agents" }),
    ).toHaveAttribute("aria-disabled", "true");
    await userEvent.click(body.getByRole("menuitem", { name: "Repositories" }));
    await expect(c.getByRole("tabpanel")).toHaveTextContent(
      "Selected repositories",
    );
    await expect(more).toHaveAttribute("aria-current", "page");
    await userEvent.click(c.getByRole("button", { name: "Widen toolbar" }));
    const tab = await c.findByRole("tab", { name: "Repositories" });
    await expect(tab).toHaveAttribute("aria-selected", "true");
    await expect(
      c.queryByRole("button", { name: "More fixture views" }),
    ).toBeNull();
    await expect(c.getByRole("tabpanel")).toHaveAttribute(
      "aria-labelledby",
      tab.id,
    );
    await userEvent.click(c.getByRole("button", { name: "Narrow toolbar" }));
    await c.findByRole("button", { name: "More fixture views" });
    const root = canvasElement.querySelector(".store-application__tabs")!;
    await waitFor(() =>
      expect(root.scrollWidth).toBeLessThanOrEqual(root.clientWidth + 1),
    );
  }}
/>
<Story
  name="Expanding search fills the toolbar and closing restores fitted navigation"
  play={async ({ canvasElement }) => {
    const c = within(canvasElement);
    await userEvent.click(c.getByRole("button", { name: "Search fixture" }));
    const input = c.getByRole("textbox", { name: "Search fixture" });
    await expect(input).toHaveFocus();
    await expect(c.queryByRole("tablist")).toBeNull();
    await userEvent.type(input, "retained{Enter}");
    await expect(input).toHaveValue("retained");
    await userEvent.click(c.getByRole("button", { name: "Clear search" }));
    await expect(input).toHaveFocus();
    await userEvent.keyboard("{Escape}");
    await expect(
      c.getByRole("button", { name: "Search fixture" }),
    ).toHaveFocus();
    await c.findByRole("button", { name: "More fixture views" });
  }}
/>

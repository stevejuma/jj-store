<script module lang="ts">
  import { defineMeta } from "@storybook/addon-svelte-csf";
  import { expect, userEvent, within, waitFor } from "storybook/test";
  import { page as browser } from "vitest/browser";
  import Fixture from "../../fixtures/ProjectsHomeSpacingFixture.svelte";
  const { Story } = defineMeta({
    title: "JJ Store/Projects home spacing",
    component: Fixture,
    tags: ["visual-pending"],
    parameters: { layout: "fullscreen" },
  });
</script>

<Story
  name="Project rows remain contiguous across home repositories"
  play={async ({ canvasElement }) => {
    const c = within(canvasElement);
    await c.findByRole(
      "button",
      { name: "Open project Team project 3" },
      { timeout: 10000 },
    );
    const group = canvasElement.querySelector<HTMLElement>(
      ".community-projects__collection-group",
    )!;
    for (const width of [1440, 900, 480]) {
      await browser.viewport(width, 900);
      await waitFor(() => {
        const rows = Array.from(
          group.querySelectorAll<HTMLElement>(
            ".community-projects__project-row",
          ),
        );
        expect(rows).toHaveLength(4);
        for (let index = 1; index < rows.length; index++)
          expect(
            Math.abs(
              rows[index].getBoundingClientRect().top -
                rows[index - 1].getBoundingClientRect().bottom,
            ),
          ).toBeLessThan(1);
      });
    }
    await browser.viewport(1280, 800);
    await userEvent.click(c.getByRole("button", { name: /^Mine$/ }));
    await waitFor(() =>
      expect(
        c.queryByRole("button", { name: "Open project Personal project" }),
      ).not.toBeInTheDocument(),
    );
    await userEvent.click(c.getByRole("button", { name: "Mine 4" }));
    await expect(
      await c.findByRole("button", { name: "Open project Personal project" }),
    ).toBeVisible();
    await userEvent.click(c.getByRole("radio", { name: "Grid layout" }));
    for (const list of group.querySelectorAll<HTMLElement>('[role="list"]')) {
      await expect(list).toHaveAttribute("data-layout", "grid");
      for (const row of list.querySelectorAll<HTMLElement>("article"))
        await expect(row.scrollWidth).toBeLessThanOrEqual(row.clientWidth + 1);
    }
  }}
/>

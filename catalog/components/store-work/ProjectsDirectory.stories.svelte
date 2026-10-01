<script module lang="ts">
  import { defineMeta } from "@storybook/addon-svelte-csf";
  import { expect, userEvent, within, waitFor } from "storybook/test";
  import ProjectsHubFixture from "../../fixtures/ProjectsHubFixture.svelte";
  import { expectReviewRowUsesWidth } from "../../fixtures/projects-hub-layout.js";
  const { Story } = defineMeta({
    title: "JJ Store/Projects directory",
    component: ProjectsHubFixture,
    tags: ["visual-pending"],
    parameters: { layout: "fullscreen" },
  });
</script>

<Story
  name="Repository metadata in list and cards"
  play={async ({ canvasElement }) => {
    const c = within(canvasElement);
    await waitFor(() =>
      expect(c.getByRole("button", { name: "New project" })).toBeVisible(),
    );
    await userEvent.click(c.getByRole("tab", { name: "Repositories" }));
    await waitFor(() =>
      expect(c.getByText("Native parsing and storage services")).toBeVisible(),
    );
    for (const layout of ["List layout", "Grid layout"]) {
      await userEvent.click(c.getByRole("radio", { name: layout }));
      const row = c
        .getByRole("button", { name: "Open repository Parser repository" })
        .closest("article")!;
      const view = within(row);
      await expect(
        view.getByRole("button", { name: "View Maya Chen's profile" }),
      ).toBeVisible();
      await expect(
        view.getByRole("group", { name: "24 commits, 1 reviews, 2 tasks" }),
      ).toBeVisible();
      await expect(row.querySelector("time")).toBeVisible();
      await expect(row.scrollWidth).toBeLessThanOrEqual(row.clientWidth + 1);
    }
  }}
/>

<Story
  name="Review metadata in list and cards"
  play={async ({ canvasElement }) => {
    const c = within(canvasElement);
    await waitFor(() =>
      expect(c.getByRole("button", { name: "New project" })).toBeVisible(),
    );
    await userEvent.click(c.getByRole("tab", { name: "Reviews" }));
    await waitFor(() =>
      expect(
        c.getByText("Preserve the exact operation when this retry races"),
      ).toBeVisible(),
    );
    const row = c
      .getByText("Preserve the exact operation when this retry races")
      .closest("article")!;
    await expect(
      within(row).getByRole("button", {
        name: `Open review ${"e1".repeat(16)}`,
      }),
    ).toHaveAttribute("title", "e1".repeat(16));
    await expect(within(row).getByLabelText("3 comments")).toBeVisible();
    await expectReviewRowUsesWidth(
      row,
      c.getByRole("button", { name: "Parser repository", expanded: true }),
    );
    await userEvent.click(c.getByRole("radio", { name: "Grid layout" }));
    await expect(c.getByRole("list", { name: "Reviews" })).toHaveAttribute(
      "data-layout",
      "grid",
    );
    await expect(row.querySelector("time")).toBeVisible();
    const count = within(row).getByLabelText("3 comments");
    await expect(count.scrollWidth).toBeLessThanOrEqual(count.clientWidth);
    await expect(
      within(row).getByRole("button", { name: /View e2e2/ }),
    ).toBeVisible();
    await expect(row.scrollWidth).toBeLessThanOrEqual(row.clientWidth + 1);
    await userEvent.click(c.getByRole("radio", { name: "List layout" }));
    await expect(c.getByRole("list", { name: "Reviews" })).toHaveAttribute(
      "data-layout",
      "list",
    );
  }}
/>

<Story
  name="Failed repository details retain successful metadata"
  args={{ failedHome: true }}
  play={async ({ canvasElement }) => {
    const c = within(canvasElement);
    await waitFor(() =>
      expect(c.getByRole("button", { name: "New project" })).toBeVisible(),
    );
    await userEvent.click(c.getByRole("tab", { name: "Repositories" }));
    await waitFor(() =>
      expect(c.getByText("Native parsing and storage services")).toBeVisible(),
    );
    await expect(c.getByRole("alert")).toHaveTextContent("Home unavailable");
    for (const layout of ["List layout", "Grid layout"]) {
      await userEvent.click(c.getByRole("radio", { name: layout }));
      const target = c.getByRole("button", {
        name: "Open repository UI repository",
      });
      await expect(target).toBeVisible();
      await expect(target.getBoundingClientRect().width).toBeGreaterThan(64);
    }
    await expect(
      c.getByRole("group", { name: /Loaded activity: 24 commits/ }),
    ).toBeVisible();
  }}
/>

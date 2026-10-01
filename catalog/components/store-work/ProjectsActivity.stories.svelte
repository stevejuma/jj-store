<script module lang="ts">
  import { defineMeta } from "@storybook/addon-svelte-csf";
  import { expect, userEvent, within, waitFor } from "storybook/test";
  import ProjectsHubFixture from "../../fixtures/ProjectsHubFixture.svelte";
  const { Story } = defineMeta({
    title: "JJ Store/Projects activity",
    component: ProjectsHubFixture,
    tags: ["visual-pending"],
    parameters: { layout: "fullscreen" },
  });
</script>

<Story
  name="Inset skeletons across all main tabs"
  args={{ loading: true }}
  play={async ({ canvasElement }) => {
    const c = within(canvasElement);
    for (const name of [
      "Projects",
      "Tasks",
      "Reviews",
      "Repositories",
      "Activity",
    ]) {
      await userEvent.click(c.getByRole("tab", { name }));
      await waitFor(() =>
        expect(
          canvasElement.querySelector(
            '.store-projects__loading [role="status"]',
          ),
        ).toBeVisible(),
      );
      for (const loading of canvasElement.querySelectorAll<HTMLElement>(
        ".store-projects__loading",
      )) {
        const style = getComputedStyle(loading);
        await expect(
          Number.parseFloat(style.paddingLeft),
        ).toBeGreaterThanOrEqual(16);
        await expect(
          Number.parseFloat(style.paddingTop),
        ).toBeGreaterThanOrEqual(16);
        await expect(
          Number.parseFloat(style.paddingRight),
        ).toBeGreaterThanOrEqual(16);
        await expect(
          Number.parseFloat(style.paddingBottom),
        ).toBeGreaterThanOrEqual(16);
      }
    }
  }}
/>

<Story
  name="Shared repository activity and agent context"
  play={async ({ canvasElement }) => {
    const c = within(canvasElement);
    await userEvent.click(await c.findByRole("tab", { name: "Activity" }));
    await expect(
      await c.findByRole("list", { name: "Repository activity" }),
    ).toBeVisible();
    await userEvent.click(c.getByRole("button", { name: "Agent context" }));
    await expect(
      c.getByRole("log", { name: "Agent conversation activity" }),
    ).toHaveTextContent("Updated the parser");
    const toggle = c.getByRole("button", { name: "Toggle details" });
    if (toggle.getAttribute("aria-expanded") === "false")
      await userEvent.click(toggle);
    const rail = within(c.getByRole("region", { name: "Project filters" }));
    await userEvent.type(
      rail.getByRole("textbox", { name: "Search activity" }),
      "no matching revision",
    );
    await userEvent.click(rail.getByRole("button", { name: /^Search$/ }));
    await expect(await c.findByText("No matching activity")).toBeVisible();
  }}
/>

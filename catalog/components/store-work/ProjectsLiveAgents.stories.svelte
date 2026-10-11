<script module lang="ts">
  import { defineMeta } from "@storybook/addon-svelte-csf";
  import { expect, userEvent, within } from "storybook/test";
  import Fixture from "../../fixtures/ProjectsLiveAgentsFixture.svelte";
  const { Story } = defineMeta({
    title: "JJ Store/Projects interactive sessions",
    component: Fixture,
    tags: ["visual-pending"],
    parameters: { layout: "fullscreen" },
  });
</script>

<Story
  name="Interactive sessions use repository groups and avatar rows"
  play={async ({ canvasElement }) => {
    const c = within(canvasElement);
    const parser = await c.findByRole(
      "region",
      { name: "Agents in Parser repository" },
      { timeout: 10000 },
    );
    const ui = await c.findByRole("region", {
      name: "Agents in UI repository",
    });
    await expect(
      c.queryByRole("region", { name: "Interactive agent sessions" }),
    ).not.toBeInTheDocument();
    for (const [region, title] of [
      [parser, "Bound repository chat"],
      [parser, "Shared repository chat"],
      [ui, "Legacy repository chat"],
      [ui, "Shared repository chat"],
    ] as const) {
      const local = within(region);
      const button = await local.findByRole("button", {
        name: new RegExp(`Open ${title} .*interactive agent session`),
      });
      const row = button.closest("[data-kind=agent]")!;
      await expect(
        within(row as HTMLElement).queryByText("Online"),
      ).not.toBeInTheDocument();
      await expect(
        row.querySelector('[data-ui-component="community-member-avatar"]'),
      ).toBeVisible();
      await expect(
        row.querySelector('[data-ui-component="community-member-avatar"] img'),
      ).toHaveAttribute("src", "/agent-icons/codex.svg");
    }
    await expect(
      within(ui).queryByText("Bound repository chat"),
    ).not.toBeInTheDocument();
    await expect(
      c.queryByText("Inaccessible repository chat"),
    ).not.toBeInTheDocument();
    await expect(
      within(parser).getByRole("button", { name: /Open Codex ended/ }),
    ).toBeVisible();
    await userEvent.click(
      within(parser).getByRole("button", {
        name: /Open Bound repository chat .*interactive agent session/,
      }),
    );
    await expect(
      c.getByLabelText("Selected interactive session"),
    ).toHaveTextContent(`session:${"a1".repeat(16)}`);
  }}
/>

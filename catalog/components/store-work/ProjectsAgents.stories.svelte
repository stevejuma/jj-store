<script module lang="ts">
  import { defineMeta } from "@storybook/addon-svelte-csf";
  import { expect, userEvent, within } from "storybook/test";
  import ProjectsHubFixture from "../../fixtures/ProjectsHubFixture.svelte";
  const { Story } = defineMeta({
    title: "JJ Store/Projects agents",
    component: ProjectsHubFixture,
    tags: ["visual-pending"],
    parameters: { layout: "fullscreen" },
  });
</script>

<Story
  name="Landing agents and native session navigation"
  args={{ agents: true }}
  play={async ({ canvasElement }) => {
    const c = within(canvasElement);
    await c.findByRole("button", { name: /Open Codex ended/ });
    await expect(c.getByRole("tab", { name: "Agents" })).toHaveAttribute(
      "aria-selected",
      "true",
    );
    await expect(
      c.getByRole("region", { name: "Agents in UI repository" }),
    ).toBeVisible();
    const row = canvasElement.querySelector<HTMLElement>(
      '.store-collaboration__row[data-kind="agent"]',
    )!;
    const heading = row
      .closest("section")!
      .querySelector<HTMLElement>(".community-projects__list-group-heading")!;
    await expect(getComputedStyle(row).borderBottomWidth).toBe("0px");
    await expect(
      Math.abs(
        row.getBoundingClientRect().left - heading.getBoundingClientRect().left,
      ),
    ).toBeLessThan(1);
    await expect(
      Math.abs(
        row.getBoundingClientRect().right -
          heading.getBoundingClientRect().right,
      ),
    ).toBeLessThan(1);
    await userEvent.click(c.getByRole("button", { name: /Open Codex ended/ }));
    await expect(
      c.getByRole("region", { name: "Projects and tasks" }),
    ).toBeVisible();
    await expect(row.isConnected).toBe(true);
    await expect(
      c.getByRole("complementary", { name: "Agent response" }),
    ).toBeVisible();
    await c.findByRole("log", { name: "Agent conversation activity" });
    await userEvent.click(
      c.getByRole("button", { name: "Close agent response" }),
    );
    await expect(
      c.queryByRole("complementary", { name: "Agent response" }),
    ).toBeNull();
    await expect(row.isConnected).toBe(true);
  }}
/>
<Story
  name="Selected project agents"
  args={{ projectDetail: true }}
  play={async ({ canvasElement }) => {
    const c = within(canvasElement);
    await c.findByRole("button", { name: "Edit project" });
    await userEvent.click(c.getByRole("tab", { name: "Agents" }));
    await c.findByRole("button", { name: /Open Codex ended/ });
    await expect(
      c.getByRole("button", { name: /Open OpenCode active/ }),
    ).toBeVisible();
  }}
/>
<Story
  name="Agent loading uses a skeleton"
  args={{ agents: true, loading: true }}
  play={async ({ canvasElement }) => {
    const c = within(canvasElement);
    await expect(await c.findByLabelText("Loading agents")).toBeVisible();
    await expect(c.queryByText("No agent sessions")).toBeNull();
  }}
/>

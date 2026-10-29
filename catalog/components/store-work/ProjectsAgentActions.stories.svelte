<script module lang="ts">
  import { defineMeta } from "@storybook/addon-svelte-csf";
  import { expect, userEvent, within, waitFor } from "storybook/test";
  import Fixture from "../../fixtures/ProjectsAgentActionsFixture.svelte";
  const { Story } = defineMeta({
    title: "JJ Store/Projects agent actions",
    component: Fixture,
    tags: ["visual-pending", "application"],
    parameters: { jjStoreStory: { kind: "application" }, layout: "fullscreen" },
  });
</script>

<Story
  name="Every Projects tab uses a single icon launcher and Agents manages in its title row"
  play={async ({ canvasElement }) => {
    const c = within(canvasElement),
      portal = within(document.body);
    for (const tab of [
      "Agents",
      "Activity",
      "Projects",
      "Tasks",
      "Reviews",
      "Repositories",
    ]) {
      await userEvent.click(await c.findByRole("tab", { name: tab }));
      const launch = await c.findByRole("button", {
        name: "New agent session",
      });
      await expect(
        c.getAllByRole("button", { name: "New agent session" }),
      ).toHaveLength(1);
      await expect(launch.textContent?.trim()).toBe("");
      await expect(
        launch.closest(".community-projects__surface-header"),
      ).not.toBeNull();
      launch.focus();
      await expect(await portal.findByRole("tooltip")).toHaveTextContent(
        "New agent session",
      );
      await userEvent.keyboard("{Escape}");
      if (tab === "Agents") {
        const manage = c.getByRole("button", { name: "Manage agents" });
        await expect(manage.textContent?.trim()).toBe("");
        await expect(
          manage.closest(".community-projects__surface-header"),
        ).not.toBeNull();
        await expect(
          c.queryByRole("region", { name: "Interactive agent sessions" }),
        ).toBeNull();
      } else
        await expect(
          c.queryByRole("button", { name: "Manage agents" }),
        ).toBeNull();
    }
  }}
/>

<Story
  name="Project launcher retains the selected project context"
  args={{ projectDetail: true }}
  play={async ({ canvasElement }) => {
    const c = within(canvasElement);
    const launch = await c.findByRole("button", { name: "New agent session" });
    await expect(launch.textContent?.trim()).toBe("");
    await userEvent.click(launch);
    await waitFor(() =>
      expect(c.getByLabelText("Launch context")).toHaveTextContent(
        "Project ID:",
      ),
    );
  }}
/>

<Story
  name="Task detail keeps an icon launcher and exact task context"
  args={{ detail: true }}
  play={async ({ canvasElement }) => {
    const c = within(canvasElement);
    await c.findByRole("article", { name: "Task details" });
    const launch = await c.findByRole("button", { name: "New agent session" });
    await expect(
      c.getAllByRole("button", { name: "New agent session" }),
    ).toHaveLength(1);
    await expect(launch.textContent?.trim()).toBe("");
    await expect(launch.closest("header")).not.toBeNull();
    await userEvent.click(launch);
    await waitFor(() =>
      expect(c.getByLabelText("Launch context")).toHaveTextContent("Task ID:"),
    );
  }}
/>

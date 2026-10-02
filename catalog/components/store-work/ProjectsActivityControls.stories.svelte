<script module lang="ts">
  import { defineMeta } from "@storybook/addon-svelte-csf";
  import { expect, userEvent, within, waitFor } from "storybook/test";
  import Fixture from "../../fixtures/ProjectsHubFixture.svelte";
  const { Story } = defineMeta({
    title: "JJ Store/Projects activity controls",
    component: Fixture,
    tags: ["visual-pending"],
    parameters: { layout: "fullscreen" },
  });
  async function verifyFilter(canvasElement: HTMLElement) {
    const c = within(canvasElement),
      body = within(document.body);
    await userEvent.click(c.getByRole("tab", { name: "Activity" }));
    await waitFor(() =>
      expect(c.getByRole("tab", { name: "Activity" })).toHaveAttribute(
        "aria-selected",
        "true",
      ),
    );
    const filter = await c.findByRole("button", { name: "Filter activity" });
    await expect(filter.closest(".community-projects__tabs")).not.toBeNull();
    await userEvent.click(filter);
    await userEvent.type(
      body.getByPlaceholderText("Find an activity type..."),
      "Without",
    );
    await userEvent.click(
      body.getByRole("option", { name: /Without agent context/ }),
    );
    await userEvent.click(
      body.getByRole("button", { name: "Close activity filter" }),
    );
    await waitFor(() =>
      expect(c.getByText("No matching activity")).toBeVisible(),
    );
    await expect(filter).toHaveAttribute("aria-pressed", "true");
    await userEvent.click(filter);
    await userEvent.click(
      body.getByRole("option", { name: /With agent context/ }),
    );
    await userEvent.click(
      body.getByRole("button", { name: "Close activity filter" }),
    );
    await waitFor(() =>
      expect(
        c.getByRole("list", { name: "Repository activity" }),
      ).toBeVisible(),
    );
    await expect(
      c.getAllByRole("button", { name: "Agent context" }).length,
    ).toBeGreaterThan(0);
    await userEvent.click(filter);
    await userEvent.click(body.getByRole("option", { name: /All commits/ }));
    await userEvent.click(
      body.getByRole("button", { name: "Close activity filter" }),
    );
    await expect(filter).toHaveAttribute("aria-pressed", "false");
  }
</script>

<Story
  name="Hub activity filters recorded native context"
  play={async ({ canvasElement }) => {
    const c = within(canvasElement);
    await waitFor(() =>
      expect(
        c.getByRole("button", { name: "Open project Native release" }),
      ).toBeVisible(),
    );
    await expect(c.getAllByRole("tab")[0]).toHaveTextContent("Activity");
    await verifyFilter(canvasElement);
  }}
/>
<Story
  name="Project activity starts before tasks and filters linked repositories"
  play={async ({ canvasElement }) => {
    const c = within(canvasElement);
    await waitFor(() =>
      expect(
        c.getByRole("button", { name: "Open project Native release" }),
      ).toBeVisible(),
    );
    await userEvent.click(
      c.getByRole("button", { name: "Open project Native release" }),
    );
    await waitFor(() =>
      expect(c.getByRole("tab", { name: "Tasks" })).toBeVisible(),
    );
    await expect(c.getAllByRole("tab")[0]).toHaveTextContent("Activity");
    await verifyFilter(canvasElement);
  }}
/>
<Story
  name="Repository projects retain repository tabs and reuse Community rows"
  args={{ repositoryProjects: true }}
  play={async ({ canvasElement }) => {
    const c = within(canvasElement);
    await waitFor(() =>
      expect(
        c.getByRole("region", { name: "Linked repository projects" }),
      ).toBeVisible(),
    );
    await expect(c.getByRole("tab", { name: "Projects" })).toHaveAttribute(
      "aria-selected",
      "true",
    );
    await expect(c.getByRole("tab", { name: "Overview" })).toBeVisible();
    await expect(c.queryByRole("tab", { name: "Activity" })).toBeNull();
    await expect(
      c.getByRole("button", { name: "Open project Native release" }),
    ).toBeVisible();
    await userEvent.click(
      c.getByRole("button", { name: "Open project Native release" }),
    );
    await expect(c.getByRole("tab", { name: "Activity" })).toBeVisible();
  }}
/>

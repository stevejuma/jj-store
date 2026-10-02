<script module lang="ts">
  import { defineMeta } from "@storybook/addon-svelte-csf";
  import { expect, userEvent, within, waitFor } from "storybook/test";
  import Fixture from "../../fixtures/ProjectsHubFixture.svelte";
  const { Story } = defineMeta({
    title: "JJ Store/Projects header filters",
    component: Fixture,
    tags: ["visual-pending"],
    parameters: { layout: "fullscreen" },
  });
  async function verifyHeaders(canvasElement: HTMLElement) {
    const c = within(canvasElement),
      body = within(document.body);
    await c.findByRole("tab", { name: "Projects" });
    for (const name of [
      "Projects",
      "Tasks",
      "Reviews",
      "Agents",
      "Repositories",
      "Activity",
    ]) {
      const tab = name.toLowerCase();
      await userEvent.click(c.getByRole("tab", { name }));
      if (name !== "Repositories") {
        const trigger = await c.findByRole("button", {
          name: `Filters for ${tab}`,
        });
        await expect(
          trigger.closest(".community-projects__tabs"),
        ).not.toBeNull();
        await expect(
          trigger.closest(".community-projects__surface-header"),
        ).toBeNull();
        await userEvent.click(trigger);
        const panel = await body.findByRole("dialog", {
          name: `${name} filters`,
        });
        await expect(
          within(panel).queryByRole("textbox", { name: `Search ${tab}` }),
        ).toBeNull();
        await userEvent.keyboard("{Escape}");
        await waitFor(() => expect(trigger).toHaveFocus());
      }
      if (name === "Reviews") {
        await expect(
          c.queryByRole("button", { name: "Search reviews" }),
        ).toBeNull();
        continue;
      }
      const trigger = c.getByRole("button", { name: `Search ${tab}` });
      await expect(trigger.closest(".community-projects__tabs")).not.toBeNull();
      await userEvent.click(trigger);
      const input = c.getByRole("textbox", { name: `Search ${tab}` });
      await waitFor(() => expect(input).toHaveFocus());
      await expect(c.queryByRole("tablist")).toBeNull();
      const group = input.closest('[data-ui-part="input-group"]')!;
      await expect(getComputedStyle(group).borderTopWidth).toBe("0px");
      await expect(getComputedStyle(group).boxShadow).toBe("none");
      await expect(getComputedStyle(input).boxShadow).toBe("none");
      await userEvent.type(input, "draft");
      await waitFor(() =>
        expect(c.getByRole("button", { name: "Close search" })).toBeVisible(),
      );
      if (name !== "Repositories") {
        await userEvent.click(
          c.getByRole("button", { name: `Filters for ${tab}` }),
        );
        await userEvent.click(
          body.getByRole("button", { name: "Close filters" }),
        );
        await expect(input).toHaveValue("draft");
      }
      await userEvent.click(c.getByRole("button", { name: "Close search" }));
      await waitFor(() =>
        expect(c.getByRole("button", { name: `Search ${tab}` })).toHaveFocus(),
      );
      await expect(c.getByRole("tablist")).toBeVisible();
    }
    await expect(
      c.getByRole("button", { name: "Filter activity" }),
    ).toBeVisible();
  }
</script>

<Story
  name="Header filters without the details rail"
  play={async ({ canvasElement }) => verifyHeaders(canvasElement)}
/>
<Story
  name="Mobile header filters"
  args={{ mobile: true }}
  play={async ({ canvasElement }) => verifyHeaders(canvasElement)}
/>

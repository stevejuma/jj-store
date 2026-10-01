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
    await waitFor(() =>
      expect(c.getByRole("tab", { name: "Projects" })).toBeVisible(),
    );
    for (const name of [
      "Projects",
      "Tasks",
      "Reviews",
      "Repositories",
      "Activity",
    ]) {
      await userEvent.click(c.getByRole("tab", { name }));
      const trigger = await c.findByRole("button", {
        name: `Filters for ${name.toLowerCase()}`,
      });
      await expect(
        trigger.closest(".community-projects__surface-header"),
      ).not.toBeNull();
      const header = trigger.closest<HTMLElement>(
        ".community-projects__surface-header",
      )!;
      await expect(
        header.firstElementChild!.getBoundingClientRect().left -
          header.getBoundingClientRect().left,
      ).toBeLessThanOrEqual(8);
      const tabs = canvasElement.querySelector<HTMLElement>(
        ".community-projects__tabs",
      )!;
      await expect(
        Number.parseFloat(getComputedStyle(tabs).paddingInlineStart),
      ).toBeLessThanOrEqual(8);
      await expect(trigger.querySelector("svg")).not.toBeNull();
      await expect(trigger.textContent?.trim()).toBe("");
      await expect(
        c.queryByRole("region", { name: "Project filters" }),
      ).toBeNull();
      await userEvent.click(trigger);
      const panel = await body.findByRole("dialog", {
        name: `${name} filters`,
      });
      await expect(panel).toBeVisible();
      if (name !== "Reviews") {
        const search = within(panel).getByRole("textbox", {
          name: `Search ${name.toLowerCase()}`,
        });
        await userEvent.type(search, "draft");
      }
      await userEvent.keyboard("{Escape}");
      await waitFor(() => expect(trigger).toHaveFocus());
      await waitFor(() =>
        expect(
          body.queryByRole("dialog", { name: `${name} filters` }),
        ).toBeNull(),
      );
      await userEvent.click(trigger);
      if (name !== "Reviews")
        await expect(
          body.getByRole("textbox", { name: `Search ${name.toLowerCase()}` }),
        ).toHaveValue("draft");
      await userEvent.click(
        body.getByRole("button", { name: "Close filters" }),
      );
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

<script module lang="ts">
  import { selectViewTab } from "../../fixtures/view-tabs.js";
  import { defineMeta } from "@storybook/addon-svelte-csf";
  import { expect, userEvent, within, waitFor } from "storybook/test";
  import Fixture from "../../fixtures/ProjectsHubFixture.svelte";
  const { Story } = defineMeta({
    title: "JJ Store/Projects header filters",
    component: Fixture,
    tags: ["visual-pending", "application"],
    parameters: { jjStoreStory: { kind: "application" }, layout: "fullscreen" },
  });
  async function expectCircle(button: HTMLElement) {
    const bounds = button.getBoundingClientRect();
    await expect(bounds.width).toBeGreaterThan(0);
    await expect(bounds.width).toBeCloseTo(bounds.height, 1);
    await expect(
      parseFloat(getComputedStyle(button).borderTopLeftRadius),
    ).toBeGreaterThanOrEqual(bounds.width / 2);
    await expect(getComputedStyle(button).backgroundColor).not.toBe(
      "rgba(0, 0, 0, 0)",
    );
  }
  async function verifyHeaders(canvasElement: HTMLElement) {
    const c = within(canvasElement),
      body = within(document.body);
    await selectViewTab(canvasElement, "Projects");
    for (const name of [
      "Projects",
      "Tasks",
      "Reviews",
      "Agents",
      "Repositories",
      "Activity",
    ]) {
      const tab = name.toLowerCase();
      await selectViewTab(canvasElement, name);
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
      const closedBounds = trigger.getBoundingClientRect();
      await userEvent.click(trigger);
      const input = c.getByRole("textbox", { name: `Search ${tab}` });
      await waitFor(() => expect(input).toHaveFocus());
      await expect(c.queryByRole("tablist")).toBeNull();
      const group = input.closest('[data-ui-part="input-group"]')!;
      await expect(getComputedStyle(group).borderTopWidth).toBe("0px");
      await expect(getComputedStyle(group).boxShadow).toBe("none");
      await expect(getComputedStyle(input).boxShadow).toBe("none");
      const close = c.getByRole("button", { name: "Close search" });
      await expect(close).toHaveAttribute("data-variant", "outline");
      await expect(getComputedStyle(close).borderTopWidth).toBe("1px");
      const openBounds = close.getBoundingClientRect();
      for (const axis of ["x", "y", "width", "height"] as const)
        await expect(
          openBounds[axis],
          JSON.stringify({
            axis,
            closed: closedBounds[axis],
            open: openBounds[axis],
            buttonWidth: getComputedStyle(close).width,
            shrink: getComputedStyle(close).flexShrink,
            spacing: getComputedStyle(close).getPropertyValue("--spacing"),
            groupWidth: getComputedStyle(group).width,
          }),
        ).toBeCloseTo(closedBounds[axis], 1);
      await expectCircle(close);
      await expect(
        c.queryByRole("button", { name: "Clear search" }),
      ).toBeNull();
      const searchGlyph = close.querySelector(
        ".store-toolbar-search__search-icon",
      )!;
      const closeGlyph = close.querySelector(
        ".store-toolbar-search__close-icon",
      )!;
      await userEvent.unhover(close);
      await expect(searchGlyph).toBeVisible();
      await expect(closeGlyph).not.toBeVisible();
      await userEvent.hover(close);
      await expect(closeGlyph).toBeVisible();
      await expect(searchGlyph).not.toBeVisible();
      await userEvent.unhover(close);
      await expect(searchGlyph).toBeVisible();
      await userEvent.keyboard("{Shift>}{Tab}{/Shift}");
      await expect(close).toHaveFocus();
      await expect(closeGlyph).toBeVisible();
      await userEvent.click(input);
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
      const clear = c.getByRole("button", { name: "Clear search" });
      await expectCircle(clear);
      await expect(clear.getBoundingClientRect().width).toBe(24);
      await expect(close.getBoundingClientRect().width).toBe(32);
      await userEvent.click(clear);
      await expect(input).toHaveValue("");
      await expect(input).toHaveFocus();
      await expect(c.queryByRole("tablist")).toBeNull();
      await expect(
        c.queryByRole("button", { name: "Clear search" }),
      ).toBeNull();
      await userEvent.type(input, "submitted{Enter}");
      await expect(input).toHaveValue("submitted");
      await userEvent.click(c.getByRole("button", { name: "Clear search" }));
      await waitFor(() => expect(input).toHaveFocus());
      await expect(input).toHaveValue("");
      await expect(c.queryByRole("tablist")).toBeNull();
      await expect(
        c.queryByRole("button", { name: "Clear search" }),
      ).toBeNull();
      await userEvent.click(c.getByRole("button", { name: "Close search" }));
      await waitFor(() =>
        expect(c.getByRole("button", { name: `Search ${tab}` })).toHaveFocus(),
      );
      await waitFor(() =>
        expect(
          c.queryByRole("tablist") ??
            c.queryByRole("button", { name: /^More projects views$/i }),
        ).toBeVisible(),
      );
      await userEvent.click(c.getByRole("button", { name: `Search ${tab}` }));
      await userEvent.keyboard("{Escape}");
      await waitFor(() =>
        expect(c.getByRole("button", { name: `Search ${tab}` })).toHaveFocus(),
      );
    }
    await expect(
      c.getByRole("button", { name: "Filters for activity" }),
    ).toBeVisible();
  }
</script>

<Story
  name="Header filters without the details rail"
  play={async ({ canvasElement }) => verifyHeaders(canvasElement)}
/>
<Story
  parameters={{ viewport: { defaultViewport: "mobile1" } }}
  name="Mobile header filters"
  args={{ mobile: true }}
  play={async ({ canvasElement }) => verifyHeaders(canvasElement)}
/>

<script module lang="ts">
  import { defineMeta } from "@storybook/addon-svelte-csf";
  import { expect, userEvent, waitFor, within } from "storybook/test";
  import Fixture from "../../fixtures/StoreThreadsFixture.svelte";
  const { Story } = defineMeta({
    title: "JJ Store/Thread directory search",
    component: Fixture,
    tags: ["visual-pending"],
    args: { searchable: true },
    parameters: { layout: "fullscreen" },
  });
  async function search({ canvasElement }: { canvasElement: HTMLElement }) {
    const c = within(canvasElement);
    await c.findByRole("button", { name: "Open thread general" });
    await expect(
      getComputedStyle(
        canvasElement.querySelector('[aria-label="Thread toolbar"]')!,
      ).borderBottomWidth,
    ).toBe("0px");
    const trigger = c.getByRole("button", { name: "Search threads" });
    const bounds = trigger.getBoundingClientRect();
    await userEvent.click(trigger);
    const input = c.getByRole("textbox", { name: "Search threads" });
    await waitFor(() => expect(input).toHaveFocus());
    await expect(c.queryByRole("heading", { name: "Threads" })).toBeNull();
    const close = c.getByRole("button", { name: "Close search" });
    for (const axis of ["x", "y", "width", "height"] as const)
      await expect(close.getBoundingClientRect()[axis]).toBeCloseTo(
        bounds[axis],
        1,
      );
    await expect(bounds.width).toBe(32);
    await expect(bounds.height).toBe(32);
    await expect(getComputedStyle(input).boxShadow).toBe("none");
    await expect(
      getComputedStyle(input.closest('[data-ui-part="input-group"]')!)
        .borderTopWidth,
    ).toBe("0px");
    await expect(c.queryByRole("button", { name: "Clear search" })).toBeNull();
    await userEvent.type(input, "GENERAL{Enter}");
    await expect(
      c.getByRole("button", { name: "Open thread general" }),
    ).toBeVisible();
    await expect(
      c.queryByRole("button", { name: "Open thread planning" }),
    ).toBeNull();
    await userEvent.click(c.getByRole("button", { name: "Clear search" }));
    await expect(input).toHaveValue("");
    await expect(input).toHaveFocus();
    await expect(
      c.getByRole("button", { name: "Open thread planning" }),
    ).toBeVisible();
    await userEvent.type(input, "release checklist{Enter}");
    await expect(
      c.getByRole("button", { name: "Open thread planning" }),
    ).toBeVisible();
    await expect(
      c.queryByRole("button", { name: "Open thread general" }),
    ).toBeNull();
    await userEvent.click(
      c.getByRole("button", { name: "Open thread planning" }),
    );
    await expect(
      await c.findByText("Release planning discussion"),
    ).toBeVisible();
    await expect(
      c.queryByRole("button", { name: "Search threads" }),
    ).toBeNull();
    await userEvent.click(c.getByRole("button", { name: "All threads" }));
    const restored = await c.findByRole("textbox", { name: "Search threads" });
    await expect(restored).toHaveValue("release checklist");
    await userEvent.click(c.getByRole("button", { name: "Clear search" }));
    await userEvent.type(restored, "no-match{Enter}");
    await expect(c.getByText("No matching threads")).toBeVisible();
    await expect(c.queryByText("No threads yet")).toBeNull();
    await userEvent.keyboard("{Escape}");
    await waitFor(() =>
      expect(c.getByRole("button", { name: "Search threads" })).toHaveFocus(),
    );
    await expect(
      c.getByRole("button", { name: "Open thread general" }),
    ).toBeVisible();
  }
</script>

<Story
  name="Search thread titles and previews with shared toolbar controls"
  play={search}
/>
<Story
  name="Mobile thread search keeps toolbar geometry and keyboard behavior"
  args={{ displayMode: "mobile" }}
  play={search}
/>
<Story
  name="Filtered pages remain available and repository changes clear search"
  args={{ pagedSearch: true, otherRepository: true }}
  play={async ({ canvasElement }) => {
    const c = within(canvasElement);
    await userEvent.click(
      await c.findByRole("button", { name: "Search threads" }),
    );
    const input = c.getByRole("textbox", { name: "Search threads" });
    await userEvent.type(input, "release checklist{Enter}");
    await expect(c.getByText("No matching threads")).toBeVisible();
    await userEvent.click(c.getByRole("button", { name: "More threads" }));
    await expect(
      await c.findByRole("button", { name: "Open thread planning" }),
    ).toBeVisible();
    await expect(input).toHaveValue("release checklist");
    const navigation = c.getByRole("navigation", {
      name: "Repository navigation",
    });
    await userEvent.click(
      within(navigation).getByRole("button", {
        name: "Expand Other repository",
      }),
    );
    const owner = within(navigation)
      .getByRole("button", { name: "Other repository" })
      .closest(".lc-community-navigation__item-tree")!;
    await userEvent.click(
      within(owner as HTMLElement).getByRole("button", {
        name: "Browse threads",
      }),
    );
    await expect(
      await c.findByRole("button", { name: "Open thread general" }),
    ).toBeVisible();
    await expect(
      c.queryByRole("textbox", { name: "Search threads" }),
    ).toBeNull();
    await expect(
      c.getByRole("button", { name: "Search threads" }),
    ).toBeVisible();
  }}
/>

<script module lang="ts">
  import { defineMeta } from "@storybook/addon-svelte-csf";
  import { expect, userEvent, within } from "storybook/test";
  import StoreWorkLayoutControls from "../../../web/src/lib/ui/store-work/StoreWorkLayoutControls.svelte";
  const { Story } = defineMeta({
    parameters: {
      jjStoreStory: {
        kind: "component",
        subject: "web/src/lib/ui/store-work/StoreWorkLayoutControls.svelte",
      },
    },
    title: "JJ Store/Layout selection",
    component: StoreWorkLayoutControls,
    tags: ["visual-pending", "component"],
  });
  async function selection(canvasElement: HTMLElement) {
    const c = within(canvasElement),
      grid = c.getByRole("radio", { name: "Grid layout" }),
      list = c.getByRole("radio", { name: "List layout" });
    for (const target of [list, grid, list, grid]) {
      const other = target === list ? grid : list;
      await userEvent.click(target);
      await expect(target).toBeChecked();
      await expect(other).not.toBeChecked();
      await userEvent.click(target);
      await expect(target).toBeChecked();
      await expect(other).not.toBeChecked();
    }
    await grid.focus();
    await userEvent.keyboard("{ArrowRight} ");
    await expect(list).toBeChecked();
    await expect(grid).not.toBeChecked();
    await userEvent.keyboard(" ");
    await expect(list).toBeChecked();
  }
</script>

<Story
  name="Exactly one layout remains selected"
  args={{ value: "list" }}
  play={async ({ canvasElement }) => selection(canvasElement)}
/>
<Story
  name="Grid initially selected"
  args={{ value: "grid" }}
  play={async ({ canvasElement }) => selection(canvasElement)}
/>
